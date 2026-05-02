# websocket channels

The Coinbase Advanced Trade Market Data WebSocket feed provides the following channels:

Channel

Description

Requires Authentication

**[heartbeats](#heartbeats-channel)**

**Real-time server pings to keep all connections open**

No

[candles](#candles-channel)

Real-time updates on product candles

No

[status](#status-channel)

Sends all products and currencies on a preset interval

No

[ticker](#ticker-channel)

Real-time price updates every time a match happens

No

[ticker\_batch](#ticker-batch-channel)

Real-time price updates every 5000 milli-seconds

No

[level2](#level2-channel)

All updates and easiest way to keep order book snapshot

No

[user](#user-channel)

Only sends messages that include the authenticated user

*Yes*

[market\_trades](#market-trades-channel)

Real-time updates every time a market trade happens

No

[futures\_balance\_summary](#futures-balance-summary-channel)

Real-time updates every time a user’s futures balance changes

*Yes*

Refer to the documentation on [subscribing to a WebSocket channel](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-overview#subscribing).

## Heartbeats Channel

Subscribe to the `heartbeats` channel to receive heartbeats messages every second. Heartbeats include a `heartbeat_counter` which verifies that no messages were missed.

```
// Request
{
  "type": "subscribe",
  "channel": "heartbeats",
  "jwt": "XYZ"
}

```

A heartbeats message is of the type `heartbeats` as seen below.

```
// Heartbeats Message
{
  "channel": "heartbeats",
  "client_id": "",
  "timestamp": "2023-06-23T20:31:26.122969572Z",
  "sequence_num": 0,
  "events": [
    {
      "current_time": "2023-06-23 20:31:56.121961769 +0000 UTC m=+91717.525857105",
      "heartbeat_counter": "3049"
    }
  ]
}

```

## Candles Channel

Subscribe to the `candles` channel to receive candles messages for specific products with updates every second. Candles are grouped into buckets (granularities) of five minutes.

```
// Request
{
  "type": "subscribe",
  "product_ids": ["ETH-USD"],
  "channel": "candles",
  "jwt": "XYZ"
}

```

A candles message is of the type `candles` and some of its parameters include:

-   `start` - string representation of the UNIX timestamp of the candle.
-   `high` and `low` - highest and lowest prices during the bucket interval.
-   `open` and `close` - prices of the first and last trade respectively.
-   `volume` - base amount that has been traded during this interval.
-   `product_id` - product identifier for this candle

```
// Candles Message
{
  "channel": "candles",
  "client_id": "",
  "timestamp": "2023-06-09T20:19:35.39625135Z",
  "sequence_num": 0,
  "events": [
    {
      "type": "snapshot",
      "candles": [
        {
          "start": "1688998200",
          "high": "1867.72",
          "low": "1865.63",
          "open": "1867.38",
          "close": "1866.81",
          "volume": "0.20269406",
          "product_id": "ETH-USD"
        }
      ]
    }
  ]
}

```

## Market Trades Channel

The `market_trades` channel sends market trades for a specified product on a preset interval. Clients should provide an array of `product_ids` for which they would like status subscriptions.

```
// Request
{
  "type": "subscribe",
  "product_ids": ["ETH-USD", "BTC-USD"],
  "channel": "market_trades",
  "jwt": "XYZ"
}

```

A market trades message is of the type `snapshot` or `update`, and contains an array of market trades. Each market trade belongs to a `side`, which refers to the makers side, and can be of type `BUY`, or `SELL`. The channel collects all updates over the last 250 ms and sends them as an `update` — so an `update` can contain one or many trades, depending on the last 250 ms of trading volume.

```
// Market Trades Message
{
  "channel": "market_trades",
  "client_id": "",
  "timestamp": "2023-02-09T20:19:35.39625135Z",
  "sequence_num": 0,
  "events": [
    {
      "type": "snapshot",
      "trades": [
        {
          "trade_id": "000000000",
          "product_id": "ETH-USD",
          "price": "1260.01",
          "size": "0.3",
          "side": "BUY",
          "time": "2019-08-14T20:42:27.265Z"
        }
      ]
    }
  ]
}

```

## Status Channel

The `status` channel sends all products and currencies on a preset interval. Clients should provide an array of `product_ids` for which they would like status subscriptions.

```
// Request
{
  "type": "subscribe",
  "product_ids": ["ETH-USD", "BTC-USD"],
  "channel": "status",
  "jwt": "XYZ"
}

```

```
// Status Message
{
  "channel": "status",
  "client_id": "",
  "timestamp": "2023-02-09T20:29:49.753424311Z",
  "sequence_num": 0,
  "events": [
    {
      "type": "snapshot",
      "products": [
        {
          "product_type": "SPOT",
          "id": "BTC-USD",
          "base_currency": "BTC",
          "quote_currency": "USD",
          "base_increment": "0.00000001",
          "quote_increment": "0.01",
          "display_name": "BTC/USD",
          "status": "online",
          "status_message": "",
          "min_market_funds": "1"
        }
      ]
    }
  ]
}

```

## Ticker Channel

The `ticker` channel provides real-time price updates every time a match happens. It batches updates in case of cascading matches, greatly reducing bandwidth requirements.

```
// Request
{
  "type": "subscribe",
  "product_ids": ["ETH-USD", "BTC-USD"],
  "channel": "ticker",
  "jwt": "XYZ"
}

```

```
// Ticker message
{
  "channel": "ticker",
  "client_id": "",
  "timestamp": "2023-02-09T20:30:37.167359596Z",
  "sequence_num": 0,
  "events": [
    {
      "type": "snapshot",
      "tickers": [
        {
          "type": "ticker",
          "product_id": "BTC-USD",
          "price": "21932.98",
          "volume_24_h": "16038.28770938",
          "low_24_h": "21835.29",
          "high_24_h": "23011.18",
          "low_52_w": "15460",
          "high_52_w": "48240",
          "price_percent_chg_24_h": "-4.15775596190603",
          "best_bid": "21931.98",
          "best_bid_quantity": "8000.21",
          "best_ask": "21933.98",
          "best_ask_quantity": "8038.07770938"
        }
      ]
    }
  ]
}

```

## Ticker Batch Channel

The `ticker_batch` channel provides latest price updates **every 5000 milliseconds** (5 seconds) if there is a change. It has the same JSON message schema as the [`ticker` channel](#ticker-channel), except the `channel` field will have a value of `ticker_batch` and it currently doesn’t provide best bid or best ask fields.

```
// Request
{
  "type": "subscribe",
  "product_ids": ["ETH-USD", "BTC-USD"],
  "channel": "ticker_batch",
  "jwt": "XYZ"
}

```

## Level2 Channel

The `level2` channel guarantees delivery of all updates and is the easiest way to keep a snapshot of the order book.

```
// Request
{
  "type": "subscribe",
  "product_ids": ["ETH-USD", "BTC-USD"],
  "channel": "level2",
  "jwt": "XYZ"
}

```

The level2 channel sends a message with fields, `type` (“snapshot” or “update”), `product_id`, and `updates`. The field `updates` is an array of objects of `{price_level, new_quantity, event_time, side}` to represent the entire order book. The`event_time` property is the time of the event as recorded by our trading engine.

```
// Example:
{
  "channel": "l2_data",
  "client_id": "",
  "timestamp": "2023-02-09T20:32:50.714964855Z",
  "sequence_num": 0,
  "events": [
    {
      "type": "snapshot",
      "product_id": "BTC-USD",
      "updates": [
        {
          "side": "bid",
          "event_time": "1970-01-01T00:00:00Z",
          "price_level": "21921.73",
          "new_quantity": "0.06317902"
        },
        {
          "side": "bid",
          "event_time": "1970-01-01T00:00:00Z",
          "price_level": "21921.3",
          "new_quantity": "0.02"
        }
      ]
    }
  ]
}

```

## User Channel

The `user` channel sends updates on all of a user’s open orders and current positions, including all subsequent updates of those orders and positions. The `user` channel expects one connection per user:

-   This connection accepts multiple product IDs in a `product_ids` array. If none are provided, the WebSocket subscription is open to all product IDs.
-   To subscribe to new `product_ids`, close your previous connection by unsubscribing and open a new connection with `product_ids` added to the array.

```
// Request
{
  "type": "subscribe",
  "channel": "user",
  "product_ids": ["BTC-USD"],
  "jwt": "XYZ"
}

```

```
// User message
{
  "channel": "user",
  "client_id": "",
  "timestamp": "2023-02-09T20:33:57.609931463Z",
  "sequence_num": 0,
  "events": [
    {
      "type": "snapshot",
      "orders": [
        {
          "avg_price": "50000",
          "cancel_reason": "",
          "client_order_id": "XXX",
          "completion_percentage": "100.00",
          "contract_expiry_type": "UNKNOWN_CONTRACT_EXPIRY_TYPE",
          "cumulative_quantity": "0.01",
          "filled_value": "500",
          "leaves_quantity": "0",
          "limit_price": "50000",
          "number_of_fills": "1",
          "order_id": "YYY",
          "order_side": "BUY",
          "order_type": "Limit",
          "outstanding_hold_amount": "0",
          "post_only": "false",
          "product_id": "BTC-USD",
          "product_type": "SPOT",
          "reject_reason": "",
          "retail_portfolio_id": "ZZZ",
          "risk_managed_by": "UNKNOWN_RISK_MANAGEMENT_TYPE",
          "status": "FILLED",
          "stop_price": "",
          "time_in_force": "GOOD_UNTIL_CANCELLED",
          "total_fees": "2",
          "total_value_after_fees": "502",
          "trigger_status": "INVALID_ORDER_TYPE",
          "creation_time": "2024-06-21T18:29:13.909347Z",
          "end_time": "0001-01-01T00:00:00Z",
          "start_time": "0001-01-01T00:00:00Z"
        }
      ],
      "positions": {
        "perpetual_futures_positions": [
          {
            "product_id": "BTC-PERP-INTX",
            "portfolio_uuid": "018c4b12-9f87-7c36-897d-28fb6a1ea88d",
            "vwap": "63049.9",
            "entry_vwap": "0",
            "position_side": "Long",
            "margin_type": "Cross",
            "net_size": "0.0041",
            "buy_order_size": "0",
            "sell_order_size": "0",
            "leverage": "1",
            "mark_price": "63049.9",
            "liquidation_price": "0",
            "im_notional": "258.5046",
            "mm_notional": "17.061304",
            "position_notional": "258.5046",
            "unrealized_pnl": "0",
            "aggregated_pnl": "258.50459"
          }
        ],
        "expiring_futures_positions": [
          {
            "product_id": "BIT-28JUN24-CDE",
            "side": "Long",
            "number_of_contracts": "1",
            "realized_pnl": "0",
            "unrealized_pnl": "-21.199999999999932",
            "entry_price": "64150"
          }
        ]
      }
    }
  ]
}

```

#### Orders Fields

Field

Description

`avg_price`

Average filled price of the order so far

`cancel_reason`

Reason for order cancellation

`client_order_id`

Unique identifier of order specified by client

`completion_percentage`

Percentage of order completion

`contract_expiry_type`

Can be one of:

-   `UNKNOWN_CONTRACT_EXPIRY`
-   `EXPIRING`
-   `PERPETUAL`

`cumulative_quantity`

Amount the order is filled, in base currency

`filled_value`

Value of the filled order

`leaves_quantity`

Amount remaining, in same currency as order was placed in (quote or base)

`limit_price`

Can be one of:

-   `Limit Price`: Order is Limit or Stop Limit type
-   `0`: Order is not Limit or Stop Limit type

`number_of_fills`

Number of fills for the order

`order_id`

Unique identifier of order

`order_side`

Can be one of:

-   `BUY`
-   `SELL`

`order_type`

Can be one of:

-   `LIMIT`
-   `MARKET`
-   `STOP_LIMIT`

`outstanding_hold_amount`

Outstanding hold amount for the order

`post_only`

Can be one of:

-   `true`
-   `false`

`product_id`

The product ID for which this order was placed

`product_type`

Can be one of:

-   `UNKNOWN_PRODUCT_TYPE`
-   `SPOT`
-   `FUTURE`

`reject_Reason`

Reason for order rejection

`retail_portfolio_id`

The ID of the portfolio this order is associated with.

`risk_managed_by`

Can be one of:

-   `UNKNOWN_RISK_MANAGEMENT_TYPE`
-   `MANAGED_BY_FCM`
-   `MANAGED_BY_VENUE`

`status`

Can be one of:

-   `PENDING`: Order is not yet open
-   `OPEN`: Order is waiting to be fully filled
-   `FILLED`: Order is 100% filled
-   `CANCEL_QUEUED`: Order queued to be cancelled by user or system
-   `CANCELLED`: Order was cancelled by user or system
-   `EXPIRED`: TWAP order was not filled by the expiry time
-   `FAILED`: Order cannot be placed at all

`stop_price`

Can be one of:

-   `Stop Price`: Order is Stop Limit type
-   `0`: Order is not Stop Limit type

`time_in_force`

Can be one of:

-   `UNKNOWN_TIME_IN_FORCE`
-   `GOOD_UNTIL_DATE_TIME`
-   `GOOD_UNTIL_CANCELLED`
-   `IMMEDIATE_OR_CANCEL`
-   `FILL_OR_KILL`

`total_fees`

Commission paid for the order

`total_value_after_fees`

Total value of the order after fees

`trigger_status`

Can be one of:

-   `UNKNOWN_TRIGGER_STATUS`
-   `INVALID_ORDER_TYPE`
-   `STOP_PENDING`
-   `STOP_TRIGGERED`

`creation_time`

When the order was placed

`end_time`

-   `End Time`: Order has end time
-   `0001-01-01T00:00:00Z`: End Time not applicable

`start_time`

-   `Start Time`: Order has start time
-   `0001-01-01T00:00:00Z`: Start Time not applicable

#### Positions Fields

Numeric values are in units of USDC.

##### Perpetual Futures

Field

Description

`product_id`

Name of the instrument the position is in, e.g. `BTC-PERP-INTX`

`portfolio_uuid`

The uuid of the portfolio this order is associated with

`vwap`

The price of the position based on the last settlement period

`entry_vwap`

Volume weighted entry price of the position (not reset to the last funding price)

`position_side`

The side of the position. Can be one of:

-   `Long`
-   `Short`

`margin_type`

The margin type of the position. Can be one of:

-   `Cross`: Indicating a cross margin position
-   `Isolated`: Indicating an isolated margin position

`net_size`

The size of the position with positive values reflecting a long position and negative values reflecting a short position

`buy_order_size`

Cumulative size of all the open buy orders

`sell_order_size`

Cumulative size of all the open sell orders

`leverage`

The leverage of the position

`mark_price`

The current mark price value for the instrument of this position used in risk and margin calculations

`liquidation_price`

Price at which the position will be liquidated at

`im_notional`

The amount this position contributes to the initial margin

`mm_notional`

The amount this position contributes to the maintenance margin

`position_notional`

The notional value of the position

`unrealized_pnl`

The profit or loss of this position (resets to 0 after settlement)

`aggregated_pnl`

The total profit or loss of this position since the initial opening of the position

#### Expiring Futures

Field

Description

`product_id`

Name of the instrument the position is in, e.g. `BTC-12Jun24-CDE`

`side`

The side of the position. Can be one of:

-   `Long`
-   `Short`

`number_of_contracts`

The size of your position in contracts

`realized_pnl`

Your realized PnL for your position

`unrealized_pnl`

Your current unrealized PnL for your position

`entry_price`

The average entry price at which you entered your current position

## Futures Balance Summary Channel

The `futures_balance_summary` channel sends updates on all of a user’s futures balances, including all subsequent updates of those balances.

```
// Request
{
  "type": "subscribe",
  "channel": "futures_balance_summary",
  "jwt": "XYZ"
}

```

```
// Futures Balance Summary Message:
{
  "channel": "futures_balance_summary",
  "client_id": "",
  "timestamp": "2023-02-09T20:33:57.609931463Z",
  "sequence_num": 0,
  "events": [
    {
      "type": "snapshot",
      "fcm_balance_summary":{
        "futures_buying_power": "100.00",
        "total_usd_balance": "200.00",
        "cbi_usd_balance": "300.00",
        "cfm_usd_balance": "400.00",
        "total_open_orders_hold_amount": "500.00",
        "unrealized_pnl": "600.00",
        "daily_realized_pnl": "0",
        "initial_margin": "700.00",
        "available_margin": "800.00",
        "liquidation_threshold": "900.00",
        "liquidation_buffer_amount": "1000.00",
        "liquidation_buffer_percentage": "1000",
        "intraday_margin_window_measure":{
          "margin_window_type":"FCM_MARGIN_WINDOW_TYPE_INTRADAY",
          "margin_level":"MARGIN_LEVEL_TYPE_BASE",
          "initial_margin":"100.00",
          "maintenance_margin":"200.00",
          "liquidation_buffer_percentage":"1000",
          "total_hold":"100.00",
          "futures_buying_power":"400.00"
        },
        "overnight_margin_window_measure":{
          "margin_window_type":"FCM_MARGIN_WINDOW_TYPE_OVERNIGHT",
          "margin_level":"MARGIN_LEVEL_TYPE_BASE",
          "initial_margin":"300.00",
          "maintenance_margin":"200.00",
          "liquidation_buffer_percentage":"1000",
          "total_hold":"-30.00",
          "futures_buying_power":"2000.00"
        }
      }
    }
  ]
}

```

Field

Description

`futures_buying_power`

The amount of your cash balance that is available to trade CFM futures

`total_usd_balance`

Aggregate USD maintained across your CFTC-regulated futures account and your Coinbase Inc. spot account

`cbi_usd_balance`

USD maintained in your Coinbase Inc. spot account

`cfm_usd_balance`

USD maintained in your CFTC-regulated futures account. Funds held in your futures account are not available to trade spot

`total_open_orders_hold_amount`

Your total balance on hold for spot and futures open orders

`unrealized_pnl`

Your current unrealized PnL across all open positions

`daily_realized_pnl`

Your realized PnL from the current trade date. May include profit or loss from positions you’ve closed on the current trade date

`initial_margin`

Margin required to initiate futures positions. Once futures orders are placed, these funds cannot be used to trade spot. The actual amount of funds necessary to support executed futures orders will be moved to your futures account

`available_margin`

Funds available to meet your anticipated margin requirement. This includes your CBI spot USD, CFM futures USD, and Futures PnL, less any holds for open spot or futures orders

`liquidation_threshold`

When your available funds for collateral drop to the liquidation threshold, some or all of your futures positions will be liquidated

`liquidation_buffer_amount`

Funds available in excess of the liquidation threshold, calculated as available margin minus liquidation threshold. If your liquidation buffer amount reaches 0, your futures positions and/or open orders will be liquidated as necessary

`liquidation_buffer_percentage`

Funds available in excess of the liquidation threshold expressed as a percentage. If your liquidation buffer percentage reaches 0%, your futures positions and/or open orders will be liquidated as necessary

`intraday_margin_window_measure`

The period of time used to calculate margin requirements for positions held intraday before settling overnight  
Includes:

-   `margin_window_type`
    -   FCM\_MARGIN\_WINDOW\_TYPE\_UNSPECIFIED
    -   FCM\_MARGIN\_WINDOW\_TYPE\_OVERNIGHT
    -   FCM\_MARGIN\_WINDOW\_TYPE\_WEEKEND
    -   FCM\_MARGIN\_WINDOW\_TYPE\_INTRADAY
    -   FCM\_MARGIN\_WINDOW\_TYPE\_TRANSITION
-   `margin_level`
    -   MARGIN\_LEVEL\_TYPE\_UNSPECIFIED
    -   MARGIN\_LEVEL\_TYPE\_BASE
    -   MARGIN\_LEVEL\_TYPE\_WARNING
    -   MARGIN\_LEVEL\_TYPE\_DANGER
    -   MARGIN\_LEVEL\_TYPE\_LIQUIDATION
-   `initial_margin`
-   `maintenance_margin`
-   `liquidation_buffer_percentage`
-   `total_hold`
-   `futures_buying_power`

`overnight_margin_window_measure`

The period of time used to calculate increased margin requirements for positions held and left unsettled overnight  
Includes:

-   `margin_window_type`
    -   FCM\_MARGIN\_WINDOW\_TYPE\_UNSPECIFIED
    -   FCM\_MARGIN\_WINDOW\_TYPE\_OVERNIGHT
    -   FCM\_MARGIN\_WINDOW\_TYPE\_WEEKEND
    -   FCM\_MARGIN\_WINDOW\_TYPE\_INTRADAY
    -   FCM\_MARGIN\_WINDOW\_TYPE\_TRANSITION
-   `margin_level`
    -   MARGIN\_LEVEL\_TYPE\_UNSPECIFIED
    -   MARGIN\_LEVEL\_TYPE\_BASE
    -   MARGIN\_LEVEL\_TYPE\_WARNING
    -   MARGIN\_LEVEL\_TYPE\_DANGER
    -   MARGIN\_LEVEL\_TYPE\_LIQUIDATION
-   `initial_margin`
-   `maintenance_margin`
-   `liquidation_buffer_percentage`
-   `total_hold`
-   `futures_buying_power`

**See Also:**

-   [Subscribing to WebSocket Channels](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels)
-   [WebSocket Sequence Numbers](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-overview#sequence-numbers)
-   [WebSocket Best Practices](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/websocket)