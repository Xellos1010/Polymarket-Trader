# create wallet address

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The ID of the wallet to create the address in.

#### Body

The public key from which the address will be derived.

An attestation signed by the private key that is associated with the wallet. The attestation will be a hex-encoded signature of a json payload with fields `wallet_id` and `public_key`, signed by the private key associated with the public\_key set in the request.

#### Response

The ID of the wallet that owns the address

Example:

`"d91d652b-d020-48d4-bf19-5c5eb5e280c7"`

The ID of the blockchain network.

The public key from which the address is derived.

The onchain address derived on the server-side.

Example:

`"0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a"`