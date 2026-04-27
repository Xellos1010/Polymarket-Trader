# security definitions

## Security Definition (35=d)

Sent when contract states are updated.

Tag

Name

FIX Type

Req

Description

980

SecurityUpdateAction

Char

Y

Included in the message on the Incremental feed when a mid-week deletion or modification (i.e., extension) occurs.  

-   `980=A`: Add represents Security Definition messages that are:

-   Newly added during the current week
-   Disseminated during the Sunday Startup period
-   Resent by the system during the week

-   `980=M`: Modify represents modifications to a Security Definition
-   `980=D`: Delete represents deletions of a Security Definition

779

LastUpdateTime

UTCTimestamp

Y

Timestamp of when the instrument was last added, modified or deleted. UTC Timestamps are sent in nanoseconds since Unix epoch synced to a master clock to microsecond accuracy.

55

Symbol

String

Y

Instrument name or symbol.

167

SecurityType

String

C

See \[Security Type (Tag167)\]

762

SecuritySubType

String

C

See SecuritySubType (Tag762)  
Represents spread strategy if applicable

107

SecurityDesc

String

Y

Instrument Name

200

MaturityMonthYear

Month-year

C

This field provides the calendar month reflected in the instrument symbol (tag 55-Symbol in MDP 3.0; tag 107-SecurityDesc). Format YYYYMM (e.g., 201912)  
For futures spreads and options spreads, this field contains the first leg’s calendar month reflected in the instrument symbol.

6937

Asset

String

Y

String field that indicates the underlying asset code (Product Code). Example: EU (Euro/USD FX Rate).  
Product Code was previously communicated in tag 1151-SecurityGroup.

461

CFICode

String

Y

ISO standard instrument categorization code.

462

UnderlyingProduct

Int

Y

2=Commodity/Agriculture  
4=Currency  
5=Equity  
12=Other  
14=Interest Rate  
15=FX Cash  
16=Energy  
17=Metals  
99=Crypto

1682

MDSecurityTradingStatus

Int

N

21 = Pre Open  
27 = Pre Open (No Cancel)  
17 = Ready to Trade  
2 = Trading Halt  
4 = Close  
99 = Pause  
18 = Expired  
98 = Forbidden

15

Currency

Currency

Y

Identifies currency used for price.

120

SettlCurrency

Currency

C

Identifies currency used for settlement price, if different from trade price currency.

562

MinTradeVol

Qty

Y

The minimum trading quantity for a security.

1140

MaxTradeVol

Qty

Y

The maximum trading quantity for a security.

969

MinPriceIncrement

Float

Y

Minimum constant tick for the instrument.

864

NoEvents

NuminGroup

Y

Number of repeating EventType entries.  
Indicates number of repeating groups and length of each repeating group in the message.

→865

EventType

Int

Y

Code to represent the type of event.  
5=Activation  
7=Last eligible trade date

→1145

EventTime

String

Y

Trading Session Date of event. Format yyyyMMdd

996

UnitOfMeasure

String

C

Unit of measure for the product’s original contract size. This will be populated for all products listed on Coinbase Derivatives Exchange.  
USD unit of measure can be in U.S. dollars or cents.

1147

UnitOfMeasureQty

Float

C

This field contains the contract size for each instrument. Use in combination with tag 996-UnitofMeasure.  
For example:  
Euro futures  
\-Tag 1147=10000  
\-Tag 996=Euro  
For variable-quantity products, the contract size reflects the original contract size, before the application of the multiplier

1149

HighLimitPrice

Price

N

Allowable high limit price for the trading day.  
A key parameter in validating order price.  
Used as the upper band for validating order prices. Orders submitted with prices above the upper limit will be rejected.  
This price protects off prices for quoting.  
Note: This value is indicative only and may not reflect the actual real-time high limit price.

1148

LowLimitPrice

Price

N

Allowable low limit price for the trading day.  
A key parameter in validating order price.  
Used as the lower band for validating order prices. Orders submitted with prices below the lower limit will be rejected.  
This price protects off prices for quoting.  
Note: This value is indicative only and may not reflect the actual real-time low limit price.

1143

MaxPriceVariation

Price

C

Differential value for price banding.

5796

TradingReferenceDate

LocalMktDate

C

Date of current trading session

734

PriorSettlPrice

Price

C

Prior Settlement Price (Settlement price of the prior trading session day)

730

SettlPrice

Price

C

Settlement Price

731

SettlPriceType

Int

C

Type of settlement price. The value of 2 (Theoretical) will be sent during the initial seeding of a new contract. This tag is sent in conjunction with Tag 734 (PriorSettlPrice)

9001

FundingRateApplicable

Boolean

C

True if Perp Style Futures security. If False, tag is omitted.

9002

FundingInterval

Integer

C

Time in minutes between funding periods.

9003

FairValueLimit

Float

C

Level up and down from fair value determining fair value limits.

555

NoLegs

NuminGroup

N

Number of legs that make up security (for spreads)

→600

LegSymbol

String

N

Symbol of underlying leg

→620

LegSecurityDesc

String

N

Name of underlying leg

→609

LegSecurityType

String

N

Security type of underlying leg

→624

LegSide

Char(1)

N

Side of underlying leg

→608

LegCFICode

String

N

ISO standard instrument categorization code of underlying leg

→556

LegCurrency

Currency

N

Currency of underlying leg

## SecurityListRequest (35=x)

Tag

Name

FIX Type

Req

Description

320

SecurityReqID

String

Y

Unique security request ID

559

SecurityListRequestType

String

Y

0 = Symbol

55

Symbol

String

Y

`55=NA` gets all securities

167

SecurityType

String

C

Not required if `55=NA`

## SecurityList (35=y)

Full instrument contact state snapshot request.

Tag

Name

FIX Type

Req

Description

320

SecurityReqID

String

Y

Unique security request ID

322

SecurityResponseID

String

Y

Unique response ID

560

SecurityRequestResult

String

Y

0 = ValidReq  
1 = InvalidReq

393

TotNoRelatedSym

Int

N

Total number of securities to be returned for this request

893

LastFragment

String

N

N = Not Last message  
Y = Last Message for SecurityListRequest

146

NoRelatedSym

NumInGroup

Y

number of securities in the list

55

Symbol

String

Y

instrument code

167

SecurityType

String

Y

See Security Type (Tag167)

762

SecuritySubType

String

C

See SecuritySubType (Tag762)  
Represents spread strategy if applicable

107

SecurityDesc

String

Y

Instrument Name

200

MaturityMonthYear

Month-year

C

This field provides the calendar month reflected in the instrument symbol (tag 55-Symbol in MDP 3.0; tag 107-SecurityDesc). Format YYYYMM (e.g., 201912)  
For futures spreads and options spreads, this field contains the first leg’s calendar month reflected in the instrument symbol.

6937

Asset

String

Y

String field that indicates the underlying asset code (Product Code). Example: EU (Euro/USD FX Rate).  
Product Code was previously communicated in tag 1151-SecurityGroup.

461

CFICode

String

Y

ISO standard instrument categorization code.

462

UnderlyingProduct

Int

Y

2=Commodity/Agriculture  
4=Currency  
5=Equity  
12=Other  
14=Interest Rate  
15=FX Cash  
16=Energy  
17=Metals  
99=Crypto

1682

MDSecurityTradingStatus

Int

N

21 = Pre Open  
27 = Pre Open (No Cancel)  
17 = Ready to Trade  
2 = Trading Halt  
4 = Close  
99 = Pause  
18 = Expired  
98 = Forbidden

15

Currency

Currency

Y

Identifies currency used for price.

120

SettlCurrency

Currency

C

Identifies currency used for settlement price, if different from trade price currency.

562

MinTradeVol

Qty

Y

The minimum trading quantity for a security.

1140

MaxTradeVol

Qty

Y

The maximum trading quantity for a security.

969

MinPriceIncrement

Float

Y

Minimum constant tick for the instrument.

864

NoEvents

NuminGroup

Y

Number of repeating EventType entries.  
Indicates number of repeating groups and length of each repeating group in the message.

→865

EventType

Int

Y

Code to represent the type of event.  
5=Activation  
7=Last eligible trade date

→1145

EventTime

String

Y

Trading Session Date of event. Format yyyyMMdd

231

ContractMultiplier

Float

C

Number of deliverable units per instrument, e.g., peak days in expiration month or number of calendar days in expiration month.  
The market data Security Definition (tag 35-MsgType=d) message for the variable quantity spread will be populated with the value ‘0’ for tag 231-ContractMultiplier.  
The market data Security Definition (tag 35-MsgType=d) message is populated with values for the outright legs for tag 231-ContractMultiplier and customers must extract this value.

996

UnitOfMeasure

String

C

Unit of measure for the product’s original contract size. This will be populated for all products listed on Coinbase Derivatives Exchange.  
USD unit of measure can be in U.S. dollars or cents.

1147

UnitOfMeasureQty

Float

C

This field contains the contract size for each instrument. Use in combination with tag 996-UnitofMeasure.  
For example:  
Euro futures  
\-Tag 1147=10000  
\-Tag 996=Euro  
For variable-quantity products, the contract size reflects the original contract size, before the application of the multiplier

1149

HighLimitPrice

Price

N

Allowable high limit price for the trading day.  
A key parameter in validating order price.  
Used as the upper band for validating order prices. Orders submitted with prices above the upper limit will be rejected.  
This price protects off prices for quoting.  
Note: This value is indicative only and may not reflect the actual real-time high limit price.

1148

LowLimitPrice

Price

N

Allowable low limit price for the trading day.  
A key parameter in validating order price.  
Used as the lower band for validating order prices. Orders submitted with prices below the lower limit will be rejected.  
This price protects off prices for quoting.  
Note: This value is indicative only and may not reflect the actual real-time low limit price.

1143

MaxPriceVariation

Price

C

Differential value for price banding.

5796

TradingReferenceDate

LocalMktDate

C

Date of current trading session

734

PriorSettlPrice

Price

C

Prior Settlement Price (Settlement price of the prior trading session day)

730

SettlPrice

Price

C

Settlement Price

731

SettlPriceType

Int

C

Type of settlement price. When `731=2`, a theoretical settlement price is sent during initial seeding of a brand new contract. This tag is sent in conjunction with PriorSettlPrice (734).

555

NoLegs

NuminGroup

N

Number of legs that make up security (for spreads)

→600

LegSymbol

String

N

Symbol of underlying leg

→620

LegSecurityDesc

String

N

Name of underlying leg

→609

LegSecurityType

String

N

Security type of underlying leg

→624

LegSide

Char(1)

N

Side of underlying leg

→608

LegCFICode

String

N

ISO standard instrument categorization code of underlying leg

→556

LegCurrency

Currency

N

Currency of underlying leg