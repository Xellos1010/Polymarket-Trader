# incrementals

Incremental update messages are sent on dual A/B multicast channels. Each message has a long (8-byte) monotonically increasing sequence number that is never reset. The stream of messages on the A/B channels are identical; however, the grouping of messages into packets may vary between A and B channels. The [packet header](https://developer.chrome.com/derivatives/udp/message-structure#packet-header) sequence number is the sequence number of the first message in the packet, or the next expected sequence number if the packet contains no messages (a heartbeat packet). Thus, the expected sequence number of the next packet is always the current packet sequence number plus the current packet message count.

All incremental messages are sent in a transaction. A transaction contains all incremental messages that are published as a result of a single inbound message or event in the trading system. Single-message transactions have both the `start-of-transaction` and `end-of-transaction` flags set.

All incremental messages contain the following component as the first field after the [message header](https://developer.chrome.com/derivatives/udp/message-structure#message-header). Side is null (`-128`) unless otherwise noted for particular messages.

Instrument Header

Type

Length

Offset

Description

Flags

uint8

1

0

Message header bitset:  
`0x01` - start of transaction  
`0x02` - end of transaction  
`0x04` - clear book (reserved for future use)

Side

int8

1

1

`1` - Buy  
`-1` - Sell  
`0` - opening fill  
`-128` - null value (used in messages in which side is not applicable)

InstrumentId

int32

4

2

Instrument identifier

InstrSeqNum

uint32

4

6

Per-instrument sequence number. Reset each trading day.

TradingSessionDate

int16

2

10

Days since Unix epoch

Padding2

int16

2

12

2 bytes of padding (reserved for future use)

TransactTime

int64

8

14

Event timestamp - nanoseconds since Unix epoch

## Outright Instrument Definition

Outright Instrument Definition (10)

Type

Length

Offset

Description

Symbol

char24

24

32

Instrument name or symbol

ProductCode

char8

8

56

Code of underlying product/asset. Example: TEC (Nano SuperTech Fut)

Description

char32

32

64

Instrument name

PriceIncrement

int64

8

96

Minimum constant tick for instrument, encoded with 9 decimal places

CfiCode

char8

8

104

ISO standard instrument categorization code

Currency

char8

8

112

Currency used for price

FirstTradingSessionDate

uint16

2

120

Days since Unix epoch

LastTradingSessionDate

uint16

2

122

Days since Unix epoch

OldContractSize

int32

4

124

Contract size encoded with 0 decimal places (deprecated from version 1.5 of this UDP Market Data API specification document).

PriorSettlementPrice

int64

8

128

Price encoded with 9 decimal places

SettlementPrice

int64

8

136

Price encoded with 9 decimal places

LimitDownPrice

int64

8

144

Minimum price at which an instrument can currently trade

LimitUpPrice

int64

8

152

Maximum price at which an instrument can currently trade

ProductId

int32

4

160

Product identifier

ProductGroup

uint8

1

164

  
`0` - Currency  
`1` - Equity  
`2` - Energy  
`3` - Metals  
`4` - Interest Rate  
`5` - Agriculture  
`6` - Crypto

TradingStatus

uint8

1

165

Trading session status  
`0` - Pre-open  
`1` - Open  
`2` - Halt  
`3` - Pause  
`4` - Close  
`5` - Pre-open (No Cancel)  
`6` - Expired  
`7` - Forbidden

InstrumentDefinitionFlags

uint16

2

166

Bitset  
`0x01` - isPriorSettlementTheoretical  
`0x02` - isAnnounced  
`0x04` - isCall (applicable for options)  
`0x08` - isStrikeDelisted  
`0x10` - fundingRateApplicable

ContractSize

int64

8

168

Contract size encoded with 8 decimal places (added in version 1.5 of this UDP Market Data API specification document)

FundingIntervalMinutes

int32

4

176

Time in minutes between funding periods

FairValueLimit

int32

4

180

Level up and down from fair value determining fair value limits

## Spread Instrument Definition

Spread Instrument Definition (11)

Type

Length

Offset

Description

Symbol

char24

24

32

Instrument name or symbol

ProductCode

char8

8

56

Code of underlying product/asset. Example: TEC (Nano SuperTech Fut)

Description

char32

32

64

Instrument name

PriceIncrement

int64

8

96

Minimum constant tick for instrument, encoded with 9 decimal places

CfiCode

char8

8

104

ISO standard instrument categorization code

Currency

char8

8

112

Currency used for price

FirstTradingSessionDate

uint16

2

120

Days since Unix epoch

LastTradingSessionDate

uint16

2

122

Days since Unix epoch

OldContractSize

int32

4

124

Contract size encoded with 0 decimal places (deprecated from version 1.5 of this UDP Market Data API specification document)

PriorSettlementPrice

int64

8

128

Price encoded with 9 decimal places

SettlementPrice

int64

8

136

Price encoded with 9 decimal places

LimitDownPrice

int64

8

144

Minimum price at which an instrument can currently trade

LimitUpPrice

int64

8

152

Maximum price at which an instrument can currently trade

ProductId

int32

4

160

Product identifier

ProductGroup

uint8

1

164

`0` - Currency  
`1` - Equity  
`2` - Energy  
`3` - Metals  
`4` - Interest Rate  
`5` - Agriculture  
`6` - Crypto

TradingStatus

uint8

1

165

Trading session status  
`0` - Pre-open  
`1` - Open  
`2` - Halt  
`3` - Pause  
`4` - Close  
`5` - Pre-open (No Cancel)  
`6` - Expired  
`7` - Forbidden

Leg1InstrumentId

int32

4

166

Instrument identifier for near leg

Leg2InstrumentId

int32

4

170

Instrument identifier for far leg

SpreadBuyConvention

int8

1

174

`1` - Use far leg as bid  
`-1` - Use near leg as bid

InstrumentDefinitionFlags

uint16

2

175

Bitset  
`0x01` - isPriorSettlementTheoretical  
`0x02` - isAnnounced  
`0x04` - isCall (applicable for options)  
`0x08` - isStrikeDelisted

## Option Instrument Definition

Option Instrument Definition (12)

Type

Length

Offset

Description

Symbol

char24

24

32

Instrument name or symbol

ProductCode

char8

8

56

Code of underlying product/asset. Example: TEC (Nano SuperTech Fut)

Description

char32

32

64

Instrument name

SmallTick

int64

8

96

Small tick size, encoded with 9 decimal places

CfiCode

char8

8

104

ISO standard instrument categorization code

LargeTick

int64

8

112

Large tick size, encoded with 9 decimal places

LargeTickThreshold

int64

8

120

Large tick size applies if price is >= this threshold price, encoded with 9 decimal places

StrikePrice

int64

8

128

Strike price, encoded with 9 decimal places

FirstTradingSessionDate

uint16

2

136

Days since Unix epoch

LastTradingSessionDate

uint16

2

138

Days since Unix epoch

PriorSettlementPrice

int64

8

140

Price encoded with 9 decimal places

SettlementPrice

int64

8

148

Price encoded with 9 decimal places

ProductId

int32

4

156

Product identifier

UnderlyingInstrumentId

int32

4

160

Instrument id of the underlying outright contract

ProductGroup

uint8

1

164

`0` - Currency  
`1` - Equity  
`2` - Energy  
`3` - Metals  
`4` - Interest Rate  
`5` - Agriculture  
`6` - Crypto

TradingStatus

uint8

1

165

Trading session status  
`0` - Pre-open  
`1` - Open  
`2` - Halt  
`3` - Pause  
`4` - Close  
`5` - Pre-open (No Cancel)  
`6` - Expired  
`7` - Forbidden

InstrumentDefinitionFlags

uint16

2

166

Bitset  
`0x01` - isPriorSettlementTheoretical  
`0x02` - isAnnounced  
`0x04` - isCall (applicable for options)  
`0x08` - isStrikeDelisted

## Trading Status Update

Trading Status Update (17)

Type

Length

Offset

Description

LimitDownPrice

int64

8

32

Minimum price at which an instrument can currently trade

LimitUpPrice

int64

8

40

Maximum price at which an instrument can currently trade

TradingStatus

uint8

1

48

Trading session status:  
`0` - Pre-open  
`1` - Open  
`2` - Halt  
`3` - Pause  
`4` - Close  
`5` - Pre-open (No Cancel)  
`6` - Expired  
`7` - Forbidden

## Order Put

Sent when a resting order is added or updated.

Order Put (20)

Type

Length

Offset

Description

OrderId

int64

8

32

Unique order ID.  
Unique across entire channel and across time, but not necessarily across different channels. Never reused for an unrelated order.

Price

int64

8

40

Price encoded with 9 decimal places

Quantity

int32

4

48

Quantity encoded with 0 decimal places

## Order Delete

`Side` of deleted order is defined in instrument header (1 or -1).

Order Delete (21)

Type

Length

Offset

Description

OrderId

int64

8

32

Unique identifier for order

## Implied Order Update

`Side` is defined in instrument header (1 or -1).

Implied Order Update (22)

Type

Length

Offset

Description

BestPrice

int64

8

32

First level implied price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

NextPrice

int64

8

40

Second level implied price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

BestQty

int32

4

48

First level implied quantity encoded with 0 decimal places

NextQty

int32

4

52

Second level implied quantity encoded with 0 decimal places

## Trade Summary

Summarizes all fills of an aggressor order. Sent before individual trade messages, as well as order put/delete and market stat messages.

Trade Summary (33)

Type

Length

Offset

Description

AggressorOrderId

int64

8

32

Order identifier of aggressing order

AggressorReceiveTime

int64

8

40

Nanoseconds since Unix epoch when we received aggressor new/replace order message on gateway

VwapPrice

int64

8

48

Volume weighted average price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

DeepestPrice

int64

8

56

Price of deepest/last resting order that an aggressing order matched

Quantity

int32

4

64

Quantity encoded with 0 decimal places

## Trade

Trade message does not implicitly delete or update matched resting order; a separate `OrderPut` or `OrderDelete` is sent (in the same transaction).

Trade (30)

Type

Length

Offset

Description

MatchId

int64

8

32

Transaction id representing match, shared by all trades within match

BuyOrderId

int64

8

40

Unique identifier for trade buy order. In case of implied order, encoded as `0x8000000000000000`

SellOrderId

int64

8

48

Unique identifier for trade sell order. In case of implied order, encoded as `0x8000000000000000`

Price

int64

8

56

Price encoded with 9 decimal places

Quantity

int32

4

64

Quantity encoded with 0 decimal places

## Trade Amend

Trade Amend (31)

Type

Length

Offset

Description

MatchId

int64

8

32

Transaction id representing match, shared by all trades within match

BuyOrderId

int64

8

40

Unique identifier for trade buy order. In case of implied order, encoded as `0x8000000000000000`

SellOrderId

int64

8

48

Unique identifier for trade sell order. In case of implied order, encoded as `0x8000000000000000`

OldPrice

int64

8

56

Price encoded with 9 decimal places

NewPrice

int64

8

64

Price encoded with 9 decimal places

## Trade Bust

Trade Bust (32)

Type

Length

Offset

Description

MatchId

int64

8

32

Transaction id representing match, shared by all trades within match

BuyOrderId

int64

8

40

Unique identifier for trade buy order. In case of implied order, encoded as `0x8000000000000000`

SellOrderId

int64

8

48

Unique identifier for trade sell order. In case of implied order, encoded as `0x8000000000000000`

## Spread Trade Amend

Spread Trade Amend (34)

Type

Length

Offset

Description

MatchId

int64

8

32

Transaction id representing match, shared by all trades within match

BuyOrderId

int64

8

40

Unique identifier for trade buy order. In case of implied order, encoded as `0x8000000000000000`

SellOrderId

int64

8

48

Unique identifier for trade sell order. In case of implied order, encoded as `0x8000000000000000`

OldPrice

int64

8

56

Price encoded with 9 decimal places

NewPrice

int64

8

64

Price encoded with 9 decimal places

OldLeg1Price

int64

8

72

Spread leg price encoded with 9 decimal places

NewLeg1Price

int64

8

80

Spread leg price encoded with 9 decimal places

OldLeg2Price

int64

8

88

Spread leg price encoded with 9 decimal places

NewLeg2Price

int64

8

96

Spread leg price encoded with 9 decimal places

## Market Stat

Market Stat (40)

Type

Length

Offset

Description

Price

int64

8

32

Price encoded with 9 decimal places

StatType

char

1

40

`4` - Day Opening Price  
`5` - Closing Price  
`6` - Settlement Price  
`7` - Trading Session High Price  
`8` - Trading Session Low Price  
`F` - Reference Price  
`I` - Initial Opening Price

## Trade Session Volume

Trade Session Volume (41)

Type

Length

Offset

Description

VwapPrice

int64

8

32

Volume weighted average price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

TradeVolume

int32

4

40

Total day traded volume for instrument as of the last trade in message

## Open Interest

Open Interest (42)

Type

Length

Offset

Description

Quantity

int32

4

32

Quantity encoded with 0 decimal places

## Funding Rate

Funding Rate (43)

Type

Length

Offset

Description

FundingRate

int64

8

40

Funding rate encoded with 9 decimal places

FuturesMarkPrice

int64

8

48

Futures mark price encoded with 9 decimal places

SpotMarkPrice

int64

8

56

Spot mark price encoded with 9 decimal places

FairValue

int64

8

64

Fair value encoded with 9 decimal places

FinalFundingRateTimestamp

int64

8

72

Timestamp when funding rate becomes final - nanoseconds since Unix epoch. Null value encoded as `0x8000000000000000`

CorrelationId

int64

8

80

Correlation ID

Flags

uint8

1

88

Flags bitset:  
`0x01` - IsFinal (indicates if this is the final funding rate)