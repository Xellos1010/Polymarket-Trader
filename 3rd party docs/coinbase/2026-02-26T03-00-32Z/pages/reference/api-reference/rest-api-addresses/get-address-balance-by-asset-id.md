# get address balance by asset id

Get address balance by asset ID

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the blockchain network.

The onchain address to fetch the balance for.

The symbol of the asset to fetch the balance for.

#### Response

The balance of the asset in the address

The balance of an asset onchain

The amount in the atomic units of the asset

An asset onchain scoped to a particular network, e.g. ETH on base-sepolia, or the USDC ERC20 Token on ethereum-mainnet.