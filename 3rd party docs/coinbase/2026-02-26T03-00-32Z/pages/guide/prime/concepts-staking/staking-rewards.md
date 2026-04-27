# staking rewards

Staking-related transaction events can be tracked using either the [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions) or [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions) endpoints. Both endpoints return identical response structures; the wallet-level endpoint simply scopes the results to a single wallet. These APIs can be used to monitor staking lifecycle events (e.g., initiation, exit) and to track reward distribution over time. For all non-ETH assets, rewards are deposited directly into the wallet from which the asset was staked.

## Tracking Rewards

To track all reward distributions over time, use the [List Portfolio Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions) or [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions) endpoints and filter for `REWARD` transaction type. Below is an example of querying for staking rewards at the portfolio level.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
TransactionsService transactionsService = PrimeServiceFactory.createTransactionsService(client);
ListPortfolioTransactionsRequest request = new ListPortfolioTransactionsRequest.Builder()
.portfolioId("PORTFOLIO_ID_HERE")
.types(new TransactionType[] { TransactionType.REWARDS })
.build();
ListPortfolioTransactionsResponse response = transactionsService.listPortfolioTransactions(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var transactionsService = new TransactionsService(client);
var request = new ListPortfolioTransactionsRequestBuilder.
  WithPortfolioId("PORTFOLIO_ID_HERE").
  WithTypes([TransactionType.REWARD])
  .Build();
var response = transactionsService.ListPortfolioTransactions(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```

transactionsService := transactions.NewTransactionsService(client)
request := &transactions.ListPortfolioTransactionsRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    Types: ["REWARD"]
}
response, err := transactionsService.ListPortfolioTransactions(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
transactions_service = TransactionsService(client)
request = ListPortfolioTransactionsRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    types=["REWARD]",
)
response = transactions_service.list_portfolio_transactions(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const transactionsService = new TransactionsService(client);
transactionsService.listPortfolioTransactions({
  portfolioId: 'PORTFOLIO_ID_HERE'
  types: [TransactionType.Reward],
}).then(async (response) => {
  console.log('Transactions: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl list-portfolio-transactions --help

```

## Rewards by Asset

### Solana (SOL)

**Balance Types**

Balance Type

Calculation

Conditions

`bondable_amount`

Spendable - pending delegation

Returns 0 if already bonded since the wallet is already staking. Excludes a small amount to ensure that the account is in good standing.

`bonded_amount`

staked + pending delegation

Sum of active + pending stakes

`unbonding_amount`

pending undelegation

SOL has a cooldown after the unstake tx is processed

`withdrawable_amount`

total - staking

Withdrawals are subject to Solana’s rent-exempt minimum; approximately 0.0023 SOL must remain in the account.

**Rewards**

-   **MEV Rewards** are distributed as liquid SOL and increase the Withdrawable balance. The Withdrawable balance only appears once the total exceeds 0.05 SOL.
-   **Inflationary Rewards** are automatically added to stake. Once distributed, they increase the Bonded balance.
-   **Block Reward Sweeps** are a special category of rewards, only distributed to clients running dedicated validators.
-   Rewards accrue at the epoch boundary; timing of availability is dependent on Solana’s epoch schedule.

### Ethereum (ETH)

**Balance Types**

Balance Type

Calculation

Conditions

`bondable_amount`

Withdrawable balance

Returns 0 for Staking Transaction Rewards wallet. Requires minimum 32 ETH

`bonded_amount`

Pulled from validator status directly

Requires at least one validator active

`unbonding_amount`

Pulled from validator status directly

Represents funds awaiting exit but not yet withdrawable; exits may take multiple days depending on validator queue

`unbondable_amount`

Pulled from validator status directly

Validator status should be active for 256 epochs

**Rewards** ETH staking rewards are earned in the form of Consensus Layer and Execution Layer rewards.

-   **Consensus Layer Rewards** autocompound up to a maximum effective balance of 2,048 ETH per validator. Once this threshold is reached, additional rewards are swept directly to the vault wallet that was staked.
-   **Execution Layer Rewards** (MEV and Transaction Rewards) are paid out to the Staking Transaction Rewards wallet as liquid ETH and are not automatically staked.

For details on claiming rewards and tracking unstaking progress, see [Staking Lifecycle](https://developer.chrome.com/prime/concepts/staking/staking-lifecycle).

### Cardano (ADA)

**Balance Types**

Balance Type

Calculation

Conditions

`bondable_amount`

Available balance for staking - 2

Returns 0 if already bonded since wallet is already staking. Requires 2 ADA bond deposit. Cannot stake if already bonded

`bonded_amount`

Staked balance

`pending_rewards_amount`

Uses stake address metadata

Rewards available for claim

`withdrawable_amount`

Wallet balance - 1 if staking, 0 if not

**Rewards**

-   Rewards are not distributed automatically. Instead, they accrue in the Pending Rewards balance until they are manually claimed.
-   Any rewards that are earned but not yet distributed are reflected in the Pending Rewards field.

### Polkadot (DOT)

**Balance Types**

Balance Type

Calculation

Conditions

`bondable_amount`

Spendable + bonded balances

Excludes a small amount to ensure that the account is in good standing. Returns 0 if negative.

`bonded_amount`

Direct mapping from chain state

`unbonding_amount`

Direct mapping from chain state

Takes 28 days

`withdrawable_amount`

Total balance - max lock; subtracts existential deposit

`reserved_amount`

Direct from chain state

Cannot be spent or staked

`unvested_amount`

Direct from chain state

Can be staked but not withdrawn. These funds typically “unlock” on a rolling basis, with an amount that varies by account

**Rewards**

-   Rewards are automatically distributed to the client address.
-   Once distributed, they are added directly to the Bonded balance since rewards autocompound.

### Sui (SUI)

**Balance Types**

Balance Type

Calculation

Conditions

`bondable_amount`

Total balance if conditions met

Requires minimum 1 SUI. Returns 0 if already bonded since wallet is already staking. Full staking only.

`bonded_amount`

Direct mapping from chain state

Currently delegated/staked amount

`pending_rewards_amount`

Direct mapping from chain state

Rewards available for claim

**Rewards**

-   Rewards accrue in the Pending Rewards balance and must be unstaked to be claimed.
-   Pending rewards are deposited approximately 24 hours after unstaking.