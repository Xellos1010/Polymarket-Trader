# tpsl orders

Take Profit Stop Loss (TPSL) orders are supported in [FIX 5.0](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#newordersingle-35=d) with the [Create a new order](https://developer.chrome.com/api-reference/exchange-api/rest-api/orders/create-new-order) REST API. TPSL orders allow users to set predefined profit and loss levels simultaneously for their position. When an asset price reaches one of the target prices, the position is closed with a limit order. If one of the orders is triggered, the other order is canceled automatically. An order can only have one TP/SL on one side.

## NewOrderSingle (35=D)

These parameters are required for TPSL orders in addition to the standard [NewOrderSingle](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#newordersingle-35=d) fields:

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

### Caveats

-   Only GTC and GTD are supported for `TimeInForce`
-   TPSL orders cannot be submitted during auction mode.
-   The Post-Only tag is not supported. It cannot be populated or must be false.
-   Batch orders are not supported.

### Samples

#### TPSL Sell Order

This TPSL sell order for BTC-USD places a ***live*** sell limit order at price 9785 `Side=2|Price=9785|StopLimitPx=8245|StopPx=8500`

-   If the price falls below 8500 and no part of this limit order has been filled, it will be repriced from 9785 to 8245.
-   If this limit order is filled or partially filled at 9785, then it will never be repriced.

This differs from regular stop orders where a stop order is NOT live until the price is traded through the StopPx. The direction of the trigger is determined by the Side of the order and thus, `TriggerPriceDirection` is NOT accepted.

#### Message Flow

> TPSL Order Example

`-> OrdType=O|Side=2|Price=9785|StopLimitPx=8245|StopPx=8500...`

> Execution Report Example

`<- MsgType=8|OrdType=O|Side=2|Price=9785|StopLimitPx=8245|StopPx=8500...`

> Repricing Example

If the stop is triggered and the order is repriced, an ExecutionReport is returned with a RestatementReason of `3` for “Repricing of order”: `<- MsgType=8|OrdType=O|ExecType=D|ExecRestatementReason=3|Price=8245...`

#### WebSocket User Channel

If the stop is triggered and the order is repriced, a change message is published, for example:

```
{
    "new_price": "8245",
    "order_id": "...",
    "type": "change",
    "side": "sell",
    "old_price": "9785",
    "reason": "tpsl_triggered"
}

```

Only the user channel with your TPSL order displays with `tpsl_triggered`. Other clients will receive change reason, `modify_order`.

## OrderCancelReplaceRequest (35=G)

TPSL orders can be modified using [OrderCancelReplaceRequest](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordercancelreplacerequest-35=g). These parameters are specific to TPSL order modifications in addition to the standard [OrderCancelReplaceRequest](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordercancelreplacerequest-35=g) fields. The modified `Price`, `StopPx`, and `StopLimitPx` values must adhere to the [Price Rules](#newordersingle-35=d) defined above.

Tag

Name

Type

Req

Description

40

OrdType

Char

Y

Must be `O` (TPSL).

44

Price

Decimal

Y

The new take profit price.

99

StopPx

Decimal

Y

The new stop loss trigger price.

3040

StopLimitPx

Decimal

Y

The new limit order price if stop loss triggers.