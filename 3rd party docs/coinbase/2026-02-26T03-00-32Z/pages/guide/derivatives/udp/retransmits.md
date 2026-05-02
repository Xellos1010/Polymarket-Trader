# retransmits

A retransmit request can be sent to recover lost messages. The request message shares the same packet header as all channels/messages. The client may set the packet sequence number to any value.

-   If the retransmit request is rejected, the response packet sequence number is set to the request packet sequence number, and can be used as a correlationId.
-   If the retransmit request is successful, a single UDP packet is sent in response, containing messages beginning from BeginSeqNum.

The number of messages in the packet is the lesser of the number of requested messages or whatever number fit within the maximum packet size. Only one retransmit request is supported per packet. Additional messages in the same packet are ignored.

## Retransmit Request

Retransmit Request (200)

Type

Length

Offset

Description

BeginSeqNum

int64

8

10

Sequence number of first requested message

ReqMessageCount

uint8

1

18

Number of requested messages

## Retransmit Reject

Retransmit Reject (202)

Type

Length

Offset

Description

RetryDelayNanos

int64

8

10

Minimum time to wait in nanoseconds before sending another retransmit request

Details

char40

40

18

Retransmit reject reason in text

Reason

uint8

1

58

`1` - Sequence too low  
`2` - Sequence too high  
`3` - Rate limit exceeded  
`4` - Other error