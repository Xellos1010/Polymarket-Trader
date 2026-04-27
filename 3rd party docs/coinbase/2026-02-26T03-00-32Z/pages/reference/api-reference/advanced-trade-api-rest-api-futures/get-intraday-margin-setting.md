# get intraday margin setting

Get the futures intraday margin setting

GET

/

api

/

v3

/

brokerage

/

cfm

/

intraday

/

margin\_setting

Get Intraday Margin Setting

```
curl --request GET \
  --url https://api.coinbase.com/api/v3/brokerage/cfm/intraday/margin_setting \
  --header 'Authorization: Bearer <token>'
```

```
{
  "setting": "INTRADAY_MARGIN_SETTING_UNSPECIFIED"
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Response

setting

enum<string>

default:INTRADAY\_MARGIN\_SETTING\_UNSPECIFIED

Available options

:

`INTRADAY_MARGIN_SETTING_UNSPECIFIED`,

`INTRADAY_MARGIN_SETTING_STANDARD`,

`INTRADAY_MARGIN_SETTING_INTRADAY`