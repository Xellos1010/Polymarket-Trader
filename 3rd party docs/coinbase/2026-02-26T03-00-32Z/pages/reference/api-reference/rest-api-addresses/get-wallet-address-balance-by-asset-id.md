# get wallet address balance by asset id

Get wallet address balance by asset ID

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the wallet to fetch the balance for.

The onchain address of the address that is being fetched.

The symbol of the asset to fetch the balance for.

#### Response

The balance of an asset onchain

The amount in the atomic units of the asset

An asset onchain scoped to a particular network, e.g. ETH on base-sepolia, or the USDC ERC20 Token on ethereum-mainnet.