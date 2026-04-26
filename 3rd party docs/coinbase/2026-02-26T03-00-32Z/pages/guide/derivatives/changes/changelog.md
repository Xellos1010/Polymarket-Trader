# changelog

These release notes list changes to all Coinbase Derivatives Exchange products.

### 2026-Feb-20

-   Added block trade endpoints to the Public REST API:
    -   **Block Trades**
        -   `GET /rest/block-trade` - Get all block trades where user owns the reporting firm/entity
        -   `POST /rest/block-trade` - Get filtered block trades matching specified filter criteria
        -   `POST /rest/block-trade/new` - Book a new block trade between two parties
    -   **Block Trade Booking**
        -   `GET /rest/block-trade/booking/bookable-accounts` - Get block trade accounts the user can book trades for
        -   `GET /rest/block-trade/booking/bookable-entities` - Get block trade entities the user can book trades for
        -   `GET /rest/block-trade/booking/bookable-traders` - Get block traders the user can book trades for
    -   **Block Trade Accounts**
        -   `GET /rest/block-trade-accounts` - Get all block trade accounts
        -   `GET /rest/block-trade-accounts/{uuid}` - Get a specific block trade account
    -   **Block Trade Account Permissions**
        -   `GET /rest/block-trade-account-permissions` - Get all block trade account permissions
        -   `GET /rest/block-trade-account-permissions/{uuid}` - Get a specific block trade account permission
    -   **Block Trade Entities**
        -   `GET /rest/block-trade-entities` - Get all block trade entities
        -   `GET /rest/block-trade-entities/{uuid}` - Get a specific block trade entity
    -   **Block Trade Position Limits**
        -   `GET /rest/block-trade-position-limits` - Get all block trade position limits
        -   `GET /rest/block-trade-position-limits/{uuid}` - Get a specific block trade position limit
    -   **Block Traders**
        -   `GET /rest/block-traders` - Get all block traders
        -   `GET /rest/block-traders/{uuid}` - Get a specific block trader

### 2026-Feb-03

-   Removed deprecated v1 positions endpoints from the Public REST API:
    -   `GET /rest/positions` - Use `GET /rest/v2/positions` instead
    -   `GET /rest/positions/{firm_uuid}` - Use `GET /rest/v2/positions/{firm_uuid}` instead

### 2026-Jan-26

-   Added instrument related endpoints to the Public REST API with optional filtering:
    -   **Get Instrument Details**
        -   `POST /rest/instruments` - Get all instruments and their details
        -   This endpoint accepts an optional request body for filtering, can be omitted to get all instruments

### 2025-Dec-05

-   Added v2 endpoints to the Public REST API with enhanced position tracking and limit management:
    -   **Positions V2**
        -   `GET /rest/v2/positions` - Get all firm positions with daily and real position tracking
        -   `GET /rest/v2/positions/{firm_uuid}` - Get firm position for a specific firm
        -   Added support for separate long/short initial margins
        -   Added daily and real positions
        -   Added product-level position tracking with `product_positions` map
    -   **Firm Position Limits V2**
        -   `GET /rest/v2/firm-position-limits` - List all position limits with daily and real limits
        -   `GET /rest/v2/firm-position-limits/{firm_uuid}` - Get firm position limits for a specific firm
        -   `POST /rest/v2/firm-position-limits/{firm_uuid}` - Update firm position limits
        -   Added separate `long_daily_limit`, `short_daily_limit`, `long_real_limit`, and `short_real_limit` fields
    -   **Firm Product Limits V2**
        -   `GET /rest/v2/firm-product-limit/{firm_uuid}` - Get firm product limits with enhanced limit details
        -   `POST /rest/v2/firm-product-limit` - Set firm product limit
        -   `POST /rest/v2/firm-product-limit/batch` - Batch set firm product limits for multiple products
        -   `POST /rest/v2/firm-product-limit/remove` - Remove firm product limit
        -   `POST /rest/v2/firm-product-limit/batch-remove` - Batch remove firm product limits for multiple products
        -   Added `trading24x7_disabled` flag and daily/real position limits per product
    -   **Firm Product Group Limits V2**
        -   `GET /rest/v2/firm-product-group-limit/{firm_uuid}` - Get firm product group limits with product limit details
        -   `POST /rest/v2/firm-product-group-limit` - Create zero firm product group limit
        -   `POST /rest/v2/firm-product-group-limit/remove` - Remove firm product group limit

### 2025-NOV-17

-   (To Ship on Dec 5) Adds explicit firm/ITM override support so FIX order entry can target alternative ITM.
    -   **FIX Order Entry**
        -   Added the override Parties block for override firm and ITM
    -   **FIX Drop Copy**
        -   Expanded the Parties group to include the CLIENT\_ID party, shows the ITM used (default or override).

### 2025-AUG-01

-   Added Public REST API docs, and updated connectivity information, [REST API](https://developer.chrome.com/api-reference/derivatives-api/rest-api/introduction).

### 2025-JUL-01

-   Added support for US Perp Style Futures (PSF) Market Data APIs:
    -   **FIX Market Data**:
        -   Added conditional MD Refresh tags: `MDEntryDate` (272) and `MDEntryTime` (273) for Funding Time messages
        -   Added new `MDEntryType` (269) enum values: `t` (Funding Time), `k` (Final Mark Price), `f` (Final Funding Rate), `m` (Mark Price), `p` (Predicted Funding Rate), `s` (Spot Mark Price), `v` (Fair Value)
    -   **FIX Security Definitions**:
        -   Added conditional tags: `FundingRateApplicable` (9001), `FundingInterval` (9002), and `FairValueLimit` (9003)
    -   **UDP Market Data**:
        -   Added new `FundingRate` (43) message
        -   Added `fundingIntervalMinutes` and `fairValueLimit` fields to `OutrightInstrumentDefinition` and `StartOfOutrightInstrumentSnapshot`
        -   Added `fundingRateApplicable` flag to `InstrumentDefinitionFlags`
        -   Added seven new fields to `EndOfSnapshot`: `FinalFundingRate`, `FinalFuturesMarkPrice`, `FinalFundingRateTimestamp`, `FuturesMarkPrice`, `PredictedFundingRate`, `SpotMarkPrice`, and `FairValue`
    -   All changes apply only to PSF products - non-PSF futures products are not impacted

### 2025-JUN-06

-   Added support for replacing OCO orders to FIX 4.4, [Order Cancel/Replace Request (35=G)](https://developer.chrome.com/fix/order-entry#order-cancelreplace-request-35g).
-   Updated [cde-fix44-xml-dictionaries-latest.zip](https://developer.chrome.com/derivatives/downloads/cde-fix44-xml-dictionaries-latest.zip) for download.

### 2025-APR-04

-   Effective Friday, April 04, 2025, and going forward, the weekly sequence number reset will move up to every Friday at 16:05 CT.

### 2025-JAN-31

-   Added UUIDv4 support for `ClOrdID` (tag 11) on FIX Order Entry

### 2024-SEP-13

-   Removed support for `MDPriceLevel` (tag 1023) from FIX Market Data
-   Removed support for `MaxShow` (tag 210) from FIX Order Entry and FIX Copy

### 2024-JUL-15

-   Added support for OCO (one-cancels-the-other) orders to FIX 4.4, [New Order Single (35=D)](https://developer.chrome.com/fix/order-entry#oco-orders). OCO orders combine the features of limit and stop orders (and CDE treats them as a single order, not two linked ones). OCO orders let users set predefined limit and stop levels simultaneously depending on their view of the market. Initially, an OCO order acts like a regular limit order, contributing to market data. When its stop price condition is met, it shifts and behaves like a stop limit order.

### 2024-JUN-21

Restored the [downloads page](https://developer.chrome.com/introduction/downloads) with newly updated XML files.

#### FIX Diffs since the last available downloads

FIX Drop Copy Diffs

```
121,122d120
<             <field name="TrdType" required="N"/>
<             <field name="ExecInst" required="N"/>
177,179d174
<         <field number="18" name="ExecInst" type="MULTIPLEVALUESTRING">
<             <value enum="6" description="POST_ONLY"/>
<         </field>
390,391d384
<             <value enum="30" description="BROKERAGE_FIRM"/>
<             <value enum="36" description="ENTERING_BROKERAGE_FIRM_TRADER"/>
393d385
<             <value enum="99" description="BROKER_CODE"/>
424,426d415
<         <field number="828" name="TrdType" type="INT">
<             <value enum="1" description="BLOCK_TRADE"/>
<         </field>

```
FIX Market Data Diffs

```
119d118
<                 <field name="TrdType" required="N"/>
400c399
<             <value enum="1" description="SNAPSHOT_PLUS_UPDATES_ON_FUTURES"/>
---
>             <value enum="1" description="SNAPSHOT_PLUS_UPDATES"/>
402d400
<             <value enum="3" description="UPDATES_ON_BLOCKS"/>
578,580d575
<         <field number="828" name="TrdType" type="INT">
<                 <value enum="1" description="BLOCK_TRADE"/>
<         </field>

```
FIX Order Diffs

```
120d119
<             <field name="ExecInst" required="N"/>
173d171
<             <field name="ExecInst" required="N"/>
327c325,364
<             <value enum="6" description="POST_ONLY"/>
---
>             <value enum="1" description="NOT_HELD"/>
>             <value enum="2" description="WORK"/>
>             <value enum="3" description="GO_ALONG"/>
>             <value enum="4" description="OVER_THE_DAY"/>
>             <value enum="5" description="HELD"/>
>             <value enum="6" description="PARTICIPATE_DONT_INITIATE"/>
>             <value enum="7" description="STRICT_SCALE"/>
>             <value enum="8" description="TRY_TO_SCALE"/>
>             <value enum="9" description="STAY_ON_BIDSIDE"/>
>             <value enum="0" description="STAY_ON_OFFERSIDE"/>
>             <value enum="A" description="NO_CROSS"/>
>             <value enum="B" description="OK_TO_CROSS"/>
>             <value enum="C" description="CALL_FIRST"/>
>             <value enum="D" description="PERCENT_OF_VOLUME"/>
>             <value enum="E" description="DO_NOT_INCREASE"/>
>             <value enum="F" description="DO_NOT_REDUCE"/>
>             <value enum="G" description="ALL_OR_NONE"/>
>             <value enum="H" description="REINSTATE_ON_SYSTEM_FAILURE"/>
>             <value enum="I" description="INSTITUTIONS_ONLY"/>
>             <value enum="J" description="REINSTATE_ON_TRADING_HALT"/>
>             <value enum="K" description="CANCEL_ON_TRADING_HALT"/>
>             <value enum="L" description="LAST_PEG"/>
>             <value enum="M" description="MID_PRICE_PEG"/>
>             <value enum="N" description="NON_NEGOTIABLE"/>
>             <value enum="O" description="OPENING_PEG"/>
>             <value enum="P" description="MARKET_PEG"/>
>             <value enum="Q" description="CANCEL_ON_SYSTEM_FAILURE"/>
>             <value enum="R" description="PRIMARY_PEG"/>
>             <value enum="S" description="SUSPEND"/>
>             <value enum="U" description="CUSTOMER_DISPLAY_INSTRUCTION"/>
>             <value enum="V" description="NETTING"/>
>             <value enum="W" description="PEG_TO_VWAP"/>
>             <value enum="X" description="TRADE_ALONG"/>
>             <value enum="Y" description="TRY_TO_STOP"/>
>             <value enum="Z" description="CANCEL_IF_NOT_BEST"/>
>             <value enum="a" description="TRAILING_STOP_PEG"/>
>             <value enum="b" description="STRICT_LIMIT"/>
>             <value enum="c" description="IGNORE_PRICE_VALIDITY_CHECKS"/>
>             <value enum="d" description="PEG_TO_LIMIT_PRICE"/>
>             <value enum="e" description="WORK_TO_TARGET_STRATEGY"/>

```

#### SBE Diffs since the last available downloads

SBE Order Diffs

```
6,7c6,7
<                    version="7"
<                    semanticVersion="4.6"
---
>                    version="6"
>                    semanticVersion="3.4"
108,110d107
<         <set name="NewOrderFlags" encodingType="uint8">
<             <choice name="postOnly">0</choice>
<         </set>
164c161
<         <field name="flags"                          id="7" type="NewOrderFlags" sinceVersion="7"/>
---
>         <field name="padding"                        id="7" type="int8" sinceVersion="5"/>

```

### 2024-APR-09

-   Replaced SBE [NewOrder](https://developer.chrome.com/sbe/order-entry#neworder) padding with flags where `1` = Post Only

### 2024-MAR-29

Added FIX **Drop Copy** support for **ExecInst, Post-only** (`18=6`) in:

-   [Trade (150=F)](https://developer.chrome.com/fix/drop-copy#trade-150f)
-   [New/Canceled/Replaced Order (35=8, 150=0/4/5)](https://developer.chrome.com/fix/drop-copy#newcanceledreplaced-order-358-150045)
-   [Rejected Order (150=8)](https://developer.chrome.com/fix/drop-copy#rejected-order-1508)

### 2024-FEB-28

Added FIX **Order Entry** support for **ExecInst, Post-only** (`18=6`) in:

-   [New Order Single (35=D)](https://developer.chrome.com/fix/order-entry#new-order-single-35d)
-   [Execution Report (35=8)](https://developer.chrome.com/fix/order-entry#execution-report-358)
-   [Rejected Order (35=8, 150=8)](https://developer.chrome.com/fix/order-entry#rejected-order-358-1508)
-   [Trade (35=8, 150=F)](https://developer.chrome.com/fix/order-entry#trade-358-150f)

### 2023-NOV-06

-   Added [Block Trade](https://developer.chrome.com/fix/drop-copy#trade-150f) support to the Drop Copy API. For block trades, TrdType is set to 1 (`828=1`). For all other trade types, TrdType is null. For more, see [Block Trades](https://www.coinbase.com/derivatives#block_trades).

### 2023-OCT-31

-   Public preview of the Coinbase Derivatives Exchange FIX, SBE, and UDP APIS.