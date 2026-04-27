# get current margin window

Get Current Margin Window

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

margin\_profile\_type

enum<string>

default:MARGIN\_PROFILE\_TYPE\_UNSPECIFIED

The margin profile type for your account.

Available options

:

`MARGIN_PROFILE_TYPE_UNSPECIFIED`,

`MARGIN_PROFILE_TYPE_RETAIL_REGULAR`,

`MARGIN_PROFILE_TYPE_RETAIL_INTRADAY_MARGIN_1`

#### Response

is\_intraday\_margin\_killswitch\_enabled

True if intraday margin killswitch is enabled

is\_intraday\_margin\_enrollment\_killswitch\_enabled

True if intraday margin enrollment killswitch is enabled