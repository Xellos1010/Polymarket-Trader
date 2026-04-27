# uselinkapple

```
function useLinkApple(): {
  linkApple: () => Promise<void>;
  oauthState:   | null
     | OAuthFlowState;
};

```

A hook for linking an Apple account to the current user. This is a convenience wrapper around useLinkOAuth for Apple provider.

## 

[​](#returns)

Returns

```
{
  linkApple: () => Promise<void>;
  oauthState:   | null
     | OAuthFlowState;
}

```

An object containing the linkApple function and oauthState.

### 

[​](#linkapple)

linkApple()

```
linkApple: () => Promise<void>;

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