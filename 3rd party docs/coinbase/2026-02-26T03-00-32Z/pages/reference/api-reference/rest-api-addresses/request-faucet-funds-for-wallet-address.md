# request faucet funds for wallet address

Request faucet funds for wallet address

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the wallet the address belongs to.

The onchain address of the address that is being fetched.

#### Response

Successful response for requesting faucet funds.

The faucet transaction

The transaction hash of the transaction the faucet created.

Example:

`"0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42"`

Link to the transaction on the blockchain explorer.

Example:

`"https://sepolia.basescan.org/tx/0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42"`