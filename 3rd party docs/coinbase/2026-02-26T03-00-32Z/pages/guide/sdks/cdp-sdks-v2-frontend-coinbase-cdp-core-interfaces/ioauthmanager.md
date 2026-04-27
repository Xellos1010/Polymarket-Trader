# ioauthmanager

Interface for OAuth manager implementations.

## Methods

### getOAuthFlowState()

```
getOAuthFlowState(): Promise<
  | null
| OAuthFlowState>;

```

Gets the OAuth flow state.

#### Returns

`Promise`< | `null` | [`OAuthFlowState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/OAuthFlowState)\>

* * *

### setOAuthFlowState()

```
setOAuthFlowState(oauthFlowState: OAuthFlowState): Promise<void>;

```

Sets the OAuth flow state.

#### Parameters

Parameter

Type

`oauthFlowState`

[`OAuthFlowState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/OAuthFlowState)

#### Returns

`Promise`<`void`\>

* * *

### addOAuthStateChangeCallback()

```
addOAuthStateChangeCallback(callback: OnOAuthStateChangeFn): Promise<void>;

```

Adds a callback to be called when the OAuth state changes.

#### Parameters

Parameter

Type

`callback`

[`OnOAuthStateChangeFn`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/OnOAuthStateChangeFn)

#### Returns

`Promise`<`void`\>

* * *

### clearOAuthFlowState()

```
clearOAuthFlowState(): Promise<void>;

```

Clears the OAuth flow state.

#### Returns

`Promise`<`void`\>

* * *

### handleOAuthCode()

```
handleOAuthCode(url?: string): Promise<void>;

```

Awaitable method whose promise only resolves when the OAuth manager is ready to be used.

#### Parameters

#### Returns

`Promise`<`void`\>