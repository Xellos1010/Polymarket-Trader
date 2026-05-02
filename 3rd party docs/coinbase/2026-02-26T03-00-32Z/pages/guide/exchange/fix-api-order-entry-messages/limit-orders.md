# limit orders

Limit With Funds (LWF) orders allow users to fully execute an order up to a notional value specified in the product quote currency.

## Parameters

Tag

Name

Type

Required

Description

40

OrdType

Char

Y

Order Type must be `2` (Limit)

152

CashOrderQty

Decimal

Y

The notional value you wish to trade

All values for `TimeInForce`, `SelfTradePrevention`, and `PostOnly` are supported.

## Summary

Today, if you want to trade $30,000 worth of BTC, you might submit a market order: `OrdType=1|Side=1|CashOrderQty=30000` However, a market order leaves the executed price out of the client’s control. With the following market state:

Bid Size

Bid

Ask

Ask Size

60,000

10

10

59,000

You can try to trade $30,000 of BTC by placing a limit buy order. To do this you must perform the `notional/price` division to calculate the necessary `OrderQty`. The resulting order would look like: `OrdType=2|Side=1|Price=60000|OrderQty=0.5` If this order is filled at $2,000, then everything works as intended. In actuality, the market may change from the time in between when the user submits the order and when it is received by the exchange. To illustrate, consider the case where the market state has changed to the following at the time when the exchange receives your order:

Bid Size

Bid

Ask

Ask Size

59,000

10

10

58,999

Your order now fills at 59,000forquantity‘0.5‘.Theresultingnotionalforthistradeis59,000 for quantity \`0.5\`. The resulting notional for this trade is 29,500, which is less than what you originally wanted. To solve this issue, you can specify `CashOrderQty` instead of `OrderQty`: `OrdType=2|Side=1|Price=60000|CashOrderQty=30000` Given the same market state:

Bid Size

Bid

Ask

Ask Size

59,000

10

10

58,999

The order will be filled as taker with quantity `0.50847457`, making the total notional of this order $29,999.9996. In a separate example, with the following market state:

Bid Size

Bid

Ask

Ask Size

60,500

1

59,500

0.3

10

59,999

Your order will fill as a taker at price 59,500withquantity‘0.3‘.Thusonly59,500 with quantity \`0.3\`. Thus only 17850 notional will be filled. The remaining 12,150notionalwillthenrestonthebookatprice12,150 notional will then rest on the book at price 60,000 with quantity `0.2025`.

## Caveats

-   Orders may be filled at less than the notional specified due to fees and truncation of product base increment.
-   [TPSL](https://developer.chrome.com/exchange/fix-api/order-entry-messages/tpsl-orders) / [Stop](https://developer.chrome.com/api-reference/exchange-api/rest-api/orders/cancel-all-orders) / [Batch](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#neworderbatch-35u6) orders are not be supported with this feature.
-   [OrderCancelReplaceRequest](https://developer.chrome.com/exchange/fix-api/order-entry-messages/order-entry-messages5#ordercancelreplacerequest-35g) with `CashOrderQty` is not supported.

## Market Data

-   `OrderQty`/`CumQty`/`LeavesQty` is supplied in all Execution Reports. In particular `OrderQty` is calculated after the order has been processed as a taker (i.e., it is **NOT** calculated with `CashOrderQty / Price`), such that all quantity tags are consistent with one another.
-   `Size` field is populated in all WebSocket feed messages. No changes are expected from the perspective of a WebSocket consumer.

## Examples

In the following examples, all irrelevant tags are omitted.

Example 1 - A buy GTC order partially fills before resting

```
$> -> BeginString=FIX.4.2 MsgType=ORDER_SINGLE OrdType=LIMIT Price=2952 Side=BUY Symbol=ETH-USD TimeInForce=GOOD_TILL_CANCEL CashOrderQty=1000 CheckSum=026
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT CumQty=0 ExecID=07debc62-aa75-4734-b1b5-91eb9ce8d49c ExecTransType=NEW OrderID=fa5d17b0-8344-4237-ab33-8c658595f960 OrderQty=0.33740512 OrdStatus=NEW Price=2952 Side=BUY Symbol=ETH-USD TransactTime=20240529-20:59:24.039 ExecType=NEW LeavesQty=0.33740512 CashOrderQty=996.01591424 CheckSum=091

```

Note that the value of `CashOrderQty` in the ExecutionReport is not $1,000 due to fees and base increment truncation.

```
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT AvgPx=2951.81 CumQty=0.01 ExecID=d7b9d643-5567-5ba1-ae59-eca93bc15723 ExecTransType=NEW LastPx=2951.81 LastShares=0.01 OrderID=fa5d17b0-8344-4237-ab33-8c658595f960 OrderQty=0.33740512 OrdStatus=PARTIALLY_FILLED Price=2951.81 Side=BUY Symbol=ETH-USD TransactTime=20240529-20:59:24.039 ExecType=PARTIAL_FILL LeavesQty=0.32740512 1003=1267476 AggressorIndicator=YES
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT AvgPx=2951.8666666666666667 CumQty=0.03 ExecID=83a0385b-96f4-532b-a704-cc8fbcbfecd8 ExecTransType=NEW LastPx=2951.92 LastShares=0.006 OrderID=fa5d17b0-8344-4237-ab33-8c658595f960 OrderQty=0.33740512 OrdStatus=PARTIALLY_FILLED Price=2951.92 Side=BUY Symbol=ETH-USD TransactTime=20240529-20:59:24.039 NoMiscFees=1 MiscFeeAmt=0.004 MiscFeeCurr=USD MiscFeeType=EXCHANGE_FEES ExecType=PARTIAL_FILL LeavesQty=0.30740512 MiscFeeBasis=Percentage 1003=1267479 AggressorIndicator=YES CheckSum=124

```

Your order fills as a taker with a notional of `0.03 * $2951.86666... = $88.556`.The remaining notional of `$996.01591412 - $88.556 = $907.45991412` will rest on the book with a quantity of `$907.45991412 / $2952 = 0.30740512`.Note that `OrderQty = CumQty + LeavesQty`. In the case of this specific order we have: `0.03 + 0.30740512 = 0.33740512`.The order is now rested on the book and be filled as passive.

```
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT AvgPx=2951.9 CumQty=0.04 ExecID=ef6a5354-e928-5815-b55b-46361fea6423 ExecTransType=NEW LastPx=2952 LastShares=0.01 OrderID=fa5d17b0-8344-4237-ab33-8c658595f960 OrderQty=0.33740512 OrdStatus=PARTIALLY_FILLED Price=2952 Side=BUY Symbol=ETH-USD TransactTime=20240529-20:59:51.230 NoMiscFees=1 MiscFeeAmt=0.0025 MiscFeeCurr=USD MiscFeeType=EXCHANGE_FEES ExecType=PARTIAL_FILL LeavesQty=0.29740512 MiscFeeBasis=Percentage AggressorIndicator=NO

```

Example 2 - A buy IOC order that results in no fills

```
$> -> BeginString=FIX.4.2 MsgType=ORDER_SINGLE ClOrdID=a79d33c4-7cc1-4948-b318-b878b5fbcc72 OrdType=LIMIT Price=1 Side=BUY Symbol=ETH-USD TimeInForce=IMMEDIATE_OR_CANCEL TransactTime=20240530-14:40:07.180 CashOrderQty=1000
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT ClOrdID=a79d33c4-7cc1-4948-b318-b878b5fbcc72 CumQty=0 ExecID=087c5712-6805-4e66-83b6-81c9fc92a78e ExecTransType=NEW OrderID=34cf5775-eee6-462b-8db0-9ec51f212663 OrderQty=0 OrdStatus=NEW Price=1 Side=BUY Symbol=ETH-USD TransactTime=20240530-14:40:07.245 ExecType=NEW LeavesQty=0 CashOrderQty=0 CheckSum=003
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT SenderCompID=Coinbase SendingTime=20240530-14:40:07.247 CumQty=0 ExecID=fcce5dea-9e83-492a-909c-30253e0b0d2f ExecTransType=NEW OrderID=34cf5775-eee6-462b-8db0-9ec51f212663 OrderQty=0 OrdStatus=CANCELED Price=1 Side=BUY Symbol=ETH-USD Text=101:Time In Force TransactTime=20240530-14:40:07.245 ExecType=CANCELED LeavesQty=0 CheckSum=214

```

Note that `OrderQty` is `0` in this case and this order will not be published in public WebSocket channels. It appears in the user channel only for this particular client.

Example 3 - A sell GTC order with Self Trade Prevention (decrement and cancel)

```
$> -> BeginString=FIX.4.2 MsgType=ORDER_SINGLE MsgSeqNum=41 ClOrdID=26dbf769-d812-4b0a-892a-bdf226387c0c OrdType=LIMIT Price=39944 Side=SELL Symbol=BTC-USD TimeInForce=GOOD_TILL_CANCEL TransactTime=20240531-14:24:32.566 CashOrderQty=10000 SelfTradePrevention=DECREMENT_AND_CANCEL CheckSum=225
$> <- BeginString=FIX.4.2 BodyLength=367 MsgType=EXECUTION_REPORT MsgSeqNum=48 SenderCompID=Coinbase SendingTime=20240531-14:24:32.705 ClOrdID=26dbf769-d812-4b0a-892a-bdf226387c0c CumQty=0 ExecID=f4054875-f834-409b-824a-2466bd3c066c ExecTransType=NEW OrderID=e19138ee-f97b-4bce-8cbe-ac8f6105ce30 OrderQty=0.2503 OrdStatus=NEW Price=39944 Side=SELL Symbol=BTC-USD TransactTime=20240531-14:24:32.702 ExecType=NEW LeavesQty=0.2503 CashOrderQty=9998.273235 CheckSum=061
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT MsgSeqNum=49 AvgPx=39949.3007 CumQty=0.05 ExecID=0aaee39c-fc85-5d82-b6f2-07d9b5b9d02a ExecTransType=NEW LastPx=39949.3007 LastShares=0.05 OrderID=e19138ee-f97b-4bce-8cbe-ac8f6105ce30 OrderQty=0.2503 OrdStatus=PARTIALLY_FILLED Price=39949.3007 Side=SELL Symbol=BTC-USD TransactTime=20240531-14:24:32.702 ExecType=PARTIAL_FILL LeavesQty=0.2003 MiscFeeBasis=Percentage 1003=23863011 AggressorIndicator=YES CheckSum=130
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT MsgSeqNum=50 AvgPx=39949.3007 CumQty=0.05 ExecID=30e4d848-a18b-469e-925a-1dbc83c17aea ExecTransType=NEW OrderID=e19138ee-f97b-4bce-8cbe-ac8f6105ce30 OrderQty=0.2253 OrdStatus=PARTIALLY_FILLED Price=39945 Side=SELL Symbol=BTC-USD TransactTime=20240531-14:24:32.702 ExecType=RESTATED LeavesQty=0.1753 ExecRestatementReason=PARTIAL_DECLINE_OF_ORDERQTY CheckSum=044

```

And this is the buy order with `LeavesQty` of `0.025` that was self trade cancelled:

```
$> <- BeginString=FIX.4.2 MsgType=EXECUTION_REPORT MsgSeqNum=51 CumQty=0 ExecID=0f4f96ad-160a-49a3-add0-eb25196fed70 ExecTransType=NEW OrderID=ca7e5097-cb6f-4381-8da8-629bc2656217 OrderQty=0.025 OrdStatus=CANCELED Price=39945 Side=BUY Symbol=BTC-USD Text=102:Self Trade Prevention TransactTime=20240531-14:24:32.702 ExecType=CANCELED LeavesQty=0 CheckSum=055

```