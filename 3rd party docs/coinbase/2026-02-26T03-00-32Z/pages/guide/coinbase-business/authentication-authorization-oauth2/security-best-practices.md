# security best practices

## Storing Credentials Securely

You should take great care to ensure your credentials are stored securely. If someone obtains your `access_token` with the `wallet:transactions:send` permission, s/he will be able to send all the bitcoin, litecoin or ethereum out of your account. You should avoid storing API keys in your code base (which gets added to version control). The recommended best practice is to store them in environment variables. Separating credentials from your code base and database is always good practice. OAuth2 access tokens and refresh tokens should be stored encrypted, with the encryption key stored in environment variables. To increase the security of your OAuth2 implementation, you should always specify a `state` parameter, request moderate [`wallet:transactions:send` limits](https://developer.chrome.com/coinbase-app/authentication-authorization/oauth2/scopes) and implement [2FA authentication](https://developer.chrome.com/coinbase-app/authentication-authorization/oauth2/2fa).

## State Variable

To help protect against cross-site request forgery (CSRF), we recommended that you include a state `GET` parameter during the `OAuth2` authorization process. Verifying that this variable matches upon receipt of an authorization code will mitigate CSRF attempts. Make sure that you use a string that is at least 8 characters long. An example of a request with `state` is as follows:

```
https://login.coinbase.com/oauth2/auth?response_type=code&client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_CALLBACK_URL&scope=user+balance&state=CSRFToken123

```

Once user has authorized your application, the same `state` param will be passed back via the redirect url with `code` param. You can read more about it [here](http://homakov.blogspot.com/2012/07/saferweb-most-common-oauth2.html).

## OAuth2 Redirect URI

For added security, all `redirect_uris` must use SSL (i.e. begin with `https://`). URIs without SSL can only be used for development and testing and will not be supported in production.

## Validating SSL Certificates

It is also very important that your application validates our SSL certificate when it connects over `https`. This helps prevent a [man in the middle attack](http://en.wikipedia.org/wiki/Man-in-the-middle_attack). If you are using a client library, this may be turned on by default, but you should confirm this. Anytime you see a setting to ‘verify SSL’ you should ensure it is set to true.