# reference

`GET https://login.coinbase.com/oauth2/auth`

Parameter

Description

`client_id`

Client ID you received after registering your application.

`response_type`

Only current option is `code`

`redirect_uri` *Optional*

URL in your app where users will be sent after authorization (see below). This value needs to be URL encoded. If left out, your application’s first redirect URI will be used by default.

`scope` *Optional*

Comma separated list of permissions (scopes) your application requests access to. Required scopes are listed under endpoints in the [Scopes reference](https://developer.chrome.com/coinbase-app/oauth2-integration/scopes)

`state` *Optional*

An unguessable random string to protect against cross-site request forgery attacks. Must be at least 8 characters long. [Read more about security](https://developer.chrome.com/coinbase-app/authentication-authorization/oauth2/security-best-practices)

Coinbase Business supports many platform specific parameters (in addition to common OAuth2 parameters) which can be used to customize the user experience.

Parameter

Description

`layout` *Optional*

For logged out users, login view is shown by default. You can show the sign up page instead with value `signup`

`referral` *Optional*

Earn a referral bonus from new users who sign up via OAuth2. Value needs to be set to developer’s referral ID (username). [Read more](https://support.coinbase.com/customer/portal/articles/683805-how-does-the-coinbase-referral-program-work-).

## Token

`POST https://login.coinbase.com/oauth2/token` The `token` endpoint can be used to get a new access token after user authorization (`grant_type=authorization_code`) or to refresh an access token (`grant_type=refresh_token`).

### Parameters to Request New Access Tokens

**Request**

Parameter

Description

`grant_type`

**Required** Value `authorization_code`

`code`

**Required** Value which was received from redirect uri

`client_id`

**Required** Client ID you received after registering your application.

`client_secret`

**Required** Client secret you received after registering your application.

`redirect_uri`

**Required** Your application’s redirect URI

**Response**

Parameter

Description

`access_token`

New active access token

`token_type`

Value `bearer`

`expires_in`

Access token expiration in seconds

`refresh_token`

Refresh token which can be used to refresh expired access token

`scope`

List of permissions applied to given access token

### Parameters to Refresh Access Tokens

**Request**

Parameter

Description

`grant_type`

**Required** Value `refresh_token`

`refresh_token`

**Required** Most recent refresh token

`client_id`

**Required** Client ID you received after registering your application.

`client_secret`

**Required** Client secret you received after registering your application.

**Response**

Parameter

Description

`access_token`

New active access token

`token_type`

Value `bearer`

`expires_in`

Access token expiration in seconds

`refresh_token`

New refresh token which can be used to refresh expired access token

`scope`

List of permissions applied to given access token

## Revoke

`POST https://login.coinbase.com/oauth2/revoke` Active access tokens can be revoked at any time. This request needs to be authenticated like any other API request (with the `access_token` parameter, or with the `Authentication` header and bearer token). This request also requires the client ID and secret to be passed in. A `200 OK` is returned for both successful and unsuccessful requests. This can be useful, for example, when implementing log-out feature.

Parameter

Description

`token`

Active access token

`client_id`

**Required** Client ID you received after registering your application.

`client_secret`

**Required** Client secret you received after registering your application.