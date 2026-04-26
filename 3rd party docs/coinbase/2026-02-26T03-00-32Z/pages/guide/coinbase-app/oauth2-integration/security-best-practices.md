# security best practices

## Storing Credentials Securely

You should take great care to ensure your credentials are stored securely. If someone obtains your `access_token` with the `wallet:transactions:send` permission, s/he will be able to send all the bitcoin, litecoin or ethereum out of your account. You should avoid storing API keys in your code base (which gets added to version control). The recommended best practice is to store them in environment variables. Separating credentials from your code base and database is always good practice. OAuth2 access tokens and refresh tokens should be stored encrypted, with the encryption key stored in environment variables. To increase the security of your OAuth2 implementation, you should always specify a `state` parameter, request moderate [`wallet:transactions:send` limits](https://developer.chrome.com/coinbase-app/oauth2-integration/scopes) and implement [2FA authentication](https://developer.chrome.com/coinbase-app/oauth2-integration/2fa).

## State Variable

To help protect against cross-site request forgery (CSRF), we recommended that you include a state `GET` parameter during the `OAuth2` authorization process. Verifying that this variable matches upon receipt of an authorization code will mitigate CSRF attempts. Make sure that you use a string that is at least 8 characters long. An example of a request with `state` is as follows:

```
https://login.coinbase.com/oauth2/auth?response_type=code&client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_CALLBACK_URL&scope=user+balance&state=CSRFToken123

```

Once user has authorized your application, the same `state` param will be passed back via the redirect url with `code` param. You can read more about it [here](http://homakov.blogspot.com/2012/07/saferweb-most-common-oauth2.html).

## OAuth2 Redirect URI

For added security, all `redirect_uris` must use SSL (i.e. begin with `https://`). URIs without SSL can only be used for development and testing and will not be supported in production.

## PKCE (Proof Key for Code Exchange)

For additional security, you can implement PKCE (Proof Key for Code Exchange) in your OAuth2 flow. PKCE provides protection against authorization code interception attacks, especially important for mobile and single-page applications. To use PKCE:

1.  **Generate a code verifier**: Create a cryptographically random string between 43-128 characters.
2.  **Create a code challenge**: Transform the code verifier using one of the supported methods.
3.  **Include in authorization request**: Add `code_challenge` and `code_challenge_method` parameters to your `/oauth2/auth` request.
4.  **Include in token exchange**: Add the original `code_verifier` to your `/oauth2/token` request.

### Code Challenge Methods

Coinbase supports two methods for generating the code challenge:

#### S256 Method (Recommended)

The `S256` method provides the strongest security by using SHA256 hashing:

-   Hash the code verifier using SHA256
-   Base64url-encode the hash (without padding)
-   This method protects against code verifier exposure even if the authorization request is intercepted

#### Plain Method

The `plain` method uses the code verifier directly as the code challenge:

-   The code challenge equals the code verifier
-   Simpler to implement but provides less security protection
-   Suitable for environments where SHA256 hashing is not available

**Method Selection:**

-   If `code_challenge_method` is not specified, it defaults to `plain`
-   We strongly recommend using `S256` whenever possible for enhanced security
-   The `plain` method should only be used when SHA256 hashing is not feasible in your environment

### PKCE Implementation Examples

#### S256 Method

```
// Generate code verifier
const codeVerifier = generateRandomString(128);
// Generate code challenge using S256
const encoder = new TextEncoder();
const data = encoder.encode(codeVerifier);
const hash = await crypto.subtle.digest("SHA-256", data);
const codeChallenge = base64URLEncode(hash);
// Use in authorization request
const authUrl =
  `https://login.coinbase.com/oauth2/auth?` +
  `response_type=code&` +
  `client_id=${clientId}&` +
  `code_challenge=${codeChallenge}&` +
  `code_challenge_method=S256&` +
  `redirect_uri=${redirectUri}`;

```

#### Plain Method

```
// Generate code verifier
const codeVerifier = generateRandomString(128);
// For plain method, code challenge equals code verifier
const codeChallenge = codeVerifier;
// Use in authorization request (method defaults to 'plain' if not specified)
const authUrl =
  `https://login.coinbase.com/oauth2/auth?` +
  `response_type=code&` +
  `client_id=${clientId}&` +
  `code_challenge=${codeChallenge}&` +
  `code_challenge_method=plain&` +
  `redirect_uri=${redirectUri}`;

```

#### Helper Functions

```
function generateRandomString(length) {
  const charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
  let result = "";
  const values = new Uint8Array(length);
  crypto.getRandomValues(values);
  values.forEach((value) => (result += charset[value % charset.length]));
  return result;
}
function base64URLEncode(buffer) {
  return btoa(String.fromCharCode(...new Uint8Array(buffer)))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=/g, "");
}

```

## Validating SSL Certificates

It is also very important that your application validates our SSL certificate when it connects over `https`. This helps prevent a [man in the middle attack](http://en.wikipedia.org/wiki/Man-in-the-middle_attack). If you are using a client library, this may be turned on by default, but you should confirm this. Anytime you see a setting to ‘verify SSL’ you should ensure it is set to true.