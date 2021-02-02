import React, {useEffect, useState} from 'react';
import * as nearAPI from 'near-api-js';
import { GAS, parseNearAmount } from '../state/near';
import { 
	createAccessKeyAccount,
	getContract,
} from '../utils/near-utils';

const {
	KeyPair,
	utils: { format: { formatNearAmount } }
} = nearAPI;

export const Contract = ({ near, update, localKeys = {}, account }) => {
	if (!localKeys || !localKeys.accessPublic) return null;

	const [balanceDropped, setBalanceDropped] = useState('0');
	const [balanceTokens, setBalanceTokens] = useState('0');
	const [receiver, setReceiver] = useState('');
    
	useEffect(() => {
		if (!localKeys.accessPublic) return;
		checkDrop();
	}, [localKeys.accessPublic]);


	const checkDrop = async () => {
		const contract = getContract(createAccessKeyAccount(near, KeyPair.fromString(localKeys.accessSecret)));
		setBalanceDropped(await contract.get_balance_dropped({ public_key: localKeys.accessPublic }));
	};
    
	const checkReceiver = async () => {
		const contract = getContract(createAccessKeyAccount(near, KeyPair.fromString(localKeys.accessSecret)));
		setBalanceTokens(await contract.get_balance_tokens({ account_id: receiver }));
	};

	const handleClaimDrop = async () => {
		const contract = getContract(createAccessKeyAccount(near, KeyPair.fromString(localKeys.accessSecret)));
		try {
			await contract.drop({}, GAS);
		} catch (e) {
			if (!/Tokens already dropped/.test(e.toString())) {
				throw e;
			}
			alert('Tokens already dropped');
		}
		checkDrop();
	};

	const handleTransfer = async () => {
		if (!receiver.length) {
			alert('set a receiver');
			return;
		}
		const contract = getContract(createAccessKeyAccount(near, KeyPair.fromString(localKeys.accessSecret)));
		try {
			await contract.transfer({ account_id: receiver}, GAS);
		} catch (e) {
			if (!/No tokens/.test(e.toString())) {
				throw e;
			}
			alert('No tokens');
		}
		checkDrop();
		checkReceiver();
	};

	return <>
		<h3>Social Token Drop Zone</h3>
		<p>Dropped Tokens to App Key: { balanceDropped }</p>
		<button onClick={() => handleClaimDrop()}>Claim Drop</button>
		<button onClick={() => handleTransfer()}>Transfer Dropped Tokens to AccountId</button>
		<br />
		<p>Receiver Tokens: { balanceTokens }</p>
		<input value={receiver} onChange={(e) => setReceiver(e.target.value)} placeholder="AccountId of Receiver" />
		<br />
		<button onClick={() => checkReceiver()}>Check Receiver Balance</button>
	</>;
};

