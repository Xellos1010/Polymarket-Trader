# send a user operation

```
{
  "network": "base",
  "userOpHash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
  "calls": [
    {
      "to": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
      "value": "0",
      "data": "0xa9059cbb000000000000000000000000fc807d1be4997e5c7b33e4d8d57e60c5b0f02b1a0000000000000000000000000000000000000000000000000000000000000064"
    },
    {
      "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
      "value": "1000000000000000",
      "data": "0x"
    }
  ],
  "status": "pending",
  "transactionHash": "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
  "receipts": [
    {
      "revert": {
        "data": "0x123",
        "message": "reason for failure"
      },
      "blockHash": "0x386544b58930c0ec9e8f3ed09fb4cdb76b9ae0a1a37ddcacebe3925b57978e65",
      "blockNumber": 29338819,
      "gasUsed": "100000"
    }
  ]
}
```