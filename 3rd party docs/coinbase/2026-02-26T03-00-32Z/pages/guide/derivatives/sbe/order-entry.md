# order entry

## InstrumentInfoRequest

Field

Name

Type

Length

Description

InstrumentInfoRequest

103

40

Request all instrument available

1

correlationId

int64

8

## InstrumentInfo

Sent in response to InstrumentInfoRequest message, and later when the status of an instrument changes.

Field

Name

Type

Length

Description

InstrumentInfo

203

80

Information about an instrument

1

correlationId

int64

8

requestId for correlation

2

instrumentId

int32

4

Numeric instrumentId

3

securityType

uint8

1

SecurityType  
`0` = FUTURES  
`1` = OPTIONS

4

status

uint8

1

`1` = PRE\_OPEN  
`2` = PRE\_OPEN\_NO\_CANCEL  
`3` = READY\_TO\_TRADE (aka, OPEN)  
`4` = TRADING\_HALTED (aka, PAUSE)  
`3` = PAUSE  
`5` = CLOSE  
`6` = EXPIRED (aka, POST CLOSE)

5

isLastMessage

int8

1

`0` = not last instrument info  
`1` = last instrument info for requestId”

5

reserved

int8

1

6

symbol

String32

32

Instrument symbol

## SetAccount

Send SetAccount message to set the account to be used for subsequent NewOrder messages.

Field

Name

Type

Length

Description

SetAccount

105

56

Set the current account

1

correlationId

int64

8

Client-assigned ID

2

account

String16

16

0-padded ASCII string

## SetTrader

Send SetTrader message to set the trader to be used for subsequent [NewOrder](#neworder) messages.

Field

Name

Type

Length

Description

SetTrader

106

56

Set the current trader

1

correlationId

int64

8

Client-assigned ID

2

trader

String16

16

0-padded ASCII string

## SetAck

Sent in response to [SetAccount](#setaccount) and [SetTrader](#settrader) messages.

Field

Name

Type

Length

Description

SetAck

205

40

Acknowledgment of Set request

1

correlationId

int64

8

Client-assigned ID

## NewOrder

Used to enter an order in the system.

Field

Name

Type

Length

Description

NewOrder

110

68

Place a new order

1

clientOrderId

int64

8

Numeric client order ID

2

correlationId

int64

8

Client-assigned ID

3

limitPrice

int64

8

Price with 9 decimal places

4

quantity

int32

4

Quantity with 0 decimal places

5

instrumentId

int32

4

Numeric instrumentId

6

side

int8

1

`1` = BUY  
`-1` = SELL

7

flags

int8

1

`1`\= Post Only

8

goodTilDate

uint16

2

(Optional) The trade date that a GTD order should be expired on, in the count of days since 1970-01-0 (ISO).  
Null value = `0` for DAY order

## NewIocOrder

Used to enter an IOC Order.

Field

Name

Type

Length

Description

NewIocOrder

111

69

Place a new IOC order

1

clientOrderId

int64

8

Numeric client order ID

2

correlationId

int64

8

Client-assigned ID

3

limitPrice

int64

8

Price with 9 decimal places

4

quantity

int32

4

Quantity with 0 decimal places

5

minQty

Int32

4

Minimum quantity for the order to be executed; values > 1

6

instrumentId

int32

4

Numeric instrumentId

7

side

int8

1

`1` = BUY  
`-1` = SELL

## OrderEntered

OrderEntered messages are sent in response to a [NewOrder](#neworder) message if successful.

Field

Name

Type

Length

Description

OrderEntered

210

80

NewOrder acknowledgement

1

transactTime

int64

8

Nanoseconds since epoch

2

execId

int64

8

Exchange-assigned event ID (akin to execId in FIX)

3

clientOrderId

int64

8

Numeric client order ID

4

correlationId

int64

8

Return the client-assigned ID

5

orderId

int64

8

Numeric exchange-assigned order ID

6

receiveTime

int64

8

Nanoseconds since Unix epoch when we received the NewOrder message on gateway.

## ReplaceOrder

The ReplaceOrder message allows you to alter the price and quantity of an order in a single message. This is more efficient than canceling an existing order and immediately succeeding it with a new orderRequest to modify an order. The order is canceled if `newQuantity` is less than or equal the current total filled quantity.

Field

Name

Type

Length

Description

ReplaceOrder

120

67

Replace an order

1

clientOrderId

int64

8

Numeric client order ID

2

correlationId

int64

8

Client-assigned ID

3

newLimitPrice

int64

8

Price with 9 decimal places

4

newQuantity

int32

4

Quantity with 0 decimal places

5

instrumentId

int32

4

Numeric instrumentId

6

goodTilDate

uint16

2

Required if timeInForce = `1`  
The trade date that a GTD order should be expired on, in the count of days since 1970-01-01 (ISO)

7

timeInForce

int8

1

`0` = “DAY” - Good for trading day  
`1` = “GOOD\_TILL\_DATE” - Good until the specified date

## OrderReject

OrderReject messages are sent in response to [NewOrder](#neworder) and [ReplaceOrder](#replaceorder) if the request is rejected.

Field

Name

Type

Length

Description

OrderReject

221

112

Reject message for NewOrder and ReplaceOrder

1

transactTime

int64

8

Nanoseconds since epoch

2

clientOrderId

int64

8

Client order ID

3

correlationId

int64

8

Return the client-assigned ID

4

orderId

int64

8

Exchange assigned order ID Set to 0 if order is unknown

5

rejectReason

uint8

1

`1` = ERROR  
`2` = INVALID\_INSTRUMENT  
`3` = CL\_ORD\_ID\_IN\_USE  
`8` = VALIDATION\_FAILURE  
`9` = UNKNOWN\_ORDER

6

details

char

47

Null (0) padded string

## OrderReplaced

This message acknowledges the receipt and acceptance of a valID [ReplaceOrder](#replaceorder).

Field

Name

Type

Length

Description

OrderReplaced

220

92

Reply to ReplaceOrder after success

1

transactTime

int64

8

Nanoseconds since epoch

2

execId

int64

8

Exchange-assigned event ID

3

clientOrderId

int64

8

Client order ID

4

correlationId

int64

8

Return the client-assigned ID

5

orderId

int64

8

Exchange-assigned order ID

6

receiveTime

int64

8

Nanoseconds since Unix epoch when we received the replace/update message on gateway.

7

totalFilled

int32

4

Filled amount

8

availableQty

int32

4

Remaining quantity available for matching

9

instrumentId

int32

4

## CancelOrder

The CancelOrder message is used to request that an order be canceled.

Field

Name

Type

Length

Description

CancelOrder

130

52

Cancel an Order

1

clientOrderId

int64

8

Numeric client order ID

2

correlationId

int64

8

Client-assigned ID

3

instrumentId

int32

4

instrumentId

## OrderCanceled

An OrderCanceled message informs you that an order has been canceled. This could be acknowledging a [CancelOrder](#cancelorder) message, or it could be the result of the order timing out or being canceled automatically.

Field

Name

Type

Length

Description

OrderCanceled

230

81

Sent when an order is canceled

1

transactTime

int64

8

Nanoseconds since epoch

2

execId

int64

8

Exchange-assigned event ID

3

clientOrderId

int64

8

Client order ID

4

correlationId

int64

8

Return the client-assigned ID

5

orderId

int64

8

Exchange-assigned order ID

6

receiveTime

int64

8

Nanoseconds since Unix epoch when we received the cancel message on gateway.  
Null value = `0x8000000000000000L` for unsolicited cancels

6

totalFilled

int32

4

Filled amount

7

instrumentId

int32

4

Numeric instrumentId

8

cancelReason

uint8

1

`0` = EXPIRED  
`1` = CANCELED\_BY\_USER  
`2` = SELF\_MATCH\_PREVENTION  
`3` = CLIENT\_DISCONNECT  
`4` = PRICE\_LIMIT  
`5` = ADMIN\_CANCEL  
`6` = MASS\_CANCEL  
`8` = ACTIVE\_LIMIT\_EXCEEDED

## CancelOrderReject

A CancelOrderReject message may be sent in response to a [CancelOrder](#cancelorder) if the cancel cannot be accepted at this time.

Field

Name

Type

Length

Description

CancelOrderReject

233

88

Reject message for order replace

1

transactTime

int64

8

Nanoseconds since epoch

2

clientOrderId

int64

8

Client order ID

3

correlationId

int64

8

Return the client-assigned ID

4

orderId

int64

8

Exchange assigned order ID

5

rejectReason

uint8

1

`1` = ERROR  
`2` = UNKNOWN\_ORDER  
`3` = ORDER\_FILLED

6

details

char

23

Null (0) padded string

## MassCancelOrder

The MassCancelOrder message is used to cancel multiple orders matching various optional criteria. Use it to cancels buys with limit prices at or above the specified limit, or sells with limit prices at or below the specified limit.

-   If `limitPrice` is defined, `instrumentId` and `side` are required, otherwise, they are optional.
-   The `currentSessionOnly` flag is used to specify that only orders submitted via the current session should be considered versus all sessions of the client’s firm.
-   The `requestTradingLock` flag can be used to thereafter reject subsequent orders until a TradingUnlock request is sent.

[OrderCanceled](#ordercanceled) messages are sent for each order canceled, followed by a [MassCancelOrderAck](#masscancelorderack) message.

Field

Name

Type

Length

Description

MassCancelOrder

131

55

Cancel multiple Orders

1

correlationId

int64

8

Client-assigned ID

2

limitPrice

int64

8

Optional. Null value = `0x8000000000000000L`

3

instrumentId

int32

4

Required if `limitPrice` exists, otherwise optional. Null value = `0x80000000`

4

side

int8

1

Required if `limitPrice` exists, otherwise optional.  
`-128` = both sides  
`1` = BUY  
`-1` = SELL

5

currentSessionOnly

int8

1

`1` = orders of current session only  
`0` = orders of all sessions of Firm

6

requestTradingLock

int8

1

`1` = lock trading for all sessions under cancel scope  
`0` = just cancel orders without engaging trading lock

## MassCancelOrderAck

The MassCancelOrderAck message is sent in response to a [MassCancelOrder](#masscancelorder) message following [OrderCanceled](#ordercanceled) messages for each canceled order. If a `tradingLock` was requested, the scope of sessions affected are reported as well.

Field

Name

Type

Length

Description

MassCancelOrderAck

231

62

Acknowledgement of MassCancelOrder request.

1

transactTime

int64

8

Nanoseconds since epoch

2

execId

int64

8

Exchange-assigned event ID

3

correlationId

int64

8

Client-assigned ID

4

canceledCount

int32

4

Total number of orders canceled

5

onlyCurrentSession

int8

1

`1` = only current session affected  
`0` = all sessions under Firm affected

6

tradingLockApplied

int8

1

`1` = trading lock applied  
`0` = not applied

## MassCancelOrderReject

A MassCancelOrderReject message may be sent in response to a [MassCancelOrder](#masscancelorder) if the mass cancel cannot be accepted at this time.

Field

Name

Type

Length

Description

MassCancelOrderReject

232

80

Reject message for order replace

1

transactTime

int64

8

Nanoseconds since epoch

2

correlationId

int64

8

Return the client-assigned ID

3

errorMessage

char

32

Null (0) padded string

## UnlockTrading

Sent to unlock trading after a [MassCancelOrder](#masscancelorder) has been sent when the flag `requestTradingLock`. This message disengages all client requested trading locks on all sessions under the firm unless `currentSessionOnly` is set, in which case, it only disengages trading locks on the current session. Trading locks reject all incoming orders until disengaged by either client through this message or exchange administrator.

Field

Name

Type

Length

Description

UnlockTrading

132

41

Unlock Client Requested Trading Lock

1

correlationId

int64

8

Client-assigned ID

2

currentSessionOnly

int8

1

`1` = unlock trading for current session only  
`0` = unlock trading for all sessions of Firm

## UnlockTradingAck

An UnlockTradingAck message reports the result of an unlock trading request, including the number of users affected.

Field

Name

Type

Length

Description

UnlockTradingAck

234

60

Acknowledgement of UnlockTrading

1

transactTime

int64

8

Nanoseconds since epoch

2

execId

int64

8

Exchange-assigned event ID

3

correlationId

int64

8

Client-assigned ID

4

numUsersAffected

int32

4

Total number of users unlocked

## UnlockTradingReject

Reports a reject of a requested trading unlock.

Field

Name

Type

Length

Description

UnlockTradingReject

235

80

Reject of UnlockTrading

1

transactTime

int64

8

Nanoseconds since epoch

2

correlationId

int64

8

Client-assigned ID

3

errorMessage

char

32

Null (0) padded string

## OrderFilled

An OrderFilled message informs you that all or part of an outright order has been executed.

Field

Name

Type

Length

Description

OrderFilled

240

113

Sent when an order is partially or fully filled

1

transactTime

int64

8

Nanoseconds since epoch

2

execId

int64

8

Exchange-assigned event ID

3

matchId

int64

8

Transaction ID representing match, shared by all fills within match

4

clientOrderId

int64

8

Client order ID

5

correlationId

int64

8

Return the client-assigned ID

6

orderId

int64

8

Exchange assigned order ID

7

filledVwap

int64

8

Filled VWAP with 9 decimal places

8

totalFilled

int32

4

Filled amount

9

availableQty

int32

4

Remaining quantity available for matching

10

fillPrice

int64

8

Price filled with 9 decimals

11

fillQty

int32

4

Fill quantity

12

instrumentId

int32

4

Numeric instrumentId

13

isAggressor

uint8

1

`0` - False  
`1` - True

## SpreadOrderFilled

A SpreadOrderFilled message informs you that all or part of a spread order has been executed.

Field

Name

Type

Length

Description

SpreadOrderFilled

241

129

Sent when an order is partially or fully filled

1

transactTime

int64

8

Nanoseconds since epoch

2

execId

int64

8

Exchange-assigned event ID

3

matchId

int64

8

Transaction ID representing match, shared by all fills within match

4

clientOrderId

int64

8

Client order ID

5

correlationId

int64

8

Return the client-assigned ID

6

orderId

int64

8

Exchange assigned order ID

7

filledVwap

int64

8

Filled VWAP with 9 decimal places

8

totalFilled

int32

4

Filled amount

9

availableQty

int32

4

Remaining quantity available for matching

10

fillPrice

int64

8

Price filled with 9 decimals

11

leg1fillPrice

int64

8

Price filled with 9 decimals on underlying leg 1

12

leg2fillPrice

int64

8

Price filled with 9 decimals on underlying leg 2

13

fillQty

int32

4

Fill quantity

14

instrumentId

int32

4

Numeric instrumentId

15

isAggressor

uint8

1

`0` - False  
`1` - True

## LastExecIdRequest

Send this message to request the `execId` of the last (most recent) event sent by the trading system to this user/session. Can be used to determine if the client missed any events while disconnected. Also serves as a means of validating that the trading system is available and accepting requests.

Field

Name

Type

Length

Description

LastExecIdRequest

150

40

Request execId of last event sent to user

1

correlationId

int64

8

Client-assigned ID

## LastExecId

Sent in response to `LastExecIdRequest`.

Field

Name

Type

Length

Description

LastExecId

250

56

Response to LastExecIdRequest

1

timestamp

int64

8

Nanoseconds since epoch

2

lastExecId

int64

8

ExecId of last event sent to this user.

3

correlationId

int64

8

Client-assigned ID

## EventResendRequest

Send this message to request order events, in the specified range, be resent. As this is an application-level request, resent messages will have new sequence numbers and the resend flag in the message will not be set. Rejects (and any other message that does not contain an execId) are not resent.

Field

Name

Type

Length

Description

EventResendRequest

152

56

Response to LastExecIdRequest

1

correlationId

int64

8

Client-assigned ID

2

beginExecId

int64

8

Lower bound (inclusive) of execIds.

3

endExecId

int64

8

Upper bound (inclusive) of execIds if positive. Resend all events up the last known event if non-positive.

## EventResentComplete

Sent in response to a successful [EventResendRequest](#eventresendrequest) following all resent events.

Field

Name

Type

Length

Description

EventResendComplete

252

44

Sent on fulfillment of an EventResendRequest

1

correlationId

int64

8

Client-assigned ID

2

resentEventCount

int32

4

Total number of events resent.

## EventResendReject

Sent in response to an [EventResendRequest](#eventresendrequest) if the request cannot be fulfilled.

Field

Name

Type

Length

Description

EventResendReject

253

64

Reject an EventResendRequest

1

correlationId

int64

8

Client-assigned ID

2

rejectReason

uint8

1

`1` = BEGIN\_EXEC\_ID\_TOO\_SMALL  
`2` = END\_EXEC\_ID\_TOO\_LARGE  
`3` = RESEND\_ALREADY\_IN\_PROGRESS  
`4` = TOO\_MANY\_RESEND\_REQUESTS  
`5` = SERVER\_ERROR

3

details

char

55

Null (0) padded string

## Default Session Values

During session creation, the following values are hard-coded for the Liquidity Provider.

Name

Type

Default Value

Description

[CustOrderCapacity](https://developer.chrome.com/derivatives/fix/code-sets#custordercapacity-582)

Char(1)

`2`

`2` = PROPRIETARY. CTI represents a proprietary account

[ManualOrderIndicator](https://developer.chrome.com/derivatives/fix/code-sets#manualorderindicator-1028)

Boolean(1)

`N`

`N` = AUTOMATIC. Message is generated by automated trading logic.

[OrderCapacity](https://developer.chrome.com/derivatives/fix/code-sets#ordercapacity-528)

Int(1)

`1`

`1` = PRINCIPAL. Order placed by the firm or principal.

[CustOrderHandlingInst](https://developer.chrome.com/derivatives/fix/code-sets#custorderhandlinginst-1031)

String(1)

`Y`

`Y` = ELECTRONIC. Order is delivered electronically.