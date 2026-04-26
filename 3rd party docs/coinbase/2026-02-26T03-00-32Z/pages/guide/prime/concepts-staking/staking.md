# staking

Staking enables clients to earn rewards by participating in blockchain consensus. With Prime’s Staking APIs, clients can stake supported assets directly from eligible wallets and initiate unstaking when desired. While many assets are available for staking through the Prime UI, the Stake and Unstake APIs currently support Ethereum (ETH) and Solana (SOL) only, with additional assets to be supported over time. Reward accrual, stake and unstake activity, and associated balance states can all be monitored through existing API endpoints for all stakable assets on Prime. Coinbase Prime supports both partial staking and multiple concurrent staking operations from the same wallet for ETH only. In practice, this means a specific amount can be staked (e.g. 64 ETH) without needing to stake the entire wallet balance, and multiple stakes can be initiated without waiting for earlier ones to complete. Additional stake and unstake operations after an initial 32 ETH are supported in increments of 1 ETH or greater, and out to 6 decimal places. The cumulative maximum that can be staked to a single validator is 1,800 ETH across all stake operations. Post-Pectra, the maximum effective balance per validator is 2,048 ETH. For Solana, partial staking and unstaking are not yet supported. This means that the only valid actions on a SOL vault wallet are to fully stake or fully unstake. Solana rewards may be either added to the staked or unstaked balance, depending on reward type.

## Staking from a Wallet

To stake from a Vault wallet, use [Create Stake](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-stake-or-delegate). Instead of requiring a public address, this endpoint operates using **Wallet IDs**. For a refresher on Vault wallets and Wallet IDs, refer to the [Wallets](https://developer.chrome.com/prime/concepts/wallets/wallets-overview) page. Once the request is successfully submitted, the response will include an `activity_id` and `transaction_id`. As with other sensitive operations in Prime, this request must be approved in the Prime UI or Mobile App before it is processed. Once approved, the asset selected for staking will soon begin the bonding process, assuming the underlying protocol has a bonding period.

-   Go
    
-   Python
    
-   TS/JS
    

```

stakingService := staking.NewStakingService(client)
request := &staking.CreateStakeRequest{
    PortfolioId:   "PORTFOLIO_ID_HERE",
    WalletId:      "WALLET_ID_HERE",
    IdempotencyKey: uuid.New().String(),
    Inputs:         &staking.CreateStakeInputs{
        Amount: '32'
    }
}
response, err := stakingService.CreateStake(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk import Credentials, Client, StakingService
from prime_sdk.services.staking import CreateStakeRequest, StakingInputs
import uuid
def main():
    credentials = Credentials.from_env("PRIME_CREDENTIALS")
    client = Client(credentials)
    staking_service = StakingService(client)
    request = CreateStakeRequest(
        portfolio_id="PORTFOLIO_ID_HERE",
        wallet_id="WALLET_ID_HERE",
        idempotency_key=str(uuid.uuid4()),
        inputs=StakingInputs(
            amount="32",
        ),
    )
    try:
        response = staking_service.create_stake(request)
        print(response)
    except Exception as e:
        print(f"failed to create stake: {e}")
if __name__ == "__main__":
    main()

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```

const stakingService = new StakingService(client);
stakingService.createStake({
    portfolioId: 'PORTFOLIO_ID_HERE',
    walletId: 'WALLET_ID_HERE',
    idempotency_key: crypto.randomUUID(),
    inputs: {
        amount: '32',
    },
}).then(async (response) => {
    console.log('Stake: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Unstaking from a wallet

The process for unstaking is nearly identical to staking. As with staking, partial unstaking for ETH is supported. For example, if a wallet has 320 ETH staked, an unstake request can be submitted for just 5 ETH. Unstaking is not immediate. For Ethereum, after initiating the request and approving it in the Prime UI, ETH enters the Ethereum exit queue and becomes withdrawable only once the network finalizes the exit. During this period, ETH rewards will continue to be generated and received. For Solana, withdrawals typically take 2-4 days. For ETH, use the [Get Unstaking Status](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/get-unstaking-status) endpoint to track when unstaking will complete. The [Preview Unstake](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/preview-unstake) endpoint shows the estimated amount that will be received, including any rewards earned during the unbonding period. See [Staking Lifecycle](https://developer.chrome.com/prime/concepts/staking/staking-lifecycle) for complete details on tracking unstaking progress.

-   Go
    
-   Python
    
-   TS/JS
    

```

stakingService := staking.NewStakingService(client)
request := &staking.CreateUnstakeRequest{
    PortfolioId:   "PORTFOLIO_ID_HERE",
    WalletId:      "WALLET_ID_HERE",
    IdempotencyKey: uuid.New().String(),
    Inputs:         &staking.CreateUnstakeInputs{
        Amount: '1'
    }
}
response, err := stakingService.CreateUnstake(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk import Credentials, Client, StakingService
from prime_sdk.services.staking import CreateStakeRequest, StakingInputs
import uuid
def main():
    credentials = Credentials.from_env("PRIME_CREDENTIALS")
    client = Client(credentials)
    staking_service = StakingService(client)
    request = CreateUnstakeRequest(
        portfolio_id="PORTFOLIO_ID_HERE",
        wallet_id="WALLET_ID_HERE",
        idempotency_key=str(uuid.uuid4()),
        inputs=StakingInputs(
          amount="32",
        ),
    )
    try:
        response = staking_service.create_unstake(request)
        print(response)
    except Exception as e:
        print(f"failed to create unstake: {e}")
if __name__ == "__main__":
    main()

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const stakingService = new StakingService(client);
stakingService.createUnstake({
    portfolioId: 'PORTFOLIO_ID_HERE',
    walletId: 'WALLET_ID_HERE',
    idempotency_key: crypto.randomUUID(),
    inputs: {
        amount: '1',
    },
}).then(async (response) => {
    console.log('Unstake: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Portfolio-Level Staking

In addition to wallet-level operations, assets can be staked and unstaked at the portfolio level. This effectively batches the operation across all eligible stakable wallets in the given portfolio. To stake across a portfolio, use [Create Portfolio Stake](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-to-stake-currency-in-a-portfolio). This will stake the specified currency from all eligible wallets within the portfolio. To unstake across a portfolio, use [Create Portfolio Unstake](https://developer.chrome.com/api-reference/prime-api/rest-api/staking/request-to-unstake-currency-across-a-portfolio). This will initiate unstaking for the specified currency across all staked wallets in the portfolio. Portfolio-level operations follow the same approval workflow as wallet-level operations and must be approved in the Prime UI or Mobile App before processing.

## Balance Tracking for Staked Assets

The [Get Wallet Balance](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/get-wallet-balance) endpoint provides several staking-specific balance subtypes that help monitor the complete lifecycle of staked assets:

-   **`bondable_amount`** - Withdrawable balance that can be staked. Returns 0 for the ETH Staking Transaction Rewards wallet
-   **`bonded_amount`** - How much balance is currently bonded
-   **`unbonding_amount`** - Amount in the process of unstaking (not yet withdrawable)
-   **`unbondable_amount`** - Amount that can be unstaked
-   **`pending_rewards_amount`** - Accrued rewards awaiting distribution

These balance subtypes provide real-time visibility into staking positions and can be used to programmatically manage staking operations or build dashboards. For more information on calling the SDKs for balances, please visit the [Balances](https://developer.chrome.com/prime/concepts/balances) page.