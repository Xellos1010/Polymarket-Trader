# reference apps

Coinbase provides reference applications to help you integrate with Coinbase Prime. These applications demonstrate key integration patterns and technical concepts. **IMPORTANT:** These are sample applications for demonstration purposes only. Test thoroughly and confirm they meet your requirements before using them in any production environment.

## Trading & Order Management

-   [**Prime FIX Protocol Client**](https://github.com/coinbase-samples/prime-fix-go) - Connect to Coinbase Prime via FIX for order entry and trading workflows
-   [**Trading Fees**](https://github.com/coinbase-samples/prime-trading-fees-go) - Implement custom fees on top of Coinbase Prime trading surfaces
-   [**Liquidator**](https://github.com/coinbase-samples/prime-liquidator-go) - Monitor trading wallets and automatically convert assets to fiat
-   [**Trader Shell**](https://github.com/coinbase-samples/trader-shell-go) - Simple order and execution management system for Coinbase Prime

## Market Data

-   [**FIX Market Data Client**](https://github.com/coinbase-samples/prime-fix-md-go) - Receive real-time and snapshot market data via FIX
-   [**WebSocket Best Bid/Ask**](https://github.com/coinbase-samples/prime-best-bid-ask-java) - Maintain top-of-book prices from the Prime L2 Data WebSocket

## Deposits & Withdrawals

-   [**Send & Receive Subledger**](https://github.com/coinbase-samples/prime-send-receive-go) - Manage a deposit and withdrawal subledger for end-user send/receive
-   [**Deposit Listener**](https://github.com/coinbase-samples/prime-deposit-listener-go) - Poll for transaction deposit events and publish to Amazon SNS

## Staking

-   [**Ethereum Validator Timing Scripts**](https://github.com/coinbase-samples/eth-queue-timing-scripts-py) - Estimate Ethereum validator activation and withdrawal timing

## Activity Monitoring

-   [**Activity Listener**](https://github.com/coinbase-samples/prime-activity-listener-go) - Poll for portfolio activities and broadcast to external services