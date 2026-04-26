# create session token

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Body

Request to create an onramp/offramp session token

Use this parameter to provide the addresses customers funds should be delivered to for this session. Each entry in the record represents a wallet address and the networks it is valid for. There should only be a single address for each network your app supports. Users will be able to buy/sell any asset supported by any of the networks you specify. See the assets param if you want to restrict the available assets.

For example:

Support all assets that are available for sending on the base network, only on the base network: `[{ address: "0x1", blockchains: ["base"] }]`

This optional parameter will restrict the assets available for the user to buy/send. It acts as a filter on the networks specified in the addresses param. Each string in the list should be an asset ticker e.g. `BTC`, `ETH`. The list of available asset tickers can be retrieved from the Buy Options and Sell Options APIs.

For example:

Support only USDC on either the base network or the ethereum network: `addresses: [{ address: "0x1", blockchains: ["base", "ethereum"] }], assets: ["USDC"]`

\[Deprecated\] Please use the addresses and assets params instead. This parameter controls which crypto assets your user will be able to buy/sell, which wallet address their asset will be delivered to, and which networks their assets will be delivered on.

For example:

Support all assets that are available for sending on the base network, only on the base network: `[{ address: "0x1", blockchains: ["base"] }]`

Support only USDC on either the base network or the ethereum network: `[{ address: "0x1", assets: ["USDC"], supportedNetworks: ["base", "ethereum"] }]`

The client IP address of the end user. This parameter ensures the quote can only be used by the requesting user. Do not trust HTTP headers like `X-Forwarded-For` — these can be easily spoofed.

#### Response

Response from the Create Token API

The single use token that can be used to initialize an onramp/offramp session.