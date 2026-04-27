# useoauthstate

```
function useOAuthState(): {
  oauthState:   | null
     | OAuthFlowState;
};

```

Hook to access the current OAuth flow state. This can be used to track the status of an ongoing OAuth authentication flow.

## Returns

```
{
  oauthState:   | null
     | OAuthFlowState;
}

```

The current OAuth flow state

### oauthState

```
oauthState: 
  | null
  | OAuthFlowState;

```

## Example

```
function OAuthStatus() {
  const { oauthState } = useOAuthState();
  if (!oauthState) {
    return null;
  }
  return (
    <div>
      <p>OAuth Status: {oauthState.status}</p>
      {oauthState.status === "error" && (
        <p>Error: {oauthState.errorDescription}</p>
      )}
    </div>
  );
}

```