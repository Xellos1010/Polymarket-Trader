# session

The Session protocol assures client identification, sequential request processing, session state control, and the ability to restore the session after downtime.

Session Message Types

Client and server use the following administrative/session messages:

-   [Logon (35=A)](#logon-35a): Initiates (client) or approves (server) session opening.
-   [Logout (35=5)](#logout-355): Initiates or approves session closing.
-   [Resend Request (35=2)](#resend-request-352): Requests missed FIX messages.
-   [Sequence Reset (35=4)](#sequence-reset-354): Gap Fill, must be used instead of resend of administrative messages.
-   [Test Request (35=1)](#test-request-351): Controls the session state. Requires Heartbeat reply with `TestReqID (112)`.
-   [Heartbeat (35=0)](#heartbeat-350): Controls the session connection state.
-   [Reject (35=3)](#reject-353): For administrative message reject.

## Logon (35=A)

The Logon message initiates the connection from the client side and approves the connection if sent by the exchange.

Tag

Name

FIX Type

Req

Description

98

EncryptMethod

Int

Y

Encryption method where `0`\= None. Security must be guaranteed at the transport level

108

HeartBtInt

Int

Y

Heartbeat interval in seconds. Valid values are between 5s and 60s—30s is recommended. The same value is used by both parties. Value is set by the initiator and echoed back by the acceptor.

141

ResetSeqNumFlag

Boolean

N

Flag used to reset the session sequence numbers and start a new session.

**Use with caution**, especially during a trading session as this might lead to business data loss.

-   `N` = Use previous sequences
-   `Y` = Reset sequences (start new session)

If client cannot recover previous session, start new session with `1` and set this field to `Y`.

553

Username

Y

Username

554

Password

Y

Password

## Logout (35=5)

The Logout message initiates or confirms the termination of a FIX session.

Tag

Name

FIX Type

Req

Description

58

Text

String (200)

N

Logout reason

## Resend Request (35=2)

The Resend Request message can be used to recover an inbound session sequence if a message was missed.

Tag

Name

FIX Type

Req

Description

7

BeginSeqNo

SeqNum

Y

Sequence number of first message in range to be resent

16

EndSeqNo

SeqNum

Y

Sequence number of last message in range to be resent

## Sequence Reset (35=4)

The Sequence Reset message can be used in two modes:

#### Reset Mode

-   Reset Mode forces the counterparty to adjust inbound message sequence, GapFillFlag = “N” or omitted.

#### Fill Gap Mode

-   Fill Gap Mode is used during retransmission of messages missed by a client. Administrative messages and rejected business messages are not to be retransmitted. Instead a Sequence Reset message with GapFillFlag = “Y” must be used.

Tag

Name

FIX Type

Req

Description

123

GapFillFlag

String (20)

Y

Flag signalling wether to restore missed business/admin messages.

-   `123=N`: (Reset Mode) Sequence was reset, counterparty must adjust inbound sequence number
-   `123=Y`: (Fill Gap Mode) Missed messages (business messages) were gap-filled.

Default is `123=N`. Derivatives exchange sends `35=4` without gap-filling missed business messages.

36

NewSeqNo

String (20)

Y

New adjusted sequence number

## Test Request (35=1)

Test Request lets you check sequence numbers, or verify the communication line status in conjunction with a Heartbeat message.

Tag

Name

FIX Type

Req

Description

58

TestReqID

String (20)

Y

Id sent to verify communication status. Recipient returns TestReqID in Heartbeat message

## Heartbeat (35=0)

Heartbeat confirms the status of a communication line by replying to a Test Request message.

Tag

Name

FIX Type

Req

Description

112

TestReqID

String (20)

Y

Id sent in response to Test Request message, verifying communication status

## Reject (35=3)

A Reject message is issued by a party if an incoming FIX message is unsupported or not property formed. Rejected messages must not be resent if a Resend Request is received; instead a SequenceReset with [GapFillFlag = “Y”](#fill-gap-mode) is expected.

Tag

Name

FIX Type

Req

Description

45

RefSeqNum

SeqNum

Y

Sequence number of the rejected message

58

Text

String (200)

Y

Error message

373

SessionRejectReason

Int (2)

Y

[Session Reject Reason Code](https://developer.chrome.com/derivatives/fix/code-sets#sessionrejectreason-373)