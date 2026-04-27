# staking lifecycle

Understanding when staking becomes active and when unstaking completes is important for managing staking positions. This page covers how to track the full lifecycle of staked assets, from initial stake request through activation to unstaking completion.

## Staking Activation

After a stake transaction is submitted and approved, there is a delay before the staked assets begin earning rewards. This activation period varies by network and depends on protocol-level queues and processing times.

### Ethereum (ETH)

After a deposit transaction is processed, the validator enters the activation queue. The queue processes validators based on a per-epoch churn limit, resulting in variable wait times depending on network demand. Top-up deposits to existing active validators have a shorter activation delay than new validator activations. Historical activation times for validators are available in the Prime UI. **Tracking Validator Status** The [Query Transaction Validators](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/list-transaction-validators) endpoint returns the per-validator status that determines when rewards begin accruing.

Validator Status

Description

`VALIDATOR_STATUS_PENDING`

Validator is activating and not yet earning rewards

`VALIDATOR_STATUS_ACTIVE`

Validator is staked and actively earning rewards

`VALIDATOR_STATUS_EXITING`

Validator is in the process of unstaking

`VALIDATOR_STATUS_EXITED`

Validator has completed unstaking

`VALIDATOR_STATUS_WITHDRAWN`

Validator funds have been withdrawn

The `validator_status` field reflects the underlying protocol state. A validator must reach `VALIDATOR_STATUS_ACTIVE` before it begins earning rewards.

### Solana (SOL)

Solana staking becomes active at the next epoch boundary after processing. Each epoch takes roughly 2-3 days depending on remaining slots and block times. **Balance Tracking Considerations** While polling balances can provide directional insight, note that `bonded_balance` does not directly indicate whether assets are actively earning rewards. Rewards accrue at epoch boundaries, and timing depends on Solana’s epoch schedule.

## Unstaking and Withdrawal

### Ethereum (ETH)

ETH unstaking has two different forms:

-   **Full Validator Exit** - Unstaking that reduces the staked balance to zero triggers a validator exit. This is subject to the exit queue, which has variable wait times depending on network conditions.
-   **Partial Withdrawal** - Unstaking while keeping at least 32 ETH staked does not result in the validator exiting the network.

**Tracking Unstaking Progress** The [Get Unstaking Status](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/get-unstaking-status) endpoint monitors ETH unstaking progress and provides:

-   Estimated completion timestamp
-   Hours remaining until funds become withdrawable
-   Amount being unstaked
-   Whether it’s a partial or full unstake

```
{
  "portfolio_id": "<string>",
  "wallet_id": "<string>",
  "wallet_address": "<string>",
  "current_timestamp": "2025-10-17T15:30:00.000Z",
  "validators": [
    {
      "validator_address": "<string>",
      "statuses": [
        {
          "amount": "16",
          "estimate_type": "UNSPECIFIED",
          "estimate_description": "Live estimate based on current network conditions",
          "unstake_type": "UNSTAKE_TYPE_UNSPECIFIED",
          "finishing_at": "2025-10-27T00:00:00.000Z",
          "remaining_hours": 672,
          "requested_at": "2025-09-29T12:00:00.000Z"
        }
      ]
    }
  ]
}

```

**Previewing Unstake Amounts** Since rewards continue to accumulate during the unbonding period, the final amount received may differ from the originally staked amount. The [Preview Unstake](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/preview-unstake) endpoint shows the estimated total that will be received, including any rewards earned while unbonding. **Unstaking Transactions** During and after the ETH unstaking process, the following transaction types are generated:

Transaction Type

Description

`COMPLETE_UNBONDING`

Generated when unstaking completes and funds become withdrawable

`REWARD`

Reward transactions may continue to arrive during the unbonding period as the validator remains active in the exit queue

These transaction types can be tracked using the [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions) or [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions) endpoints. **Claiming Rewards** Claiming rewards via Prime and partial unstaking (where at least 32 ETH remains staked) both use the same underlying Ethereum partial withdrawal mechanism, but differ in what can be withdrawn. Claim Rewards restricts withdrawals to accumulated rewards only, whereas unstake allows withdrawal of both principal and rewards. The [Claim Wallet Staking Rewards](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/claim-wallet-staking-rewards-alpha) endpoint initiates a reward claim. Transaction-level execution rewards are automatically deposited into the Staking Transaction Rewards wallet and do not require manual claiming.

```
POST /v1/portfolios/{portfolio_id}/wallets/{wallet_id}/staking/claim_rewards

```

An amount can optionally be specified, or omitted to claim the maximum available.

### Solana (SOL)

Solana unstaking completes at the next epoch boundary, the same as staking activation. Each epoch takes roughly 2-3 days. During this cooldown period, the funds appear in `unbonding_amount` and cannot be withdrawn. Reward accrual ends when the cooldown completes. To detect when unstaking is complete, monitor the wallet balance until `unbonding_amount` transitions to either `withdrawable_amount` or `bondable_amount`.

## API Reference

Endpoint

Purpose

Supported Assets

[Query Transaction Validators](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/list-transaction-validators)

Monitor validator activation status

ETH

[Get Unstaking Status](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/get-unstaking-status)

Track unstaking completion time

ETH

[Preview Unstake](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/preview-unstake)

Estimate total unstake amount including rewards

ETH

[Claim Wallet Staking Rewards](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/claim-wallet-staking-rewards-alpha)

Claim accumulated staking rewards

ETH