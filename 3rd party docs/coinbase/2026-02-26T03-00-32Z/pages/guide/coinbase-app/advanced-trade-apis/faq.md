# faq

### What is Advanced Trade API?

[Advanced Trade REST API](https://developer.chrome.com/coinbase-app/advanced-trade-apis/rest-api) supports trading and order management, plus a [WebSocket protocol](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-overview) for real-time market data.

### What is Advanced Trade on Coinbase?

[Advanced Trade on Coinbase](https://www.coinbase.com/advanced-trade) offers:

-   Real-time order books and live trade history backed by a simple order process.
-   The same markets and liquidity as Coinbase Pro, the same volume-based fee structure.
-   Charts powered by TradingView with EMA, MA, MACD, RSI, Bollinger Brand, and drawing tools.
-   Access to Coinbase staking, Borrow, Wallet, and Coinbase Card using a single platform balance.
-   DeFi Rewards where you can earn up to 5% APY rewards on your USDC, ETH2, DAI, ALGO, ATOM, XTZ.
-   Enhanced security with 2FA, biometrics for mobile, FDIC-insured USD balances of up to $250k, YubiKey for mobile, Coinbase Vault, and address whitelisting.
-   Availability on Coinbase.com and in the Coinbase mobile app.

### What has happened to Coinbase Pro?

Coinbase Pro has been disabled for use and all customers have been migrated as of December 1, 2023. This was accelerated from a prior announcement of Pro deprecation in 2024. Unauthenticated Pro APIs are still available for customers with access, and the Exchange APIs (FIX, REST WebSocket) continue to be available to Exchange (Institutional) clients.

### Can I use my existing Coinbase Pro API keys to access Advanced Trade?

No, you cannot use existing Pro API keys to trade with Advanced Trade.

### How do I migrate from Pro APIs to Advanced Trade API?

If you are a developer, you must regenerate your API key and adjust your code implementation. If you are using a third party bot, you must regenerate your API keys and update them in the trading bot.

### What are the differences between Advanced Trade and Coinbase Exchange?

#### Advanced Trade

-   **Target Users**: Individual traders.
-   **API Access**: Offers REST API for trading and order management, along with WebSocket for real-time data.
-   **Features and Tools**: Includes real-time order books, live trade history, TradingView charts, staking, borrowing, and Coinbase Card integration.
-   **Security**: Enhanced security with 2FA, biometrics, FDIC-insured USD balances up to $250k, YubiKey for mobile, Coinbase Vault, and address whitelisting.
-   **API Keys**: Uses CDP API Keys, which can be obtained from the Coinbase Developer Platform (CDP).
-   **Availability**: Accessible on Coinbase.com and the Coinbase mobile app.

#### Coinbase Exchange

-   **Target Users**: Institutional clients and high-volume traders.
-   **API Access**: Provides FIX, REST, and WebSocket APIs specifically for institutional clients.
-   **Features and Tools**: Focuses on high-volume trading with tools tailored for institutional needs, including advanced order types and market data.
-   **Security**: Institutional-grade security protocols.
-   **API Keys**: Requires a Coinbase Exchange Account to obtain API keys.
-   **Availability**: Designed for high-volume trading and integration with institutional trading systems.

By understanding these differences, you can choose the platform that best suits your trading needs, whether you are an individual trader looking for a comprehensive trading experience with Advanced Trade or an institutional client needing robust tools for high-volume trading with Coinbase Exchange.

### Can I continue to access the FIX API?

Due to low use among retail users, the FIX API is not supported on Coinbase Advanced Trade.

### Why am I receiving an `UNKNOWN_FAILURE_REASON` when placing an order?

When you place an order and receive a response indicating `"success": true` along with `"failure_reason": "UNKNOWN_FAILURE_REASON"`, it means that your order was successfully accepted despite the confusing failure reason message. The `"failure_reason"` field can be ignored in this context as the success status confirms that the order was processed correctly. Here is an example of a successful order placement with an `UNKNOWN_FAILURE_REASON`:

```
{
  "success": true,
  "failure_reason": "UNKNOWN_FAILURE_REASON",
  "order_id": "62475e97-7485-45eb-a5ac-594805fa2748",
  "success_response": {
    "order_id": "62475e97-7485-45eb-a5ac-594805fa2748",
    "product_id": "BTC-USD",
    "side": "BUY",
    "client_order_id": "12345678"
  },
  "order_configuration": {
    "market_market_ioc": {
      "quote_size": "1.0"
    }
  }
}

```

In this example, the `"success":` true confirms that the order was placed successfully. The `"UNKNOWN_FAILURE_REASON"` does not indicate an issue with the order itself and can be disregarded.

### Why am I receiving an `UNKNOWN` value within the `warning` key in a response and what does it mean?

`UNKNOWN` is the default response for the `warning` key within responses and can therefore be seen as **no warning** being returned.

### Why am I receiving `PREVIEW_INVALID_BASE_SIZE_TOO_SMALL` on my perpetual order?

All perpetual orders are subject to a 10 USDC min notional value.

### What is the difference between my Advanced Trade accounts and my Coinbase App accounts?

Advanced Trade accounts and Coinbase App accounts are tied to the same balances, but Advanced Trade include futures, perpetuals, and margin accounts whereas Coinbase App does not.

### Why am I receiving an `Unsupported account in this conversion` when using the Advanced Trade Converts Endpoints?

Currently the converts endpoints only supports USD `<->` USDC, USD `<->` PYUSD, and EUR `<->` EURC conversions