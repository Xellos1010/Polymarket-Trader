# code sets

This pages lists supported code sets and their respective tags.

## OrdStatus (39)

Value

Description

0

New

1

Partially Filled

2

Filled

3

Done for Day

4

Canceled

5

Modify

8

Rejected

C

Expired

U

Undefined

## OrdType (40)

Value

Description

1

Market

2

Limit

3

Stop

4

Stop Limit

O

[OCO (One Cancels the Other)](https://developer.chrome.com/derivatives/fix/order-entry#oco-orders)

## Side (54) / LegSide (624)

Value

Description

1

Buy Side

2

Sell Side

## TimeInForce (59)

Value

Description

0

Day

1

Good Till Cancel (GTC)

3

Fill and Kill (FAK) / Immediate or Cancel (IOC)

4

Fill or Kill (FOK)

6

Good Till Date (GTD)

## PositionEffect (77)

Value

Description

O

Open

C

Close

D

Default

## CXRejReason (102)

Value

Description

0

Too late to cancel

1

Unknown order

## OrdRejReason (103)

Value

Description

1

Unknown symbol

2

Exchange closed

6

Duplicate order

18

Invalid price increment (submitted price precision exceeds the one supported for the instrument)

99

Other

## ExecType (150)

Value

Description

0

New

3

Done for Day

4

Canceled

5

Replaced

8

Rejected

C

Expired

F

Trade

H

Trade Cancel

L

Stop Triggered

## SecurityType (167) / LegSecurityType (609)

Value

Description

FUT

Future

OPT

Option

## MDEntryType (269)

Value

Description

0

Bid

1

Offer

2

Trade

4

Opening Price

5

Closing Price

6

Settlement Price

7

Trading Session High Price

8

Trading Session Low Price

9

Trading Session Vwap Price

B

Trading Session Traded Volume

C

Open Interest

E

Implied Bid

F

Implied Offer

J

Empty Book

z

Last Traded Price

t

Funding Time

k

Final Mark Price

f

Final Funding Rate

m

Mark Price

p

Predicted Funding Rate

s

Spot Mark Price

v

Fair Value

## SessionRejectReason (373)

Value

Description

0

Invalid tag number

1

Required tag missing

3

Undefined Tag

4

Tag specified without a value

5

Value is incorrect (out of range) for this tag

6

Incorrect data format for value

9

CompID problem

99

Other

## ExecRestatementReason (378)

Value

Description

8

Exchange

100

Cancel on disconnect

103

Cancel oldest (resting) due to Self-Match Prevention

104

Cancel from exchange credit controls violation

105

Cancel from exchange website

106

Cancel from Risk Management API

107

Cancel newest (aggressing) due to Self-Match Prevention

108

Cancel due to resting order quantity less than min lot size

109

Cancel both due to Self-Match Prevention

## MultiLegReporting (442)

Value

Description

1

Outright

2

Leg of Spread

3

Spread

## CFICode (461) / LegCFICode (608)

Value

Description

FXXXXX

Futures

OCXXXS

Options Call

OPXXXS

Options Put

## OrderCapacity (528)

Value

Description

A

Agency. Order placed by customer

P

Principal. Order placed by the firm

## CustOrderCapacity (582)

Value

Description

1

CTI 1: Transactions initiated and executed by individual member for their own **personal account**, for an account they control, or for an account in which they have ownership or financial interest.

2

CTI 2: Transactions executed for the **proprietary account** of a clearing member or non-clearing member firm.

3

CTI 3: Transactions where an individual member or authorized trader executes for the **personal account of another individual member**, for an account the other individual member controls, or for an account in which the other individual member has ownership or financial interest.

4

CTI 4: Transactions that do not meet the definition of CTI 1, 2 or 3. These should be non-member customer transactions.

## SecuritySubType (762)

Value

Description

SP

Standard Calendar Spreads

## ManualOrderIndicator (1028)

Value

Description

Y

Manually created order

N

Automatically generated with trading software

## CustOrderHandlingInst (1031)

Value

Description

W

Desk

Y

Electronic (Default)

C

Vendor provided platform Billed by Executing Broker

G

Sponsored Access via exchange API

H

Premium Algorithmic trading provided by executing broker

D

Other

## AggressorIndicator (1057)

Value

Description

Y

Match aggressor

N

Resting at match

## SelfMatchPreventionStrategy (8000)

Value

Description

N

Cancel aggressing order

O

Cancel resting order

Q

Cancel both aggressing and resting orders