# snapshots

Per-instrument, snapshots are sent continuously on the dual A/B snapshot channels. The frequency of repeated snapshots is intentionally not specified, but the maximum delay between snapshots (for different instruments) is 5 seconds. A single (instrument) snapshot consists of 2 or more messages, starting with instrument-type-specific start message, which includes the instrument definition, followed by an order snapshot message for each active order of the instrument, followed by an `EndOfSnapshot` message. The snapshot messages may be split across multiple packets if the full snapshot does not fit in a single packet. All packets of the snapshot share the same packet sequence number, which corresponds to the most recent incremental feed sequence number included in the snapshot. All snapshot messages have `SnapshotSeqNum` as their first field. This sequence number always starts at 0 for the first message of a snapshot and does not wrap around. A single instrument does not have more than 65534 active orders. The end of cycle message represents the end of a cycle where all active instruments in our system have sent their respective snapshots. The message has a count of the number of instruments sent since the previous cycle so that end users can verify they have all required snapshots and check against packet loss.

## Start of Outright Instrument Snapshot

Start Of Outright Instrument Snapshot (110)

Type

Length

Offset

Description

SnapshotSeqNum

uint16

2

10

Sequence number of message within snapshot

LastInstrSeqNum

uint32

4

12

Snapshot incorporates all incremental messages with `instrSeqNum` up and including this value

Symbol

char24

24

16

Instrument name or symbol

ProductCode

char8

8

40

Code of underlying product/asset. Example: TEC (Nano SuperTech Fut)

Description

char32

32

48

Instrument name

PriceIncrement

int64

8

80

Minimum constant tick for instrument, encoded with 9 decimal places

CfiCode

char8

8

88

ISO standard instrument categorization code

Currency

char8

8

96

Currency used for price

ProductId

int32

4

104

Product identifier

OldContractSize

int32

4

108

Contract size encoded with 0 decimal places (deprecated from version 1.5 of this UDP Market Data API specification document)

OrderCount

int32

4

112

Number of orders in snapshot

FirstTradingSessionDate

uint16

2

116

Days since Unix epoch

LastTradingSessionDate

uint16

2

118

Days since Unix epoch

TradingSessionDate

int16

2

120

Days since Unix epoch

ProductGroup

uint8

1

122

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

123

Trading session status  
`0` - Pre-open  
`1` - Open  
`2` - Halt  
`3` - Pause  
`4` - Close  
`5` - Pre-open (No Cancel)  
`6` - Expired

ContractSize

int64

8

124

Contract size encoded with 8 decimal places (added in version 1.5 of this UDP Market Data API specification document)

LogicalExpiry

LogicalExpiry

8

132

Contract expiration  
`1-2`: year  
`3-4`: month  
`5-6`: weekOfMonth  
`7-8`: dayOfMonth

FundingIntervalMinutes

int32

4

140

Time in minutes between funding periods

FairValueLimit

int32

4

144

Level up and down from fair value determining fair value limits

## Start of Spread Instrument Snapshot

Start Of Spread Instrument Snapshot (111)

Type

Length

Offset

Description

SnapshotSeqNum

uint16

2

10

Sequence number of message within snapshot

LastInstrSeqNum

uint32

4

12

Snapshot incorporates all incremental messages with instrSeqNum up and including this value

Symbol

char24

24

16

Instrument name or symbol

ProductCode

char8

8

40

Code of underlying product/asset. Example: TEC (Nano SuperTech Fut)

Description

char32

32

48

Instrument name

PriceIncrement

int64

8

80

Minimum constant tick for instrument, encoded with 9 decimal places

CfiCode

char8

8

88

ISO standard instrument categorization code

Currency

char8

8

96

Currency used for price

ProductId

int32

4

104

Product identifier

OldContractSize

int32

4

108

Contract size encoded with 0 decimal places (deprecated from version 1.5 of this UDP Market Data API specification document)

OrderCount

int32

4

112

Number of orders in snapshot

FirstTradingSessionDate

uint16

2

116

Days since Unix epoch

LastTradingSessionDate

uint16

2

118

Days since Unix epoch

TradingSessionDate

int16

2

120

Days since Unix epoch

ProductGroup

uint8

1

122

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

123

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

124

Instrument identifier for near leg

Leg2InstrumentId

int32

4

128

Instrument identifier for far leg

SpreadBuyConvention

int8

1

132

  
`1` - Use far leg as bid  
`-1` - Use near leg as bid

## Start of Option Instrument Snapshot

Start Of Option Instrument Snapshot (112)

Type

Length

Offset

Description

SnapshotSeqNum

uint16

2

10

Sequence number of message within snapshot

LastInstrSeqNum

uint32

4

12

Snapshot incorporates all incremental messages with instrSeqNum up and including this value

Symbol

char24

24

16

Instrument name or symbol

ProductCode

char8

8

40

Code of underlying product/asset. Example: TEC (Nano SuperTech Fut)

Description

char32

32

48

Instrument name

SmallTick

int64

8

80

Small tick size, encoded with 9 decimal places

CfiCode

char8

8

88

ISO standard instrument categorization code

LargeTick

int64

8

96

Large tick size, encoded with 9 decimal places

LargeTickThreshold

int64

8

104

Large tick size applies if price is >= this threshold price, encoded with 9 decimal places

StrikePrice

int64

8

112

Strike price, encoded with 9 decimal places

ProductId

int32

4

120

Product identifier

UnderlyingInstrumentId

int32

4

124

Instrument id of the underlying outright contract

OrderCount

int32

4

128

Number of orders in snapshot

FirstTradingSessionDate

uint16

2

132

Days since Unix epoch

LastTradingSessionDate

uint16

2

134

Days since Unix epoch

TradingSessionDate

int16

2

136

Days since Unix epoch

ProductGroup

uint8

1

138

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

139

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

140

Bitset:  
`0x01` - isPriorSettlementTheoretical  
`0x02` - isAnnounced  
`0x04` - isCall (applicable for options)  
`0x08` - isStrikeDelisted

## Order Snapshot

Order Snapshot (120)

Type

Length

Offset

Description

SnapshotSeqNum

uint16

2

10

Sequence number of message within snapshot

SignedQuantity

int32

4

12

Signed quantity with 0 decimal places. Positive value = buy; negative value = sell

TransactTime

int64

8

16

Event timestamp - nanoseconds since Unix epoch

OrderId

int64

8

24

Unique identifier for order

Price

int64

8

32

Price encoded with 9 decimal places

## End of Snapshot

End Of Snapshot (122)

Type

Length

Offset

Description

SnapshotSeqNum

uint16

2

10

Sequence number of message within snapshot

TradeVolume

int32

4

12

Total day traded volume for instrument as of the last trade in message

IndicativeOpenPrice

int64

8

16

Price encoded with 9 decimal places

DayOpenPrice

int64

8

24

Price encoded with 9 decimal places

ClosePrice

int64

8

32

Price encoded with 9 decimal places

LowPrice

int64

8

40

Price encoded with 9 decimal places

HighPrice

int64

8

48

Price encoded with 9 decimal places

VwapPrice

int64

8

56

Volume weighted average price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

SettlementPrice

int64

8

64

Price encoded with 9 decimal places

LastTradePrice

int64

8

72

Price encoded with 9 decimal places

LastTradeTime

int64

8

80

Nanoseconds since Unix epoch

BestBidImpliedPrice

int64

8

88

First level implied price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

BestAskImpliedPrice

int64

8

96

First level implied price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

NextBidImpliedPrice

int64

8

104

Second level implied price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

NextAskImpliedPrice

int64

8

112

Second level implied price encoded with 9 decimal places. Null price encoded as `0x8000000000000000`

LimitDownPrice

int64

8

120

Minimum price that an instrument may currently trade at

LimitUpPrice

int64

8

128

Maximum price that an instrument may currently trade at

LastTradeQty

int32

4

136

Quantity encoded with 0 decimal places

OpenInterest

int32

4

140

The total open interest for the market at the close of the prior trading session

BestBidImpliedQty

int32

4

144

First level implied quantity encoded with 0 decimal places

BestAskImpliedQty

int32

4

148

First level implied quantity encoded with 0 decimal places

NextBidImpliedQty

int32

4

152

Second level implied quantity encoded with 0 decimal places

NextAskImpliedQty

int32

4

156

Second level implied quantity encoded with 0 decimal places

PriorSettlementPrice

int64

8

160

Price encoded with 9 decimal places

InstrumentDefinitionFlags

uint16

2

168

”Bitset:  
`0x01`\- isPriorSettlementTheoretical  
`0x02` - isAnnounced  
`0x04` - isCall (applicable for options)  
`0x08` - isStrikeDelisted  
`0x10` - fundingRateApplicable”

FinalFundingRate

int64

8

170

Final funding rate encoded with 9 decimal places. Null value encoded as `0x8000000000000000`

FinalFuturesMarkPrice

int64

8

178

Final futures mark price encoded with 9 decimal places. Null value encoded as `0x8000000000000000`

FinalFundingRateTimestamp

int64

8

186

Timestamp when funding rate becomes final - nanoseconds since Unix epoch. Null value encoded as `0x8000000000000000`

FuturesMarkPrice

int64

8

194

Futures mark price encoded with 9 decimal places. Null value encoded as `0x8000000000000000`

PredictedFundingRate

int64

8

202

Predicted funding rate encoded with 9 decimal places. Null value encoded as `0x8000000000000000`

SpotMarkPrice

int64

8

210

Spot mark price encoded with 9 decimal places. Null value encoded as `0x8000000000000000`

FairValue

int64

8

218

Fair value encoded with 9 decimal places. Null value encoded as `0x8000000000000000`

## End of Cycle

End Of Cycle (124)

Type

Length

Offset

Description

ActiveInstrumentCount

int32

4

10

Total number of instruments sent in snapshot cycle