# update a policy

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Headers

An optional [UUID v4](https://www.uuidgenerator.net/version4) request header for making requests safely retryable. When included, duplicate requests with the same key will return identical responses. Refer to our [Idempotency docs](https://docs.cdp.coinbase.com/api-reference/v2/idempotency) for more information on using idempotency keys.

Required string length: `36`

#### Path Parameters

The ID of the policy to update.

#### Body

rules

(SignEvmTransactionRule · object | SendEvmTransactionRule · object | SignEvmMessageRule · object | SignEvmTypedDataRule · object | SignSolTransactionRule · object | SendSolTransactionRule · object | SignSolMessageRule · object | SignEvmHashRule · object | PrepareUserOperationRule · object | SendUserOperationRule · object)\[\]

required

A list of rules that comprise the policy. There is a limit of 10 rules per policy.

A rule that limits the behavior of an account.

-   SignEvmTransactionRule
    
-   SendEvmTransactionRule
    
-   SignEvmMessageRule
    
-   SignEvmTypedDataRule
    
-   SignSolTransactionRule
    
-   SendSolTransactionRule
    
-   SignSolMessageRule
    
-   SignEvmHashRule
    
-   PrepareUserOperationRule
    
-   SendUserOperationRule
    

Example:

```
{  
  "action": "accept",  
  "operation": "signEvmTransaction",  
  "criteria": [  
    {  
      "type": "ethValue",  
      "ethValue": "1000000",  
      "operator": ">="  
    },  
    {  
      "type": "evmAddress",  
      "addresses": [  
        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"  
      ],  
      "operator": "in"  
    }  
  ]  
}
```

An optional human-readable description for the policy. Policy descriptions can consist of alphanumeric characters, spaces, commas, and periods, and be 50 characters or less.

#### Response

Successfully updated policy.

The unique identifier for the policy.

Example:

`"123e4567-e89b-12d3-a456-426614174000"`

The scope of the policy. Only one project-level policy can exist at any time.

Available options

:

`project`,

`account`

rules

(SignEvmTransactionRule · object | SendEvmTransactionRule · object | SignEvmMessageRule · object | SignEvmTypedDataRule · object | SignSolTransactionRule · object | SendSolTransactionRule · object | SignSolMessageRule · object | SignEvmHashRule · object | PrepareUserOperationRule · object | SendUserOperationRule · object)\[\]

required

A list of rules that comprise the policy.

A rule that limits the behavior of an account.

-   SignEvmTransactionRule
    
-   SendEvmTransactionRule
    
-   SignEvmMessageRule
    
-   SignEvmTypedDataRule
    
-   SignSolTransactionRule
    
-   SendSolTransactionRule
    
-   SignSolMessageRule
    
-   SignEvmHashRule
    
-   PrepareUserOperationRule
    
-   SendUserOperationRule
    

Example:

```
{  
  "action": "accept",  
  "operation": "signEvmTransaction",  
  "criteria": [  
    {  
      "type": "ethValue",  
      "ethValue": "1000000",  
      "operator": ">="  
    },  
    {  
      "type": "evmAddress",  
      "addresses": [  
        "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"  
      ],  
      "operator": "in"  
    }  
  ]  
}
```

Example:

```
[  
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
]
```

The ISO 8601 timestamp at which the Policy was created.

Example:

`"2025-03-25T12:00:00Z"`

The ISO 8601 timestamp at which the Policy was last updated.

Example:

`"2025-03-26T12:00:00Z"`

An optional human-readable description of the policy. Policy descriptions can consist of alphanumeric characters, spaces, commas, and periods, and be 50 characters or less.