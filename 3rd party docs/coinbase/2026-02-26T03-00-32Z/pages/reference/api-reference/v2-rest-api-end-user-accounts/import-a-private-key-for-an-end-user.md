# import a private key for an end user

```
curl --request POST \
  --url https://api.cdp.coinbase.com/platform/v2/end-users/import \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --header 'X-Wallet-Auth: <x-wallet-auth>' \
  --data '
{
  "userId": "user-001",
  "authenticationMethods": [
    {
      "type": "email",
      "email": "user@example.com"
    }
  ],
  "encryptedPrivateKey": "U2FsdGVkX1+vupppZksvRf5X5YgHq4+da+Q4qf51+Q4=",
  "keyType": "evm"
}
'
```

```
{  
  "userId": "user-001",  
  "authenticationMethods": [  
    {  
      "type": "email",  
      "email": "user@example.com"  
    }  
  ],  
  "evmAccounts": [  
    "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"  
  ],  
  "evmAccountObjects": [  
    {  
      "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",  
      "createdAt": "2025-11-17T10:00:00Z"  
    }  
  ],  
  "evmSmartAccounts": [],  
  "evmSmartAccountObjects": [],  
  "solanaAccounts": [],  
  "solanaAccountObjects": [],  
  "createdAt": "2025-11-17T10:00:00Z"  
}
```

Imports an existing private key for an end user into the developer’s CDP Project. The private key must be encrypted using the CDP SDK’s encryption scheme before being sent to this endpoint. This API should be called from the [CDP SDK](https://github.com/coinbase/cdp-sdk) to ensure that the associated private key is properly encrypted.

This endpoint allows developers to import existing keys for their end users, supporting both EVM and Solana key types. The end user must have at least one authentication method configured.

```
curl --request POST \
  --url https://api.cdp.coinbase.com/platform/v2/end-users/import \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --header 'X-Wallet-Auth: <x-wallet-auth>' \
  --data '
{
  "userId": "user-001",
  "authenticationMethods": [
    {
      "type": "email",
      "email": "user@example.com"
    }
  ],
  "encryptedPrivateKey": "U2FsdGVkX1+vupppZksvRf5X5YgHq4+da+Q4qf51+Q4=",
  "keyType": "evm"
}
'
```

```
{  
  "userId": "user-001",  
  "authenticationMethods": [  
    {  
      "type": "email",  
      "email": "user@example.com"  
    }  
  ],  
  "evmAccounts": [  
    "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"  
  ],  
  "evmAccountObjects": [  
    {  
      "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",  
      "createdAt": "2025-11-17T10:00:00Z"  
    }  
  ],  
  "evmSmartAccounts": [],  
  "evmSmartAccountObjects": [],  
  "solanaAccounts": [],  
  "solanaAccountObjects": [],  
  "createdAt": "2025-11-17T10:00:00Z"  
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

A JWT signed using your Wallet Secret, encoded in base64. Refer to the [Generate Wallet Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-wallet-token) section of our Authentication docs for more details on how to generate your Wallet Token.

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Body

A stable, unique identifier for the end user. The `userId` must be unique across all end users in the developer's CDP Project. It must be between 1 and 100 characters long and can only contain alphanumeric characters and hyphens.

Example:

`"e051beeb-7163-4527-a5b6-35e301529ff2"`

authenticationMethods

(EmailAuthentication · object | SmsAuthentication · object | DeveloperJWTAuthentication · object | OAuth2Authentication · object | object)\[\]

required

The list of valid authentication methods linked to the end user.

Information about how the end user is authenticated.

-   EmailAuthentication
    
-   SmsAuthentication
    
-   DeveloperJWTAuthentication
    
-   OAuth2Authentication
    
-   Option 5
    

Example:

```
[  
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
]
```

The base64-encoded, encrypted private key to import. The private key must be encrypted using the CDP SDK's encryption scheme. This is a 32-byte raw private key.

Example:

`"U2FsdGVkX1+vupppZksvRf5X5YgHq4+da+Q4qf51+Q4="`

The type of key being imported. Determines what type of account will be associated for the end user.

Available options

:

`evm`,

`solana`

#### Response

Successfully imported key and created end user with the associated account.

Information about the end user.

A stable, unique identifier for the end user. The `userId` must be unique across all end users in the developer's CDP Project. It must be between 1 and 100 characters long and can only contain alphanumeric characters and hyphens.

Example:

`"e051beeb-7163-4527-a5b6-35e301529ff2"`

authenticationMethods

(EmailAuthentication · object | SmsAuthentication · object | DeveloperJWTAuthentication · object | OAuth2Authentication · object | object)\[\]

required

The list of valid authentication methods linked to the end user.

Information about how the end user is authenticated.

-   EmailAuthentication
    
-   SmsAuthentication
    
-   DeveloperJWTAuthentication
    
-   OAuth2Authentication
    
-   Option 5
    

Example:

```
[  
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
]
```

evmAccounts

string\[\]

required

deprecated

DEPRECATED: Use `evmAccountObjects` instead for richer account information. The list of EVM account addresses associated with the end user. End users can have up to 10 EVM accounts.

The address of the EVM account associated with the end user.

Example:

```
[  
  "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"  
]
```

The list of EVM accounts associated with the end user. End users can have up to 10 EVM accounts.

Example:

```
[  
  {  
    "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",  
    "createdAt": "2025-01-15T10:30:00Z"  
  },  
  {  
    "address": "0x1234567890abcdef1234567890abcdef12345678",  
    "createdAt": "2025-01-15T11:00:00Z"  
  }  
]
```

evmSmartAccounts

string\[\]

required

deprecated

DEPRECATED: Use `evmSmartAccountObjects` instead for richer account information including owner relationships. The list of EVM smart account addresses associated with the end user. Each EVM EOA can own one smart account.

The address of the EVM smart account associated with the end user.

Example:

```
[  
  "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"  
]
```

The list of EVM smart accounts associated with the end user. Each EVM EOA can own one smart account.

Example:

```
[  
  {  
    "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",  
    "ownerAddresses": [  
      "0x1234567890abcdef1234567890abcdef12345678",  
      "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"  
    ],  
    "createdAt": "2025-01-15T12:00:00Z"  
  }  
]
```

solanaAccounts

string\[\]

required

deprecated

DEPRECATED: Use `solanaAccountObjects` instead for richer account information. The list of Solana account addresses associated with the end user. End users can have up to 10 Solana accounts.

The base58 encoded address of the Solana account associated with the end user.

Example:

```
[  
  "HpabPRRCFbBKSuJr5PdkVvQc85FyxyTWkFM2obBRSvHT"  
]
```

The list of Solana accounts associated with the end user. End users can have up to 10 Solana accounts.

Example:

```
[  
  {  
    "address": "HpabPRRCFbBKSuJr5PdkVvQc85FyxyTWkFM2obBRSvHT",  
    "createdAt": "2025-01-15T10:30:00Z"  
  },  
  {  
    "address": "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",  
    "createdAt": "2025-01-15T11:30:00Z"  
  }  
]
```

createdAt

string<date-time>

required

The date and time when the end user was created, in ISO 8601 format.

Example:

`"2025-01-15T10:30:00Z"`

Information about the end user's MFA enrollments.

Example:

```
{  
  "enrollmentPromptedAt": "2025-01-15T10:30:00Z",  
  "totp": { "enrolledAt": "2025-01-15T10:30:00Z" },  
  "sms": { "enrolledAt": "2025-01-15T10:30:00Z" }  
}
```