# accept quote

Accepts the quote received by the quote request and creates an order with the provided quote ID.

Always required: portfolio\_id, product\_id, side, quote\_id, client\_quote\_id.

#### Path Parameters

The ID of the portfolio that owns the order

#### Body

-   UNKNOWN\_ORDER\_SIDE: nil value (-- api-linter: core::0126::unspecified=disabled --)
-   BUY: Buy order
-   SELL: Sell order

Available options

:

`BUY`,

`SELL`

A client-generated ID used for reference purposes (note: order will be rejected if this ID is not unique among all currently active orders)

Example:

`"f69a20b1-4ac4-420e-90b5-814a12565bfa"`

A quote id that was returned from the quote request

Example:

`"f69a20b1-4ac4-420e-90b5-814a12565bfa"`

#### Response