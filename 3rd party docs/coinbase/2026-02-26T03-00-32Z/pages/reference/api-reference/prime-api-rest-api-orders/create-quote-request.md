# create quote request

A Quote Request is the start of the RFQ process. Coinbase Prime sends a Quote Request to Liquidity Providers (LPs) on behalf of a customer looking to participate in an RFQ trade.

Always required: portfolio\_id, product\_id, side, client\_quote\_id, and limit\_price. One of either base\_quantity or quote\_value is always required.

#### Path Parameters

The ID of the portfolio that owns the order

#### Body

based off PostOrderPreviewRequest

-   UNKNOWN\_ORDER\_SIDE: nil value (-- api-linter: core::0126::unspecified=disabled --)
-   BUY: Buy order
-   SELL: Sell order

Available options

:

`BUY`,

`SELL`

A client-generated order ID used for reference purposes (note: order will be rejected if this ID is not unique among all currently active orders)

Example:

`"f69a20b1-4ac4-420e-90b5-814a12565bfa"`

#### Response