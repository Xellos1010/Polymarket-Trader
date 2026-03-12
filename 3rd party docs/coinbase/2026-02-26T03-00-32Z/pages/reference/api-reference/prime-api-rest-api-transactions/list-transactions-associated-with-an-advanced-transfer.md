# list transactions associated with an advanced transfer

```
{
  "transactions": [
    {
      "id": "BTC-USD",
      "wallet_id": "cde8dd34-b6cf-4c2c-82bc-5f86adacc868",
      "portfolio_id": "0a66a8c0-24ea-4f18-b14f-8c9cf7c1ba40",
      "type": "DEPOSIT",
      "status": "TRANSACTION_CREATED",
      "symbol": "BTC",
      "created_at": "2021-05-31T11:59:59.000Z",
      "completed_at": "2021-05-31T12:09:31.000Z",
      "amount": "100",
      "transfer_from": {
        "type": "PAYMENT_METHOD",
        "value": "0bf7bf1e-bafa-4d7e-9312-fa0bf3b63f27",
        "address": 6.6325114945411165e+47,
        "account_identifier": "387879289"
      },
      "transfer_to": {
        "type": "PAYMENT_METHOD",
        "value": "0bf7bf1e-bafa-4d7e-9312-fa0bf3b63f27",
        "address": 6.6325114945411165e+47,
        "account_identifier": "387879289"
      },
      "network_fees": "1.99",
      "fees": "4.53",
      "fee_symbol": "USD",
      "blockchain_ids": [
        7.004643996595406e+76
      ],
      "transaction_id": "A1B2C3D4",
      "destination_symbol": "USD",
      "estimated_network_fees": {
        "lower_bound": "1.99",
        "upper_bound": "2.99"
      },
      "network": "ethereum-mainnet",
      "estimated_asset_changes": [
        {
          "type": "BALANCE_TRANSFER",
          "symbol": "ETH",
          "amount": "100",
          "collection": {
            "name": "<string>"
          },
          "item": {
            "name": "<string>"
          }
        }
      ],
      "metadata": {
        "match_metadata": {
          "reference_id": "<string>",
          "settlement_date": "<string>"
        },
        "web3_transaction_metadata": {
          "label": "<string>",
          "confirmed_asset_changes": [
            {
              "type": "BALANCE_TRANSFER",
              "symbol": "ETH",
              "amount": "100",
              "collection": {
                "name": "<string>"
              },
              "item": {
                "name": "<string>"
              }
            }
          ]
        },
        "reward_metadata": {
          "subtype": "MEV_REWARD"
        }
      },
      "idempotency_key": "<string>",
      "onchain_details": {
        "signed_transaction": "01ac1872bb2967df00124a7fd15e470a10aab04852e19fc6f...",
        "risk_assessment": {
          "compliance_risk_detected": false,
          "security_risk_detected": false
        },
        "chain_id": "1",
        "nonce": "42",
        "replaced_transaction_id": "123e4567-e89b-12d3-a456-426614174000",
        "destination_address": 6.6325114945411165e+47,
        "skip_broadcast": false,
        "failure_reason": "insufficient_funds",
        "signing_status": "SIGNED"
      },
      "network_info": {
        "id": "<string>",
        "type": "<string>"
      },
      "process_requirements": {
        "travel_rule_status": "TRAVEL_RULE_STATUS_UNSPECIFIED"
      }
    }
  ]
}
```