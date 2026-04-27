# tpsl orders

Take Profit Stop Loss (TPSL) orders are supported in FIX with [New Order Single (35=D)](https://developer.chrome.com/international-exchange/fix-api/order-entry-messages#newordersingle-35d) and the [Create order](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/orders/create-order) REST API. TPSL orders allow users to set predefined profit and loss levels simultaneously for their position. When an asset price reaches one of the target prices, the position is closed with a limit order. If one of the orders is triggered, the other order is canceled automatically. An order can only have one TP/SL on one side.

## Parameters

These parameters are required for TPSL orders:

Tag

Name

Type

Required

Description

40

OrdType

Char

Y

Order Type must be `O` (the letter Oh)

44

Price

Decimal

Y

Take profit price (in this context)  
*See Price Rules below.*

99

StopPx

Decimal

Y

Stop loss trigger price

3040

StopLimitPx

Decimal

Y

Limit order price if stop loss triggers

## Caveats

-   Only GTC and GTD are supported for `TimeInForce`
-   The Post-Only tag is not supported. It cannot be populated or must be false.

## Order Replace

-   Order replace is supported with these editable fields:
    -   `Price`
    -   `StopPx`
    -   `StopLimitPx`
    -   `OrderQty`
-   Replacing an TP/SL order is only allowed when none of the take profit or stop loss leg has been triggered.
-   Only one pending replace is allowed for TP/SL orders. A replace request will be rejected if there is already a pending replace request for the same order.

## Samples

### TPSL Sell Order

This TPSL sell order for BTC-USD places a ***live*** sell limit order at price `9785`: `Side=2|Price=9785|StopLimitPx=8245|StopPx=8500`

-   If the price falls below 8500 and no part of this limit order has been filled, it will be repriced from 9785 to 8245.
-   If this limit order is filled or partially filled at 9785, then it will never be repriced.

This differs from regular stop orders where a stop order is NOT live until the price is traded through the StopPx.

### Message Flow

> TPSL Order Example

`-> OrdType=O|Side=2|Price=9785|StopLimitPx=8245|StopPx=8500...`

> Execution Report Example

`<- MsgType=8|OrdType=O|Side=2|Price=9785|StopLimitPx=8245|StopPx=8500...`