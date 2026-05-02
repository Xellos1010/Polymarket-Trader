# run sql against decoded onchain data

```
{  
  "result": [  
    {  
      "event_signature": "Transfer(address,address,uint256)",  
      "from": "0x1234567890abcdef",  
      "to": "0x1234567890abcdef",  
      "amount": 1000000000000000000  
    },  
    {  
      "event_signature": "Transfer(address,address,uint256)",  
      "from": "0x1234567890abcdef",  
      "to": "0x1234567890abcdef",  
      "amount": 2000000000000000000  
    }  
  ],  
  "schema": {  
    "columns": [  
      {  
        "name": "block_number",  
        "type": "UInt64"  
      },  
      {  
        "name": "transaction_hash",  
        "type": "String"  
      }  
    ]  
  },  
  "metadata": {  
    "cached": false,  
    "executionTimestamp": "2025-01-01T00:00:00.000Z",  
    "executionTimeMs": 145,  
    "rowCount": 2  
  }  
}
```