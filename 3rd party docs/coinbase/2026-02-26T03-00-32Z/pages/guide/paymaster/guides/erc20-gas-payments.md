# erc20 gas payments

CDP Paymaster supports ERC20 token gas payments, enabling users to pay for gas in tokens like USDC instead of ETH. This eliminates the need for users to hold native tokens while still covering their own transaction costs.

## How it works

Instead of the developer sponsoring gas, the **user pays for gas in an ERC20 token**:

1.  User approves the Paymaster to spend their tokens
2.  User submits a transaction
3.  Paymaster takes tokens from the user and pays gas in ETH
4.  Transaction executes

This creates a “gasless” experience where users don’t need ETH, but they still pay for their own transactions using tokens they already have.

## When to use this

Scenario

Recommended approach

Developer pays all gas costs

[Gas Sponsorship](https://developer.chrome.com/paymaster/introduction/quickstart) with `useCdpPaymaster: true`

User pays gas, but in tokens (no ETH needed)

**ERC20 Gas Payments** (this guide)

User pays gas in ETH

No Paymaster needed

## Supported tokens

Token

Address (Base)

USDC

`0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913`

Additional tokens coming soon.

## Implementation

The key requirement is ensuring the user has approved the Paymaster to spend their tokens. Include an approval in your transaction batch if the allowance is insufficient.

```
const tokenDecimals = 6;
const minTokenThreshold = 1 * 10 ** tokenDecimals; // $1
const tokenApprovalTopUp = 20 * 10 ** tokenDecimals; // $20
const tokenAddress = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"; // USDC on Base
// CDP Paymaster contract address on Base — users must approve this address
// to allow the Paymaster to transfer tokens as payment for gas
const paymasterAddress = "0x2FAEB0760D4230Ef2aC21496Bb4F0b47D634FD4c";
// Your main transaction
const mintCall = {
  abi: nftAbi,
  functionName: "mintTo",
  to: nftContractAddress,
  args: [account.address, 1],
};
let calls = [mintCall];
// Check current allowance
const allowance = await client.readContract({
  abi: parseAbi(["function allowance(address owner, address spender) returns (uint256)"]),
  address: tokenAddress,
  functionName: "allowance",
  args: [account.address, paymasterAddress],
});
// If allowance is low, add an approval to the batch
if (allowance < minTokenThreshold) {
  calls.unshift({
    abi: parseAbi(["function approve(address spender, uint256 amount) returns (bool)"]),
    functionName: "approve",
    to: tokenAddress,
    args: [paymasterAddress, tokenApprovalTopUp],
  });
}
// Send the transaction — Paymaster handles the rest
const hash = await bundlerClient.sendUserOperation({ calls });

```

## Paymaster contract address

Users must approve the Paymaster contract to spend their tokens. This is the address that will transfer tokens from the user’s wallet as payment for gas.

Network

Paymaster address

Base Mainnet

`0x2FAEB0760D4230Ef2aC21496Bb4F0b47D634FD4c`

## RPC methods

CDP Paymaster implements the following methods for ERC20 gas payments:

### pm\_getAcceptedPaymentTokens

Returns the tokens the Paymaster accepts for payment. **Request:**

```
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "pm_getAcceptedPaymentTokens",
  "params": ["0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789", "0x2105", {}]
}

```

**Response:**

```
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "acceptedTokens": [
      {
        "name": "USDC",
        "address": "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"
      }
    ]
  }
}

```

### pm\_getPaymasterData with ERC20 context

To request ERC20 payment, include the token address in the context field: **Request:**

```
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "pm_getPaymasterData",
  "params": [
    { /* userOperation fields */ },
    "0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789",
    "0x2105",
    {
      "erc20": "0x833589fcd6edb6e08f4c7c32d4f71b54bda02913"
    }
  ]
}

```

**Successful response:** The response includes a `tokenPayment` field showing the fee details:

```
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "paymasterAndData": "0x2faeb0760d4230ef2ac21496bb4f0b47d634fd4c...",
    "tokenPayment": {
      "name": "USDC",
      "address": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      "maxFee": "0xa7c8",
      "decimals": 6
    }
  }
}

```

**Rejection response (insufficient allowance):**

```
{
  "id": 1,
  "jsonrpc": "2.0",
  "error": {
    "code": -32002,
    "message": "request denied - no sponsorship and address can not pay with accepted token",
    "data": {
      "acceptedTokens": [
        {
          "name": "USDC",
          "address": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
        }
      ]
    }
  }
}

```

Aspect

Gas sponsorship

ERC20 gas payments

**Who pays**

Developer

User

**Payment currency**

Developer billed in USD

User pays in tokens (USDC)

**User needs ETH**

No

No

**User needs tokens**

No

Yes (for payment)

**Approval required**

No

Yes (token approval to Paymaster)

**Use case**

Onboarding, removing friction

Users cover own costs without ETH

## Next steps

-   [Gas sponsorship quickstart](https://developer.chrome.com/paymaster/introduction/quickstart) — Developer-paid gas
-   [Paymaster FAQs](https://developer.chrome.com/paymaster/faqs) — Common questions
-   [Base Account documentation](https://docs.base.org/base-account/guides/pay-gas-erc20) — Additional context on ERC20 gas payments