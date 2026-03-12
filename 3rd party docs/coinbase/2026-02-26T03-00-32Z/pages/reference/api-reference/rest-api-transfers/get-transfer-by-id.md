# get transfer by id

```
{
  "network_id": "base-sepolia",
  "wallet_id": "d91d652b-d020-48d4-bf19-5c5eb5e280c7",
  "address_id": "0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a",
  "destination": "0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a",
  "amount": "12345678",
  "asset_id": "USDC",
  "asset": {
    "network_id": "base-sepolia",
    "asset_id": "USDC",
    "decimals": 18,
    "contract_address": "0x036CbD53842c5426634e7929541eC2318f3dCF7e"
  },
  "transfer_id": "d91d652b-d020-48d4-bf19-5c5eb5e280c7",
  "gasless": true,
  "transaction": {
    "network_id": "base-sepolia",
    "from_address_id": "0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a",
    "unsigned_payload": "<string>",
    "status": "pending",
    "signed_payload": "<string>",
    "transaction_hash": "0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42",
    "transaction_link": "https://sepolia.basescan.org/tx/0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42",
    "content": {
      "from": "0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a",
      "to": "0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a",
      "gas": 1000,
      "gas_price": 1000,
      "hash": "0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42",
      "input": "<string>",
      "nonce": 136,
      "index": "0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42",
      "value": 100,
      "type": 2,
      "max_fee_per_gas": 190,
      "max_priority_fee_per_gas": 100,
      "priority_fee_per_gas": 1000,
      "transaction_access_list": {
        "access_list": [
          {
            "address": "<string>",
            "storage_keys": [
              "<string>"
            ]
          }
        ]
      },
      "flattened_traces": [
        {
          "error": "<string>",
          "type": "<string>",
          "from": "<string>",
          "to": "<string>",
          "value": "<string>",
          "gas": 123,
          "gas_used": 123,
          "input": "<string>",
          "output": "<string>",
          "sub_traces": 123,
          "trace_address": [
            123
          ],
          "trace_type": "<string>",
          "call_type": "<string>",
          "trace_id": "<string>",
          "status": 123,
          "block_hash": "<string>",
          "block_number": 123,
          "transaction_hash": "<string>",
          "transaction_index": 123
        }
      ],
      "block_timestamp": "2023-04-01T12:00:00Z",
      "mint": "0",
      "rlp_encoded_tx": "0x02f582426882013d8502540be4008502540be41c830493e094a55416de5de61a0ac1aa8970a280e04388b1de4b6f843a4b66f1c0808080"
    }
  },
  "sponsored_send": {
    "to_address_id": "0xfc807D1bE4997e5C7B33E4d8D57e60c5b0f02B1a",
    "raw_typed_data": "<string>",
    "typed_data_hash": "<string>",
    "status": "pending",
    "signature": "<string>",
    "transaction_hash": "0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42",
    "transaction_link": "https://sepolia.basescan.org/tx/0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42"
  },
  "unsigned_payload": "<string>",
  "signed_payload": "<string>",
  "transaction_hash": "0x53e11e94ebb2438d6ddcfa07dabc9b551d2f440f8363fea941083bc397a86a42",
  "status": "pending"
}
```