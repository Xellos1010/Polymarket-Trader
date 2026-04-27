# set intraday margin settings

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Body

setting

enum<string>

default:INTRADAY\_MARGIN\_SETTING\_UNSPECIFIED

The margin setting for the account. Describes whether the account is opted in to receive increased leverage during weekdays (8am-4pm ET), excluding [market holidays](https://www.coinbase.com/derivatives/market-notices).

Available options

:

`INTRADAY_MARGIN_SETTING_UNSPECIFIED`,

`INTRADAY_MARGIN_SETTING_STANDARD`,

`INTRADAY_MARGIN_SETTING_INTRADAY`

#### Response

The response is of type `SetIntradayMarginSettingResponse · object`.