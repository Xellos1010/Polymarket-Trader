# common components

## Standard Header

Fields that go at the beginning of every message.

Tag

Name

Type

Required

Description

8

BeginString

String

Y

Must be set to `FIXT.1.1`

Since FIX version 5.0 this field represents the session version. The application version gets specified in Logon message’s DefaultApplVerID (1137) tag.

9

BodyLength

Int

Y

Message length in bytes up to the checksum field, must be the second field in message

35

MsgType

String

Y

The type of message proceeding the header, must be the third field in the message. Supported values include:

All Sessions  
A = Logon  
0 = Heartbeat  
1 = TestRequest  
3 = Reject  
5 = Logout

Order Entry  
D = NewOrderSingle  
F = OrderCancelRequest  
G = OrderCancelReplaceRequest  
8 = ExecutionReport  
9 = OrderCancelReject  
F1 = LastExecIDRequest  
F2 = LastExecID

Market Data  
x = SecurityListRequest  
y = SecurityList  
d = SecurityDefinition  
V = MarketDataRequest  
Y = MarketDataRequestReject  
W = MarketDataSnapshotFullRefresh  
X = MarketDataIncrementalRefresh

Drop Copy  
8 = ExecutionReport  
F1 = LastExecIDRequest  
F2 = LastExecID  
F3 = EventResendRequest  
F4 = EventResendComplete  
F5 = EventResendReject

49

SenderCompID

String

Y

The API key assigned by the exchange.

56

TargetCompID

String

Y

Defined based on the type of FIX session:

CBINTLOE = Order Entry  
CBINTLMD = Market Data  
CBINTLDC = Drop Copy

34

MsgSeqNum

Int

Y

Monotonically increasing sequence number

97

PossResend

Boolean

C

Indicates if the message may have already been sent with a different sequence number. This is useful when performing an application level replay on the drop copy session.

N = Original transmission  
Y = Possible resend

52

SendingTime

UTC timestamp

Y

When sending a message to Coinbase International, SendingTime should be in milliseconds expressed in UTC: `YYYYMMDD-HH:MM:SS.sss`. When receiving a message from Coinbase International SendingTime will be in microseconds expressed in UTC: `YYYYMMDD-HH:MM:SS.ssssss`. SendingTime must be within 1 second of the server’s current time, or the message will be rejected with a SendingTime Accuracy Problem (SessionRejectReason=10).

## Standard Trailer

Fields that go at the end of every message

Tag

Name

Type

Required

Description

10

CheckSum

String

Y

Three byte checksum calculated by summing every byte in the message up to and not including the checksum field itself. This value is then moduloed by 256 and written with prefixed 0s (if necessary) to meet the 3 byte requirement.

## Parties

A repeating group of identifiers that indicates components of the user model (e.g. portfolio) affiliated with the message.

Tag

Name

Type

Required

Description

453

NoPartyIDs

int

C

Marks the beginning of the Parties repeating group component. Currently a max of 1 party is supported to specify the portfolio UUID. In the future additional parties may get added.

Supported values:  
1 = Specify portfolio UUID or client UUID

If no party is specified the message applies to the default portfolio UUID affiliated with the API key.

```
→ 448
```

PartyID

string

C

The unique identifier used to represent the party entry.

```
→ 452
```

PartyRole

int

C

The type of party entry that the PartyID (448) value represents. Currently supports:

3 = Client ID  
24 = Customer account

The Customer Account (24) value indicates the PartyID value contains the UUID of the portfolio affiliated with the message. The Client ID (3) value indicates the PartyID value contains the UUID of a client managed by the broker (only applies to brokers).