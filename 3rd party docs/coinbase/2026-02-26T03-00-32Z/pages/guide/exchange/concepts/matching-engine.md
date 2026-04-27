# matching engine

Coinbase Exchange operates a continuous first-come, first-serve order book. Orders are executed in price-time priority as received by the matching engine.

## Self-Trade Prevention

Self-trading is not allowed on Coinbase Exchange. When two orders from the same user cross, they do not fill one another.

You can define your self-trade prevention behavior when [placing an order](https://developer.chrome.com/api-reference/exchange-api/rest-api/orders/create-new-order) with the STP flag:

Self-Trade Prevention Option

STP Flag

Description

Decrement & cancel (default)

`dc`

Cancel smaller order and decrement larger order by the smaller size. If the same size, cancel both.

Cancel oldest

`co`

Cancel older (resting) order in full. Continue to execute the newer taking order.

Cancel newest

`cn`

Cancel newer (taking) order in full. Let the old resting order remain on the order book.

Cancel both

`cb`

Cancel both orders immediately.

## Market Orders

When a `market` order using decrement and cancel (`dc` ) self-trade prevention encounters an open limit order, the behavior depends on which fields were specified for the market order.

-   If `funds` and `size` are specified:
    -   For a market buy order, size is decremented internally within the matching engine and funds remain unchanged. The intent is to offset your target size without limiting your buying power.
-   If `funds` is specified (and not `size`):
    -   For a market buy order, funds are decremented.
    -   For a market sell order, size is decremented when encountering existing limit orders.

## Price Improvement

Orders are matched against existing order book orders at the price of the order *on the book*, not at the price of the taker order. **Example** User A places a buy order for 1 BTC at 100 USD. Then User B places a sell order for 1 BTC at 80 USD. The result is that the trade occurs at 100 USD because User A’s order was first to the trading engine and User A has price priority.

## Order Lifecycle

Order State

Description

`received`

Valid orders that are sent to the matching engine and **confirmed** immediately.

`open`

Any part of the order **not filled** immediately. Orders stay open until canceled or filled by new orders.

`done`

An full order **executed** against another order immediately. A partial order filled or canceled (and no longer eligible for matching)