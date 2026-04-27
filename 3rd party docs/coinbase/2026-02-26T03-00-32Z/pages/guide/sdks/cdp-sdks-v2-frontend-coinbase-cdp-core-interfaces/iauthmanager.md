# iauthmanager

Interface for auth manager implementations. Defines the public contract that both real and mock implementations must satisfy.

## Methods

### getUser()

```
getUser(): 
  | null
  | User;

```

Gets the current user, or null if there is no user signed in.

#### Returns

| `null` | [`User`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/User)

* * *

### isSignedIn()

```
isSignedIn(): Promise<boolean>;

```

Returns whether the user is signed in.

#### Returns

`Promise`<`boolean`\>

* * *

### signOut()

```
signOut(): Promise<void>;

```

Signs out the user, clearing all authentication state.

#### Returns

`Promise`<`void`\>

* * *

### addAuthStateChangeCallback()

```
addAuthStateChangeCallback(callback: OnAuthStateChangeFn): void;

```

Adds a callback to be called when the auth state changes.

#### Parameters

Parameter

Type

`callback`

[`OnAuthStateChangeFn`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/OnAuthStateChangeFn)

#### Returns

`void`

* * *

### getToken()

```
getToken(options?: {
  forceRefresh?: boolean;
}): Promise<null | string>;

```

Gets the access token, refreshing if needed, or null if the user is not signed in.

#### Parameters

Parameter

Type

Description

`options?`

{ `forceRefresh?`: `boolean`; }

The options for getting the token.

`options.forceRefresh?`

`boolean`

Whether to force a refresh of the token.

#### Returns

`Promise`<`null` | `string`\>

* * *

### getTokenExpiration()

```
getTokenExpiration(): Promise<null | number>;

```

Gets the expiration time of the access token, or null if the user is not signed in.

#### Returns

`Promise`<`null` | `number`\>

* * *

### getWalletSecretId()

```
getWalletSecretId(): Promise<string>;

```

Gets the currently registered wallet secret ID, refreshing if needed.

#### Returns

`Promise`<`string`\> The wallet secret ID.

* * *

### getXWalletAuth()

```
getXWalletAuth(options: {
  requestMethod: string;
  requestHost: string;
  requestPath: string;
  requestData?: Record<string, unknown>;
}): Promise<string>;

```

Gets the X-Wallet-Auth header value. Throws an error if the user is not signed in.

#### Parameters

Parameter

Type

`options`

{ `requestMethod`: `string`; `requestHost`: `string`; `requestPath`: `string`; `requestData?`: `Record`<`string`, `unknown`\>; }

`options.requestMethod`

`string`

`options.requestHost`

`string`

`options.requestPath`

`string`

`options.requestData?`

`Record`<`string`, `unknown`\>

#### Returns

`Promise`<`string`\>

* * *

### getAuthState()

```
getAuthState(): 
  | null
  | AuthState;

```

Gets the authentication state.

#### Returns

| `null` | [`AuthState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/AuthState)

* * *

### setAuthState()

```
setAuthState(authState: AuthState): Promise<void>;

```

Sets the authentication state.

#### Parameters

Parameter

Type

`authState`

[`AuthState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/AuthState)

#### Returns

`Promise`<`void`\>

* * *

### clearAuthState()

```
clearAuthState(): Promise<void>;

```

Clears the authentication state.

#### Returns

`Promise`<`void`\>

* * *

### ensureInitialized()

```
ensureInitialized(): Promise<void>;

```

Awaitable method whose promise only resolves when the auth manager is ready to be used.

#### Returns

`Promise`<`void`\>