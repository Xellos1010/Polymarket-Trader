# get assets by id

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the blockchain network.

The ID of the asset to fetch. This could be a symbol or an ERC20 contract address.

#### Response

An asset onchain scoped to a particular network, e.g. ETH on base-sepolia, or the USDC ERC20 Token on ethereum-mainnet.

The ID of the blockchain network.

The ID for the asset on the network

The number of decimals the asset supports. This is used to convert from atomic units to base units.

The optional contract address for the asset. This will be specified for smart contract-based assets, for example ERC20s.

Example:

`"0x036CbD53842c5426634e7929541eC2318f3dCF7e"`