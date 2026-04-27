# overview

Dedicated ETH Staking enables staking natively to reserved ETH validators running in Coinbase’s best-in-class staking infrastructure. This solution requires developers to **stake in 32 ETH increments**. Each validator is reserved for your staking purpose - there is no commingling of funds with other Coinbase Staking customers. This approach ensures **end-users are always in custody of their funds**.

### Who this solution is for

-   Custodians with users who will be staking larger amounts of ETH above the typical 32 ETH deposit minimum.
-   Developers who do not want to see their funds commingled with other customers.
-   Developers looking for higher ETH yields.

### Validator details

-   Hoodi validators
    -   Created in either *Frankfurt* or *Singapore*.
-   Mainnet validators
    -   Created in either *Singapore*, *Tokyo*, *Hong Kong*, *Ireland* or *Frankfurt*.

Dedicate ETH Staking uses a mixture of both Lighthouse and Prysm as consensus clients.  
See our [blog post](https://www.coinbase.com/developer-platform/discover/insights-analysis/execution-client-diversity) for more information on our client diversification strategy.

### Rewards scope

Data Type

Network

Details

Historical Depth

Addresses

Aggregations

Historical Rewards

Mainnet Only

All Reward Types

Jan 1st, 2024

All Addresses (Validator Addresses)

Daily

Historical Staking Balances

Mainnet Only

Jan 1st, 2024

All Addresses (Validator Addresses)

Daily

### Billing

Dedicated ETH offers two billing methods: onchain billing and offchain invoicing.

#### Onchain billing

Onchain billing distributes execution layer rewards and commission automatically between you, your users, and Coinbase, eliminating the need for offchain invoicing.

##### How it works

With onchain billing, you work with the Coinbase team to define a Reward Distribution Plan that specifies percentages of execution layer rewards distributed to you, your users, and Coinbase, as well as the Ethereum address that will receive your portion of execution layer rewards. Once your Plan’s details are finalized, the Coinbase team will provide you with a Plan ID, which you include when calling Coinbase Staking API. Calling the Staking API with your Plan ID and the Ethereum address of your end user will provision an Ethereum validator, and deploy a reward distribution smart contract (“split contract”) as the validator’s fee recipient address. The split contract receives execution layer rewards, while consensus layer rewards flow to your withdrawal address as usual. The split contract will distribute rewards at the agreed-upon percentages to you, your users, and Coinbase. Coinbase will initiate reward distribution on a monthly cadence, but if desired, you can independently do so at any time as shown below.

-   Typescript
    
-   Go
    
-   HTTP
    

```
import { Coinbase, ExternalAddress, StakeOptionsMode } from "@coinbase/coinbase-sdk";
let address = new ExternalAddress(
    Coinbase.networks.EthereumHoodi,
    "WALLET_ADDRESS",
);
let feeRecipient = "END_USER_ADDRESS"
let planId = "PLAN_ID"
let op = await address.buildStakeOperation(
    32,
    Coinbase.assets.Eth,
    StakeOptionsMode.NATIVE,
    {"fee_recipient_address": feeRecipient, "reward_splitter_plan_id": planId},
);
await op.wait();

```

```
address := coinbase.NewExternalAddress(coinbase.EthereumHoodi,"WALLET_ADDRESS")
feeRecipient := "END_USER_ADDRESS"
planID := "PLAN_ID"
opts := []coinbase.StakingOperationOption{
  coinbase.WithStakingOperationMode(coinbase.StakingOperationModeNative),
  // Select 0x02 for Pectra validators or 0x01 for pre-pectra validators.
  coinbase.With0x02WithdrawalCredentialType(),
  coinbase.WithFeeRecipientAddress(feeRecipient),
  coinbase.WithRewardSplitterPlanID(planID),
}
op, err := client.BuildStakeOperation(
    context.Background(),
    big.NewFloat(32),
    coinbase.Eth,
    address,
    opts...,
)
if err != nil {
     log.Fatal(err)
}
stakingOperation, err = client.Wait(
  context.Background(),
  op,
  coinbase.WithWaitTimeoutSeconds(600)
)
if err != nil {
  log.Fatal(err)
}

```

```
export JWT="generated-jwt-token" # Replace with your JWT token
curl -s -L 'https://api.cdp.coinbase.com/platform/v1/stake/build' \
-H 'Content-Type: application/json' \
-H 'Accept: application/json' \
-H "Authorization: Bearer ${JWT}" \
-d '{
    "network_id": "ethereum-hoodi",
    "asset_id": "ETH",
    "address_id": "WALLET_ADDRESS",
    "action": "stake",
    "options": {
      "mode": "native",
      "amount": "32000000000000000000",
      "fee_recipient_address": "END_USER_ADDRESS",
      "reward_splitter_plan_id": "PLAN_ID"
    }
  }'

```

##### Retrieving Split Contract Metadata

You can determine the split contract addresses associated with your validators using the listValidators endpoint. Validators with a non-empty forwarded\_fee\_recipient have been created with a split contract. The contract address is defined in the fee\_recipient field of the validator. You can use the get split metadata function on a given split contract address to determine the recipient addresses and split percentages. Alternatively, you can inspect the contract creation transaction ([example](https://holesky.etherscan.io/tx/0xc0360c58960e6e5184712926b733d4ca542ac8cbfe4be63f2ee7d66752592818#eventlog#305)) to determine the contract configuration.

##### Distributing Rewards

Coinbase will initiate reward distribution from the split contract on a monthly cadence. However, you may independently initiate distribution at any time. Reach out to your Coinbase account team for supplementary instructions.

#### Offchain invoicing

With this option, you receive a monthly invoice for the amount you owe Coinbase in validator commission. You can pay with a fiat payment method, or with credits purchased using crypto. This is the default option for Dedicated ETH staking.