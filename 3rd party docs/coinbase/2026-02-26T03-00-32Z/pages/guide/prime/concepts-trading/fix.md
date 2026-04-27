# fix

Financial Information eXchange (FIX) is the de‑facto protocol used by institutions for electronic trading. Coinbase Prime offers a FIX 4.2 gateway for order entry, status, and cancellations, so you can plug existing trading infrastructure straight into the crypto market.

## How FIX fits into the Prime API stack

Layer

What it does

Typical use

FIX

Order entry, cancel, drop‑copy

Systematic trading, HFT, post‑trade reconciliation

Websocket (`l2_data`, `orders`)

Streaming order book + order updates

UI embed, order oversight

REST

Trading, transfers, reporting

Wallet creation, reconciliation, trading, withdrawals

Most automated desks run two FIX sessions:

1.  Order‑entry session – only returns Execution Reports for orders submitted on that session (DropCopyFlag `9406=N`).
2.  Drop‑copy session – streams *all* portfolio fills regardless of source (DropCopyFlag `9406=Y`).

* * *

## Best‑practice playbook

### Order & fill management

-   Persist to disk (or DB) so a restart doesn’t lose state
-   On startup, call REST Open Orders and reconcile with your cache

### Resiliency & recovery

1.  Re‑establish FIX; historical Execution Reports will **automatically replay** from Coinbase for any messages missed while disconnected.
2.  Use FIX message type `H` or the REST Get Order by ID endpoint to retrieve specific order info.
3.  **Daily release window**: Between **5:00–5:05 PM ET**, sessions may forcibly disconnect.

-   Avoid manual disconnects or resets 2–3 minutes before 5 PM. GTD orders may cancel at expiry, and you could miss final reports
-   If disconnected during this window, reconnect **after 5:05 PM ET** and confirm all state via FIX `H` or REST
-   **Note:** Execution Reports are retained during downtime and will be sent upon reconnection

## Funds Available for Trading

### Portfolio Margin enabled:

Use the `Buying Power` and other [Portfolio Margin APIs](https://developer.chrome.com/api-reference/prime-api/rest-api/financing/get-margin-information) to determine trading availability.

### Financing **not** enabled:

Use the REST [balances](https://developer.chrome.com/api-reference/prime-api/rest-api/balances/get-wallet-balance) endpoint with:

```
GET /balances?balance_type=TRADING_BALANCES

```

### Real-time Monitoring

Use any or all of the following for fill + order state monitoring:

-   FIX Execution Reports (entry or drop-copy session)
-   WebSocket orders channel (shows orders across all sources: FIX, REST, UI)

To scope FIX messages:

-   `DropCopyFlag=9406=N` → Only orders created via current FIX session
-   `DropCopyFlag=9406=Y` → All orders and fills across the portfolio

Multiple connections are suggested for redundancy. Your code should gracefully handle disconnects and reconnects.

## Go Sample App

To test FIX with a Prime API key, clone the [Prime FIX Go](https://github.com/coinbase-samples/prime-fix-go) sample app. For full setup details, review the README. The Go client exposes one REPL command per flow (`new`, `status`, `cancel`). Execution Reports are flushed to `orders.json` for later reference.

## See also

-   [Reference Go client](https://github.com/coinbase-samples/prime-fix-go)
-   [Prime FIX Connectivity](https://developer.chrome.com/prime/fix-api/connectivity)
-   [Prime FIX Order Entry Messages](https://developer.chrome.com/prime/fix-api/order-entry-messages)
-   [Prime FIX Administrative Messages](https://developer.chrome.com/prime/fix-api/admin-messages)
-   [FIX 4.2 spec](https://www.fixtrading.org/standards/)