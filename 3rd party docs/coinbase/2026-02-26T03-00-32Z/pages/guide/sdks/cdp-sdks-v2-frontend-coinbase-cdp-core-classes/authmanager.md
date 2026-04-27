# authmanager

Manages the authentication state and access token for standard CDP authentication (email/SMS/OAuth flows).

## Implements

-   [`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager)

## Constructors

### Constructor

```
new AuthManager(projectId: string): AuthManager;

```

Initializes the token manager.

#### Parameters

Parameter

Type

Description

`projectId`

`string`

The project ID.

#### Returns

`AuthManager`

## Methods

### getUser()

```
getUser(): 
  | null
  | User;

```

Gets the current user, or null if there is no user signed in.

#### Returns

| `null` | [`User`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/User) The current user.

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`getUser`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#getuser)

* * *

### isSignedIn()

```
isSignedIn(): Promise<boolean>;

```

Returns whether the user is signed in - i.e., whether there is an unexpired access token and user. Attempts to refresh the token if it’s expired.

#### Returns

`Promise`<`boolean`\> True if the user is signed in, false otherwise.

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`isSignedIn`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#issignedin)

* * *

### signOut()

```
signOut(): Promise<void>;

```

Signs out the user, clearing all authentication state.

#### Returns

`Promise`<`void`\>

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`signOut`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#signout)

* * *

### addAuthStateChangeCallback()

```
addAuthStateChangeCallback(callback: OnAuthStateChangeFn): void;

```

Adds a callback to be called when the auth state changes.

#### Parameters

Parameter

Type

Description

`callback`

[`OnAuthStateChangeFn`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/OnAuthStateChangeFn)

The function to call when the auth state changes.

#### Returns

`void`

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`addAuthStateChangeCallback`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#addauthstatechangecallback)

* * *

### getToken()

```
getToken(options?: {
  forceRefresh?: boolean;
}): Promise<null | string>;

```

Gets the access token, refreshing it if it is expired. Returns null if the user is not signed in.

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

`Promise`<`null` | `string`\> The access token.

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`getToken`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#gettoken)

* * *

### getTokenExpiration()

```
getTokenExpiration(): Promise<null | number>;

```

Gets the expiration time of the access token, or null if the user is not signed in.

#### Returns

`Promise`<`null` | `number`\> The expiration time of the access token.

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`getTokenExpiration`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#gettokenexpiration)

* * *

### getWalletSecretId()

```
getWalletSecretId(): Promise<string>;

```

Gets the currently registered wallet secret ID. Rejects if the user is not signed in.

#### Returns

`Promise`<`string`\> The wallet secret ID.

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`getWalletSecretId`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#getwalletsecretid)

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

Gets the X-Wallet-Auth header. Rejects if the user is not signed in.

#### Parameters

Parameter

Type

Description

`options`

{ `requestMethod`: `string`; `requestHost`: `string`; `requestPath`: `string`; `requestData?`: `Record`<`string`, `unknown`\>; }

The options for the request.

`options.requestMethod`

`string`

The HTTP method of the request.

`options.requestHost`

`string`

The host of the request.

`options.requestPath`

`string`

The path of the request.

`options.requestData?`

`Record`<`string`, `unknown`\>

The data of the request.

#### Returns

`Promise`<`string`\> The X-Wallet-Auth header.

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`getXWalletAuth`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#getxwalletauth)

* * *

### getAuthState()

```
getAuthState(): 
  | null
  | AuthState;

```

Gets the authentication state.

#### Returns

| `null` | [`AuthState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/AuthState) The authentication state.

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`getAuthState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#getauthstate)

* * *

### setAuthState()

```
setAuthState(authState: AuthState): Promise<void>;

```

Sets the authentication state.

#### Parameters

Parameter

Type

Description

`authState`

[`AuthState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/AuthState)

The authentication state.

#### Returns

`Promise`<`void`\>

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`setAuthState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#setauthstate)

* * *

### clearAuthState()

```
clearAuthState(): Promise<void>;

```

Clears the authentication state.

#### Returns

`Promise`<`void`\>

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`clearAuthState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#clearauthstate)

* * *

### ensureInitialized()

```
ensureInitialized(): Promise<void>;

```

Ensures the AuthManager is initialized before proceeding. If initialization is already in progress, waits for it to complete.

#### Returns

`Promise`<`void`\>

#### Implementation of

[`IAuthManager`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager).[`ensureInitialized`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager#ensureinitialized)

* * *

### \_doInitialize()

```
protected _doInitialize(): Promise<void>;

```

Internal async initialization logic.

#### Returns

`Promise`<`void`\>

* * *

### shouldRefreshToken()

```
protected shouldRefreshToken(): boolean;

```

Returns whether the token should be refreshed.

#### Returns

`boolean` True if the token should be refreshed, false otherwise.

* * *

### refreshAccessToken()

```
protected refreshAccessToken(): Promise<void>;

```

Refreshes the access token and transitions the auth state accordingly. If a refresh is already in progress, it will wait for that refresh to complete. Retries with exponential backoff on transient failures only. Unauthorized error short-circuit immediately since retrying won’t help.

#### Returns

`Promise`<`void`\> The new access token.

* * *

### scheduleTokenRefresh()

```
protected scheduleTokenRefresh(): void;

```

Schedules a token refresh to occur exactly when shouldRefreshToken() would return true. Uses the same REFRESH\_CREDENTIALS\_BUFFER\_MS timing as the rest of the auth system.

#### Returns

`void`

* * *

### cancelTokenRefresh()

```
protected cancelTokenRefresh(): void;

```

Cancels any scheduled token refresh.

#### Returns

`void`

## Properties

Property

Modifier

Type

Default value

`projectId`

`protected`

`string`

`undefined`

`authState`

`protected`

| `null` | [`AuthState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/AuthState)

`null`

`walletSecret`

`protected`

| `null` | [`WalletSecret`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/WalletSecret)

`null`

`authStateChangeCallbacks`

`protected`

[`OnAuthStateChangeFn`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/OnAuthStateChangeFn)\[\]

`[]`

`initPromise`

`protected`

`null` | `Promise`<`void`\>

`null`

`refreshTimeout`

`protected`

`null` | `Timeout`

`null`