# usage

The Coinbase Staking API enables users to stake or unstake any amount of SOL. A user only needs to specify a wallet and a stake amount, and the API will handle the rest. See the [quickstart](https://developer.chrome.com/staking/staking-api/introduction/quickstart) to familiarize yourself with Coinbase Staking API and basic usage. The API automatically creates and manages the underlying Solana [stake accounts](https://solana.com/docs/references/staking/stake-accounts) on your behalf. The staking rewards automatically accrue on the derives stake accounts, which can be claimed in the two-step process of [unstaking](#unstake) and [claiming rewards](#claim-rewards).

## Stake

Before staking, **ensure that your address contains enough SOL** to cover the stake amount and network fees. For devnet funds, you can use [this faucet](https://faucet.solana.com/). The amount of SOL that is ultimately staked will be the user input subtracted by the [rent reserve](https://solana.com/docs/core/fees#rent) amount. The rent reserve amount is a Solana-mandated account minimum. So if the user input is 1 SOL and the rent reserve is 0.02 SOL, the amount that will ultimately be staked is ~0.98 SOL. Without this rent adjustment, the stake wouldn’t become active. We currently only support delegating to the Coinbase public validator [`6D2jqw9hyVCpppZexquxa74Fn33rJzzBx38T58VucHx9`](https://solanabeach.io/validator/6D2jqw9hyVCpppZexquxa74Fn33rJzzBx38T58VucHx9). This validator is operated by Coinbase and is located in Ireland.

### Step 1. Create a Stake Operation

### Step 2. Sign and Broadcast

Once the unstake operation has been built, the transaction has been constructed based on your inputs, but not signed or broadcasted. Now, you must relay the transactions to your end-user for signing and broadcasting. If you’d like to sign and broadcast in your own system, refer to the [signing and broadcasting transactions](#signing-and-broadcasting-transactions) section for an example.

### Step 3. Wait

Once the transaction is signed and broadcasted, the SOL will be “pending staked” for ~2 days. This delay is a direct consequence of the Solana network’s staking mechanics. After this period, the SOL will be staked and begin to earn rewards. You can tell your SOL is fully staked and earning rewards by checking for when your SOL is added to the unstakeable balance.

## Unstake

Unstaking is the first part of a two-step process to withdraw your staked assets. The second step is [Claim Stake](#claim-stake). In direct Solana staking, a user would need to select a specific [stake account](https://solana.com/docs/references/staking/stake-accounts) and unstake each stake account individually. With the Coinbase Staking API, the user can simply specify the wallet and the desired unstake amount, and the API will handle the rest. The Coinbase Staking API hides this complexity by automatically creating, merging, and splitting the underlying Solana stake accounts for you. All a user must do is sign and broadcast the transactions and the API will handle the rest.

### Step 1. Create a Stake Operation

### Step 2. Sign and Broadcast

Once the unstake operation has been built, the transaction has been constructed based on your inputs, but not signed or broadcasted. Now, you must relay the transactions to your end-user for signing and broadcasting. If you’d like to sign and broadcast in your own system, refer to the [signing and broadcasting transactions](#signing-and-broadcasting-transactions) section for an example.

### Step 3. Wait

After the transaction is signed and broadcasted, the SOL will be “pending unstaked” for ~2 days. This delay is a direct consequence of the Solana network’s staking mechanics. After the SOL in unstaked, the SOL will be in a claimable state and can be claimed using the [Claim Stake](#claim-stake) operation.

## Claim Stake

Claim Stake is the second part of a two-step process to withdraw your staked assets. The first step is [Unstake](#unstake). After SOL is unstaked and the necessary time has passed (~2 days), the unstaked SOL will be sitting idle on the underlying stake account. This SOL is ready to be claimed. The claim stake operation allows you to claim the unstaked SOL and transfer it back to your wallet.

### Step 1. Create a Stake Operation

Refer to the [ExternalAddress docs](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_external_address.ExternalAddress.html) for a full list of supported methods. Refer to the [ExternalAddress](https://pkg.go.dev/github.com/coinbase/coinbase-sdk-go/pkg/coinbase#NewExternalAddress), [GetStakeableBalance](https://pkg.go.dev/github.com/coinbase/coinbase-sdk-go/pkg/coinbase#Client.GetStakeableBalance) and [BuildStakeOperation](https://pkg.go.dev/github.com/coinbase/coinbase-sdk-go/pkg/coinbase#Client.BuildStakeOperation) functions for more details.

### Step 2. Sign and Broadcast

Once the claim operation has been built, the transaction has been constructed based on your inputs, but not signed or broadcasted. Now, you must relay the transactions to your end-user for signing and broadcasting. If you’d like to sign and broadcast in your own system, refer to the [signing and broadcasting transactions](#signing-and-broadcasting-transactions) section for an example. After the transaction is signed and broadcasted, the SOL will be transferred back to your wallet almost immediately. At this stage, the full SOL staking lifecycle is complete.

## View Staking Rewards

After staking your SOL, rewards will begin to accrue on the blockchain that are withdrawable by your wallet. The staking rewards endpoint allows you to view these rewards earned by your wallet over time.

The API provides rewards both in native units (i.e. SOL) and in equivalent USD value. The USD value is calculated using the Coinbase exchange rate in the ~30 seconds after the reward period concluded. As an example, if we provide aggregated rewards on January 20th, 2024 UTC, the underlying SOL value is calculated based on the USD value of SOL within the first 30 seconds of January 21st, 2024. Look up staking rewards for a specific address.

Refer to the [StakingReward docs](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_staking_reward.StakingReward.html) for a full list of supported methods and their parameters. Refer to the [ListStakingRewards](https://pkg.go.dev/github.com/coinbase/coinbase-sdk-go/pkg/coinbase#Client.ListStakingRewards) function for more details.

## View Historical Staking Balances

The staking balances endpoint allows you to view the historical staking balances of your wallet over time, accounting for accruing rewards and auto-compounding stake. Look up historical staking balances for a specific address.

Refer to the [StakingBalance docs](https://coinbase.github.io/coinbase-sdk-nodejs/interfaces/client_api.StakingBalance.html) for a full list of supported methods and their parameters. Refer to the [ListHistoricalStakingBalances documentation](https://pkg.go.dev/github.com/coinbase/coinbase-sdk-go/pkg/coinbase#Client.ListHistoricalStakingBalances) function for more details.

## Signing and Broadcasting Transactions

Here’s an example of how to sign and broadcast transactions surfaced via the staking operation resource.