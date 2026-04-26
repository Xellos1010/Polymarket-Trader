# get all transfers

#### Authorizations

#### Query Parameters

Returns list of transfers from this portfolio id.

Used for pagination. Sets start cursor to `before` date.

Used for pagination. Sets end cursor to `after` date.

Limit on number of results to return.

type

enum<string>

default:deposit

Filter results by a specific transfer type. Internal transfer types represent transfers made between the user's profiles. Internal transfer types are excluded from the response when this field is not set

Available options

:

`deposit`,

`withdraw`,

`internal_deposit`,

`internal_withdraw`

Filter results by type of currency. Possible values: \[`crypto`, `fiat`\]

Filter results by reason of transfer. Possible values: \[`usdc_reward`\]

Filter results by currency.

#### Response

type

enum<string>

default:deposit

required

Available options

:

`deposit`,

`withdraw`,

`internal_deposit`,

`internal_withdraw`

created\_at

string<date-time>

required

completed\_at

string<date-time>

required

canceled\_at

string<date-time>

required

processed\_at

string<date-time>

required