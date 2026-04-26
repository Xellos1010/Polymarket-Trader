# uselinkgoogle

```
function useLinkGoogle(): {
  linkGoogle: () => Promise<void>;
  oauthState:   | null
     | OAuthFlowState;
};

```

A hook for linking a Google account to the current user. This is a convenience wrapper around useLinkOAuth for Google provider.

## 

[​](#returns)

Returns

```
{
  linkGoogle: () => Promise<void>;
  oauthState:   | null
     | OAuthFlowState;
}

```

An object containing the linkGoogle function and oauthState.

### 

[​](#linkgoogle)

linkGoogle()

```
linkGoogle: () => Promise<void>;

```

#### 

[​](#returns-2)

Returns

`Promise`<`void`\>

### 

[​](#oauthstate)

oauthState

```
oauthState: 
  | null
  | OAuthFlowState;

```