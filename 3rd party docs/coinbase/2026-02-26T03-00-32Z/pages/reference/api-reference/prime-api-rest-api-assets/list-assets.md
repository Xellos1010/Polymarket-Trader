# list assets

```
{
  "assets": [
    {
      "name": "Bitcoin",
      "symbol": "BTC",
      "decimal_precision": "8",
      "trading_supported": true,
      "explorer_url": "https://live.blockcypher.com/btc/",
      "networks": [
        {
          "network": {
            "id": "<string>",
            "type": "<string>"
          },
          "name": "Ethereum",
          "max_decimals": "8",
          "default": true,
          "trading_supported": true,
          "vault_supported": true,
          "prime_custody_supported": true,
          "destination_tag_required": true,
          "network_link": "https://live.blockcypher.com/btc/",
          "network_scoped_symbol": "BASEUSDC"
        }
      ]
    }
  ]
}
```