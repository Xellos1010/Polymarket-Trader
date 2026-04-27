# get product trades

## Side

The `side` of a trade indicates the maker order side. The maker order is the order that was open on the order book. A `buy` side indicates a down-tick because the maker was a buy order and their order was removed. A `sell` side indicates an up-tick.

This request is paginated. See [Pagination](https://developer.chrome.com/exchange/rest-api/pagination) for more information.

#### Path Parameters

list trades for specific product.

#### Query Parameters

limit

integer<int64>

default:1000

Limit on number of results to return.

Used for pagination. Sets start cursor to `before` id.

Used for pagination. Sets end cursor to `after` id.

#### Response

side

enum<string>

default:buy

required

Available options

:

`buy`,

`sell`

time

string<date-time>

required