# admin messages

## Standard

### Logon (35=A)

First message that is required immediately upon connection to authenticate the connection.

Tag

Name

Type

Required

Description

98

EncryptMethod

int

N

Must be 0 (None)

108

HeartBtInt

int

O

Must be ≤ 30 (secs). Values greater are capped at 30. Server sends Test Request if client messages are not received in approximately (HeartBtInt x 1.5) seconds. Server terminates session if client messages are not received in approximately (HeartBtInt x 2 seconds). Defaults to 10 seconds if not value provided.

141

ResetSeqNumFlag

boolean

Y

Sequence numbers always get reset after a disconnect. Defaults to Y and a value of N will result in a Reject on the Login message.

553

Username

string

Y

Client API Key

554

Password

string

Y

Passphrase for the API key

58

Text

string

Y

A base64 encoded signature generated using HMAC SHA-256 with the secret of the API Key specified in Username (553) and the following payload: `Time + Client API Key + Session + Passphrase`

Where time is the value in SendingTime (52), Client API key is the value in Username (553), Session is the value in TargetCompID (56), and Passphrase is the value in Password (554).

**Important:** SendingTime (52) ***must be*** in UTC milliseconds `YYYYMMDD-HH:MM:SS.sss`.

1137

DefaultApplVerID

string

Y

Contains the version of the FIX protocol the exchange uses. Only FIX50SP2 is supported. Supported values: `9`

8001

DefaultSelfTradePreventionStrategy

char

N

The default SelfTradePreventionStrategy applied to all orders sent on the session unless overridden on a per order basis using the SelfTradePreventionStrategy (8000) in the order request message. The following values specify what to do when two orders submitted by the same Organization/Account attempt to match:

N = Cancel aggressing order  
Q = Cancel both orders

Default if not specified is Cancel both orders (Q).

8013

CancelOrdersOnDisconnect

char

N

Cancels all the open orders upon a disconnection. Only applies to Order Entry sessions. The configuration values include:

N = No cancel on disconnect  
Y = Only cancels orders from this session  
A = Cancels all open orders for this API key

Defaults to Y, only cancel orders from this session, if the tag is not provided.

Note: using the session level cancel on disconnect may result in cancellations even when the FIX connection remains healthy. This can occur when the FIX gateway momentarily loses connectivity to its downstream component due to the component getting upgraded or from an AWS failure.

8014

CancelOrdersOnInternalDisconnect

boolean

N

Cancels all the open orders upon an internal disconnection. This happens if the FIX gateway experiences a disconnect to internal systems or other internal applications experience a disconnect that would impact the ability to cancel orders. These internal disconnects can occur from any scenario including planned software updates or unexpected technical issues. Only applies to Order Entry sessions. The configuration values include:

N = No cancel on internal disconnect  
Y = Cancel on internal disconnect

Defaults to Y, cancel orders on internal disconnect, if the tag is not provided.

### Heartbeat (35=0)

Sent at a prearranged interval from both sides to indicate liveness of the connections and used in response to a TestRequest message (35=1).

Tag

Name

Type

Required

Description

112

TestReqID

string

C

Conditionally required when the heartbeat message is sent in response to a TestRequest (35=1) message.

### TestRequest (35=1)

This message forces the other side of the connection to send a heartbeat message (35=0) with the TestReqID (tag 112) populated with the same value provided on this message. |

Tag

Name

Type

Required

Description

112

TestReqID

string

Y

A unique identifier used to track the response to a test request

### Reject (35=3)

A session level reject message sent when the FIX session can’t process a message.

Tag

Name

Type

Required

Description

45

RefSeqNum

int

Y

The MsgSeqNum of the referenced message that was rejected.

371

RefTagID

int

N

The tag number of the FIX field referenced in the reject.

372

RegMsgType

string

N

The MsgType of the FIX message referenced in the reject.

373

SessionRejectReason

int

N

A code to quickly identify common reasons for a reject. Values could include:

0 = Invalid Tag Number  
1 = Required Tag Missing  
2 = Tag not defined for this message type  
3 = Undefined tag  
4 = Tag specified without a value  
5 = Value is incorrect (out of range) for this tag  
6 = Incorrect data format for value  
8 = Signature problem  
9 = CompID problem  
10 = SendingTime Accuracy Problem (SendingTime must be within 1 second of server time)  
11 = Invalid MsgType  
13 = Tag appears more than once  
14 = Tag specified out of required order  
15 = Repeating group fields out of order  
16 = Incorrect NumInGroup count for repeating group  
17 = Non-data value includes field delimiter (`<SOH>` character)  
18 = Invalid/Unsupported Application Version  
99 = Other

58

Text

string

N

A message explaining why the message was rejected.

### Logout (35=5)

Sent by either side to terminate the FIX session

Tag

Name

Type

Required

Description

58

Text

string

N

Description of the disconnection reason

## Replay

### LastExecIDRequest (35=F1)

Send this message to request the execId of the last (most recent) event sent by the trading system to this user/session. Can be used to determine if the client missed any events while disconnected. Also serves as a means of validating that the trading system is available and accepting requests.

Tag

Name

Type

Required

Description

112

TestReqID

string

Y

A unique identifier for the LastExecIDRequest

### LastExecID (35=F2)

Sent in response to LastExecIdRequest.

Tag

Name

Type

Required

Description

45

RefSeqNum

int

Y

MsgSeqNum (tag 34) of request message.

17

ExecID

string

Y

ExecID of the last event sent to this user.

### EventResendRequest (35=F3)

Send this message to request order events in the specified range be resent. Since this is an application-level request, resent messages will have new sequence numbers and PossDupFlag (tag 43) will not be set; rather, PossResend (tag 97) will be set. Rejects (and any other message that does not contain an execId) will not be resent. Only available on the drop copy session.

Tag

Name

Type

Required

Description

22003

BeginExecID

string

Y

Lower bound (inclusive) of execIds.

22004

EndExecID

string

N

Upper bound (inclusive) of execIds.  
Resend all events up to the most current event if not set.

### EventResendComplete (35=F4)

Sent in response to a successful Event Resend Request following all resent events. Only available on the drop copy session.

Tag

Name

Type

Required

Description

45

RefSeqNum

int

Y

MsgSeqNum (tag 34) of request message.

22005

ResentEventCount

int

Y

Total number of events resent.

### EventResendReject (35=F5)

Sent in response to an Event Resend Request if the request cannot be fulfilled. Only available on the drop copy session.

Tag

Name

Type

Required

Description

45

RefSeqNum

int

Y

MsgSeqNum (tag 34) of request message.

22006

EventResendRejectReason

int

Y

Code identifying reject reason:

1 = BeginExecID is too small  
2 = EndExecID is too large  
3 = Resend already in progress  
4 = Too many resend requests  
5 = Server error

58

Text

string

N

Description of the reject reason