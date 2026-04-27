# evm accounts

The EVM Account APIs enable you to create and use accounts across blockchains that are compatible with the Ethereum Virtual Machine (EVM). An **account** is a private/public key pair that is used to sign transactions and messages. The private key is generated and used only in CDP’s Trusted Execution Environment (TEE), and never exposed to Coinbase or the developer. An EVM account is identified by its **address**, which is a 0x-prefixed hexadecimal string. The same address can be used across multiple EVM networks. Accounts can optionally be assigned an **account name** at creation time for easier identification in subsequent API calls. EVM account names must be globally unique across all EVM accounts in the developer’s CDP Project.

## Authentication

The EVM Account API uses two layers of authentication to ensure the security of your accounts’ private keys:

-   **CDP Secret API Key**: This key is used to authenticate all requests to the entire suite of REST APIs offered on Coinbase Developer Platform.
-   **Wallet Secret**: This secret is used specifically to authenticate sensitive wallet operations to `POST` and `DELETE` endpoints in the EVM and Solana Account APIs.

To learn more about creating and using these keys, visit our [Authentication docs](https://docs.cdp.coinbase.com/api-reference/v2/authentication).