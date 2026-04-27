# policy engine

The Policy Engine APIs evaluate policies (a set of rules) to govern the behavior of accounts or projects, such as enforce allowlists and denylists.

-   A **policy** is a collection of `rules` with defined criteria.
-   Each **rule** contains a specified `action`, `operation`, and `criteria`:
    -   An `action` can either `accept` or `reject` a transaction if the criteria in the rule are met.
    -   `criteria` is an array of logical expressions. All parameters must evaluate to true for the action to be applied.
    -   An `operation` corresponds to a CDP v2 API:
        -   `signEvmTransaction` or `signSolTransaction` for signing transactions (to set a transaction limit).
        -   `sendEvmTransaction` for signing a transaction, and sending it to a supported network.
        -   `signEvmHash` for signing an arbitrary 32 byte hash.
        -   `signEvmMessage` for signing an [EIP-191](https://eips.ethereum.org/EIPS/eip-191) message.
        -   `prepareUserOperation` for preparing user operations on a smart account.
        -   `sendUserOperation` for sending user operations using a smart account.
-   A **rule** indicates how an operation should behave, specifying whether a request with defined criteria should be accepted or rejected.

## Policy Scope

Policies can be applied at the project and/or account level:

-   **Project-level policy**: A `project`\-level policy applies to all accounts in a CDP Project. Only one project-level policy can be applied to accounts within a CDP Project at any given time.
-   **Account-level policy**: An `account`\-level policy applies to one or more accounts. An account can have at most one account-level policy at any given time.

Thus, a single account can be assigned at most two policies at any given time: one project-level policy and one account-level policy. Scope is specified in the `scope` field of a policy:

```
  "description": "Project-level policy",
  "scope": "account",
  ...

```

## Policy Evaluation

**Project-level policies** are evaluated first, followed by **account-level policies**. The Policy Engine will process the request against each rule in the order it is defined within the `rules` array:

1.  If the rule’s `criteria` (processed as a logical **AND** operation applied to a list of independently evaluated boolean expressions) are met, `accept` or `reject` behavior is applied immediately and the engine stops further evaluation of the policy.
2.  If after policy evaluation, no rule’s `criteria` are met, the engine moves to processing the next policy (i.e., an `account`\-level policy).
3.  If no further policies exist, the request is rejected.

For example, the following policy is a project-level policy with two rules. The Policy Engine will:

1.  **Evaluate the first rule:** For a `signEvmTransaction` request, accept the request if the transaction is less than or equal to 1000000000000000000 wei OR
2.  **Evaluate the second rule:** if the request is a `signEvmTransaction` request, accept the request if the transaction is less than or equal to 2000000000000000000 wei AND the request is made to the address `0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE`.
3.  **If the request does not meet the criteria of either rule**, the engine will move on to evaluate an `account`\-level policy (if one exists).
4.  Otherwise, the request is rejected.

Rules are processed in the order they are defined. Once a rule applies to an operation, subsequent rules are ignored.

```
{
  "description": "Project-level policy",
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
            "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"
          ],
          "operator": "in"
        }
      ]
    }
  ]
}

```

## Policy Application

Project-level policies are applied to all accounts in a CDP Project. They will apply retroactively even if the project-level policy is created after the account was created. To disable a project-level policy, you must remove the project-level policy from the CDP Project using the `deletePolicy` operation. Account-level policies can be applied in two ways:

-   By specifying the `accountPolicy` field in the request body of the `createEvmAccount` and `createSolAccount` operations.
-   By specifying the `accountPolicy` field in the request body of the `updateEvmAccount` and `updateSolanaAccount` operations.

## Criteria

The following criteria are supported:

### SignEvmTransaction Criteria

#### ethValue

A criterion based on the value of the transaction. The transaction’s `value` field is compared to the criterion’s `ethValue` field using the `operator` field.

#### evmAddress

A criterion based on the recipient address of the transaction. The transaction’s `to` field is compared to the criterion’s `addresses` field using the `operator` field.

#### evmData

A criterion based on encoded transaction data that evaluates the function being called, as well as any number of arguments accessed by either name or index. Currently this criterion only supports primitive types; `string`, `bool`, `uint(8,16,32,64,256)`, `int(8,16,32,64,256)`, `address`, and both fixed and dynamic length `bytes`.

#### netUSDChange

A criterion based on the USD denominated market value of assets being transferred, or exposing the sender to. The types of assets included in the calculation include native assets, `ERC20`, `ERC721`, and `ERC1155` tokens. The sum total USD amount of assets being transferred and exposed is compared to the criterion’s `changeCents` field using the `operator` field. If signing a testnet transaction, then this criterion configuration will be ignored.

### SendEvmTransaction Criteria

#### ethValue

A criterion based on the value of the transaction. The transaction’s `value` field is compared to the criterion’s `ethValue` field using the `operator` field.

#### evmAddress

A criterion based on the recipient address of the transaction. The transaction’s `to` field is compared to the criterion’s `addresses` field using the `operator` field.

#### evmNetwork

A criterion based on the intended network of the transaction. The `network` field in the `sendEvmTransaction` request body is compared to the criterion’s `networks` field using the `operator` field. Valid networks for this criterion include:

-   `base`
-   `base-sepolia`
-   `ethereum`
-   `ethereum-sepolia`
-   `avalanche`
-   `polygon`
-   `optimism`
-   `arbitrum`

#### evmData

A criterion based on encoded transaction data that evaluates the function being called, as well as any number of arguments accessed by either name or index. Currently this criterion only supports primitive types; `string`, `bool`, `uint(8,16,32,64,256)`, `int(8,16,32,64,256)`, `address`, and both fixed and dynamic length `bytes`.

#### netUSDChange

A criterion based on the USD denominated market value of assets being transferred, or exposing the sender to. The types of assets included in the calculation include native assets, `ERC20`, `ERC721`, and `ERC1155` tokens. The sum total USD amount of assets being transferred and exposed is compared to the criterion’s `changeCents` field using the `operator` field. If sending a testnet transaction, then this criterion configuration will be ignored.

### SendUserOperation Criteria

#### ethValue

A criterion based on the value of the user operation. The operation’s `value` fields are compared to the criterion’s `ethValue` field using the `operator` field.

#### evmAddress

A criterion based on the recipient address of the operation. The operation’s `to` fields are compared to the criterion’s `addresses` field using the `operator` field.

#### evmData

A criterion based on encoded transaction data that evaluates the function being called, as well as any number of arguments accessed by either name or index. Currently this criterion only supports primitive types; `string`, `bool`, `uint(8,16,32,64,256)`, `int(8,16,32,64,256)`, `address`, and both fixed and dynamic length `bytes`.

### PrepareUserOperation Criteria

#### ethValue

A criterion based on the value of the user operation. The operation’s `value` fields are compared to the criterion’s `ethValue` field using the `operator` field.

#### evmAddress

A criterion based on the recipient address of the user operation. The operation’s `to` fields are compared to the criterion’s `addresses` field using the `operator` field.

#### evmNetwork

A criterion based on the intended network of the user operation. The `network` field in the `prepareUserOperation` request body is compared to the criterion’s `networks` field using the `operator` field. Valid networks for this criterion include:

-   `base-sepolia`
-   `base`
-   `arbitrum`
-   `optimism`
-   `zora`
-   `polygon`
-   `bnb`
-   `avalanche`
-   `ethereum`
-   `ethereum-sepolia`

#### evmData

A criterion based on encoded transaction data that evaluates the function being called, as well as any number of arguments accessed by either name or index. Currently this criterion only supports primitive types; `string`, `bool`, `uint(8,16,32,64,256)`, `int(8,16,32,64,256)`, `address`, and both fixed and dynamic length `bytes`.

### SignEvmHash Criteria

The `signEvmHash` operation does not accept any criteria. To prevent this operation from being executed by any account, specify a rule with `signEvmHash` as the operation, and `reject` as its action.

### SignEvmMessage Criteria

#### evmMessage

A criterion based on the intended message to be signed. The `match` field in the criteria is a [RE2](https://github.com/google/re2/wiki/Syntax) compliant regular expression that will be executed against the message in the API request.

### SignSolMessage Criteria

#### solMessage

A criterion based on the intended message to be signed. The `match` field in the criteria is a [RE2](https://github.com/google/re2/wiki/Syntax) compliant regular expression that will be executed against the message in the API request.

### SignSolTransaction Criteria

#### solAddress

A criterion based on the recipient addresses of the transaction. The criterion’s `address` field is compared to the list of addresses in the transaction’s `accountKeys` (for legacy transactions) or `staticAccountKeys` (for V0 transactions) array using the `operator` field.

#### solValue

A criterion based on the value of the transaction. The criterion’s `solValue` field is compared to the transaction’s `value`, which is the amount of SOL in lamports being transferred, using the `operator` field.

#### splAddress

A criterion based on the recipient addresses of SPL token transfer instructions in the transaction. The criterion’s `addresses` field is compared to the list of SPL token transfer recipient addresses in the transaction’s `accountKeys` (for legacy transactions) or `staticAccountKeys` (for V0 transactions) array using the `operator` field.

#### splValue

A criterion based on the SPL token value of SPL token transfer instructions in the transaction. The criterion’s `splValue` field is compared to the transaction instruction’s `value` field, which is the amount of the SPL token being transferred, using the `operator` field.

#### mintAddress

A criterion based on the token mint addresses of SPL token transfer instructions in the transaction. The criterion’s `addresses` field is compared to the list of token mint addresses in the transaction’s `accountKeys` (for legacy transactions) or `staticAccountKeys` (for V0 transactions) array using the `operator` field.

#### solData

A criterion based on Solana transaction instruction data. The criterion uses Interface Definition Language (IDL) specifications to decode and validate instruction data against specific rules. The `idls` field specifies which Solana programs to validate against (either known program names like “SystemProgram” and “TokenProgram”, or custom IDL objects), and the `conditions` field defines instruction-specific validation rules including instruction names and parameter constraints.

#### programId

A criterion based on the program IDs of a transaction’s instructions. The criterion’s `programIds` field is compared to the list of program IDs in the transaction’s instructions using the `operator` field.

### SendSolTransaction Criteria

#### solAddress

A criterion based on the recipient addresses of the transaction. The criterion’s `address` field is compared to the list of addresses in the transaction’s `accountKeys` (for legacy transactions) or `staticAccountKeys` (for V0 transactions) array using the `operator` field.

#### solValue

A criterion based on the value of the transaction. The criterion’s `solValue` field is compared to the transaction’s `value`, which is the amount of SOL in lamports being transferred, using the `operator` field.

#### splAddress

A criterion based on the recipient addresses of SPL token transfer instructions in the transaction. The criterion’s `addresses` field is compared to the list of SPL token transfer recipient addresses in the transaction’s `accountKeys` (for legacy transactions) or `staticAccountKeys` (for V0 transactions) array using the `operator` field.

#### splValue

A criterion based on the SPL token value of SPL token transfer instructions in the transaction. The criterion’s `splValue` field is compared to the transaction instruction’s `value` field, which is the amount of the SPL token being transferred, using the `operator` field.

#### mintAddress

A criterion based on the token mint addresses of SPL token transfer instructions in the transaction. The criterion’s `addresses` field is compared to the list of token mint addresses in the transaction’s `accountKeys` (for legacy transactions) or `staticAccountKeys` (for V0 transactions) array using the `operator` field.

#### solData

A criterion based on Solana transaction instruction data. The criterion uses Interface Definition Language (IDL) specifications to decode and validate instruction data against specific rules. The `idls` field specifies which Solana programs to validate against (either known program names like “SystemProgram” and “TokenProgram”, or custom IDL objects), and the `conditions` field defines instruction-specific validation rules including instruction names and parameter constraints.

#### programId

A criterion based on the program IDs of a transaction’s instructions. The criterion’s `programIds` field is compared to the list of program IDs in the transaction’s instructions using the `operator` field.

#### solNetwork

A criterion based on the intended network of the transaction. The `network` field in the `sendSolanaTransaction` request body is compared to the criterion’s `networks` field using the `operator` field. Valid networks for this criterion include:

-   `solana-devnet`
-   `solana`

## Restricting Contract Interactions on Ethereum

Smart contract function restrictions serve as a critical security and governance mechanism in decentralized applications, allowing developers and organizations to implement fine-grained access controls over their protocol interactions. One of the primary use cases for function restrictions is protecting high-risk operations from unauthorized access such as:

-   Fund transfers - Contract upgrades - Parameter modifications - Emergency pauses Policy Engine supports such restrictions that evaluate against transaction data with the `evmData` criterion for the `signEvmTransaction`, and `sendEvmTransaction` operations.

## Examples

### USD Limits

The following example demonstrates a policy that only allows transactions to transfer, or expose the sender to, less than $100.00 worth of assets at a time. This USD denominated amount includes native assets, `ERC20`, `ERC721`, and `ERC1155` tokens calculated using current market prices.

```
  "scope": "account",
  "description": "Reject assets out over 100 USD",
  "rules": [
    {
      "action": "reject",
      "operation": "sendEvmTransaction",
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
      "operation": "signEvmTransaction",
      "criteria": [
        {
          "type": "netUSDChange",
          "changeCents": 10000,
          "operator": ">",
        },
      ],
    },
  ],
} ```
### Limiting USDC Spend
This policy restricts USDC transactions on the Base network to transfers of 10,000 tokens or less. It applies to both signing and sending transactions to the USDC contract address, using the ERC20 ABI to validate that only `transfer` function calls with a `value` parameter under the specified limit are permitted.
```json {
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
} ```.

```