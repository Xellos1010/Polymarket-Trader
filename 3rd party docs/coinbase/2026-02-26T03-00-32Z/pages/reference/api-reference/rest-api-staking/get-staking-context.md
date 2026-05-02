# get staking context

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

The ID of the blockchain network.

Example:

`"ethereum-mainnet"`

The symbol of the asset being staked.

The onchain address for which the staking context is being fetched

Example:

`"0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a"`

Additional options for getting the staking context. See [here](https://developer.chrome.com/staking/staking-api/introduction/api-usage#staking-options) for detailed options.

#### Response

staking context for an address fetched successfully

Context needed to perform a staking operation