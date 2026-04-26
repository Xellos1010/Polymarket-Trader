# usdc rewards

USDC held on Coinbase Prime generates rewards. These rewards accumulate daily and are automatically paid out monthly on the 7th of each calendar month at midnight UTC. Rewards provide an additional benefit for holding stablecoins within your Prime portfolio.

## How USDC Rewards Work

Key details about USDC rewards:

-   Rewards accumulate on a daily basis
-   Paid out monthly on the 7th of the calendar month at midnight UTC
-   Calculated based on total USDC balance held across all eligible balances, which may include Prime Onchain Wallet, Prime Vault, and Prime Trading Balance
-   Eligibility depends on legal entity structure and jurisdiction

## Tracking Rewards

Individual rewards are paid as a single lump payment each month into a given portfolio’s USDC Trading Balance. This can be seen from the [List Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions) and [List Wallet Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-wallet-transactions) endpoints, with the transaction type of `REWARD`. An example of this response is provided below:

```
{
         "id": "uuid",
         "wallet_id": "uuid",
         "portfolio_id": "uuid",
         "type": "REWARD",
         "status": "TRANSACTION_IMPORTED",
         "symbol": "USDC",
         "created_at": "YYYY-MM-DDThh:mm:ss.000Z",
         "completed_at": "YYYY-MM-DDThh:mm:ss.000Z",
         "amount": "1.531158",
         "transfer_from": {
            "type": "OTHER",
            "value": "Coinbase",
            "address": "",
            "account_identifier": ""
         },
         "transfer_to": {
            "type": "WALLET",
            "value": "UUID",
            "address": "",
            "account_identifier": ""
         },
         "network_fees": "0",
         "fees": "0",
         "fee_symbol": "USDC",
         "blockchain_ids": [],
         "transaction_id": "str",
         "destination_symbol": "",
         "estimated_network_fees": null,
         "network": "",
         "estimated_asset_changes": [],
         "metadata": null,
         "idempotency_key": "UUID",
         "onchain_details": null,
         "network_info": null
      }

```