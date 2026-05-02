# evm policies

## Overview

This page provides comprehensive examples and implementations for EVM-based policy configurations. These policies control EVM transaction signing, sending, message signing, and smart account operations. For general policy concepts and setup instructions, see the [Policies Overview](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview).

## Supported EVM Operations

EVM policies support the following operations:

-   `signEvmTransaction`: To identify incoming signing transactions on the EVM network
-   `sendEvmTransaction`: To identify incoming signing transactions that are then sent to a supported network
-   `signEvmTypedData`: To identify incoming typed data to be signed by an account
-   `signEvmMessage`: To identify incoming messages to be signed by an account
-   `signEvmHash`: To identify incoming hash to be signed by an account
-   `prepareUserOperation`: For preparing user operations on a smart account
-   `sendUserOperation`: For sending user operations using a smart account

## Supported Criteria Types

The following criteria types are available for EVM policies:

-   **`evmAddress`**: Criterion for recipient addresses of EVM transactions
-   **`ethValue`**: Criterion for ETH value in wei of native transfers
-   **`evmNetwork`**: Criterion for restricting transactions to specific EVM networks
-   **`evmMessage`**: Criterion for validating message content using regex patterns
-   **`evmData`**: Criterion for transaction data validation using contract ABI
-   **`evmTypedDataVerifyingContract`**: Criterion for EIP-712 verifying contract addresses
-   **`evmTypedDataField`**: Criterion for validating specific fields within EIP-712 typed data
-   **`netUSDChange`**: Criterion for USD-denominated limits on asset transfers and approvals

## Example policies

### Allowlist

The following example demonstrates a policy that allows signing transactions only to specific EVM addresses. Transactions to any address outside of this list will automatically be deleted by the policy engine.

```
{
  "description": "Allowlist policy example",
  "rules": [
    {
      "action": "accept",
      "criteria": [
        {
          "addresses": [
            "0xffffffffffffffffffffffffffffffffffffffff",
            "0x1111111111111111111111111111111111111111"
          ],
          "operator": "in",
          "type": "evmAddress"
        }
      ],
      "operation": "signEvmTransaction"
    }
  ],
  "scope": "project"
}

```

The above policy treats the set of `addresses` as an allowlist, only accepting sign transaction requests to an EVM address that is `in` the set.

### Denylist

The following example demonstrates a policy that rejects signing transactions to specific EVM addresses. Transactions with `to` field set to any address outside of this list will be accepted.

```
{
  "description": "Denylist policy example",
  "rules": [
    {
      "action": "accept",
      "criteria": [
        {
          "addresses": [
            "0xffffffffffffffffffffffffffffffffffffffff",
            "0x1111111111111111111111111111111111111111"
          ],
          "operator": "not in",
          "type": "evmAddress"
        }
      ],
      "operation": "signEvmTransaction"
    }
  ],
  "scope": "project"
}

```

The above policy treats the set of `addresses` as a denylist, rejecting any sign transaction to an address that is `not in` the set.

### Transaction limits

The following example demonstrates a policy that only permits signing transactions with a value of 2000000000000000000 wei (2 ETH) or less.

transaction-limit-policy.json

```
{
  "description": "Transaction limit policy",
  "scope": "project",
  "rules": [
    {
      "action": "accept",
      "operation": "signEvmTransaction",
      "criteria": [
        {
          "type": "ethValue",
          "ethValue": "2000000000000000000",
          "operator": "<="
        }
      ]
    }
  ]
}

```

### USD Limits

The following example demonstrates a policy that only allows transactions to transfer, or expose the sender to, less than $100.00 worth of assets at a time for both EOA accounts and Smart Accounts. This USD denominated amount includes native assets, `ERC20`, `ERC721`, and `ERC1155` tokens calculated using current market prices.

How is change calculated?

The total change represents the dollar value of assets that the user either sends directly, or allows a spender to access from their wallet, in a single transaction. This includes:

-   Native asset transfers
-   Non-native token transfers
-   Token approvals from `approve`, `setApprovalForAll`, and `permit` functions

For example, if you approve 10,000 USD worth of tokens, the calculated change is is 10,000 USD. The spender hasn’t taken anything yet, but they could take up to that amount.**High Risk Examples**

-   Approving unlimited tokens `(type(uint256).max)` to an unknown contract
-   Giving approval to addresses with known malicious history
-   `setApprovalForAll` for valuable NFT collections

**Lower Risk Examples**

-   Limited approvals to well-known DEXs like Uniswap
-   Approvals to verified, reputable protocols
-   Time-limited permit signatures

```
{
  "scope": "account",
  "description": "Accept assets out under 100 USD",
  "rules": [
    {
      "action": "accept",
      "operation": "sendEvmTransaction",
      "criteria": [
        {
          "type": "netUSDChange",
          "changeCents": 10000,
          "operator": "<",
        },
      ],
    },
    {
      "action": "accept",
      "operation": "signEvmTransaction",
      "criteria": [
        {
          "type": "netUSDChange",
          "changeCents": 10000,
          "operator": "<",
        },
      ],
    },
    {
      "action": "reject",
      "operation": "prepareUserOperation",
      "criteria": [
        {
          "type": "netUSDChange",
          "changeCents": 10000,
          "operator": ">",
        },
      ],
    },
    {
      "action": "reject",
      "operation": "sendUserOperation",
      "criteria": [
        {
          "type": "netUSDChange",
          "changeCents": 10000,
          "operator": ">",
        },
      ],
    }
  ],
}

```

### Network restrictions

The following example demonstrates a policy that only permits sending transactions on the Base Sepolia network.

restricted-network-policy.json

```
{
  "description": "Restricted network policy",
  "scope": "project",
  "rules": [
    {
      "action": "accept",
      "operation": "sendEvmTransaction",
      "criteria": [
        {
          "type": "evmNetwork",
          "networks": ["base-sepolia"],
          "operator": "in"
        }
      ]
    }
  ]
}

```

### Multi-rule policies

Learn more on combining multiple rules in a single policy.

-   [**Allowlist first**](#allowlist-first): A policy that checks the allowlist first, then the transaction limit.
-   [**Allowlist second**](#allowlist-second): A policy that checks the transaction value first, then uses a combined rule to check both the transaction value and the allowlist.

#### Allowlist first

The following example demonstrates a policy that contains both an allowlist and a transaction limit.

```
{
  "description": "Allowlist then value limit",
  "scope": "project",
  "rules": [
    {
      "action": "accept",
      "operation": "signEvmTransaction",
      "criteria": [
        {
          "type": "evmAddress",
          "addresses": [
            "0xffffffffffffffffffffffffffffffffffffffff"
          ],
          "operator": "in"
        }
      ]
    },
    {
      "action": "accept",
      "operation": "signEvmTransaction",
      "criteria": [
        {
          "type": "ethValue",
          "ethValue": "2000000000000000000",
          "operator": "<="
        }
      ]
    }
  ]
}

```

In the example above, assume a user sends a sign transaction request with a value of 4000000000000000000 wei (4 ETH) to the address 0x123:

1.  The transaction will be rejected against the first rule, as the address is not in the allowlist. However, the criteria still is not met and the engine will evaluate the transaction against the second rule.
2.  The transaction will be rejected against the second rule, as the value is greater than 2000000000000000000 wei (2 ETH).

#### Allowlist second

Let’s take a look at another combined policy example where we define the allowlist as the second rule instead of the first.

```
{
  "description": "Value limit then allowlist",
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
        }
      ]
    },
    {
      "action": "accept",
      "operation": "signEvmTransaction",
      "criteria": [
        {
          "type": "ethValue",
          "ethValue": "2000000000000000000",
          "operator": "<="
        },
        {
          "type": "evmAddress",
          "addresses": [
            "0xffffffffffffffffffffffffffffffffffffffff"
          ],
          "operator": "in"
        }
      ]
    }
  ]
}

```

In the above example, if a user sends a transaction with a value of 1500000000000000000 wei (1.5 ETH) to the address 0x123:

1.  The transaction will be rejected against the first rule, as the value is greater than 1000000000000000000 wei. However, the criteria still is not met and the engine will continue evaluating the transaction against the second rule.
2.  The transaction matches against the second rule, as the value is less than or equal to 2000000000000000000 wei **AND** the address is in the allowlist. The transaction will be accepted.

What's the difference?

The primary differences between these two examples are the rule order in which a transaction is evaluated.

-   **The first example checks the allowlist first, and then the transaction limit**
    -   This option is more restrictive, as it requires all transactions go to allowlisted addresses first, regardless of their value.
-   **The second example checks the transaction value first, then uses a combined rule to check both the transaction value and the allowlist**
    -   This option offers more granular control by allowing small transactions to be signed to any address, but restricts larger transactions to allowlisted addresses only.

### Message signing restrictions

The following example demonstrates how to guarantee any attempt to sign a message will conform to a specific template. When composing a regular expression in the `match` field, any valid [re2](https://github.com/google/re2) regular expression syntax will be accepted.

accept-sign-message-policy.json

```
{
  "description": "Accept sign message policy",
  "scope": "project",
  "rules": [
    {
      "action": "accept",
      "operation": "signEvmMessage",
      "criteria": [
        {
          "type": "evmMessage",
          "match": "^I solemnly swear that I,(.*), am up to no good\.$"
        }
      ]
    }
  ]
}

```

### Limiting USDC Spend

This policy restricts USDC transactions on the Base network to transfers of 10,000 tokens or less. It applies to both signing and sending transactions to the USDC contract address, using the ERC20 ABI to validate that only `transfer` function calls with a `value` parameter under the specified limit are permitted.

limit-usdc-spend-policy.json

```
{
  "description": "Limit USDC Spend",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "sendEvmTransaction",
      "criteria": [
        {
          "type": "evmNetwork",
          "networks": ["base"],
          "operator": "in"
        },
        {
          "type": "evmAddress",
          "addresses": ["0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"],
          "operator": "in"
        },
        {
          "type": "evmData",
          "abi": "erc20",
          "conditions": [
            {
              "function": "transfer",
              "params": [
                {
                  "name": "value",
                  "operator": "<=",
                  "value": "10000"
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "action": "accept",
      "operation": "signEvmTransaction",
      "criteria": [
        {
          "type": "evmAddress",
          "addresses": ["0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"],
          "operator": "in"
        },
        {
          "type": "evmData",
          "abi": "erc20",
          "conditions": [
            {
              "function": "transfer",
              "params": [
                {
                  "name": "value",
                  "operator": "<=",
                  "value": "10000"
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}

```

### Transaction data restrictions with custom ABI

When working with smart contracts that aren’t covered by standard token interfaces (ERC20, ERC721, ERC1155), you need to provide a custom ABI to enable the policy engine to decode and validate transaction data. This is essential for:

-   **Custom smart contracts**: Protocols with unique function signatures and parameters
-   **Complex DeFi interactions**: Multi-step operations, custom swaps, or governance functions
-   **Proprietary contracts**: Internal business logic that requires specific validation rules
-   **Non-standard token implementations**: Tokens with additional features beyond basic standards

The custom ABI allows the policy engine to understand the function being called and validate specific parameters according to your security requirements.

```
{
  "description": "Proprietary Staking Contract",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signEvmTransaction",
      "criteria": [
        {
          "type": "evmAddress",
          "addresses": ["0x1234567890123456789012345678901234567890"],
          "operator": "in"
        },
        {
          "type": "evmData",
          "abi": [
            {
              "type": "function",
              "name": "stakeTokens",
              "inputs": [
                {
                  "name": "amount",
                  "type": "uint256",
                  "internalType": "uint256"
                },
                {
                  "name": "duration",
                  "type": "uint256",
                  "internalType": "uint256"
                },
                {
                  "name": "beneficiary",
                  "type": "address",
                  "internalType": "address"
                }
              ],
              "outputs": [
                {
                  "name": "stakeId",
                  "type": "uint256",
                  "internalType": "uint256"
                }
              ],
              "stateMutability": "nonpayable"
            }
          ],
          "conditions": [
            {
              "function": "stakeTokens",
              "params": [
                {
                  "name": "amount",
                  "operator": "<=",
                  "value": "1000000000000000000000"
                },
                {
                  "name": "duration",
                  "operator": ">=",
                  "value": "86400"
                },
                {
                  "name": "beneficiary",
                  "operator": "in",
                  "values": [
                    "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd",
                    "0x1111111111111111111111111111111111111111"
                  ]
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}

```

This policy restricts the `stakeTokens` function to:

-   Maximum stake amount of 1000 tokens (assuming 18 decimals)
-   Minimum duration of 1 day (86400 seconds)
-   Only allow staking for pre-approved beneficiary addresses

### Disable signing arbitrary hashes

The example below demonstrates a policy to prevent fraud by rejecting any attempt to sign a hash (i.e., undefined or arbitrary input data) on behalf of an account.

reject-sign-hash-policy.json

```
{
  "description": "Reject sign hash policy",
  "scope": "project",
  "rules": [
    {
      "action": "reject",
      "operation": "signEvmHash"
    }
  ]
}

```

### EIP-712 verifying contract allowlist

The following example demonstrates a policy that prevents signing typed data for the zero address. This ensures that typed data signatures cannot be created for invalid or burn addresses.

sign-typed-data-verifying-contract-policy.json

```
{
  "description": "Prevent typed data signing for zero address",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signEvmTypedData",
      "criteria": [
        {
          "type": "evmTypedDataVerifyingContract",
          "addresses": ["0x0000000000000000000000000000000000000000"],
          "operator": "not in"
        }
      ]
    }
  ]
}

```

This policy uses the verifying contract address from the EIP-712 domain to ensure that typed data signatures are not created for the zero address. You can extend this to create an allowlist by using the `"in"` operator with trusted contract addresses, or a denylist by using `"not in"` with untrusted addresses.

### EIP-712 field restrictions

The following example demonstrates a more advanced policy that validates specific fields within typed data. This policy restricts an arbitrary Payment data type to a specific address, message content, and amount:

sign-typed-data-field-policy.json

```
{
  "description": "Restrict Payment data type",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "signEvmTypedData",
      "criteria": [
        {
          "type": "evmTypedDataField",
          "types": {
            "primaryType": "Payment",
            "types": {
              "EIP712Domain": [
                { "name": "name", "type": "string" },
                { "name": "version", "type": "string" },
                { "name": "chainId", "type": "uint256" },
                { "name": "verifyingContract", "type": "address" }
              ],
              "Payment": [
                { "name": "to", "type": "address" },
                { "name": "amount", "type": "uint256" },
                { "name": "message", "type": "string" },
              ]
            }
          },
          "conditions": [
            {
              "path": "to",
              "operator": "in",
              "value": ["0x1234567890123456789012345678901234567890", "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"]
            },
            {
              "path": "message",
              "match": "^hello.*"
            },
            {
              "path": "amount",
              "operator": "<=",
              "value": "100"
            }
          ]
        }
      ]
    }
  ]
}

```

This policy uses the `evmTypedDataField` criterion to inspect the actual data being signed. The `evmTypedDataField` criterion supports conditions on numerical values, addresses and strings.

Difference between evmTypedDataVerifyingContract and evmTypedDataField

-   **`evmTypedDataVerifyingContract`**: Checks only the verifying contract address in the EIP-712 domain. This is simpler and useful for allowlisting/denylisting contracts.
-   **`evmTypedDataField`**: Allows inspection of the entire typed data structure including types and field values. This enables more complex validations like checking specific field values, ranges, or data types.

### Smart Account Operations

The following policy restricts Smart Account operations to USDC transactions on the Base network, with transfers of 10,000 tokens or less. It applies to both prepare and send operations to the USDC contract address, using the ERC20 ABI to validate that only `transfer` function calls with a `value` parameter under the specified limit are permitted.

smart-account-operations-policy.json

```
{
  "description": "Smart Account USDC Limits",
  "scope": "account",
  "rules": [
    {
      "action": "accept",
      "operation": "prepareUserOperation",
      "criteria": [
        {
          "type": "evmNetwork",
          "networks": ["base"],
          "operator": "in"
        },
        {
          "type": "evmAddress",
          "addresses": ["0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"],
          "operator": "in"
        },
        {
          "type": "evmData",
          "abi": "erc20",
          "conditions": [
            {
              "function": "transfer",
              "params": [
                {
                  "name": "value",
                  "operator": "<=",
                  "value": "10000"
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "action": "accept",
      "operation": "sendUserOperation",
      "criteria": [
        {
          "type": "evmAddress",
          "addresses": ["0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"],
          "operator": "in"
        },
        {
          "type": "evmData",
          "abi": "erc20",
          "conditions": [
            {
              "function": "transfer",
              "params": [
                {
                  "name": "value",
                  "operator": "<=",
                  "value": "10000"
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}

```

## Complete implementation example

The following example shows how to create and manage multiple EVM policies programmatically:

## What to read next

-   [**Policies Overview**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview): Learn about general policy concepts and setup
-   [**Solana Policies**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/solana-policies): Learn about Solana-specific policy examples
-   [**v2 Server Wallet Security**](https://developer.chrome.com/server-wallets/v2/introduction/security): Learn more about the security features of the CDP v2 Server Wallet
-   [**v2 API Reference**](https://developer.chrome.com/api-reference/v2/rest-api/policy-engine/list-policies): Explore the API reference for CDP Policies