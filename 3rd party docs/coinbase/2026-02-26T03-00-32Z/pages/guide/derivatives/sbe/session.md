# session

Session Message Types

Client and server use the following administrative/session messages:

-   [Logon](#logon): Initiates (client) or approves (server) session opening.
-   [LogonConf](#logonconf): Initiates (client) or approves (server) session opening.
-   [Heartbeat](#heartbeat): Controls the session connection state.
-   [TestRequest](#testrequest): Controls the session state. Requires Heartbeat reply.
-   [ResendRequest](#resendrequest): Requests missed FIX messages.
-   [GapFill](#gapfill): Gap Fill, must be used instead of resend of administrative messages.
-   [Logout](#logout): Initiates or approves session closing.
-   [LoggedOut](#loggedout):
-   [Reject](#reject): For administrative message reject.

## Logon

Each client uses an assigned IP address and port to establish a TCP/IP session with the server. The client initiates a session at the start of each trading day by sending the Logon message **within a two heartbeats interval**. The client identifies itself with the `username` field. The server validates the `username` and `password` of the client.

Field

Name

Type

Length

Description

Logon

100

81

Sent to initiate connections

1

username

String16

16

Logon username

2

password

String32

32

Logon password

3

resetSeqNum

uint8

1

(`1` = true, `0` = false)

## LogonConf

Once the client is successfully authenticated, the server responds with a “Logon Confirmations” message.

Field

Name

Type

Length

Description

LogonConf

200

36

Logon confirmation

1

heartbeatIntervalSeconds

Int32

4

Heartbeat interval in seconds.

## Heartbeat

Client and server use the Heartbeat message to exercise the communication line during periods of inactivity and to verify that the interfaces at each end are available. The heartbeat interval is three seconds. The server sends a Heartbeat anytime it has not transmitted a message during a heartbeat interval. The client is expected to employ the same logic. The servers sends a logout and breaks the TCP/IP connection with the client if it detects inactivity for five heartbeat intervals. The client is expected to employ similar logic if inactivity is detected on the part of the server.

Field

Name

Type

Length

Description

Heartbeat

10

40

Connection Heartbeat, may also be sent in response to a TestRequest

1

correlationId

int64

8

Optional id if sent in response to a TestRequest

## TestRequest

TestRequest is used to force a heartbeat from the opposing application. The message is useful for checking sequence numbers or verifying communication line status. The opposite application responds to the TestRequest with a Heartbeat.

Field

Name

Type

Length

Description

TestRequest

11

40

TestRequest to request heartbeat, receiver should respond with Heartbeat message with the provided test request id included

1

correlationId

int64

8

Correlation id to be echoed by receiver

## ResendRequest

The client may send a ResendRequest to initiate retransmission of previously sent messages. Like the [FIX protocol](https://developer.chrome.com/derivatives/fix/session#fill-gap-mode), GapFill messages are sent in place of admin and missing/unavailable messages.

Field

Name

Type

Length

Description

ResendRequest

102

40

Resend request.

1

fromSequenceNumber

uint32

4

Sequence number of first message to resend.

2

toSequenceNumber

uint32

4

Sequence number of last message to resend (or 0 to resend all messages from formSequenceNumber).

## GapFill

While retransmitting messages in response to a ResendRequest, the server sends GapFill messages in place of admin and missing/unavailable messages.

Field

Name

Type

Length

Description

ResendRequest

202

40

Resend request

1

newSequenceNumber

uint32

4

Sequence number of the next message to be sent.

2

padding

uint32

4

## Logout

The client is expected to terminate each connection at the end of each trading day before the server shuts down. The client terminates a connection by sending the Logout message. The server responds with a Logout message if the client’s request is successful. The client then breaks the TCP/IP connection with the server. All open TCP/IP connections are terminated by the server when it shuts down (a Logout is sent). Under exceptional circumstances, the server may initiate the termination of a connection during the trading day by sending the Logout message. Either party that wishes to terminate the connection may wait for the heartbeat interval duration before breaking the TCP/IP connection to ensure that the other party received the Logout message.

Field

Name

Type

Length

Description

Logout

101

96

Logout message to gateway

1

reason

String64

64

Logout reason

## LoggedOut

The server sends a LoggedOut message before terminating the connection, either in response to a Logout message from the or for other reasons.

Field

Name

Type

Length

Description

Logout

201

96

Logout message from gateway to client

1

details

String64

64

Logout details

## Reject

The server sends a Reject message in response to a client message which is well-formed but is of unknown type or has an invalid [blockLength](https://developer.chrome.com/derivatives/sbe/header) for the type/version.

Field

Name

Type

Length

Description

Reject

210

104

Reject message from gateway to client

1

refSequenceNumber

uint32

4

Sequence number of the rejected message from client.

2

reason

int32

4

-   `1` = INVALID\_SCHEMA\_ID
-   `2` = INVALID\_TEMPLATE\_ID
-   `3` = INVALID\_BLOCK\_LENGTH

3

details

String64

64

Logout details