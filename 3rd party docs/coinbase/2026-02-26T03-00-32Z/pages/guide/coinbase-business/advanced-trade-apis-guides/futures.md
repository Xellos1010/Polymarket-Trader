# futures

Advanced Trade API supports trading for [Coinbase Financial Markets](https://www.coinbase.com/futures) (CFM) futures products at the following endpoints:

-   [Order Management](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/orders/get-order)
-   [Market Data](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/products/get-best-bid-ask)
-   [Futures-specific](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/get-futures-balance-summary)

## Futures Disclosures

Trading in futures involves substantial risks. You should only trade in financial products that you are familiar with and whose risks you understand. Always carefully consider whether such trading is suitable in light of your investment experience, financial position, and investment objectives. Futures products and services on Coinbase Advanced are offered by Coinbase Financial Markets, a member of the [National Futures Association](https://www.nfa.futures.org/) (NFA), and is subject to NFA’s regulatory oversight and examinations. Be aware that NFA does *not* have regulatory oversight authority over underlying or spot virtual currency products, transactions, virtual currency exchanges, custodians, or markets.

## Leverage

Leverage in futures trading can work for you or against you. The risk of loss using leverage can exceed your initial investment amount.

Futures trading through Coinbase Financial Markets is currently available under [Advanced Trade](https://www.coinbase.com/advanced-trade) and has the same [fee structure](https://www.coinbase.com/advanced-fees). During the introductory beta period, we are only charging 0.05% (the lowest Advanced Trade tier).

## Futures vs Spot Accounts

Futures and spot balances are held in different accounts. Funds used to margin your futures positions are held in your futures account with Coinbase Financial Markets Inc. (CFM). Funds in your spot account are maintained with Coinbase Inc. (CBI). CFM and CBI are separate legal entities. Funds in your futures account are held at CFM, a futures commission merchant regulated by the CFTC, and are afforded certain important customer fund protections. Funds in your spot account are held at CBI, which is not subject to CFTC or NFA oversight and those funds do not benefit from the same CFTC customer protections as funds in your futures account.

## Treatment of Cash

Cash is always deposited into your Coinbase Inc. (CBI) spot account. You can only acquire spot assets with funds in your spot account. Cash is automatically transferred to your Coinbase Financial Markets (CFM) futures account to satisfy margin requirements. Automatic transfers are only *from* CBI spot accounts *to* CFM futures accounts. You can transfer cash that isn’t being used to margin or maintain futures positions into your CBI spot account (to trade spot assets or to withdraw) with [Schedule Futures Sweep](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/schedule-futures-sweep). See the [full terms](https://help.coinbase.com/en/coinbase/trading-and-funding/derivatives/futures-transfer-cash).

## Intraday vs. Overnight Margin Health

If you are [opted in](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/set-intraday-margin-settings) to receive increased leverage on futures trades during the intraday window (from 8am-4pm ET), this endpoint will return your intraday and overnight margin health so you can monitor whether you have sufficient funds to avoid auto-liquidation.

## Margin Ratio Calculation

The margin ratio is computed using the following fields from the [Get Futures Balance Summary](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/futures/get-futures-balance-summary) endpoint:

-   `liquidation_threshold`
-   `available_margin`

### Formula

```
margin_ratio = available_margin ÷ liquidation_threshold

```