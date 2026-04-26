# get single account transfer

Get a single account's transfers

#### Authorizations

#### Path Parameters

Returns list of transfers from this account id.

#### Query Parameters

Used for pagination. Sets start cursor to `before` date.

Used for pagination. Sets end cursor to `after` date.

limit

integer<int64>

default:100

Limit on number of results to return.

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