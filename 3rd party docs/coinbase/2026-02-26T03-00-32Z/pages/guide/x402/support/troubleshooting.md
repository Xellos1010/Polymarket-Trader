# troubleshooting

This page covers common errors when integrating with x402. For general questions, see the [FAQ](https://developer.chrome.com/x402/support/faq).

## Common Errors

### `Unable to estimate gas`

This error happens when creating the payment payload, before the facilitator is called.

### `invalid_request: doesn't match schema (oneOf)`

This error means the **payload structure doesn’t match the `x402Version`** you’ve set.

The server is still returning 402 even though you included the `PAYMENT-SIGNATURE` header.

### Works on testnet, fails on mainnet

Your integration works on Base Sepolia but fails on Base mainnet.

### `No scheme registered`

The x402 client or server doesn’t have a payment scheme registered for the requested network.

## Facilitator API Error Codes

When the facilitator returns an error, check the `invalidReason` (for verify) or `errorReason` (for settle) field.

For the complete list, see the [x402 Facilitator API Reference](https://developer.chrome.com/api-reference/v2/rest-api/x402-facilitator/verify-a-payment).

## Still Need Help?

-   [x402 Discord](https://discord.gg/cdp) - Community support
-   [GitHub Issues](https://github.com/coinbase/x402/issues) - Bug reports and feature requests