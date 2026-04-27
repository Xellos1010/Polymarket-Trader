# usage

Coinbase Staking API supports **Dedicated ETH Staking** with full mainnet support for Ethereum’s latest upgrade *Pectra*. This guide covers how to stake, unstake, consolidate, top-up and manage validators — including both pre and post Pectra flows.

### What’s New with Pectra

-   Stake validators with up to **2048 ETH** (previously 32 ETH max)
-   Automatically **compound rewards** for high-balance validators
-   Unstake directly via the **execution layer** (partial or full exits)
-   **Consolidate** smaller legacy validators into fewer large ones

SDK Availability:

-   **Go SDK**: Pectra features available starting version [v0.0.27](https://github.com/coinbase/coinbase-sdk-go/releases/tag/v0.0.27)
-   **Node.js SDK**: Pectra features available starting version [v0.24.0](https://github.com/coinbase/coinbase-sdk-nodejs/releases/tag/v0.23.0)

* * *

### Stake (Pre & Post Pectra)

You can stake to either **pre-Pectra (0x01)** or **post-Pectra (0x02)** validators by selecting the appropriate withdrawal credential type.

-   **Minimum stake:** 32 ETH
-   **Maximum (post-Pectra only):** 2048 ETH
-   Ensure your external address has enough ETH to cover the stake **plus gas fees**.

The example below illustrates how to stake from an [external address](https://developer.chrome.com/server-wallets/v1/concepts/addresses#external-addresses).

### Unstake (via Execution Layer)

Post-Pectra validators can now be unstaked directly from the **execution layer** using the withdrawal address. This bypasses the consensus-layer exit process entirely. Supports both:

-   **Partial withdrawals**: Withdraw a portion of a validator’s balance
-   **Full exits**: Exit the validator completely and withdraw all funds

#### Partial Withdrawals

#### Full Exits

### Unstake (via Consensus Layer)

The consensus-layer unstaking process is still supported post-Pectra and works for both **pre-** and **post-Pectra** validators. To initiate a consensus-layer exit, a [voluntary exit message](https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#signedvoluntaryexit) must be signed by the validator and broadcast to the Ethereum network. You have two options when unstaking from external addresses:

-   **[Coinbase managed unstake](#coinbase-managed-unstake)** *(recommended)* : Coinbase signs and broadcasts the exit message on your behalf.
-   **[User managed unstake](#user-managed-unstake)**: Coinbase provides a pre-signed message, and **you** are responsible for [broadcasting](#broadcasting-exit-messages) it to the consensus layer.

#### Coinbase Managed Unstake

There are two options to build the coinbase managed unstake operation.

##### By Amount

For 0x01 validators, this amount should be in multiples of 32. If amount = 64 ETH, we pick 2 0x01 validators and exit them. This behind the scenes will identify validators to be exited, generate a voluntary exit message per validator, sign it with the validator’s private key and broadcast them for you.

##### By Validator

We support unstaking of both pre & post Pectra validators by validator pub keys. The amount is ignored in this case.

Once the unstake operation has completed successfully, congrats you’ve just exited a validator. Refer to the [View Validator Information](#view-validator-information) section to monitor your validator status. When it changes to `WITHDRAWAL_COMPLETE`, your funds should be available in the `withdrawal_address` set during staking.

#### User Managed Unstake

There are 2 options to build the coinbase managed unstake operation.

##### By Amount

For 0x01 validators this amount should be in multiples of 32. If amount = 64 ETH, we pick 2 0x01 validators and exit them. This behind the scenes will identify validators to be exited, generate a voluntary exit message per validator, sign it with the validator’s private key and broadcast them for you.

##### By Validator

We support unstaking of both pre & post Pectra validators by validator pub keys. The amount is ignored in this case.

### Validator Consolidation

You can consolidate smaller **pre-Pectra (0x01)** validators into larger **post-Pectra (0x02)** validators, without manually unstaking and re-staking. This reduces the number of active validators you manage and enables **auto-compounding rewards**. Two modes:

-   **Self-consolidation**: Convert a validator from 0x01 → 0x02 by setting the same pubkey as both source and target.
-   **Merge**: Consolidate a single 0x01 validator under an existing 0x02 validator.

### Validator Top-Ups

Validator top-ups allow you to add more ETH to an existing validator. This is useful for increasing the validator’s effective balance and rewards.

### View Staking Rewards

You can view historical staking rewards by validator address. This helps you track earnings over time, including USD-converted value and conversion rates. Refer to the [StakingReward docs](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_staking_reward.StakingReward.html) for a full list of supported methods. Look up staking rewards for a list of addresses.

### View Historical Staking Balances

Detailed information about the historical staking balances for given validator address, including bonded and unbonded stakes.

-   **Bonded Stakes**: The total amount of stake that is actively earning rewards to this address. Pending active stake is not included.
-   **Unbonded Balance**: This amount includes any ETH balance that is under the control of the wallet address but is not actively staked. Refer to the [StakingBalance docs](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_staking_balance.StakingBalance.html) for a full list of supported methods. Look up staking balances for an address.

## Validator Information

### View Validator Information

Detailed information is available for any validators that you’ve created. The validator status (i.e. `provisioned`, `active`, etc.) is available in the response and is printed to stdout in the example below. The Validator object documentation is available [here](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_validator.Validator.html) and the ListValidators documentation is available [here](https://pkg.go.dev/github.com/coinbase/coinbase-sdk-go/pkg/coinbase#Client.ListValidators)

Example output

Your validators will be listed with their respective statuses.

```
Id: 0x984209f61e2507de65de2b0b08ca9cb02c66fb5deab5eb780bfe298b4870e5babd942624c9028cb7820577a6f52ac2d2, Status: provisioned
Id: 0xa3fc791b5abb4b83fe0e9fe2f6bc5a2728f967c5e845dd353cfac6d9ed4677ad39aa32ee25a1dbdaad8248d71ee1e3a4, Status: active
Id: 0xadc25472f45a72446d0b5f7b5ec5760db14b198a21a8b0ad40ec673365c54ba1688ad0913f171135a94d4ce1f0ee684f, Status: active
Id: 0x8071b39b9cfaefc094aff22c76a30f41709ed18f00b36efd63c7c64c644b3482bdfad5018fa32246af1a6c96943c750c, Status: active
Id: 0x881eb088e400920706bf3281fcabd23bbea081d818c8a60f91faa1f2a1f2c8170b5a89f355ef832d05d8d1685c3e7a52, Status: unavailable

```

### Validator Statuses

A validator can have the following statuses, provided in the `status` field of the response:

Status

Description

Onchain State Equivalent

Action Required

Provisioning

Validator is being created by Coinbase

:no\_entry\_sign: (Coinbase Only Status)

Wait :hourglass\_flowing\_sand:

Provisioned

Validator has been created by Coinbase and is ready for a deposit

:no\_entry\_sign: (Coinbase Only Status)

Sign and broadcast the provided deposit transaction

Deposited

Deposit transaction has been signed, broadcasted, and finalized on the Ethereum network

:no\_entry\_sign: (Coinbase Only Status)

Wait :hourglass\_flowing\_sand:

Pending

Validator is in the activation queue. This means the Ethereum network has successfully executed the deposit transaction

`pending_queued`

Wait :hourglass\_flowing\_sand:

Active

Validator is active and earning rewards

`active_ongoing`

None

Exiting

Validator is in the exit queue. The validator is still earning rewards

`active_exiting`

Wait :hourglass\_flowing\_sand:

Exited

Validator is waiting to enter the withdrawal queue. This means the validator has exited the active set and rewards are no longer being earned.

`exited_unslashed`

Wait :hourglass\_flowing\_sand:

Withdrawal Available

Validator is in the withdrawal queue. The network will sweep available funds to the `withdrawal_address` on a predetermined schedule

`withdrawal_possible`

Wait :hourglass\_flowing\_sand:

Withdrawal Complete

Validator has completed its lifecycle. It no longer has any validating responsibilities and the available funds (rewards and initial stake) have been swept to the `withdrawal_address`

`withdrawal_done`

None

Unavailable

Validator was provisioned, but a deposit transaction was never broadcasted. Coinbase has spun down the provisioned validator

:no\_entry\_sign: (Coinbase Only Status)

None

Active Slashed

Validator has been slashed in a previous epoch. The validator is still in the active set, but rewards cannot be earned and a voluntary exit cannot be performed

`active_slashed`

Wait :hourglass\_flowing\_sand:

Exited Slashed

Validator has been slashed in a previous epoch. The validator has exited the active set

`exited_slashed`

None

### Filtering By Validator Statuses

You can filter the list of validators to view all validators with a specific status.

Example output

Your validators will be listed only if the status is active.

```
Id: 0xa3fc791b5abb4b83fe0e9fe2f6bc5a2728f967c5e845dd353cfac6d9ed4677ad39aa32ee25a1dbdaad8248d71ee1e3a4, Status: active
Id: 0xadc25472f45a72446d0b5f7b5ec5760db14b198a21a8b0ad40ec673365c54ba1688ad0913f171135a94d4ce1f0ee684f, Status: active
Id: 0x8071b39b9cfaefc094aff22c76a30f41709ed18f00b36efd63c7c64c644b3482bdfad5018fa32246af1a6c96943c750c, Status: active

```

## Broadcasting Exit Messages

The example below broadcasts pre-signed voluntary exit messages surfaced during an unstake process. Ethereum validator exit messages are special transaction types which are pre-signed by the validator keys and must be broadcast directly to the consensus layer.

## Signing and Broadcasting Transactions

The example below signs and broadcasts transactions surfaced via the staking operation resource. These are standard execution-layer EIP-1159 transactions and follow the normal Ethereum signing flow.