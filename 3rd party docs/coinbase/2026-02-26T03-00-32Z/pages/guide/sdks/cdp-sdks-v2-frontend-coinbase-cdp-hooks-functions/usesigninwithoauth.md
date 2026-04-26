# usesigninwithoauth

```
function useSignInWithOAuth(): {
  signInWithOAuth: (providerType: OAuth2ProviderType) => Promise<void>;
  oauthState:   | null
     | OAuthFlowState;
};

```

Hook that provides access to the OAuth sign-in functionality (Google, Apple, and X are currently supported). This is the first step in the OAuth authentication flow. In a web application, this will redirect the user to the OAuth provider sign in page. This sign in method is not yet supported on mobile.

## Returns

```
{
  signInWithOAuth: (providerType: OAuth2ProviderType) => Promise<void>;
  oauthState:   | null
     | OAuthFlowState;
}

```

### signInWithOAuth()

```
signInWithOAuth: (providerType: OAuth2ProviderType) => Promise<void>;

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
function SignInForm() {
  const { signInWithOAuth } = useSignInWithOAuth();
  const handleSignInWithGoogle = () => {
    void signInWithOAuth("google");
  };
  return (
    <button onClick={handleSignInWithGoogle}>
      Sign in with Google
    </button>
  );
}

```

```
// Apple
function SignInForm() {
  const { signInWithOAuth } = useSignInWithOAuth();
  const handleSignInWithApple = () => {
    void signInWithOAuth("apple");
  };
  return (
    <button onClick={handleSignInWithApple}>
      Sign in with Apple
    </button>
  );
}

```

```
// X
function SignInForm() {
  const { signInWithOAuth } = useSignInWithOAuth();
  const handleSignInWithX = () => {
    void signInWithOAuth("x");
  };
  return (
    <button onClick={handleSignInWithX}>
      Sign in with X
    </button>
  );
}

```