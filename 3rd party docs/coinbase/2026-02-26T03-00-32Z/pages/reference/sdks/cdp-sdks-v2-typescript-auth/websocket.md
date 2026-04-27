# websocket

## Interfaces

Defined in: [utils/ws.ts:11](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/ws.ts#L11) Options for generating WebSocket authentication headers.

#### Properties

##### apiKeyId

Defined in: [utils/ws.ts:19](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/ws.ts#L19) The API key ID Examples: ‘xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx’ ‘organizations/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/apiKeys/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx’

##### apiKeySecret

Defined in: [utils/ws.ts:28](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/ws.ts#L28) The API key secret Examples: ‘xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx==’ (Edwards key (Ed25519)) ‘-----BEGIN EC PRIVATE KEY-----\\n…\\n…\\n…==\\n-----END EC PRIVATE KEY-----\\n’ (EC key (ES256))

##### source?

Defined in: [utils/ws.ts:33](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/ws.ts#L33) The source identifier for the request

##### sourceVersion?

```
optional sourceVersion: string;

```

Defined in: [utils/ws.ts:38](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/ws.ts#L38) The version of the source making the request

##### expiresIn?

```
optional expiresIn: number;

```

Defined in: [utils/ws.ts:43](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/ws.ts#L43) Optional expiration time in seconds (defaults to 120)

##### audience?

```
optional audience: string[];

```

Defined in: [utils/ws.ts:48](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/ws.ts#L48) Optional audience claim for the JWT

## Functions

```
function getWebSocketAuthHeaders(options: GetWebSocketAuthHeadersOptions): Promise<Record<string, string>>;

```

Defined in: [utils/ws.ts:57](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/ws.ts#L57) Gets authentication headers for a WebSocket connection.

#### Parameters

##### options

[`GetWebSocketAuthHeadersOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/auth/WebSocket#getwebsocketauthheadersoptions) The configuration options for generating WebSocket auth headers

#### Returns

`Promise`<`Record`<`string`, `string`\>> Object containing the authentication headers