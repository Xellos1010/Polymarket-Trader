# signinwithoauth

```
function signInWithOAuth(providerType: OAuth2ProviderType): Promise<void>;

```

Initiates the sign in flow with an OAuth provider (Google, Apple, and X are currently supported). In a web application, this will redirect the user to the OAuth provider’s sign in page. This sign in method is not yet supported on mobile.

## Parameters

Parameter

Type

Description

`providerType`

[`OAuth2ProviderType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/OAuth2ProviderType)

The type of OAuth provider to sign in with.

## Returns

`Promise`<`void`\> The result of the sign in.

## Examples

```
// Google
try {
 void signInWithOAuth("google");
} catch (error) {
  console.error("Failed to sign in with Google:", error);
}

```

```
// Apple
try {
 void signInWithOAuth("apple");
} catch (error) {
 console.error("Failed to sign in with Apple:", error);
}

```

```
// X
try {
 void signInWithOAuth("x");
} catch (error) {
 console.error("Failed to sign in with X:", error);
}

```