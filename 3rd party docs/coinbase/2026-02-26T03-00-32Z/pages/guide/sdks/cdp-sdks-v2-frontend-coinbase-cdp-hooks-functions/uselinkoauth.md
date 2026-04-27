# uselinkoauth

```
function useLinkOAuth(): {
  linkOAuth: (providerType: OAuth2ProviderType) => Promise<void>;
  oauthState:   | null
     | OAuthFlowState;
};

```

A hook for linking an OAuth account to the current user.

## Returns

```
{
  linkOAuth: (providerType: OAuth2ProviderType) => Promise<void>;
  oauthState:   | null
     | OAuthFlowState;
}

```

An object containing the linkOAuth function and oauthState.

### linkOAuth()

```
linkOAuth: (providerType: OAuth2ProviderType) => Promise<void>;

```

#### Parameters

Parameter

Type

`providerType`

[`OAuth2ProviderType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/OAuth2ProviderType)

#### Returns

`Promise`<`void`\>

### oauthState

```
oauthState: 
  | null
  | OAuthFlowState;

```

## Examples

```
// Google
function LinkOAuthProvider() {
  const { linkOAuth } = useLinkOAuth();
  const { currentUser } = useCurrentUser();
  const handleLinkGoogle = async () => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      await linkOAuth("google");
    } catch (error) {
      console.error("Failed to link Google account:", error);
    }
  };
  return (
    <button onClick={handleLinkGoogle} disabled={!currentUser}>
      Link Google
    </button>
  );
}

```

```
// Apple
function LinkOAuthProvider() {
  const { linkOAuth } = useLinkOAuth();
  const { currentUser } = useCurrentUser();
  const handleLinkApple = async () => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      await linkOAuth("apple");
    } catch (error) {
      console.error("Failed to link Apple account:", error);
    }
  };
  return (
    <button onClick={handleLinkApple} disabled={!currentUser}>
      Link Apple
    </button>
  );
}

```

```
// X
function LinkOAuthProvider() {
  const { linkOAuth } = useLinkOAuth();
  const { currentUser } = useCurrentUser();
  const handleLinkX = async () => {
    if (!currentUser) {
      console.error("User must be signed in first");
      return;
    }
    try {
      await linkOAuth("x");
    } catch (error) {
      console.error("Failed to link X account:", error);
    }
  };
  return (
    <button onClick={handleLinkX} disabled={!currentUser}>
      Link X
    </button>
  );
}

```