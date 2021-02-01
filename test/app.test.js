const nearAPI = require('near-api-js');
const testUtils = require('./test-utils');
const getConfig = require('../src/config');

const { KeyPair, Account, utils: { format: { parseNearAmount }} } = nearAPI;
const { 
	connection, initContract, getAccount, getContract,
	contractAccount, contractName, contractMethods, createAccessKeyAccount
} = testUtils;
const { GAS } = getConfig();

jasmine.DEFAULT_TIMEOUT_INTERVAL = 50000;

describe('deploy contract ' + contractName, () => {
	let alice, bob, bobPublicKey, implicitAccountId;
    
	const DROP_AMOUNT = 100;

	beforeAll(async () => {
		alice = await getAccount();
		await initContract(alice.accountId);
	});

	// test('contract hash', async () => {
	// 	let state = (await new Account(connection, contractName)).state();
	// 	expect(state.code_hash).not.toEqual('11111111111111111111111111111111');
	// });

	test('check drop', async () => {
		const contract = await getContract(alice);

		await contract.drop({}, GAS);
        
		const accessKeys = await alice.getAccessKeys();
		const balance = await contract.get_balance_dropped({ public_key: accessKeys[0].public_key });
		expect(parseInt(balance, 10)).toEqual(DROP_AMOUNT);
	});

	test('check create with no near', async () => {
		const keyPair = KeyPair.fromRandom('ed25519');
        const public_key = bobPublicKey = keyPair.publicKey.toString();

        // contract owner adding key for drop on server side
        await contractAccount.addKey(public_key, contractName, contractMethods.changeMethods, parseNearAmount('0.1'));

        // get bob account instance for keyPair (acting as accountId === contractName)
		bob = createAccessKeyAccount(keyPair);
		const contract = await getContract(bob);
		await contract.drop({}, GAS);
		const balance = await contract.get_balance_dropped({ public_key });
        expect(parseInt(balance, 10)).toEqual(DROP_AMOUNT);
    });
    


	test('transfer dropped tokens', async () => {
        const contract = await getContract(bob);
        const account_id = alice.accountId
        await contract.transfer({ account_id }, GAS);
        const balance = await contract.get_balance_tokens({ account_id });
		expect(parseInt(balance, 10)).toEqual(DROP_AMOUNT);
	});

});