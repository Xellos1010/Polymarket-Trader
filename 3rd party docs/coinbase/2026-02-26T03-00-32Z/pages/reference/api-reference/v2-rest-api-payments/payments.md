# payments

The Payments APIs enable you to create and manage payment transfers. The Payments APIs are currently in alpha and are subject to change. The following payment apis are supported.

### API Reference

-   `GET /v2/payments/rails/payment-methods`: Get fiat payment methods.
-   `GET /v2/payments/rails/crypto`: Get crypto rails.
-   `POST /v2/payments/transfers`: Create a transfer quote.
-   `POST /v2/payments/transfers/{transferId}/execute`: Execute a transfer quote.
-   `GET /v2/payments/transfers/{transferId}`: Get a transfer by transferId.

### Payment Rails

Payment rails are the source or the target of a transfer. We support the following payment rails:

-   Crypto rails: Onchain currency and networks to send and receive crypto (e.g. Currency: USDC, Network: Ethereum, Solana, Bitcoin, Polygon, Avalanche).
-   Payment methods: Payment methods that are previously added in your account (e.g Debit Card).

### Transfer

A Transfer is a money movement between two payment rails. we support the following transfer types:

-   Fiat to Crypto transfer
    -   Source: a payment method (payment method that is already added in your account. e.g. Card)
    -   Target: onchain address to receive crypto

### Create a Transfer

1.  Use `/v2/payments/rails/payment-methods` and `/v2/payments/rails/crypto` to get all available payment rails.
2.  Choose the payment rails to use as source and target, and the amount to transfer.
    -   Use `/v2/payments/transfers` to create a transfer quote.
    -   Optionally, you can set `execute` to true to execute the transfer quote immediately.
3.  Once you have a transfer quote, use `/v2/payments/transfers/{transferId}/execute` to execute a transfer quote.
4.  Use `/v2/payments/transfers/{transferId}` to get a transfer by transferId.