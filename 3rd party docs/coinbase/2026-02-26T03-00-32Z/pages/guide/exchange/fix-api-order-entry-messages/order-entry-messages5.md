# order entry messages5

About this API:

-   **Baseline**: [FIX 5.0 SP2 specification](https://www.onixs.biz/fix-dictionary/5.0.sp2/index.html).
-   **Environments**: Production, Sandbox

## Components

### Standard Header

Fields that go at the beginning of every message. This exists for all messages sent and received.

Tag

Name

Type

Required

Description

8

BeginString

String

Y

Must be set to `FIXT.1.1` and be the first field in the message.

(Since FIX version 5.0 this field now represents the session version. The application version gets specified in [Logon](#logon-35a) message’s `DefaultApplVerID (1137)` tag.)

9

BodyLength

Int

Y

Message length in bytes up to the checksum field (tags after `BodyLength (9)` and before `Checksum (10)`). This must be the second field in message.

35

MsgType

String

Y

The type of message proceeding the header, must be the third field in the message.

**Supported values include:**

Admin Messages  
`A` = Logon  
`0` = Heartbeat  
`1` = TestRequest  
`3` = Reject  
`5` = LogoutApplication Messages  
`D` = NewOrderSingle  
`F` = OrderCancelRequest  
`G` = OrderCancelReplaceRequest  
`H` = OrderStatusRequest  
`j` = BusinessMessageReject  
`8` = ExecutionReport  
`9` = OrderCancelReject  
`U4` = OrderCancelBatch  
`U5` = OrderCancelBatchReject  
`U6` = NewOrderBatch  
`U7` = NewOrderBatchReject

49

SenderCompID

String

Y

Client API key (on messages from the client).

56

TargetCompID

String

Y

Must be `Coinbase` (on messages from client).

34

MsgSeqNum

Int

Y

Monotonically increasing sequence number of the message.

43

PossDupFlag

Boolean

C

Indicates that the message was sent in response to a ResendRequest.  
\- `Y` - Sent in response to ResendRequest. Should be ignored unless message was not previously processed  
\- `N` or null - Normal transmission

52

SendingTime

UTCTimestamp

Y

UTC time that the order was sent down to millisecond resolution in the format `YYYYMMDD-HH:MM:SS.sss`

83

RptSeq

Int

C

The feed sequence number of the message corresponding to the `RptSeq` in FIX 5.0 Market Data

50

SenderSubID

String

C

Assigned UUID to identify a specific message originator. Required for EXN orders.

### Standard Trailer

Fields that go at the end of every message.

Tag

Name

Type

Required

Description

10

CheckSum

String

Y

Three byte checksum calculated by summing every byte in the message up to and not including the checksum field itself. This value is then moduloed by `256` and written with prefixed `0`s (if necessary) to meet the 3 byte requirement.

## Administrative

### Logon (35=A)

First message that is required immediately upon connection to authenticate the connection. `MsgSeqNum` always equals 1 (`34=1`) on this message in both directions.

Tag

Name

Type

Required

Description

98

EncryptMethod

Int

N

Must be `0` (None)

108

HeartBtInt

Int

O

Must be ≤ `30` (secs). Server sends [Test Request](#testrequest-351) if client messages are not received in approximately (`HeartBtInt` x `1.5`) seconds. Server terminates session if client messages are not received in approximately (`HeartBtInt` x `2`) seconds. Defaults to `10` seconds if not value provided.

141

ResetSeqNumFlag

Boolean

Y

Resets the sequence number. Defaults to `Y`.

Sequence numbers from Customer => Coinbase always get reset after a disconnect.

Sequence numbers from Coinbase => Customer are reset after a disconnect if either of these are true:  
1\. ResetSeqNumFlag not set  
2\. ResetSeqNumFlag = `Y`  
3\. Customer was not logged on using same API key more than 1 day.

The max possible MsgSeqNum is 2147483647 and customers are responsible for resetting their sessions to avoid breaching this limit.

553

Username

String

Y

Client API Key.

554

Password

String

Y

Passphrase for Client API key.

95

RawDataLength

Int

Y

Number of bytes in `RawData` field.

96

RawData

String

Y

Client message signature (see [Signing a message](https://developer.chrome.com/exchange/rest-api/authentication#signing-a-message))

1137

DefaultApplVerID

String

Y

Contains the version of the FIX protocol the exchange uses. Only FIX50SP2 is supported.

**Supported values:**  
`9` = FIX50SP2

8001

DefaultSelfTradePreventionStrategy

Char

N

The default SelfTradePreventionStrategy applied to all orders sent on the session unless overridden on a per order basis using the SelfTradeType (7928) in the order request message.

The following values specify what to do when two orders submitted by the same portfolio attempt to match:  
`N` = Cancel aggressing order  
`Q` = Cancel both orders

Default if not specified is Cancel both orders (`Q`).

8013

CancelOrdersOnDisconnect

Char

N

`S` = Cancel all session orders on disconnect  
`Y` = Cancel all profile orders on disconnect **(recommended)**

9406

DropCopyFlag

Char

N

`N` = Normal order-entry session  
`Y` = Drop Copy Session that only returns fills (Execution Report - Filled and Execution Report - Partially Filled)

#### FIX 5.0 Logon Details:

FIX 5.0 is separated into two different layers an application layer and a session layer. The application layer is version 5.0 and the session layer is version T1.1.

-   For all LOGON messages use MsgSeqNum 1, set 553 Username to the api key, and only create one session per API Key.
-   The begin string will use the session layer version 8=FIXT.1.1
-   The application layer version will be specified on the DefaultAppVerID which is tag 1137. This should be set to 9 (This represents version 5.0).
-   Sequence numbers must not include leading zeros (e.g., “0001” is invalid, whereas “1” is valid). Using sequence numbers with leading zeros will result in an invalid signature error.
-   Fractional seconds must be specified using exactly three digits. For example, 52=20230822-20:43:30.000 is supported, while both 52=20230822-20:43:30 (no fractional seconds) and 52=20230822-20:43:30.123456789 (excessive precision) will result in a signature calculation error.

The Logon message sent by the client must be signed for security. The signing method is shown below. The prehash string is the following fields joined: `SendingTime, MsgType, MsgSeqNum, SenderCompID (API KEY), TargetCompID, Passphrase`.. There is no trailing separator. The `RawData` field should be a base64 encoding of the HMAC signature.

```
"""Example Python QuickFIX Application.py logon message setup"""
class Application(fix.Application):
    PASSPHRASE = [ENTER PASSPHRASE]
    API_KEY = [ENTER API_KEY]
    SECRET = [ENTER SECRET]
"""Setup the Logon message & call sign function"""
def toAdmin(self, message, sessionID):
    rawData = self.sign(message.getHeader().getField(52), message.getHeader().getField(35),
                                message.getHeader().getField(34), self.API_KEY, message.getHeader().getField(56),
                                self.PASSPHRASE)
    message.setField(fix.StringField(554, self.PASSPHRASE))
    message.setField(fix.StringField(96, rawData))
    message.setField(fix.StringField(8013, "Y"))
    message.setField(fix.StringField(553, self.API_KEY))
    message.setField(fix.IntField(95, len(rawData.encode('utf-8'))))
"""Create base64 encoded signature"""
def sign(self, timestamp, msg_type, seq_num, api_key, target_comp_id, passphrase):
    message = '\x01'.join([timestamp, msg_type, seq_num, api_key, target_comp_id, passphrase]).encode("utf-8")
    hmac_key = base64.b64decode(self.SECRET)
    signature = hmac.new(hmac_key, message, hashlib.sha256)
    sign_b64 = base64.b64encode(signature.digest()).decode()
    return sign_b64

```

#### FIX 5.0 How to resume a session:

When a session disconnects, the following steps outline the client-server interaction: After session disconnect:

1.  **Client Logon:** The client initiates a `LOGON` message with `MsgSeqNum` 1 and 141 (`ResetSeqNumFlag`) = N.
2.  **Server Response:** The server responds with a `LOGON` message (also with `MsgSeqNum` = 1), immediately followed by a Sequence Reset message. This Sequence Reset includes 36 (`NewSeqNo`), which communicates the last sequence number from the previous session.
3.  **Client Resend Request:**
    -   The client then sends a `Resend Request` with `MsgSeqNum` 2.
    -   This request specifies 7 (`BeginSeqNo`) = 2 and 16 (`EndSeqNo`) = `NewSeqNo` (from the server’s previous Sequence Reset).
    -   *Paging Large Gaps*: If `NewSeqNo` is greater than 1000, the client requests 7 (`BeginSeqNo`) = 2 and 16 (`EndSeqNo`) = 1000. This indicates an intent to page sequence numbers in blocks of 1000. Subsequent blocks would follow this pattern (e.g., 7 (`BeginSeqNo`) = 1001 to 16 (`EndSeqNo`) = 1999, or `NewSeqNo` if less than 2000).
4.  **Admin Message and Older Message Policy:** We do not replay administrative messages or messages older than one hour. For any such gaps, a Sequence Reset is sent.
5.  **Waiting for Resend Completion:** Before requesting the next page of sequence numbers, the client must wait for the current resend request to be completed.

### Heartbeat (35=0)

Sent at a prearranged interval from both sides to indicate liveness of the connections and used in response to a [TestRequest](#testrequest-351) message (35=1).

Tag

Name

Type

Required

Description

112

TestReqID

String

C

Conditionally required when the heartbeat message is sent in response to a [TestRequest](#testrequest-351) (35=1) message.

### TestRequest (35=1)

This message forces the other side of the connection to send a [Heartbeat](#heartbeat-350) message (35=0) with the `TestReqID` (tag 112) populated with the same value provided on this message.

Tag

Name

Type

Required

Description

112

TestReqID

String

Y

A unique identifier used to track the response to a test request.

### ResendRequest (35=2)

Sent by the customer to Coinbase to request the retransmission of a range of messages on a given FIX session. The Coinbase FIX gateway keeps a 4 hour history of messages sent to customers:

-   Administrative messages are always replaced by [SequenceReset-GapFill](#sequencereset-354) messages during retransmission.
-   Older non-administrative messages are also replaced by SequenceReset-GapFill messages.
-   Retransmitted messages, including SequenceReset-GapFill messages, have PossDupFlag enabled (`43=Y`) in the header.

Tag

Name

Type

Required

Description

7

BeginSeqNo

Int

Y

Sequence number of first message in range to be resent. Must be >= 1

16

EndSeqNo

Int

Y

Sequence number of last message in range to be resent. Must be >= 1 and >= BeginSeqNo

### SequenceReset (35=4)

Used to skip messages during retransmission. Coinbase only supports “GapFill” mode where GapFillFlag is always true. Coinbase sends SequenceReset-GapFill messages to customers with or without PossDupFlag (43) in the header:

-   Without PossDupFlag: Coinbase sends immediately after logon to reset the Coinbase sequence number to the next outbound sequence number stored for the session.
-   With PossDupFlag=Y: Coinbase sends in response to a [ResendRequest](#resendrequest-352) for all administrative messages, irrespective of time sent, as well as non-administrative messages older than 4 hours.

Tag

Name

Type

Required

Description

123

GapFillFlag

Boolean

Y

Always true (`123=Y`)

36

NewSeqNo

Int

Y

Must be > 1

### Reject (35=3)

A session level reject message sent when the FIX session can’t process a message.

Tag

Name

Type

Required

Description

45

RefSeqNum

Int

Y

The `MsgSeqNum` of the referenced message that was rejected.

371

RefTagID

Int

N

The tag number of the FIX field referenced in the reject.

372

RegMsgType

String

N

The `MsgType` of the FIX message referenced in the reject.

373

SessionRejectReason

Int

N

A code to quickly identify common reasons for a reject.

**Supported values:**  
`0` = Invalid Tag Number  
`1` = Required Tag Missing  
`2` = Tag not defined for this message type  
`3` = Undefined tag  
`4` = Tag specified without a value  
`5` = Value is incorrect (out of range) for this tag  
`6` = Incorrect data format for value  
`8` = Signature problem  
`9` = `CompID` problem  
`10` = `SendingTime` Accuracy Problem  
`11` = Invalid `MsgType`  
`13` = Tag appears more than once  
`14` = Tag specified out of required order  
`15` = Repeating group fields out of order  
`16` = Incorrect `NumInGroup` count for repeating group  
`17` = Non “Data” value includes field delimiter (<SOH> character)  
`18` = Invalid/Unsupported Application Version  
`99` = Other

58

Text

String

N

A message explaining why the message was rejected.

### Logout (35=5)

Sent by either side to initiate session termination. The side which receives this message first should reply with the same message type to confirm session termination.

Tag

Name

Type

Required

Description

58

Text

String

N

Description of the disconnection reason.

## Trading

### NewOrderSingle (35=D)

Used to submit a new spot order to the Exchange matching engine.

Tag

Name

Type

Required

Description

11

ClOrdID

UUID

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request. It must be a variant 1 UUIDv4 that follows the standard format. This means all lowercase and hyphens that group the characters in sequences of 8, 4, 4, 4, 12 (e.g. `1985ca2d-61ef-49f1-bfce-6c39d8462914`). Failure to follow this formatting will result in a reject.

This shouldn’t match the ClOrdID of any open orders.

18

ExecInst

Char

N

The execution instruction flags for the order.

**Supported values:**  
`A` = Add Liquidity Only (Post Only)

21

HandlInst

Char

N

Specifies an order is designated for EXN

**Supported values:**  
`E` = EXN

38

OrderQty

Decimal

C

The amount of the base asset to be transacted. Required except unless `CashOrderQty` is specified.

152

CashOrderQty

Decimal

C

Order size in quote units (e.g., USD) (Market or [Limit order](https://developer.chrome.com/exchange/fix-api/order-entry-messages/limit-orders) only).

40

OrdType

Char

Y

The type of order for the request which can be.

**Supported values:**  
`1` = Market  
`2` = Limit  
`4` = Stop Limit  
`O` = [Take Profit Stop Loss](https://developer.chrome.com/exchange/fix-api/order-entry-messages/tpsl-orders)

44

Price

Decimal

C

The limit price for limit orders of the quote asset. The decimal precision must fall within the requirements for each market, see the REST API for precision and decimal limits.

54

Side

Char

Y

Side of the order.

**Supported values:**  
`1` = Buy  
`2` = Sell

55

Symbol

String

Y

Symbol of the instrument being traded (e.g. `BTC-USDC`)

59

TimeInForce

Char

Y

Specifies how long the order remains in effect.

**Supported values:**  
`1` = Good Till Cancel (GTC)  
`3` = Immediate Or Cancel (IOC)  
`4` = Fill Or Kill (FOK)  
`6` = Good Till Date (GTD)

126

ExpireTime

UTCTimestamp

C

Required when `TimeInForce` (59) is set to `GTD` (6). Specifies the time when a GTD order expires. Required for GTD orders and should not be set for other orders. The order expires within one second after the specified time.

99

StopPx

Decimal

C

Specifies the quote price at which the order activates for Stop Limit order types (40=4)

1109

TriggerPriceDirection

Char

N

**Supported values:**  
`U` = Trigger if market price goes UP to or through `StopPx` (default if `StopPx` is greater than current market price)  
`D` = Trigger if market price goes DOWN to or through `StopPx` (default if `StopPx` is less than current market price)

*For stop-limit orders, this field is optional but recommended.*

7928

SelfTradeType

Char

N

The following values specify what to do when two orders are submitted by the same user attempt to match:

`D` = Decrement and Cancel (default if not specified)  
`O` = Cancel Oldest (resting order)  
`N` = Cancel Newest (aggressing order)  
`B` = Cancel Both

### NewOrderBatch (35=U6)

Used to submit new spot orders to the Exchange matching engine. Clients should use this message to submit multiple orders to the Exchange matching engine at the same time. Currently, all the orders submitted in a batch must be for the same symbol.

Tag

Name

Type

Required

Description

8014

BatchID

UUID

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request. It must be a variant 1 UUIDv4 that follows the standard format. This means all lowercase and hyphens that group the characters in sequences of 8, 4, 4, 4, 12 (e.g. `1985ca2d-61ef-49f1-bfce-6c39d8462914`). Failure to follow this formatting will result in a reject.

73

NoOrders

Int

Y

Number of orders in the request.

\=>11

ClOrdID

UUID

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request. It must be a variant 1 UUIDv4 that follows the standard format. This means all lowercase and hyphens that group the characters in sequences of 8, 4, 4, 4, 12 (e.g. `1985ca2d-61ef-49f1-bfce-6c39d8462914`). Failure to follow this formatting will result in a reject.

This shouldn’t match the ClOrdID of any open orders. Additionally, it shouldn’t match any other ClOrdIDs in this batch.

\=>18

ExecInst

Char

N

The execution instruction flags for the order.

**Supported values:**  
`A` = Add Liquidity Only (Post Only)

\=>38

OrderQty

Decimal

C

The amount of the base asset to be transacted. Required except for market orders with `CashOrderQty` specified.

\=>152

CashOrderQty

Decimal

C

The order size in quote units (e.g., USD) (Market order only).

\=>40

OrdType

Char

Y

The type of order for the request which can be.

**Supported values:**  
`1` = Market  
`2` = Limit  
`4` = Stop Limit

\=>44

Price

Decimal

C

The limit price for limit orders of the quote asset. The decimal precision must fall within the requirements for each market, see the REST API for precision and decimal limits.

\=>54

Side

Char

Y

Side of the order.

**Supported values:**  
`1` = Buy  
`2` = Sell

\=>55

Symbol

String

Y

Symbol of the instrument being traded (e.g. `BTC-USDC`)

\=>59

TimeInForce

Char

Y

Specifies how long the order remains in effect.

**Supported values:**  
`1` = Good Till Cancel (GTC)  
`3` = Immediate Or Cancel (IOC)  
`4` = Fill Or Kill (FOK)  
`6` = Good Till Date (GTD)

\=>126

ExpireTime

UTCTimestamp

C

Required when `TimeInForce` (59) is set to GTD (6). Specifies the time when a GTD order expires. Required for GTD orders and should not be set for other orders. The order expires within one second after the specified time.

\=>99

StopPx

Decimal

C

Specifies the quote price at which the order activates for Stop Limit order types (40=4).

\=>7928

SelfTradeType

Char

N

Represents type of cancel instruction when two orders submitted by the same user attempt to match.

**Supported values:**  
`D`\=Decrement and Cancel (default if not specified)  
`O`\=Cancel Oldest (resting order)  
`N`\=Cancel Newest (aggressing order)  
`B`\=Cancel Both

### NewOrderBatchReject (35=U7)

This message is sent by Coinbase Exchange back to clients when all the orders in a [New Order Batch](#neworderbatch-35u6) (35=U6) Request are rejected. When only some of the orders are rejected, Execution Report - Rejected messages are sent out for each of the orders individually.

Tag

Name

Type

Required

Description

8014

BatchID

UUID

Y

Client-supplied ID identifying the new order batch request.

58

Text

String

Y

The reason the batch of orders was rejected.

### OrderCancelRequest (35=F)

Used to cancel an order that is still live on the Exchange matching engine.

Tag

Name

Type

Required

Description

11

ClOrdID

UUID

Y

An identifier specified by the sender to uniquely identify other messages correlating to this request. It must be a variant 1 UUIDv4 that follows the standard format. This means all lowercase and hyphens that group the characters in sequences of 8, 4, 4, 4, 12 (e.g. `1985ca2d-61ef-49f1-bfce-6c39d8462914`). Failure to follow this formatting will result in a reject.

37

OrderID

UUID

Y

The exchange order ID of the order to be canceled.

41

OrigClOrdID

UUID

Y

The client order ID of the order to be canceled.

At least one of `OrigClOrdID` or `OrderID` must be specified.

55

Symbol

String

Y

Must match the message that the `OrigClOrdID` references.

### OrderCancelReject (35=9)

This message is sent by Coinbase Exchange back to clients to reflect that an order could not be canceled on the matching engine in the following situations:

-   When an [Order Cancel Request](#ordercancelrequest-35f) (35=F) is rejected (CxlRejResponseTo 434=1)
-   When an [Order Cancel/Replace Request](#ordercancelreplacerequest-35g) (35=G) is rejected (CxlRejResponseTo 434=2)
-   When an [Order Cancel Batch](#ordercancelbatch-35u4) (35=U4) is partially rejected (CxlRejResponseTo 434=1)

Tag

Name

Type

Required

Description

11

ClOrdID

UUID

Y

Echoed back from the client request.

37

OrderID

UUID

C

Echoed back from the client request.

41

OrigClOrdID

UUID

Y

Echoed back from the client request.

58

Text

String

N

Description of why the order could not be canceled.

39

OrdStatus

Char

Y

**Always:**  
`8` = Rejected

102

CxlRejReason

Int

N

**Supported values:**  
`1` = Unknown Order  
`2` = Broker

434

CxlRejResponseTo

Char

Y

**Supported values:**  
`1` = Order Cancel Request  
`2` = Order Cancel/Replace Request

### OrderCancelBatch (35=U4)

Clients should use this message to cancel multiple orders on the Exchange matching engine at the same time. Currently, all the orders canceled in a batch must be for the same symbol.

Tag

Name

Type

Required

Description

8014

BatchID

UUID

Y

Client-supplied ID identifying the order cancel batch request.

73

NoOrders

Int

Y

Number of orders in the request.

\=>11

ClOrdID

UUID

Y

Client-supplied ID identifying the order cancel request.

\=>37

OrderID

UUID

Y

The exchange order ID of the order to be canceled.

\=>41

OrigClOrdID

UUID

Y

The client order ID of the order to be canceled.

At least one of `OrigClOrdID` or `OrderID` must be specified.

\=>55

Symbol

String

Y

Must match the message that the `OrigClOrdID` references.

### OrderCancelBatchReject (35=U5)

This message is sent by Coinbase Exchange back to clients when all the orders in an [Order Cancel Batch](#ordercancelbatch-35u4) (35=U4) Request could not be canceled. When only some of the orders could not be canceled, [Order Cancel Reject](#ordercancelreject-359) (35=9) messages are sent out for the orders individually.

Tag

Name

Type

Required

Description

8014

BatchID

UUID

Y

Client-supplied ID identifying the order cancel batch request.

58

Text

String

N

The reason the order cancel batch request was rejected.

### OrderCancelReplaceRequest (35=G)

Clients should use this message to modify a single order on the Exchange matching engine. Supported order types include Limit, Stop Limit, and [Take Profit Stop Loss](https://developer.chrome.com/exchange/fix-api/order-entry-messages/tpsl-orders) orders. If order quantity is increased or order price is modified, queue priority is lost. Queue priority is maintained when order quantity is decreased. For Stop Limit and TPSL orders, the stop trigger priority is only altered when `StopPx` is modified. Modified orders share the same exchange `OrderID`(37) as the parent order. Orders are modified with “in-flight mitigation” - i.e. any partially filled quantity on the parent order is carried over to the child order and is reflected in the new order’s remaining quantity `LeavesQty`(151).

Tag

Name

Type

Required

Description

11

ClOrdID

UUID

Y

The client order ID of the new order (that will replace an existing order).

This shouldn’t match the ClOrdID of any open orders.

37

OrderID

UUID

Y

An identifier matching the `OrderID` from the [NewOrderSingle](#newordersingle-35d), [NewOrderBatch](#neworderbatch-35u6), or [OrderCancelReplaceRequest](#ordercancelreplacerequest-35g) that this request applies to.

41

OrigClOrdID

String

Y

An identifier matching the `ClOrdID` from the [NewOrderSingle](#newordersingle-35d), [NewOrderBatch](#neworderbatch-35u6), or [OrderCancelReplaceRequest](#ordercancelreplacerequest-35g) that this request applies to.

38

OrderQty

Decimal

Y

The new amount of the base asset to be transacted.

44

Price

Decimal

Y

The new desired limit price of the order.

55

Symbol

String

Y

Must match the symbol on the message that the `OrigClOrdID` references.

40

OrdType

Char

Y

**Supported values:**  
`2` = Limit  
`4` = Stop Limit  
`O` = [Take Profit Stop Loss](https://developer.chrome.com/exchange/fix-api/order-entry-messages/tpsl-orders)

99

StopPx

Decimal

C

The new stop trigger price. Required for Stop Limit and TPSL order modifications.

3040

StopLimitPx

Decimal

C

The new stop limit price. Required for TPSL order modifications only.

### OrderMassCancelRequest (35=q)

Sent by customer to Coinbase to request mass cancellation of all orders on a FIX session previously submitted by customer.

Tag

Name

Type

Required

Description

11

ClOrdID

UUID

Y

Identifier of the Mass Cancel request (not the order ID to be canceled)

530

MassCancelRequestType

Char

Y

Type of orders to be canceled:

-   `6` - Cancel Orders for a Trading Session

60

TransactTime

UTCTimestamp

Y

Request timestamp

### OrderMassCancelReport (35=r)

Sent by Coinbase to the customer as an acknowledgement of an Order Mass Cancel Request for processing or a rejection of the request. Receipt of a successful Order Mass Cancel Report does not imply that orders were canceled until “Execution Report - Canceled” is sent to customer.

Tag

Name

Type

Required

Description

11

ClOrdID

UUID

Y

ID echoed from the Order Mass Cancel Request

530

MassCancelRequestType

Char

Y

Echoed from the Order Mass Cancel Request: `6` - Cancel Orders for a Trading Session

531

MassCancelResponse

Char

Y

If the Order Mass Cancel Request was rejected:

-   `0` - Request Rejected

If successful, echoed from the request:

-   `3` - Cancel Orders for a Product on Profile
-   `6` - Cancel Orders for a Trading Session
-   `7` - Cancel All Orders on Profile

58

Text

String

N

A message explaining why the request was rejected

### ExecutionReport (35=8)

This message is sent by Coinbase Exchange back to clients to reflect changes to an order’s state (accepted, replaced, restated, partially filled, filled, expired, or canceled).

Tag

Name

Type

Required

Description

11

ClOrdID

UUID

Y

The client order ID of the (new) order.

37

OrderID

UUID

Y

A unique identifier assigned by the exchange for the order.

41

OrigClOrdID

String

C

The client order ID of the parent order for [Order Cancel/Replace Requests](#ordercancelreplacerequest-35g).

6

AvgPx

Decimal

C

The volume-weighted average price of all fills on the order.

14

CumQty

Decimal

C

The cumulative base quantity (e.g. in BTC) filled on the order.

151

LeavesQty

Decimal

C

The remaining base quantity (e.g. in BTC) on the order.

Not sent for market orders that were sent using `CashOrderQty`.

17

ExecID

UUID

Y

ID identifying this execution report.

39

OrdStatus

Char

Y

**Supported values:**  
`0` = New  
`1` = Partially Filled  
`2` = Filled  
`4` = Canceled  
`5` = Replaced  
`8` = Rejected  
`C` = Expired (For IOC expirations)

150

ExecType

Char

Y

**Supported values:**  
`0` = New  
`4` = Canceled  
`5` = Replaced  
`8` = Rejected  
`C` = Expired (For IOC expirations)  
`D` = Restated (in cases where orders are partially canceled unsolicited due to self-trade prevention)  
`F` = Trade  
`I` = Order Status (in response to Order Status Requests)

55

Symbol

String

Y

The symbol of the order (e.g. BTC-USD).

54

Side

Char

C

**Supported values:**  
`1` = Buy  
`2` = Sell

40

OrdType

Char

C

**Supported values:**  
`1` = Market  
`2` = Limit  
`4` = Stop Limit

32

LastQty

Char

C

The base quantity (e.g. in BTC) of the most recent fill on the order when `ExecType` is `F` (Trade).

31

LastPx

Decimal

C

The price of the most recent fill on the order when `ExecType` is `F` (Trade).

44

Price

Decimal

C

The limit price of the order.

38

OrderQty

Decimal

C

The base quantity (e.g. in BTC) of the order.

152

CashOrderQty

Decimal

C

The quote quantity (e.g. in USD) of the order.

For market orders that were submitted using `CashOrderQty` instead of `OrderQty`, this is the remaining quote quantity of the order.

58

Text

String

N

Description of why the order was rejected, canceled, or expired.

60

TransactTime

UTCTimestamp

Y

Matching engine timestamp.

103

OrdRejReason

Int

N

**Supported values:**  
`0` = Broker  
`1` = Unknown Symbol  
`5` = Unknown Order

378

ExecRestatementReason

Int

N

**Supported values:**  
`5` = Partial Decline of `OrderQty` (in cases where orders are partially canceled unsolicited due to self-trade prevention).

1003

TradeID

String

C

Trade ID for a given fill used for reporting.

1057

AggressorIndicator

Boolean

C

**Supported values:**  
`Y` = Taker (if aggressor or auction trade)  
`N` = Maker

59

TimeInForce

Char

C

**Supported values:**  
`1` = GTC  
`3` = IOC  
`4` = FOK  
`6` = GTD

99

StopPx

Decimal

C

For stop-limit orders, the stop price of the order.

1109

TriggerPriceDirection

Char

N

For stop-limit orders.

**Supported values:**  
`U` = Trigger if market price goes UP to or through `StopPx`  
`D` = Trigger if market price goes DOWN to or through `StopPx`

18

ExecInst

Char

N

**Supported values:**  
`A` = Add Liquidity Only.

7928

SelfTradeType

Char

N

**Supported values:**  
`D` = Decrement and Cancel (default if not specified)  
`O` = Cancel Oldest (resting order)  
`N` = Cancel Newest (aggressing order)  
`B` = Cancel Both

126

ExpireTime

UTCTimestamp

C

Timestamp at which a GTD order would expire.

1430

VenueType

Char

C

Indicates an RFQ fill. **Supported values:**  
`E` = ELECTRONIC\_EXCHANGE  
`N` = QUOTE\_NEGOTIATION

136

NoMiscFees

Int

C

Repeating group for fees charged.

Always `1` on an order partial fill or fill.

\=>137

MiscFeeAmt

Decimal

C

See `MiscFeeBasis`.

\=>138

MiscFeeCurr

String

C

The currency that the fee is charged in.

\=>139

MiscFeeType

String

C

**Always:**  
`4` = Exchange Fees.

\=>891

MiscFeeBasis

Int

C

**Supported values:**  
`0` = Absolute (`MiscFeeAmt` is in `MiscFeeCurr` terms)  
`2` = Percentage (`MiscFeeAmt` should be multiplied by the fill quantity in `MiscFeeCurr` terms to calculate the fee in `MiscFeeCurr` terms)

### BusinessMessageReject (35=j)

An application level reject message sent when the FIX session can’t process a message.

Tag

Name

Type

Required

Description

45

RefSeqNum

Int

N

The `MsgSeqNum` of the referenced message that was rejected.

372

RefMsgType

Int

Y

The message type that this reject message applies to.

**Supported values include:**

Admin Messages  
`A` = Logon  
`0` = Heartbeat  
`1` = TestRequest  
`3` = Reject  
`5` = LogoutApplication Messages  
`D` = NewOrderSingle  
`F` = OrderCancelRequest  
`G` = OrderCancelReplaceRequest  
`H` = OrderStatusRequest  
`j` = BusinessMessageReject  
`8` = ExecutionReport  
`9` = OrderCancelReject  
`U4` = OrderCancelBatch  
`U5` = OrderCancelBatchReject  
`U6` = NewOrderBatch  
`U7` = NewOrderBatchReject

379

BusinessRejectRefID

String

N

The `ClOrdID`, `OrderID`, `BatchID`, or other identifying ID on the failed request.

380

BusinessRejectReason

Int

N

A code to quickly identify common reasons for a reject.

**Supported values include:**  
`1` = Other  
`2` = Unsupported Message Type

58

Text

String

N

A message explaining why the message was rejected.

## RFQ

### Quote Request (R)

A Quote Request (R) is the start of the RFQ process. Coinbase sends a Quote Request to Liquidity Providers (LPs) on behalf of a customer looking to participate in an RFQ trade. LPs respond to a Quote Request with a [Quote](#quote-s).

Tag

Name

Type

Required

Notes

131

QuoteReqID

UUID

Y

Unique identifier for RFQ

146

NoRelatedSym

NumInGroup

Y

Always 1

55

Symbol

String32

Y

Example: `BTC-AVAX`

38

OrderQty

Float64

Y

The quantity the customer is looking to trade via RFQ

62

ValidUntilTime

UTCTimestamp

Y

The time by which quotes must be submitted for the RFQ

126

ExpireTime

UTCTimestamp

Y

The time by which the RFQ expires if there is no match

136

NoMiscFees

NumInGroup

Y

Always 1

137

MiscFeeAmt

Float64

Y

Fee as a percentage that Liquidity Providers are charged on a winning Quote.

The fee is charged in the currency the LP receives (e.g., in BTC if LP is buying BTC-AVAX, or in AVAX if LP is selling BTC-AVAX).

Example: 0.0005 (5 bps)

139

MiscFeeType

Int32

Y

Always 4 = Exchange Fees

891

MiscFeeBasis

Int32

Y

Always 2 = Percentage

528

OrderCapacity

Char

Y

A = Agency (default)  
C = Corporate

### Quote (S)

Quote (S) messages are submitted by Liquidity Providers (LP) in response to a [Quote Request](#quote-request-r) in order to participate in the competitive RFQ auction. Quotes can be submitted as either a one-way or two-way quote, and must be received by the `ValidUntilTime (62)` specified in the Quote Request. Only one side is traded if the Liquidity Provider wins the RFQ.

Tag

Name

Type

Required

Notes

131

QuoteReqID

UUID

Y

Unique identifier for RFQ echoed from Quote Request

117

QuoteID

UUID

Y

Unique identifier for Quote specified by Liquidity Provider

55

Symbol

String32

Y

Example: `BTC-AVAX`

132

BidPx

Float64

C

Required if submitting bid

133

OfferPx

Float64

C

Required if submitting offer

134

BidSize

String32

C

Required if submitting bid. Must match `OrderQty (38)` from Quote Request

135

OfferSize

String32

C

Required if submitting offer. Must match `OrderQty (38)` from Quote Request

### Quote Status Report (AI)

Quote Status Reports are sent to Liquidity Providers with [Quote](#quote-s) statuses and expired [Quote Requests](#quote-request-r).

-   If the [Quote](#quote-s) is rejected b/c validation checks failed or it was sent too late, the response to the quoter is `297=5` (QuoteStatus = Rejected).
-   If the [Quote](#quote-s) is accepted and eligible to participate in an RFQ auction, the response to the quoter is `297=16` (QuoteStatus = Active).
-   If the [Quote](#quote-s) is accepted but not selected for execution, the response to the quoter is `297=17` (QuoteStatus = Canceled).
-   If the [Quote](#quote-s) is accepted and selected for execution, the response to the quoter is `297=19` (QuoteStatus = Pending End Trade), followed by Execution Report - Filled.
-   If the [Quote Request](#quote-request-r) is unmatched by `ExpireTime (126)` on the Quote Request, `297=7` (QuoteStatus = Expired) is broadcast to all LPs.

Tag

Name

Type

Required

Notes

131

QuoteReqID

UUID

Y

Unique identifier for RFQ echoed from Quote Request

117

QuoteID

UUID

C

Unique identifier for Quote specified unless QuoteStatus = Expired (`297=7`)

55

Symbol

String32

Y

Example: `BTC-AVAX`

54

Side

Char

C

Buy: `54=1`, Sell: `54=2`  
Specified if QuoteStatus=Pending End Trade (`297=19`)

38

OrderQty

Float64

Y

Echoed from Quote Request

132

BidPx

Float64

C

Echoed from Quote

133

OfferPx

Float64

C

Echoed from Quote

134

BidSize

Float64

C

Echoed from Quote

135

OfferSize

Float64

C

Echoed from Quote

62

ValidUntilTime

UTCTimestamp

Y

Echoed from Quote Request

126

ExpireTime

UTCTimestamp

Y

Echoed from Quote Request

297

QuoteStatus

Int32

Y

`5` = **Rejected**: Quote failed validation checks or was sent too late  
`7` = **Expired**: Quote Request expired w/no match  
`16` = **Active**: Quote was acknowledged  
`17` = **Canceled**: Quote not selected b/c LP did not win auction or had insufficient funds  
`19` = **Pending End Trade**: Quote selected for execution

58

Text

String

C

Reason the Quote was rejected if QuoteStatus=5

Can also be “Unable to hold funds” if QuoteStatus=17

### RFQ Request (AH)

Request For Quote (RFQ) allows Liquidity Providers to respond to, and interact with, real-time RFQ requests. The RFQ process begins with [Quote Request (R)](#quote-request-r). RFQ is enabled for users who have been approved by Coinbase as a Liquidity Provider. Once approved, clients must send an RFQ Request message (`35=AH`) after each successful Logon message (`35=A`) for any session in which they are interested in receiving Quote Requests.

Tag

Name

Type

Required

Notes

644

RFQReqID

UUID

Y

Unique identifier for RFQ Request