# fetch historical staking balances

```
{
  "data": [
    {
      "address": "0x80000001677f23a227dfed6f61b132d114be83b8ad0aa5f3c5d1d77e6ee0bf5f73b0af750cc34e8f2dae73c21dc36f4a",
      "date": "2024-07-21",
      "bonded_stake": {
        "amount": "12345678",
        "asset": {
          "network_id": "base-sepolia",
          "asset_id": "USDC",
          "decimals": 18,
          "contract_address": "0x036CbD53842c5426634e7929541eC2318f3dCF7e"
        }
      },
      "unbonded_balance": {
        "amount": "12345678",
        "asset": {
          "network_id": "base-sepolia",
          "asset_id": "USDC",
          "decimals": 18,
          "contract_address": "0x036CbD53842c5426634e7929541eC2318f3dCF7e"
        }
      },
      "participant_type": "validator"
    }
  ],
  "has_more": true,
  "next_page": "<string>"
}
```