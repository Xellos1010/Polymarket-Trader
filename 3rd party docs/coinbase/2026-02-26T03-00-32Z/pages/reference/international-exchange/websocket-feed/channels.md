# channels

## INSTRUMENTS Channel

The `INSTRUMENTS` channel sends information about the tradable instruments. If `product_ids` is not specified, then updates to all instruments are returned.

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP", "ETH-PERP"],
  "channels": ["INSTRUMENTS"],
  "time": "1683730727",
  "key": "key",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

```
// Responses
{
  "sequence": 0,
  "product_id": "BTC-PERP",
  "instrument_type": "PERP",
  "instrument_mode": "standard",
  "base_asset_name": "BTC",
  "quote_asset_name": "USDC",
  "base_increment": "0.0001",
  "quote_increment": "0.1",
  "avg_daily_quantity": "12.0",       // 30 day average daily traded vol, updated daily
  "avg_daily_volume": "345446.4",     // 30 day avg daily traded notional amt in USDC, updated daily. Not sent for spot instruments which route orders to Coinbase Exchange.
  "total_30_day_quantity":"360.0",    // 30 day total traded vol, updated daily
  "total_30_day_volume":"10363392.0", // 30 day total traded notional amt in USDC, updated daily. Not sent for spot instruments which route orders to Coinbase Exchange.
  "total_24_hour_quantity":"11.2",    // 24 hr total traded vol, updated hourly
  "total_24_hour_volume":"297731.9",  // 24 hr total traded notional amt in USDC, updated hourly. Not sent for spot instruments which route orders to Coinbase Exchange.
  "indicative_open_price":"28787.8",  // The probable price at which the market will open
  "base_imf": "0.1",                  // Minimum initial margin requirement
  "default_imf": "0.2",               // Default initial margin requirement
  "min_quantity": "0.0001",           // Smallest qty allowed to place an order. Not sent for spot instruments which route orders to Coinbase Exchange.
  "position_size_limit": "32",        // Max size allowed for a position (This will be deprecated in a future release)
  "position_notional_limit": "59.20", // Max notional value allowed for a position
  "open_interest_notional_limit": "81.2", // Max notional value allowed for open interest
  "funding_interval": "60000000000",  // Time in nanoseconds between funding intervals
  "trading_state": "trading",         // Values include: offline, trading, paused, halt, delisted, external, auction_mode, cancel_only, post_only, limit_only, trading_disabled, cancel_only_enforced_by_coinbase_international_exchange, coinbase_exchange_unreachable
  "base_asset_multiplier": "1.0",     // Specifies the multiply factor to indicate the number of units in one contract
  "last_update_time": "2023-05-04T11:16:35.016Z",
  "time": "2023-05-10T14:58:47.000Z", // Gateway timestamp
  "underlying_type": "SPOT",          // Describes the type of the underlying asset
  "channel":"INSTRUMENTS",
  "type":"SNAPSHOT"
}
{
   "sequence": 1,
   "product_id": "ETH-PERP",
   "instrument_type": "PERP",
   "instrument_mode": "standard",
   "base_asset_name": "ETH",
   "quote_asset_name": "USDC",
   "base_increment": "0.0001",
   "quote_increment": "0.01",
   "avg_daily_quantity": "43.0",
   "avg_daily_volume": "80245.2",
   "total_30_day_quantity":"1443.0",
   "total_30_day_volume":"3040449.0",
   "total_24_hour_quantity":"48.1",
   "total_24_hour_volume":"101348.3",
   "base_imf": "0.1",
   "default_imf": "0.2",
   "min_quantity": "0.0001",
   "position_size_limit": "500",
   "position_notional_limit": "2300.20",
   "open_interest_notional_limit": "8320.64",
   "funding_interval": "60000000000",
   "trading_state": "trading",
   "base_asset_multiplier": "1.0",
   "last_update_time": "2023-05-04T11:16:33.016Z",
   "time": "2023-05-10T14:58:47.000Z",
   "underlying_type": "SPOT",
   "channel":"INSTRUMENTS",
   "type":"SNAPSHOT"
}
{
  "sequence": 2,
  "product_id": "TEST-PERP",
  "instrument_type": "PERP",
  "instrument_mode": "pre_launch",
  "base_asset_name": "TEST",
  "quote_asset_name": "USDC",
  "base_increment": "0.0001",
  "quote_increment": "0.01",
  "avg_daily_quantity": "15.0",
  "avg_daily_volume": "150.0",
  "total_30_day_quantity":"1443.0",
  "total_30_day_volume":"14430.0",
  "total_24_hour_quantity":"10.1",
  "total_24_hour_volume":"101",
  "base_imf": "0.1",
  "default_imf": "0.2",
  "min_quantity": "0.0001",
  "position_size_limit": "20",
  "position_notional_limit": "2888.2031",
  "open_interest_notional_limit": "5904.6496",
  "funding_interval": "60000000000",
  "trading_state": "trading",
  "base_asset_multiplier": "1.0",
  "last_update_time": "2024-05-16T11:16:33.016Z",
  "time": "2024-05-16T14:58:47.000Z",
  "pre_launch_conversion_time": "2024-06-10T20:01:00.000Z", // Pre-launch conversion end time. if not present, conversion hasn't been scheduled yet
  "underlying_type": "SPOT",
  "channel":"INSTRUMENTS",
  "type":"SNAPSHOT"
}
{
   "sequence": 3,
   "product_id": "BTC-PERP",
   "instrument_type": "PERP",
   "instrument_mode": "standard",
   "base_asset_name": "BTC",
   "quote_asset_name": "USDC",
   "base_increment": "0.0001",
   "quote_increment": "0.1",
   "avg_daily_quantity": "12.0",
   "avg_daily_volume": "345446.4",
   "total_30_day_quantity":"336.0",
   "total_30_day_volume":"8931957.0",
   "total_24_hour_quantity":"11.2",
   "total_24_hour_volume":"297731.9",
   "base_imf": "0.21",
   "default_imf": "0.2",
   "min_quantity": "0.00001",
   "position_size_limit": "32",
   "position_notional_limit": "59.2031",
   "open_interest_notional_limit": "88.6496",
   "funding_interval": "60000000000",
   "trading_state": "trading",
   "base_asset_multiplier": "1.0",
   "last_update_time": "2023-05-10T14:59:17.000Z",
   "time": "2023-05-10T14:59:17.000Z",
   "underlying_type": "SPOT",
   "channel": "INSTRUMENTS",
   "type": "UPDATE"
}

```

## MATCH Channel

The `MATCH` channel provides real-time information every time a trade happens. No snapshot is provided in this channel upon subscription.

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["MATCH"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

```
// Response
{
  "sequence": 0,
  "product_id": "BTC-PERP",
  "time": "2023-05-10T14:58:47.002Z",
  "match_id": "177101110052388865",
  "trade_qty": "0.006",
  "aggressor_side": "BUY", // Values include: BUY, SELL, OPENING_FILL
  "trade_price": "28833.1",
  "channel": "MATCH",
  "type": "UPDATE"
}

```

## FUNDING Channel

The `FUNDING` channel provides information on predicted/last funding intervals. The first SNAPSHOT message includes the funding rate of the last funding interval. If the field `is_final` is `false`, the message indicates the predicted funding rate for the next funding interval.

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["FUNDING"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

```
// Responses
{
   "sequence": 0,
   "product_id": "BTC-PERP",
   "time": "2023-05-10T14:58:47.000Z",
   "funding_rate": "0.001387",
   "is_final": true,
   "channel": "FUNDING",
   "type": "SNAPSHOT"
}
{
   "sequence": 1,
   "product_id": "BTC-PERP",
   "time": "2023-05-10T15:00:00.000Z",
   "funding_rate": "0.001487",
   "is_final": false,
   "channel": "FUNDING",
   "type": "UPDATE"
}

```

## RISK Channel

The `RISK` channel provides snapshots and updates on risk metrics of the markets.

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["RISK"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

```
// Responses
{
   "sequence": 0,
   "product_id": "BTC-PERP",
   "time": "2023-05-10T14:58:47.000Z",
   "limit_up": "30226.3",               // The maximum allowed price for a buy order
   "limit_down": "27347.7",             // The minimum allowed price for a sell order
   "index_price": "28787",              // A calculated price based on market quotes used to determine funding for perpetual futures and calculate the fair value.
   "mark_price": "28787",               // The median of the best bid, best offer, and last trade which gets used in various risk calculations.
   "settlement_price": "28787",         // The price used to settle open positions which resets risk calculations for margin and unrealized P&L.
   "indicative_open_price": "28787.8",  // The probable price at which the market will open
   "open_interest": "32",
   "channel": "RISK",
   "type": "SNAPSHOT"
}
{
   "sequence": 1,
   "product_id": "BTC-PERP",
   "time": "2023-05-10T14:59:17.000Z",
   "limit_up": "31233.9",
   "limit_down": "28259.3",
   "index_price": "28788",
   "mark_price": "29746.6",
   "settlement_price": "29746.6",
   "open_interest": "34",
   "channel": "RISK",
   "type": "UPDATE"
}

```

## LEVEL1 Channel

The `LEVEL1` channel provides real-time top of book updates.

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["LEVEL1"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

```
// Response
{
   "sequence": 0,
   "product_id": "BTC-PERP",
   "time": "2023-05-10T14:58:47.000Z",
   "bid_price": "28787.8",
   "bid_qty": "0.466", // This denotes an one-sided book since ask_price and ask_qty are not populated
   "channel": "LEVEL1",
   "type": "SNAPSHOT"
}
{
   "sequence": 1,
   "product_id": "BTC-PERP",
   "time": "2023-05-10T14:58:47.547Z",
   "bid_price": "28787.8",
   "bid_qty": "0.466",
   "ask_price": "28788.8",
   "ask_qty": "1.566",
   "channel": "LEVEL1",
   "type": "UPDATE"
}

```

## LEVEL2 Channel

The `LEVEL2` channel provides real-time updates on the top 20 levels of the aggregated order book.

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["LEVEL2"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

```
// Responses
{
   "sequence": 0,
   "product_id": "BTC-PERP",
   "time": "2023-05-10T14:58:47.000Z",
   "bids": [
       ["29100", "0.02"],
       ["28950", "0.01"],
       ["28900", "0.01"]
   ],
   "asks": [
       ["29267.8", "18"],
       ["29747.6", "18"],
       ["30227.4", "9"]
   ],
   "channel": "LEVEL2",
   "type": "SNAPSHOT"
}
{
   "sequence": 1,
   "product_id": "BTC-PERP",
   "time": "2023-05-10T14:58:47.375Z",
   "changes": [
       [
           "BUY",
           "28787.7",
           "6"
       ]
   ],
   "channel": "LEVEL2",
   "type": "UPDATE"
}

```

## CANDLES Channel

The candles channels provide aggregated market data in the form of OHLCV. There are five different candles channels for different granularities.

```
// Channels
{
  "channels": [
    "CANDLES_ONE_MINUTE",
    "CANDLES_FIVE_MINUTE",
    "CANDLES_THIRTY_MINUTE",
    "CANDLES_TWO_HOUR",
    "CANDLES_ONE_DAY"
  ]
}

```

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["CANDLES_ONE_MINUTE", "CANDLES_TWO_HOUR"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

Snapshot messages are sent upon subscription. Unlikely but possible, new snapshot messages may be sent during active subscription. Each snapshot provides an array of 200 hundred candles, if there is enough data. Each candle array is one granularity time long.

```
// Snapshot Response
{
   "sequence": 0,
   "product_id": "BTC-PERP",
   "channel": "CANDLES_ONE_MINUTE",
   "type": "SNAPSHOT",
   "candles": [
     {
         "time": "2023-05-10T14:58:47.000Z",
         "low": "28787.8",
         "high": "28788.8",
         "open": "28788.8",
         "close": "28787.8",
         "volume": "0.466"
      },
      {...}
   ]
}

```

Each update message updates the latest candle of the granularity time. It can replace the entire entry, empty fields means they are not updated. If `start` field is updated, a new candle is provided. For example, if current time is 2024-06-01T16:12:34, the CANDLES\_ONE\_MINUTE will update the candle of 2024-06-01T16:12:00; the CANDLES\_FIVE\_MINUTE will update the candle of 2024-06-01T16:10:00; the CANDLES\_THIRTY\_MINUTE will update the candle of 2024-06-01T16:00:00.

```
// Update Response
{
  "sequence": 1,
  "product_id": "BTC-PERP",
  "channel": "CANDLES_ONE_MINUTE",
  "type": "UPDATE",
  "start": "2024-04-30T16:12:00.000Z",
  "open": "0.13275",
  "high": "0.13311",
  "low": "0.13275",
  "close": "0.13298",
  "volume": "351262.0"
}

```

## RFQ MATCH Channel

The RFQ\_MATCH channel provides real-time information every time an RFQ trade happens. No snapshot is provided in this channel upon subscription.

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["RFQ_MATCH"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

```
// Response
{
  "sequence": 0,
  "product_id": "BTC-PERP",
  "time": "2023-05-10T14:58:47.002Z",
  "match_id": "177101110052388865",
  "trade_qty": "0.006",
  "aggressor_side": "BUY", // Values include: BUY, SELL
  "trade_price": "28833.1",
  "channel": "RFQ_MATCH",
  "type": "UPDATE"
}

```

## INDEX Channel

The `INDEX` channel provides real-time index prices that update once per second.

```
// Request
{
  "type": "SUBSCRIBE",
  "product_ids": ["COIN50"],
  "channels": ["INDEX"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

```
// Response
{
  "sequence": 1,
  "product_id": "COIN50",
  "channel": "INDEX",
  "type": "UPDATE",
  "time": "2024-07-16T16:12:00.000Z",
  "price": "321.559689"
}

```