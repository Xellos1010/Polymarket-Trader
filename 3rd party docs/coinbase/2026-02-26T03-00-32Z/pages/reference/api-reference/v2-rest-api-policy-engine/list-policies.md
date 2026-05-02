# list policies

```
{
  "policies": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "scope": "project",
      "rules": [
        {
          "action": "accept",
          "operation": "signEvmTransaction",
          "criteria": [
            {
              "type": "ethValue",
              "ethValue": "1000000000000000000",
              "operator": "<="
            },
            {
              "type": "evmAddress",
              "addresses": [
                "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
                "0x1234567890123456789012345678901234567890"
              ],
              "operator": "in"
            }
          ]
        },
        {
          "action": "accept",
          "operation": "signSolTransaction",
          "criteria": [
            {
              "type": "solAddress",
              "addresses": [
                "HpabPRRCFbBKSuJr5PdkVvQc85FyxyTWkFM2obBRSvHT"
              ],
              "operator": "in"
            }
          ]
        }
      ],
      "createdAt": "2025-03-25T12:00:00Z",
      "updatedAt": "2025-03-26T12:00:00Z",
      "description": "Default policy"
    }
  ],
  "nextPageToken": "eyJsYXN0X2lkIjogImFiYzEyMyIsICJ0aW1lc3RhbXAiOiAxNzA3ODIzNzAxfQ=="
}
```