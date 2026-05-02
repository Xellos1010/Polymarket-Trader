# pricing

## Overview

Server Wallet features a flexible, usage-based pricing model. This means you only pay for what you use, offering the benefit of no up-front subscription costs or long-term contracts. Developers are charged for the number of `Wallet operations` that are consumed. Wallet operations include wallet creation, transaction signing, transaction broadcast, and policy evaluations. We only charge for write operations; all read operations are completely free. To help you estimate costs, the following table outlines the number of Wallet operations associated with various common actions.

Server Wallet v2 action

Number of Wallet operations

Create EVM/Solana account

1

Create EVM smart account

1

Sign a message / payload

1

Send a transaction (Sign + Broadcast)

2

Policy Evaluation

1

Each wallet operation costs $0.005.

## Free Tier

There is no charge for the first 5,000 wallet operations each month.

## Billing

CDP Server Wallet charges are billed at the beginning of each calendar month for the previous month’s usage. The invoice will be available for download in the Billing tab of the [CDP Portal](https://portal.cdp.coinbase.com/).