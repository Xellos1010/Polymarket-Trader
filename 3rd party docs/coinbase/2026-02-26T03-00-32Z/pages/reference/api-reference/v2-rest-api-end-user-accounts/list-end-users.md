# list end users

```
{
  "endUsers": [
    {
      "userId": "e051beeb-7163-4527-a5b6-35e301529ff2",
      "authenticationMethods": [
        {
          "type": "email",
          "email": "user@example.com"
        },
        {
          "type": "sms",
          "phoneNumber": "+12055555555"
        },
        {
          "type": "jwt",
          "sub": "e051beeb-7163-4527-a5b6-35e301529ff2",
          "kid": "NjVBRjY5MDlCMUIwNzU4RTA2QzZFMDQ4QzQ2MDAyQjVDNjk1RTM2Qg"
        },
        {
          "type": "google",
          "sub": "115346410074741490243",
          "email": "test.user@gmail.com"
        },
        {
          "type": "telegram",
          "id": 1223456,
          "firstName": "Satoshi",
          "lastName": "Nakamoto",
          "photoUrl": "https://image.url/profile.jpg",
          "authDate": 1770681412,
          "username": "satoshinakamoto"
        }
      ],
      "evmAccounts": [
        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
      ],
      "evmAccountObjects": [
        {
          "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
          "createdAt": "2025-01-15T10:30:00Z"
        },
        {
          "address": "0x1234567890abcdef1234567890abcdef12345678",
          "createdAt": "2025-01-15T11:00:00Z"
        }
      ],
      "evmSmartAccounts": [
        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
      ],
      "evmSmartAccountObjects": [
        {
          "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
          "ownerAddresses": [
            "0x1234567890abcdef1234567890abcdef12345678",
            "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
          ],
          "createdAt": "2025-01-15T12:00:00Z"
        }
      ],
      "solanaAccounts": [
        "HpabPRRCFbBKSuJr5PdkVvQc85FyxyTWkFM2obBRSvHT"
      ],
      "solanaAccountObjects": [
        {
          "address": "HpabPRRCFbBKSuJr5PdkVvQc85FyxyTWkFM2obBRSvHT",
          "createdAt": "2025-01-15T10:30:00Z"
        },
        {
          "address": "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
          "createdAt": "2025-01-15T11:30:00Z"
        }
      ],
      "createdAt": "2025-01-15T10:30:00Z",
      "mfaMethods": {
        "enrollmentPromptedAt": "2025-01-15T10:30:00Z",
        "totp": {
          "enrolledAt": "2025-01-15T10:30:00Z"
        },
        "sms": {
          "enrolledAt": "2025-01-15T10:30:00Z"
        }
      }
    }
  ],
  "nextPageToken": "eyJsYXN0X2lkIjogImFiYzEyMyIsICJ0aW1lc3RhbXAiOiAxNzA3ODIzNzAxfQ=="
}
```