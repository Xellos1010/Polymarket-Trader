# market hours

Starting May 9, 2025, Coinbase Derivatives, LLC (CDE) will enable 24x7 trading for select cryptocurrency futures products. This document outlines key information regarding the new “Extended Trading Hours.” Participation is optional; however, non-24x7 participants will be impacted as detailed below. Please reach out to your Futures Commission Merchant (FCM) to express your interest in 24x7 trading at CDE. For any additional questions, please contact [derivatives@coinbase.com](mailto:derivatives@coinbase.com).

## Impact to Non-24x7 Participants

By default, participants will be ‘opted-out’ for 24x7 trading. Opted-out participants will be unable to submit orders during Extended Trading Hours, such as weekends and market holidays. Additional impacts are below:

1.  **GTC and GTD Restrictions** : GTC and GTD time-in-force order qualifiers will no longer be allowed for 24x7 futures products. Submitting these order types will be rejected. GTC/GTD will still be permitted for those firms participating in 24x7 trading and on non-24x7 products.
2.  **Elimination of Weekday Close** : The weekday close (4:00 PM to 5:00 PM CT) will be eliminated for 24x7 futures products. These products will remain open throughout the week (Sunday open through Friday close) for all participants. The trade date will roll Monday - Friday at 4:00 PM CT.

Market Data will be available during Extended Trading Hours for all FIX Market Data and UDP Market Data consumers.

## Products

### 24x7 Futures Products

Please refer to the ‘Market Hours’ column in the ‘Coinbase Derivatives Exchange Products’ table on [https://www.coinbase.com/derivatives](https://www.coinbase.com/derivatives) for supported products.

### Block Trades (Day 2)

Initially, block trades will not be supported during Extended Trading Hours, but this feature will be introduced shortly after launch.

## 24x7 Trading Hours

### Market Hours

For 24x7 enabled futures products, trading will be available for round the clock trading with the exception of a fifty minute weekly and quarterly extended maintenance window, to be announced. 24x7 trading hours will be from Friday 5:00 PM - Friday 4:00 PM CT.

### Removal of Daily Maintenance Window

The Monday - Thursday weekday close (4:00 PM to 5:00 PM CT) will be eliminated for 24x7 futures products. This change applies to all market participants.

### Weekly Maintenance Windows

To allow participants and vendors to deploy updates, CDE will have a weekly 50-minute downtime where all markets, including 24x7 futures products, are closed Friday\* at 4:00 PM CT until 4:50 PM CT. Upon reopening, affected markets will enter a Pre-Open phase at 4:50 PM CT, followed by a 30-second Pre-Open with no cancellations starting at 4:59:30 PM CT. Trading will resume, and the markets will officially Open at 5:00 PM CT. \*In the event of a Friday holiday, the maintenance window is pushed up to Thursday.

### Extended Maintenance Windows

Each quarter, CDE will hold a weekend maintenance window for system upgrades, lasting 3-4 hours. The maintenance schedule will be shared in advance to give participants ample time to plan any necessary system updates. In addition, ad-hoc maintenance windows may be required, and exchange participants will be notified ahead of time.

## Clearing

### Clearing Schedule and Holiday Impact

Trades executed Friday after 5:00 PM CT through Sunday will be recorded with a Monday trading date. If a market holiday occurs, trades will clear on the next trading day. See examples below: **Example 1: Normal Weekend**

-   Trade: 10 lots of BITZ24
-   Transaction Time: November 21, 2024 @ 9:45 AM CT (Saturday)
-   Clearing Trade Date: November 23, 2024 (Monday)

**Example 2: Holiday Impact**

-   Trade: 5 lots of ETF24
-   Transaction Time: November 28, 2024 @ 1:15 AM (Thanksgiving Holiday)
-   Clearing Trade Date: November 29, 2024 (Friday)

**Cleared Date for 24x7**

**Trading Hours**

**Monday**

Friday 5:00 PM CT – Monday 3:59 PM CT

**Tuesday**

Monday 4:00 PM CT – Tuesday 3:59 PM CT

**Wednesday**

Tuesday 4:00 PM CT – Wednesday 3:59 PM CT

**Thursday**

Wednesday 4:00 PM CT – Thursday 3:59 PM CT

**Friday**

Thursday 4:00 PM CT – Friday 3:59 PM CT

## Trading

### 24x7 Permissions

Participants who want to trade on weekends and exchange holidays must be enabled for 24x7 by their Clearing FCM. Configuration can be completed by Clearing FCM in CDE’s Admin Portal. If a participant is not enabled for 24x7, any orders submitted during exchange holidays or weekends will be rejected.

### Settlement Window

Settlement will occur during the contract’s regular settlement time and will not be affected by the new trading hours. Settlements will not occur during extended trading hours, such as weekends or holidays.

### Order Types

If enabled for 24x7, all order types will be available during Extended Trading Hours. GTD must expire on an exchange business day. For example,submitting a GTD for Saturday expiration will be rejected.

### Order Qualifiers/Time in Force

If a client is not enabled for 24x7 activity, GTC and GTD time in force order qualifiers will no longer be permitted and will be rejected upon entry.

## Risk Management

### Disabling 24x7

Clearing FCMs will have the ability to enable or disable 24x7 for their clients. Participation is not required, and those that opt out will be able to maintain the non-24x7 trading hours. Configuration can be completed by Clearing FCM in CDE’s Admin Portal.

### Weekend Price Limits

There will not be any changes to the existing price limits. CDE futures have a 10% price fluctuation limit based on a reference price calculated from the last 60 seconds of each hour, entering a 2-minute Halt state if limits are reached. During this time, orders can be managed but not matched, and the last price limit becomes the new reference price.

## API & Technology

### Sequence Reset

Sequence numbers on Gateways (FIX Market Data, FIX Order Entry, FIX Drop Copy) are automatically reset by CDE every Friday at 4:05 PM CT.

### Networking & Gateway Availability

Gateways will remain open on weekends, with the exception of the Friday maintenance window. Brief downtime of a single gateway is possible during deployments. Deployments will occur during the Friday maintenance window (4:00 PM CT) and may extend past OPEN (5:00 PM CT). If this happens, the Status Page will be updated accordingly. Trading will not be impacted during an extended deployment if markets are in a scheduled OPEN state. For assistance, please contact [dcm-networking@coinbase.com](mailto:dcm-networking@coinbase.com).

## Regulatory

### Reporting

There will not be any changes to the Large Trader Report or the Ownership and Control Reporting. These reports will continue to be submitted in accordance with CDE Rule 532.

### Mandatory Reporting

Extended hours trading activity should be reported in the next business day’s file. This applies to Large Trader Reporting (LTR) and Ownership & Control Reporting (OCR).

## 23x5 Trading Hours

**24x7 Product Market Hours for non-24x7 Participants:**

-   Trading is open Sunday at 5:00 PM CT through Friday at 4:00 PM CT, with continuous trading and no intermissions.

**Regular Market Hours for 23x5 Crypto Products:**

-   Trading is open Sunday at 5:00 PM CT through Friday at 4:00 PM CT, with a one-hour break each day from 4:00 PM - 5:00 PM CT.
-   Pre-open quoting begins daily at 4:50 PM CT, 10 minutes before the market opens.

**Regular Market Hours for Energy and Metals Products:**

-   Trading is open Sunday at 5:00 PM CT through Friday at 4:00 PM CT, with a one-hour break each day from 4:00 PM - 5:00 PM CT.
-   Pre-open quoting begins daily at 4:50 PM CT, 10 minutes before the market opens.

## Q&A

### If I do not opt-in to 24x7, how will I be impacted?

For products eligible for 24x7 trading, non-24x7 firms will be permitted to trade only during non-weekend and non-exchange holiday hours, with an additional hour available Monday through Thursday. To prevent trades during extended trading hours, Good-Till-Canceled (GTC) and Good-Till-Date (GTD) order qualifiers will be rejected.

### Will any order types be unavailable if I do not opt-in to 24x7?

No, all order types will be available to all participants.

### How do I enable 24x7 for my firm?

Your Clearing FCM is in control of your 24x7 permissions. Please contact them to discuss enablement.

### What FCMs will be supporting 24x7 at launch?

-   ABN Amro
-   Wedbush Securities
-   Coinbase Financial Markets

CDE offers support 24x7 for approved participants. If you wish to contact CDE support directly, please coordinate with your Clearing FCM who can authorize your access.

### Is 24x7 available for testing?

The CDE test environment (“Integration”) is available 24x7 for participant testing.

### What will be the trading fees on the weekend?

Trading fees will not be impacted by extended trading hours. You can find our fee schedule [here](https://www.coinbase.com/derivatives#fee_schedule).

### Can I place a GTD order for Saturday or Sunday?

A GTD order cannot have an expiration on an exchange holiday or Saturday or Sunday calendar date.

## Appendix

### Normal Trading Hours (Non-Extended Trading Hours)

Sunday - Friday, 5:00 PM - 4:00 PM CT, with a one-hour break each day from 4:00 PM - 5:00 PM CT. See the [Market Holiday Calendar](https://www.coinbase.com/derivatives#market_holiday_calendar) for a list of dates CDE will be closed.

### API Information

Current API information can be found on our API website. [FIX](https://developer.chrome.com/derivatives/fix/overview) [SBE](https://developer.chrome.com/derivatives/sbe/overview) [UDP](https://developer.chrome.com/derivatives/udp/overview)

### Status Page

Current API health can be found on the [Status Page](https://status.cde.coinbase.com/).

### Fee Schedule

See the [Fee Schedule](https://www.coinbase.com/derivatives#fee_schedule) for current trading fees.