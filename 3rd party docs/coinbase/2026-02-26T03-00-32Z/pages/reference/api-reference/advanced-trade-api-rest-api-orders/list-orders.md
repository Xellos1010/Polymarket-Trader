# list orders

```
{
  "orders": [
    {
      "order_id": "0000-000000-000000",
      "product_id": "BTC-USD",
      "user_id": "2222-000000-000000",
      "order_configuration": {
        "market_market_ioc": {
          "quote_size": "10.00",
          "base_size": "0.001"
        },
        "market_market_fok": {
          "quote_size": "10.00",
          "base_size": "0.001"
        },
        "sor_limit_ioc": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00"
        },
        "limit_limit_gtc": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00",
          "post_only": false
        },
        "limit_limit_gtd": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00",
          "end_time": "2021-05-31T09:59:59.000Z",
          "post_only": false
        },
        "limit_limit_fok": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00"
        },
        "twap_limit_gtd": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "start_time": "2021-05-31T07:59:59.000Z",
          "end_time": "2021-05-31T09:59:59.000Z",
          "limit_price": "10000.00",
          "number_buckets": "5",
          "bucket_size": "2.00",
          "bucket_duration": "300s"
        },
        "stop_limit_stop_limit_gtc": {
          "base_size": "0.001",
          "limit_price": "10000.00",
          "stop_price": "20000.00",
          "stop_direction": "20000.00"
        },
        "stop_limit_stop_limit_gtd": {
          "base_size": 0.001,
          "limit_price": "10000.00",
          "stop_price": "20000.00",
          "end_time": "2021-05-31T09:59:59.000Z",
          "stop_direction": "20000.00"
        },
        "trigger_bracket_gtc": {
          "base_size": 0.001,
          "limit_price": "10000.00",
          "stop_trigger_price": "20000.00"
        },
        "trigger_bracket_gtd": {
          "base_size": 0.001,
          "limit_price": "10000.00",
          "stop_trigger_price": "20000.00",
          "end_time": "2021-05-31T09:59:59.000Z"
        },
        "scaled_limit_gtc": {
          "orders": [
            {
              "quote_size": "10.00",
              "base_size": "0.001",
              "limit_price": "10000.00",
              "post_only": false
            }
          ],
          "quote_size": "<string>",
          "base_size": "<string>",
          "num_orders": 123,
          "min_price": "<string>",
          "max_price": "<string>",
          "price_distribution": "FLAT",
          "size_distribution": "UNKNOWN_DISTRIBUTION",
          "size_diff": "<string>",
          "size_ratio": "<string>"
        }
      },
      "side": "",
      "client_order_id": "11111-000000-000000",
      "status": "PENDING",
      "created_time": "2021-05-31T09:59:59.000Z",
      "completion_percentage": "50",
      "average_filled_price": "50",
      "number_of_fills": "2",
      "pending_cancel": true,
      "size_in_quote": false,
      "total_fees": "5.00",
      "size_inclusive_of_fees": false,
      "total_value_after_fees": "<string>",
      "last_update_time": "2021-05-31T09:59:59.000Z",
      "time_in_force": "UNKNOWN_TIME_IN_FORCE",
      "filled_size": "0.001",
      "fee": "<string>",
      "filled_value": "10000",
      "trigger_status": "UNKNOWN_TRIGGER_STATUS",
      "order_type": "UNKNOWN_ORDER_TYPE",
      "reject_reason": "REJECT_REASON_UNSPECIFIED",
      "settled": true,
      "product_type": "UNKNOWN_PRODUCT_TYPE",
      "reject_message": "<string>",
      "cancel_message": "<string>",
      "order_placement_source": "UNKNOWN_PLACEMENT_SOURCE",
      "outstanding_hold_amount": "<string>",
      "is_liquidation": true,
      "last_fill_time": "<string>",
      "edit_history": [
        {
          "price": "19000.00",
          "size": "0.001",
          "replace_accept_timestamp": "<string>"
        }
      ],
      "leverage": "<string>",
      "margin_type": "",
      "retail_portfolio_id": "b87a2d3f-8a1e-49b3-a4ea-402d8c389aca",
      "originating_order_id": "b87a2d3f-8a1e-49b3-a4ea-402d8c389aca",
      "attached_order_id": "b87a2d3f-8a1e-49b3-a4ea-402d8c389aca",
      "attached_order_configuration": {
        "market_market_ioc": {
          "quote_size": "10.00",
          "base_size": "0.001"
        },
        "market_market_fok": {
          "quote_size": "10.00",
          "base_size": "0.001"
        },
        "sor_limit_ioc": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00"
        },
        "limit_limit_gtc": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00",
          "post_only": false
        },
        "limit_limit_gtd": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00",
          "end_time": "2021-05-31T09:59:59.000Z",
          "post_only": false
        },
        "limit_limit_fok": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "limit_price": "10000.00"
        },
        "twap_limit_gtd": {
          "quote_size": "10.00",
          "base_size": "0.001",
          "start_time": "2021-05-31T07:59:59.000Z",
          "end_time": "2021-05-31T09:59:59.000Z",
          "limit_price": "10000.00",
          "number_buckets": "5",
          "bucket_size": "2.00",
          "bucket_duration": "300s"
        },
        "stop_limit_stop_limit_gtc": {
          "base_size": "0.001",
          "limit_price": "10000.00",
          "stop_price": "20000.00",
          "stop_direction": "20000.00"
        },
        "stop_limit_stop_limit_gtd": {
          "base_size": 0.001,
          "limit_price": "10000.00",
          "stop_price": "20000.00",
          "end_time": "2021-05-31T09:59:59.000Z",
          "stop_direction": "20000.00"
        },
        "trigger_bracket_gtc": {
          "base_size": 0.001,
          "limit_price": "10000.00",
          "stop_trigger_price": "20000.00"
        },
        "trigger_bracket_gtd": {
          "base_size": 0.001,
          "limit_price": "10000.00",
          "stop_trigger_price": "20000.00",
          "end_time": "2021-05-31T09:59:59.000Z"
        },
        "scaled_limit_gtc": {
          "orders": [
            {
              "quote_size": "10.00",
              "base_size": "0.001",
              "limit_price": "10000.00",
              "post_only": false
            }
          ],
          "quote_size": "<string>",
          "base_size": "<string>",
          "num_orders": 123,
          "min_price": "<string>",
          "max_price": "<string>",
          "price_distribution": "FLAT",
          "size_distribution": "UNKNOWN_DISTRIBUTION",
          "size_diff": "<string>",
          "size_ratio": "<string>"
        }
      },
      "current_pending_replace": {
        "price": "19000.00",
        "size": "0.001",
        "replace_accept_timestamp": "<string>"
      },
      "commission_detail_total": {
        "total_commission": "<string>",
        "gst_commission": "<string>",
        "withholding_commission": "<string>",
        "client_commission": "<string>",
        "venue_commission": "<string>",
        "regulatory_commission": "<string>",
        "clearing_commission": "<string>"
      },
      "workable_size": "3",
      "workable_size_completion_pct": "50",
      "product_details": {
        "equity_details": {
          "base_cbrn": "<string>",
          "ticker": "<string>",
          "quote_id": "<string>"
        }
      },
      "cost_basis_method": "COST_BASIS_METHOD_UNSPECIFIED",
      "displayed_order_config": "UNKNOWN_DISPLAYED_ORDER_CONFIG",
      "equity_trading_session": "UNKNOWN_EQUITY_TRADING_SESSION"
    }
  ],
  "has_next": true,
  "sequence": "<string>",
  "cursor": "789100"
}
```