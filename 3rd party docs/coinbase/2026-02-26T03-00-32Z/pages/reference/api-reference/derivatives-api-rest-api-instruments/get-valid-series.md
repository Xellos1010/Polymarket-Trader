# get valid series

Retrieves valid series information for instruments based on the provided filter criteria. Returns series data including product code, expiry, contract size, trading dates, and settlement information. Allows optional filtering by product codes, symbols, instrument IDs, trading states, activation date, and expiration date.

#### Body

Optional instrument filter criteria, can be omitted

Available options

:

`PRE_OPEN`,

`OPEN`,

`HALT`,

`PAUSE`,

`CLOSE`,

`PRE_OPEN_NO_CANCEL`,

`EXPIRED`

Example:

`"[PRE_OPEN, EXPIRED]"`

#### Response

Successfully retrieved valid series data