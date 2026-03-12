# faq

### Can I still use the Charge API?

We recommend migrating to the Payment Link API to take advantage of improved reliability and new features.

### What cryptocurrencies are supported?

Payment Link API supports USDC across multiple chains: Ethereum, Base, Polygon, Optimism, and Arbitrum. The payer pays $0 in gas costs across all chains. The Commerce Charge API supported multiple cryptocurrencies through automatic conversion. Payment Link focuses on stablecoins for predictable settlement.

### How do I handle multiple payment options?

Create multiple payment links for different amounts or purposes. Each payment link has a unique URL and ID.

### Can I update a payment link after creation?

Payment links cannot be updated after creation. To change details, deactivate the existing link and create a new one.

### What happens to existing Charge webhooks?

Existing Commerce webhooks will continue to work for Charge API. For Payment Link API, webhook support is available. See the [Webhooks documentation](https://developer.chrome.com/coinbase-business/payment-link-apis/webhooks) to set up real-time payment status notifications.

### How do I monitor payment status?

You can monitor payment status using webhooks for real-time notifications. See the [Webhooks documentation](https://developer.chrome.com/coinbase-business/payment-link-apis/webhooks) for setup instructions. Alternatively, you can periodically poll the GET endpoint to check the payment link status.

### Can customers pay with different cryptocurrencies?

Payment Link API supports USDC on Ethereum, Base, Polygon, Optimism, and Arbitrum. Customers can pay from any of these supported chains with $0 gas costs.

## Support and resources