# register a smart contract

```
{
  "smart_contract_id": "a50c4ee4-affa-408c-ae22-c4312e42966d",
  "network_id": "base-mainnet",
  "wallet_id": "d91d652b-d020-48d4-bf19-5c5eb5e280c7",
  "contract_address": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
  "contract_name": "my contract",
  "deployer_address": "0x1234567890abcdef1234567890abcdef12345678",
  "type": "erc20",
  "options": {
    "name": "MyToken",
    "symbol": "MTK",
    "total_supply": "1000000000000000000000"
  },
  "abi": "[ { name: \"mint\", type: \"function\", inputs: [ { name: \"to\", type: \"address\" }, { name: \"amount\", type: \"uint256\" }, ], outputs: [], stateMutability: \"nonpayable\", } ]",
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
  }
}
```