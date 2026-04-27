# smart wallets

A Smart Wallet is an ERC-4337 compatible account abstraction wallet. The Smart Wallet uses the same [smart contract](https://github.com/coinbase/smart-wallet/blob/main/src/CoinbaseSmartWallet.sol) as the [Frontend SDK](https://docs.base.org/identity/smart-wallet/introduction/install-web). This new feature is currently in beta. The Server Wallet supports creation of SmartWallets as a backend wallet, offering features like paymaster for sponsoring gas and batch transactions.

A Smart Wallet is a single address that works across EVM networks. For now, only Base Mainnet and Base Sepolia are supported. As we introduce new networks, existing SmartWallets will have the same address and be automatically supported. A SmartWallet has a single owner, which is the account backed by a private key that signs transactions for the wallet. Think of a private key as the password to a wallet. For more, see [What is a private key?](https://www.coinbase.com/learn/crypto-basics/what-is-a-private-key).

## Creating a Smart Wallet

-   Typescript
    
-   Python
    

We recommend using [Viem](https://viem.sh/) to create the private key and owner account of the SmartWallet.Refer to the [SmartWallet SDK docs](https://coinbase.github.io/coinbase-sdk-nodejs/types/wallets_types.SmartWallet.html) for a full list of supported methods.

```
import { generatePrivateKey, privateKeyToAccount } from "viem/accounts";
import { createSmartWallet } from "@coinbase/coinbase-sdk";
import { Coinbase } from "@coinbase/coinbase-sdk";
// This key must be stored securely and persisted across sessions! See Securing a Smart Wallet below for more information.
const privateKey = generatePrivateKey();
const owner = privateKeyToAccount(privateKey);
const smartWallet = await createSmartWallet({
    signer: owner
});
// Get the smart wallet address
const smartWalletAddress = smartWallet.address;

```

We recommend using [eth-account](https://github.com/ethereum/eth-account) to create the private key and owner account of the SmartWallet.Refer to the [SmartWallet class SDK docs](https://coinbase.github.io/cdp-sdk-python/cdp.html#cdp.SmartWallet) for a full list of supported methods.

```
from eth_account import Account
from cdp import *
# This key must be stored securely and persisted across sessions! See Securing a Smart Wallet below for more information.
private_key = Account.create().key
owner = Account.from_key(private_key)
smart_wallet = SmartWallet.create(account=owner)
# Get the smart wallet address
smart_wallet_address = smart_wallet.address

```

## Sending a User operation

A UserOperation is used to execute transactions through a Smart Wallet, using account abstraction to enable features like batch transactions and sponsored gas fees without needing a traditional EOA wallet.

-   Typescript
    
-   Python
    

`sendUserOperation` is how you send operations for the SmartWallet. It supports batch transactions and paymaster for sponsoring.For more on Paymaster, see the [Paymaster docs](https://developer.chrome.com/paymaster/introduction/welcome).You can either provide an ABI and the function to call, or the encoded function data. The next example demonstrates a batch transaction that sends ETH to a destination address and calls a function on a contract.

```
const smartWallet = await createSmartWallet({
    signer: owner
});
const ABI = [
  {
    inputs: [
      { internalType: "uint256", name: "a", type: "uint256" },
      { internalType: "bool", name: "b", type: "bool" },
      { internalType: "address", name: "c", type: "address" },
    ],
    name: "someFunction",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
] as const; // This "as const" is very important!
const userOperation = await smartWallet.sendUserOperation({
  calls: [
    {
      to: "0x1234567890123456789012345678901234567890",
      value: parseEther("0.0000005"),
      data: "0x",
    },
    {
      to: "0xb720E683CB90838F23F66a37Adb26c24e04D1b60",
      abi: ABI,
      functionName: "someFunction",
      args: [123n, true, "0x3234567890123456789012345678901234567890"],
    },
  ],
  chainId: 84532,
});
const userOperationResult = await waitForUserOperation(userOperation);
// You can now use the result to see the status and get the transactionHash if it was successful.

```

For ABIs, make sure to include as const as this is needed by Typescript for proper compilation.

`send_user_operation` is how you send operations for the SmartWallet. It supports batch transactions and paymaster for sponsoring.For more on Paymaster, see the [Paymaster docs](https://developer.chrome.com/paymaster/introduction/welcome).You can either provide an ABI and the function to call, or the encoded function data. The next example demonstrates a batch transaction that sends ETH to a destination address and calls a function on a contract.

```
smart_wallet = SmartWallet.create(account=owner)
ABI = [
  {
      "inputs": [
          {"internalType": "uint256", "name": "a", "type": "uint256"},
          {"internalType": "bool", "name": "b", "type": "bool"},
          {"internalType": "address", "name": "c", "type": "address"},
      ],
      "name": "someFunction",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function",
  },
]
value = Web3.to_wei(Decimal("0.0000005"), "ether")
user_operation = smart_wallet.send_user_operation(
    calls=[
        FunctionCall(
            to="0xb720E683CB90838F23F66a37Adb26c24e04D1b60",
            abi=ABI,
            function_name="someFunction",
            args=[123, True, "0x3234567890123456789012345678901234567890"],
        ),
        EncodedCall(to="0x1234567890123456789012345678901234567890", data="0x", value=value),
    ],
    chain_id=84532,
)
user_operation.wait(interval_seconds=0.2, timeout_seconds=20)
# You can now check the status of the operation and get the transaction hash if it was successful.

```

### Paymaster

For Base Sepolia, the CDP API automatically sponsors gas for the SmartWallet, so you don’t need any gas to send user operations! For Base Mainnet, you can pass in a Paymaster URL to the `sendUserOperation` function to sponsor the gas for the operation. Check out the [Paymaster docs](https://developer.chrome.com/paymaster/introduction/welcome) for more information and getting started with your own Paymaster.

## Securing a Smart Wallet

The private key for the owner account must be securely stored. It is your responsibility as the developer to securely store this key. For example, you may choose to store this data in an encrypted database.

### Persisting Locally

Here’s an example of how to store a private key locally. To save your private key, run the following:

In production, developers should encrypt the private key!

### Re-instantiating a Smart Wallet

The private key and the smart wallet address are required to re-instantiate a wallet when a new session is started. The following code demonstrates how to import the data required to re-instantiate a wallet.

You can now use the `smart wallet` for all your operations.