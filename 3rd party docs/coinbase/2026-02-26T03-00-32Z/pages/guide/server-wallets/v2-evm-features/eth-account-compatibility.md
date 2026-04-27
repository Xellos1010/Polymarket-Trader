# eth account compatibility

## Overview

The CDP SDK is compatible with the [eth-account](https://github.com/ethereum/eth-account) interface to sign messages, hashes, typed data, and transactions.

## Signer Examples

You can create a CDP Server Account and wrap it in `EvmLocalAccount` which is the class that is compatible with [eth-account signers](https://github.com/ethereum/eth-account/tree/main/eth_account/signers). You can call then `sign*` functions on the resulting account.

```
import asyncio
from cdp import CdpClient
from cdp.evm_local_account import EvmLocalAccount
from eth_account.messages import encode_defunct
from web3 import Web3
import dotenv
dotenv.load_dotenv()
async def main():
    cdp = CdpClient()
    server_account = await cdp.evm.create_account()
    print(f"Successfully created account: {server_account.address}")
    account = EvmLocalAccount(server_account)
    # 1. Sign a message
    message = "Hello, world!"
    signable_message = encode_defunct(text=message)
    signed_message = account.sign_message(signable_message)
    print("Signed message: ", signed_message)
    # 2. Sign a hash
    hash = "0x1234567890123456789012345678901234567890123456789012345678901234"
    signed_hash = account.unsafe_sign_hash(hash)
    print("Signed hash: ", signed_hash)
    # 3. Sign typed data
    typed_data = {
        "domain": {
            "name": "MyDomain",
            "version": "1",
            "chainId": 1,
            "verifyingContract": "0x0000000000000000000000000000000000000000",
        },
        "types": {
            "EIP712Domain": [
                {"name": "name", "type": "string"},
                {"name": "version", "type": "string"},
                {"name": "chainId", "type": "uint256"},
                {"name": "verifyingContract", "type": "address"},
            ],
            "Person": [
                {"name": "name", "type": "string"},
                {"name": "wallet", "type": "address"},
            ],
        },
        "primaryType": "Person",
        "message": {
            "name": "John Doe",
            "wallet": "0x1234567890123456789012345678901234567890",
        },
    }
    signed_typed_data = account.sign_typed_data(
        full_message=typed_data,
    )
    print("Signed typed data: ", signed_typed_data)
    # 4. Sign a transaction
    w3 = Web3(Web3.HTTPProvider("https://sepolia.base.org"))
    nonce = w3.eth.get_transaction_count(account.address)
    transaction = account.sign_transaction(
        transaction_dict={
            "to": "0x000000000000000000000000000000000000dEaD",
            "value": 10000000000,
            "chainId": 84532,
            "gas": 21000,
            "maxFeePerGas": 1000000000,
            "maxPriorityFeePerGas": 1000000000,
            "nonce": nonce,
            "type": "0x2",
        }
    )
    print("Signed transaction: ", transaction)
    await cdp.close()
asyncio.run(main())

```

## Send Transaction Example

Once you have signed your transaction, you can use [web3](https://web3py.readthedocs.io/en/stable/) to send it. Here’s an example:

```
import asyncio
from cdp import CdpClient
from cdp.evm_local_account import EvmLocalAccount
from web3 import Web3
import dotenv
dotenv.load_dotenv()
async def main():
    cdp = CdpClient()
    server_account = await cdp.evm.create_account()
    print(f"Successfully created account: {server_account.address}")
    account = EvmLocalAccount(server_account)
    w3 = Web3(Web3.HTTPProvider("https://sepolia.base.org"))
    nonce = w3.eth.get_transaction_count(account.address)
    transaction = account.sign_transaction(
        transaction_dict={
            "to": "0x000000000000000000000000000000000000dEaD",
            "value": 10000000000,
            "chainId": 84532,
            "gas": 21000,
            "maxFeePerGas": 1000000000,
            "maxPriorityFeePerGas": 1000000000,
            "nonce": nonce,
            "type": "0x2",
        }
    )
    print("Signed transaction: ", transaction)
    print("\nRequesting ETH from faucet for account...")
    faucet_hash = await cdp.evm.request_faucet(
        address=account.address, network="base-sepolia", token="eth"
    )
    w3.eth.wait_for_transaction_receipt(faucet_hash)
    print("Received funds from faucet...")
    tx_hash = w3.eth.send_raw_transaction(transaction.raw_transaction)
    print(f"Transaction sent! Hash: {tx_hash.hex()}")
    await cdp.close()
asyncio.run(main())

```

## What to read next

-   [**v2 Wallet Accounts**](https://developer.chrome.com/server-wallets/v2/introduction/accounts): Read more about the types of accounts and networks we support.
-   [**eth-account**](https://github.com/ethereum/eth-account): Learn more from the eth-account web3 library.