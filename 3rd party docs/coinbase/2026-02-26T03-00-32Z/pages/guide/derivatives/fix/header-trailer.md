# header trailer

All FIX messages (administrative and business) require a [Standard Header](#standard-header) and [Standard Trailer](#standard-trailer) component.

## Standard Header

Tag

Name

FIX Type

Req

Description

8

BeginString

String(7)

Y

FIX version for session, e.g., `FIX.4.4`.

9

BodyLength

Int(6)

Y

Number of bytes in the message body.

35

MsgType

String(2)

Y

FIX message type.

34

MsgSeqNum

Int(9)

Y

Message sequence number.

43

PossDupFlag

Boolean(1)

N

Flag that represents whether or not the message may have been retransmitted with this sequence number. Must be `Y` for messages sent in response to a Resend Request from the exchange.

49

SenderCompID

String(6)

Y

Consists of SubFirmID and SessionID. SubFirmID is left-most 3 chars. SessionID is right-most 3 chars. Example: `EBR123`

To Exchange: Connection ID assigned by Coinbase Derivatives Exchange.

From Exchange: Value from order entry tag 56, TargetCompID: `COIND`

50

SenderSubID

String(18)

Y

Unique identifier for the end trader submitting orders, issued by an exchange clearing member firm (clearing member), their contracted vendors, or assignees

116

OnBehalfOfSubID

String(18)

N

Can be used to identify the end trader submitting orders if SenderSubID (50) is populated by a third party connection provider.

52

SendingTime

UTCTimestamp(21)

Y

Time the message was transmitted, expressed in UTC, microseconds: `YYYYMMDD-HH:MM:SS.ssssss`.

56

TargetCompID

String(7)

Y

To Exchange: ID of entity receiving the message: `COIND`

From Exchange: Value from order entry SenderCompID (49), example: `EBR123`

57

TargetSubID

String(20)

Y

To Exchange: ID of destination exchange system: PROD or TEST

From Exchange: Echo back TargetSubID (57) sent by client system

122

OrigSendingTime

UTCTimestamp(21)

C

For resent messages only, contains timestamp from SendingTime (52) from original message.

369

LastMsgSeqNumProcessed

Int(9)

N

MsgSeqNum (34) of the last message from the client received and processed by the gateway.

## Standard Trailer

Tag

Name

FIX Type

Req

Description

10

CheckSum

String(3)

Y

Always the last tag in a message. Functions as end-of-message delimiter.