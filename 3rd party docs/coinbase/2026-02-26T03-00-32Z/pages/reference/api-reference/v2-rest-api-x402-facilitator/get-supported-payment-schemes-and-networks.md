# get supported payment schemes and networks

```
{
  "kinds": [
    {
      "x402Version": 1,
      "scheme": "exact",
      "network": "base"
    },
    {
      "x402Version": 1,
      "scheme": "exact",
      "network": "base-sepolia"
    },
    {
      "x402Version": 1,
      "scheme": "exact",
      "network": "solana"
    },
    {
      "x402Version": 1,
      "scheme": "exact",
      "network": "solana-devnet"
    }
  ],
  "extensions": [
    "bazaar"
  ],
  "signers": {
    "eip155:*": [
      "0x1234567890abcdef1234567890abcdef12345678",
      "0xabcdef1234567890abcdef1234567890abcdef12"
    ],
    "solana:*": [
      "5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d"
    ]
  }
}
```