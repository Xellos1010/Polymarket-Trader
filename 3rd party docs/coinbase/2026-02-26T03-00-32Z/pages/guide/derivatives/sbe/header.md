# header

Each message begins with the following SBE message header:

Field

Name

Type

Length

Description

1

protocolId

uint8

1

Constant (= `0xF1`)

2

flags

uint8

1

Bitset of flags. `0x01 = resend`

3

messageLength

uint16

2

Total length of message including this header and body.

4

sequenceNumber

uint32

4

Sequence number of message.

5

lastProcessedSeqNum

uint32

4

Sequence number of last message received/processed by the sender of this message.

6

reserved

4

Padding

7

sendTimeEpochNanos

int64

8

Sending time in nanoseconds since epoch.

8

blockLength

uint16

2

Length of message root block in bytes, before variable data commences.

9

templateId

uint16

2

Message type Id

10

schemaId

uint16

2

Message schema Id (`1100` for admin/session messages, `1101` for orders)

11

version

uint16

2

Message version number