# list public products

```
{
  "products": [
    {
      "product_id": "BTC-USD",
      "price": "140.21",
      "price_percentage_change_24h": "9.43%",
      "volume_24h": "1908432",
      "volume_percentage_change_24h": "9.43%",
      "base_increment": "0.00000001",
      "quote_increment": "0.00000001",
      "quote_min_size": "0.00000001",
      "quote_max_size": "1000",
      "base_min_size": "0.00000001",
      "base_max_size": "1000",
      "base_name": "Bitcoin",
      "quote_name": "US Dollar",
      "watched": true,
      "is_disabled": false,
      "new": true,
      "status": "<string>",
      "cancel_only": true,
      "limit_only": true,
      "post_only": true,
      "trading_disabled": false,
      "auction_mode": true,
      "base_display_symbol": "BTC",
      "quote_display_symbol": "USD",
      "product_type": "UNKNOWN_PRODUCT_TYPE",
      "quote_currency_id": "USD",
      "base_currency_id": "BTC",
      "fcm_trading_session_details": {
        "is_session_open": true,
        "open_time": "<string>",
        "close_time": "<string>",
        "session_state": "FCM_TRADING_SESSION_STATE_UNDEFINED",
        "after_hours_order_entry_disabled": true,
        "closed_reason": "FCM_TRADING_SESSION_CLOSED_REASON_UNDEFINED",
        "maintenance": {
          "start_time": "<string>",
          "end_time": "<string>"
        }
      },
      "mid_market_price": "140.22",
      "alias": "BTC-USD",
      "alias_to": [
        "BTC-USDC"
      ],
      "view_only": true,
      "price_increment": "0.00000001",
      "display_name": "BTC PERP",
      "product_venue": "neptune",
      "approximate_quote_24h_volume": "1908432",
      "new_at": "2021-07-01T00:00:00.000Z",
      "market_cap": "1500000000000",
      "icon_color": "red",
      "icon_url": "https://metadata.cbhq.net/equity_icons/123456789.png",
      "display_name_overwrite": "Bitcoin Perpetual",
      "is_alpha_testing": false,
      "about_description": "nano Crude Oil Futures is a monthly cash-settled contract that allows participants to manage risk, trade on margin, or speculate on the price of oil.",
      "future_product_details": {
        "venue": "<string>",
        "contract_code": "<string>",
        "contract_expiry": "<string>",
        "contract_size": "<string>",
        "contract_root_unit": "<string>",
        "group_description": "<string>",
        "contract_expiry_timezone": "<string>",
        "group_short_description": "<string>",
        "risk_managed_by": "UNKNOWN_RISK_MANAGEMENT_TYPE",
        "contract_expiry_type": "UNKNOWN_CONTRACT_EXPIRY_TYPE",
        "perpetual_details": {
          "open_interest": "<string>",
          "funding_rate": "<string>",
          "funding_time": "<string>",
          "max_leverage": "<string>",
          "base_asset_uuid": "<string>",
          "underlying_type": "<string>"
        },
        "contract_display_name": "<string>",
        "time_to_expiry_ms": "<string>",
        "non_crypto": true,
        "contract_expiry_name": "<string>",
        "twenty_four_by_seven": true,
        "funding_interval": "<string>",
        "open_interest": "<string>",
        "funding_rate": "<string>",
        "funding_time": "<string>",
        "display_name": "<string>",
        "region_enabled": {},
        "intraday_margin_rate": {
          "long_margin_rate": "0.5",
          "short_margin_rate": "0.5"
        },
        "overnight_margin_rate": {
          "long_margin_rate": "0.5",
          "short_margin_rate": "0.5"
        },
        "settlement_price": "<string>"
      }
    }
  ],
  "num_products": 100,
  "pagination": {
    "prev_cursor": "<string>",
    "next_cursor": "<string>",
    "has_next": true,
    "has_prev": true
  }
}
```