# get single account hold

Get a single account's holds

This request is paginated. See [Pagination](https://developer.chrome.com/exchange/rest-api/pagination) for more information.

#### Authorizations

#### Path Parameters

#### Query Parameters

Used for pagination. Sets start cursor to `before` date.

Used for pagination. Sets end cursor to `after` date.

limit

integer<int64>

default:100

Limit on number of results to return.

#### Response

created\_at

string<date-time>

required

updated\_at

string<date-time>

required

type

enum<string>

default:order

required

Available options

:

`order`,

`transfer`,

`funding`,

`profile_transfer`,

`otc_order`,

`launch_sell`,

`launch_buy`,

`engine_credit_operation`