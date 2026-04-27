# solana policies

## Overview

This page provides comprehensive examples and implementations for Solana-based policy configurations. These policies control Solana transaction signing and sending, including SOL transfers and SPL token operations. For general policy concepts and setup instructions, see the [Policies Overview](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview).

## Supported Solana Operations

Solana policies support the following operations:

-   `signSolTransaction`: To identify incoming signing transactions on the Solana network
-   `sendSolTransaction`: To identify incoming transactions that are signed and sent to the Solana network
-   `signSolMessage`: To identify incoming messages to be signed by a Solana account

## Supported Criteria Types

The following criteria types are available for Solana policies:

-   **`solAddress`**: Criterion for recipient addresses of native SOL transfers
-   **`solValue`**: Criterion for SOL value in lamports of native transfers
-   **`splAddress`**: Criterion for recipient token account addresses of SPL token transfers
-   **`splValue`**: Criterion for SPL token amounts being transferred, in units of the token decimals
-   **`mintAddress`**: Criterion for token mint addresses of SPL token transfers
-   **`solData`**: Criterion for instruction parameters given an program IDL
-   **`programId`**: Criterion based on the program IDs of a transaction’s instructions
-   **`solNetwork`**: Criterion based on the intended network of the transaction
-   **`solMessage`**: Criterion based on the intended message to be signed

## SOL Transfer Policies

### Allow SOL transfers only to known safe addresses

This policy restricts SOL transfers to a predefined list of trusted recipient addresses.

sol-allowlist-policy.json

```
{
  "description": "Allow SOL transfers only to safe addresses",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "solAddress",
          "addresses": [
            "11111111111111111111111111111112",
            "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
            "HpabPRRCFbBKSuJr5PdkVvQc85FyxyTWkFM2obBRSvHT"
          ],
          "operator": "in"
        }
      ]
    }
  ]
}

```

### Block SOL transfers exceeding 1 SOL

This policy prevents any SOL transfers that exceed 1 SOL (1,000,000,000 lamports).

```
{
  "description": "Block SOL transfers exceeding 1 SOL",
  "scope": "project",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "solValue",
          "solValue": "1000000000",
          "operator": "<="
        }
      ]
    }
  ]
}

```

### Restrict SOL transfers to trusted addresses under 5 SOL

This policy ensures SOL can only be sent to specific trusted addresses and limits transfers to 5 SOL maximum.

trusted-sol-transfers-policy.json

```
{
  "description": "Allow SOL transfers only to trusted addresses under 5 SOL",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "solAddress",
          "addresses": [
            "11111111111111111111111111111112",
            "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"
          ],
          "operator": "in"
        },
        {
          "type": "solValue",
          "solValue": "5000000000",
          "operator": "<="
        }
      ]
    }
  ]
}

```

## SPL Token Transfer Policies

### Only allow SPL token transfers for specific token mints

This policy restricts SPL token transfers to only specific token mint addresses (USDC and USDT in this example).

spl-token-mint-policy.json

```
{
  "description": "Allow only USDC and USDT transfers",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "mintAddress",
          "addresses": [
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"
          ],
          "operator": "in"
        }
      ]
    }
  ]
}

```

### Allow USDC token transfers up to 100 USDC

This policy allows USDC transfers up to 100 USDC (100,000,000 considering 6 decimals).

```
{
  "description": "Allow USDC transfers up to 100 USDC",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "mintAddress",
          "addresses": ["EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"],
          "operator": "in"
        },
        {
          "type": "splValue",
          "splValue": "100000000",
          "operator": "<="
        }
      ]
    }
  ]
}

```

### Allow transactions only if recipient token account address is allowlisted

This policy ensures SPL token transfers can only be sent to pre-approved token account addresses.

spl-recipient-allowlist-policy.json

```
{
  "description": "Allowlist SPL token recipient accounts",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "splAddress",
          "addresses": [
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
            "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R"
          ],
          "operator": "in"
        }
      ]
    }
  ]
}

```

### Advanced SPL token policy: USDT with amount and recipient restrictions

This comprehensive policy allows SPL token transfers only for USDC or USDT mints, with amounts between 100 and 10,000 tokens, and only to allowlisted recipient addresses.

```
{
  "description": "Advanced USDC/USDT policy with amount and recipient restrictions",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "mintAddress",
          "addresses": [
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"
          ],
          "operator": "in"
        },
        {
          "type": "splValue",
          "splValue": "100000000",
          "operator": ">="
        },
        {
          "type": "splValue",
          "splValue": "10000000000",
          "operator": "<="
        },
        {
          "type": "splAddress",
          "addresses": [
            "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
            "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R",
            "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj"
          ],
          "operator": "in"
        }
      ]
    }
  ]
}

```

## General Examples

### SOL and USDC amount restrictions

This policy allows sending up to 2 SOL or 1000 USDC.

sol-spl-combined-policy.json

```
{
  "description": "Allow SOL and SPL transfers with limits",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "sendSolTransaction",
      "criteria": [
        {
          "type": "solValue",
          "solValue": "2000000000",
          "operator": "<="
        }
      ]
    },
    {
      "action": "accept",
      "operation": "sendSolTransaction",
      "criteria": [
        {
          "type": "mintAddress",
          "addresses": [
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
          ],
          "operator": "in"
        },
        {
          "type": "splValue",
          "splValue": "1000000000",
          "operator": "<="
        }
      ]
    }
  ]
}

```

### Denylist malicious addresses

```
{
  "description": "Block transfers to known malicious addresses",
  "scope": "project",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "solAddress",
          "addresses": [
            "MaliciousAddress1111111111111111111111111",
            "ScamAddress111111111111111111111111111111"
          ],
          "operator": "not in"
        }
      ]
    }
  ]
}

```

### Limiting for large transfers

emergency-limits-policy.json

```
{
  "description": "Limit SOL transfers to 10 and USDC transfers to 10000",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "solValue",
          "solValue": "10000000000",
          "operator": "<="
        }
      ]
    },
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "splValue",
          "splValue": "10000000000",
          "operator": "<="
        },
        {
          type: "mintAddress",
          addresses: ["EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"],
          operator: "in",
        },
      ]
    }
  ]
}

```

### Allow only messages with custom prefix

This policy restricts message signing to only messages that start with “CDP:” prefix, helping prevent unauthorized message signing.

custom-message-policy.json

```
{
  "scope": "account",
  "description": "Allow messages with CDP prefix only",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolMessage",
      "criteria": [
        {
          "type": "solMessage",
          "match": "^CDP:.*"
        }
      ]
    }
  ]
}

```

### Allow only transactions with specific program interactions

This policy rejects transactions that interact with programs outside of your allowed list (System Program and Token Program in this example).

program-restriction-policy.json

```
{
  "scope": "account",
  "description": "Block transactions based on program allowlist",
  "rules": [
    {
      "action": "accept",
      "operation": "signSolTransaction",
      "criteria": [
        {
          "type": "programId",
          "programIds": [
            "11111111111111111111111111111111",
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
          ],
          "operator": "in"
        }
      ]
    }
  ]
}

```

## Implementation Examples

### Basic Solana policy creation

## Key Considerations for Solana Policies

### Token Decimals

When setting token amounts, remember to account for token decimals:

-   **SOL**: 9 decimals (1 SOL = 1,000,000,000 lamports)
-   **USDC**: 6 decimals (1 USDC = 1,000,000 base units)
-   **USDT**: 6 decimals (1 USDT = 1,000,000 base units)

### Common Token Mint Addresses

-   **USDC**: `EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v`
-   **USDT**: `Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB`
-   **Wrapped SOL**: `So11111111111111111111111111111111111111112`

### Common Program Addresses

-   **System Program**: `11111111111111111111111111111112`
-   **Token Program**: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
-   **Associated Token Program**: `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`

## What to read next

-   [**Policies Overview**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview): Learn about general policy concepts and setup
-   [**Solana IDL Policies**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/solana-idl-policies): Learn about Solana IDL policies
-   [**EVM Policies**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/evm-policies): Learn about EVM-specific policy examples
-   [**v2 Server Wallet Security**](https://developer.chrome.com/server-wallets/v2/introduction/security): Learn more about the security features of the CDP v2 Server Wallet
-   [**v2 API Reference**](https://developer.chrome.com/api-reference/v2/rest-api/policy-engine/list-policies): Explore the API reference for CDP Policies