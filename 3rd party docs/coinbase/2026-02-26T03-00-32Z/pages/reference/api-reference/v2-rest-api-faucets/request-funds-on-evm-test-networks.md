# request funds on evm test networks

Request funds on EVM test networks

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

The network to request funds from.

Available options

:

`base-sepolia`,

`ethereum-sepolia`,

`ethereum-hoodi`

The address to request funds to, which is a 0x-prefixed hexadecimal string.

Example:

`"0x742d35Cc6634C0532925a3b844Bc454e4438f44e"`

The token to request funds for.

Available options

:

`eth`,

`usdc`,

`eurc`,

`cbbtc`

#### Response

Successfully requested funds.

The hash of the transaction that requested the funds. Note: In rare cases, when gas conditions are unusually high, the transaction may not confirm, and the system may issue a replacement transaction to complete the faucet request. In these rare cases, the `transactionHash` will be out of sync with the actual faucet transaction that was confirmed onchain.

Example:

`"0x742d35Cc6634C0532925a3b844Bc454e4438f44e"`