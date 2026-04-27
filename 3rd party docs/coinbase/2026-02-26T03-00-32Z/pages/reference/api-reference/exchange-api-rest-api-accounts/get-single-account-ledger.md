# get single account ledger

Get a single account's ledger

List account activity of the API key’s profile. Account activity either increases or decreases your account balance.

## API Key Permissions

This endpoint requires either the “view” or “trade” permission.

## Entry Types

Entry type indicates the reason for the account change.

Type

Description

transfer

Funds moved to/from Coinbase to Coinbase Exchange

match

Funds moved as a result of a trade

fee

Fee as a result of a trade

rebate

Fee rebate as per our [fee schedule](https://exchange.coinbase.com/fees)

conversion

Funds converted between fiat currency and a stablecoin

## Details

If an entry is the result of a trade (match, fee), the details field contains additional information about the trade.

Items are paginated and sorted latest first. See [Pagination](https://developer.chrome.com/exchange/rest-api/pagination) for retrieving additional entries after the first page.

## Searching By Date

Searching by start and end dates are inclusive of the time provided and can be combined with before or after fields to narrow down the search to entries from a specific time range. Dates must be after Unix Epoch time and are restricted to the following formats:

-   [RFC3339](https://www.rfc-editor.org/rfc/rfc3339) (i.e., `2006-01-02T15:04:05.000000Z` or `2006-01-02T15:04:05+05:30`)
-   `2006-01-02`
-   `2006-01-02T15:04:05`

A `400 Bad Request` error is returned for any formats that are not accepted.

#### Authorizations

#### Path Parameters

Returns list of ledger entries from this account id.

#### Query Parameters

Search by minimum posted date time and is inclusive of time provided. Valid formats are either RFC3339, date or date time and must be after Unix Epoch time.

Search by maximum posted date time and is inclusive of time provided. Valid formats are either RFC3339, date or date time and must be after Unix Epoch time.

Used for pagination. Sets start cursor to `before` id.

Used for pagination. Sets end cursor to `after` id.

limit

integer<int64>

default:100

Limit on number of results to return.

#### Response

created\_at

string<date-time>

required

type

enum<string>

default:transfer

required

Available options

:

`transfer`,

`match`,

`fee`,

`conversion`,

`margin_interest`,

`rebate`,

`otc_fee`,

`otc_match`,

`tax_credit`,

`rfq_match`,

`rfq_fee`,

`match_conversion`,

`stake_wrap`,

`conversion_fee`,

`redeem`