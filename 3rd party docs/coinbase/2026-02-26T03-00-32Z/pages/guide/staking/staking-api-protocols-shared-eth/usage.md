# usage

Shared ETH Staking enables users to stake with any amount of ETH. See the [quickstart](https://developer.chrome.com/staking/staking-api/introduction/quickstart) to familiarize yourself with Coinbase Staking API and basic usage.

* * *

## External Address

The [external address model](https://developer.chrome.com/server-wallets/v1/concepts/addresses#external-addresses) is an address model where the private keys are **not** managed by the Coinbase SDK. The developer would be responsible for “bringing their own wallet”. All signing operations must be completed off-platform.

### Stake

To stake, **ensure that the external address contains enough ETH on the network you are using** to cover the stake amount and network transaction fees. To fund your (Hoodi) testnet address with ETH, we provide a [faucet](https://developer.chrome.com/faucets/introduction/welcome.mdx) method.

Once the stake operation has been built, relay the transactions to your end-user for signing and broadcasting. Refer to the [Signing and Broadcasting Transactions](#signing-and-broadcasting-transactions) section for an Ethers.js example. Refer to the [ExternalAddress documentation](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_external_address.ExternalAddress.html) for a full list of supported methods.

### Unstake

Unstake is the first part of a two-step process to withdraw your staked assets. This step involves submitting an exit request to the network. **Processing time varies** based on the unstake request volume on the shared ETH staking pool. In periods of high demand, wait times follow the native Ethereum validator exit queue — check [validatorqueue.com (see Exit Queue Wait)](https://validatorqueue.com/) for current estimates.

Once the unstake operation has been built, relay the transactions to your end-user for signing and broadcasting. Refer to the [Signing and Broadcasting Transactions](#signing-and-broadcasting-transactions) section for an Ethers.js example. Refer to the [ExternalAddress documentation](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_external_address.ExternalAddress.html) for a full list of supported methods.

### Claim Stake

Once your exit request has been processed by the network you may proceed with the Claim Stake method below.

Once the claim stake operation has been built, relay the transactions to your end-user for signing and broadcasting. Refer to the [Signing and Broadcasting Transactions](#signing-and-broadcasting-transactions) section for an Ethers.js example. Refer to the [ExternalAddress documentation](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_external_address.ExternalAddress.html) for a full list of supported methods.

### View Staking Rewards

After staking your ETH, reward will begin to accrue on your address. These rewards can be listed via the `stakingRewards` call. Refer to the [StakingRewards documentation](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_external_address.ExternalAddress.html#stakingRewards) for an explanation of the method’s parameters. Look up staking rewards for a specific address.

* * *

## Wallet Address

The [wallet address model](https://developer.chrome.com/server-wallets/v1/concepts/addresses#wallet-addresses) is an address model where the private keys are managed by the Coinbase SDK. This means that the SDK can sign transactions on behalf of the user. In production, it’s recommend to use a server-signer for increased security.

### Stake

Shared ETH Staking enables users to stake with any amount of ETH. To stake, **ensure that the wallet address contains enough ETH on the network you are using** to cover the stake amount and network transaction fees. To fund your (Hoodi) testnet address with ETH, we provide a [faucet](https://developer.chrome.com/faucets/introduction/welcome) method. Refer to the [WalletAddress documentation](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_wallet_address.WalletAddress.html) for a full list of supported methods.

```
import { Coinbase, Wallet, StakeOptionsMode } from "@coinbase/coinbase-sdk";
// Create a new wallet address on the ethereum-hoodi testnet network.
let wallet = await Wallet.create({ networkId: Coinbase.networks.EthereumHoodi });
// Find out how much ETH is available to stake.
let stakeableBalance = await wallet.stakeableBalance(Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);
// Create a stake operation for an amount <= stakeableBalance, in this case 0.0001 ETH.
let stakeOperation = await wallet.createStake(0.0001, Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);

```

Once the stake operation has completed successfully, congrats you’ve just staked some ETH!

### Unstake

Unstake is the first part of a two-step process to withdraw your staked assets. This step involves submitting an exit request to the network. **Processing time varies** based on the unstake request volume on the shared ETH staking pool. In periods of high demand, wait times follow the native Ethereum validator exit queue — check [validatorqueue.com (see Exit Queue Wait)](https://validatorqueue.com/) for current estimates. Refer to the [WalletAddress documentation](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_wallet_address.WalletAddress.html) for a full list of supported methods.

```
import { Coinbase, Wallet, StakeOptionsMode } from "@coinbase/coinbase-sdk";
// Create a new wallet address on the ethereum-hoodi testnet network.
let wallet = await Wallet.create({ networkId: Coinbase.networks.EthereumHoodi });
// Since the time you first staked, it is possible that the amount of staked ETH has increased.
// To determine the amount of ETH available to unstake, use the `unstakeableBalance` method as shown below.
let unstakeableBalance = await wallet.unstakeableBalance(Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);
// Create an unstake operation for an amount <= unstakeableBalance, in this case 0.0001 ETH.
let unstakeOperation =  await wallet.createUnstake(0.0001, Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);

```

Once the unstake operation has been created, it could take around 1-3 days to eventually claim your stake. Check below how to claim stake.

### Claim Stake

Once your exit request has been processed by the network, the stake can be withdrawn from the address.

```
import { Coinbase, Wallet, StakeOptionsMode } from "@coinbase/coinbase-sdk";
// Create a new wallet address on the ethereum-hoodi testnet network.
let wallet = await Wallet.create({ networkId: Coinbase.networks.EthereumHoodi });
// Check if there is any staked balance available to claim.
let claimableBalance = await wallet.claimableBalance(Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);
// Build a claim_stake operation for an amount = claimableBalance.
// The claim stake operation aims to claim all the exitable ETH available at that point in time,
// which may have been requested from multiple previous unstake attempts.
let claimStakeOperation = await wallet.createClaimStake(claimableBalance, Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);

```

Once the claim stake operation has been created, you have fully claimed all your stake! Refer to the [WalletAddress documentation](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_wallet_address.WalletAddress.html) for a full list of supported methods.

### View Staking Rewards

After staking your ETH, reward will begin to accrue on your address. These rewards can be listed via the `stakingRewards` call. Look up staking rewards for a wallet.

```
import { Coinbase, Wallet } from "@coinbase/coinbase-sdk";
// Create a new wallet address on the ethereum-hoodi testnet network.
let wallet = await Wallet.create({ networkId: Coinbase.networks.EthereumHoodi });
// Get the rewards earned from staking in the last 1 week (default window).
// Note that it can take upto a day for new rewards to show up.
let rewards = await wallet.stakingRewards(Coinbase.assets.Eth);
// Loop through the rewards and print each staking reward.
rewards.forEach(reward => console.log(reward.toString()));

```

Look up staking rewards for a list of addresses.

```
import { Coinbase, StakingReward } from "@coinbase/coinbase-sdk";
let rewards = await StakingReward.list(
    Coinbase.networks.EthereumMainnet, Coinbase.assets.Eth,
    ["ADDRESS1", "ADDRESS2"],
    tenDaysAgo.toISOString(), now.toISOString(),
);
// Loop through the rewards and print each staking reward.
rewards.forEach(reward => console.log(reward.toString()));

```

View the USD value of rewards including conversion price and time.

```
// Loop through the rewards and print each staking reward's USD conversion information
rewards.forEach(reward => {
    console.log(
        `USD value: ${reward.usdValue()},
        Conversion price: ${reward.conversionPrice().toString()},
        Conversion time: ${reward.conversionTime().toISOString()}`,
    );
});

```

Refer to the [StakingRewards documentation](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_address_external_address.ExternalAddress.html#stakingRewards) for an explanation of the method’s parameters.

* * *

The functionality below applies to both External and wallet addresses.

## Signing and Broadcasting Transactions

Here’s an example of how to sign and broadcast transactions surfaced via the staking operation resource.