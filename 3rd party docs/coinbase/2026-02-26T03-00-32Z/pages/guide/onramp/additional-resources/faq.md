# faq

### As a developer, how do I onboard to Coinbase Onramp?

You can onboard to the Coinbase Onramp product through the [Coinbase Developer Platform](https://www.coinbase.com/developer-platform). Go to [Getting Started](https://developer.chrome.com/onramp/introduction/getting-started) for details on how to create an account.

### Do users need to have a Coinbase Account in order to use Coinbase Onramp?

No. In the US, UK, and Canada, non-Coinbase account holders can also onramp without a Coinbase account using [Guest checkout](https://developer.chrome.com/onramp/coinbase-hosted-onramp/overview#from-a-debit-card-guest-checkout). Coinbase has more than [103M+ verified users](https://investor.coinbase.com/) and these users can seamlessly sign-in to their existing Coinbase Account and start using Coinbase Onramp. If users do not have a Coinbase account, they can create one or use Coinbase Onramp widget.

### Which countries are supported by Coinbase Onramp?

Coinbase Onramp is available in all countries which Coinbase operates except Japan. To get a current list, call the [Config endpoint](https://developer.chrome.com/onramp/coinbase-hosted-onramp/countries-&-currencies#countries-%26-payment-methods).

### I’m submitting my App for review to the Apple App Store and it uses Coinbase Onramp, can you help?

Yes! Please drop us a message in the **#onramp** channel of the [CDP Discord](https://discord.com/invite/cdp).

### Which payment methods are supported on Coinbase Onramp?

See [Payment Methods](https://developer.chrome.com/onramp/additional-resources/payment-methods) for a summary and use the [Config endpoint](https://developer.chrome.com/onramp/coinbase-hosted-onramp/countries-&-currencies#countries-%26-payment-methods) to get the latest.

Payment method/funding source

Currently supported countries

Crypto balance in Coinbase account

All countries in which Coinbase operates except Japan

Debit cards

US and 90+ additional countries (including EU, UK,CA)

Credit cards

90 countries (including EU, UK, CA, and excluding US)

ACH (US Bank Transfer)

US

Yes, you can configure it to open in either form. The default functionality opens the Coinbase Onramp widget in a popup when the user is logged in and logged out. To open it in a new tab you can pass in the `openIn` and `target` parameters to the `<FundButton />` component. [see here](https://onchainkit.xyz/fund/fund-button#customizing-the-funding-experience)

No, the Coinbase Onramp widget cannot be embedded in an iframe. It must be opened in either a popup or a new tab.

### How should I open the Coinbase Onramp URL in my mobile app?

Coinbase Onramp Widget URLs will not function correctly in a WebView (using `WebView` or `WKWebView`) as by default they do not support authentication via Passkeys/U2F (`WebAuthn`). We recommend using [`Chrome Custom Tabs`](https://developer.chrome.com/docs/android/custom-tabs) on Android and [`SFSafariViewController`](https://developer.apple.com/documentation/safariservices/sfsafariviewcontroller) or [`ASWebAuthenticationSession`](https://developer.apple.com/documentation/AuthenticationServices/ASWebAuthenticationSession) for iOS. For React Native mobile apps, we recommend the [`react-native-inappbrowser-reborn`](https://www.npmjs.com/package/react-native-inappbrowser-reborn) library. Reference our [Onramp Demo Mobile App](https://github.com/coinbase/onramp-v2-mobile-demo) for questions. We encourage PRs and new issues/discussions!

### Which blockchains and cryptocurrencies does Coinbase Onramp support?

Coinbase Onramp supports all assets and networks available for trade/send/receive on Coinbase.com. You can use the [Onramp Options API](https://developer.chrome.com/onramp/coinbase-hosted-onramp/countries-&-currencies) or the [Onramp Asset Availability Checker Tool](https://onramp-asset-availability.vercel.app/) to get the full list. See [Layer 2 Networks](https://developer.chrome.com/onramp/additional-resources/layer-2-networks) for details.

### What fees do you charge?

Coinbase Onramp is free for developers to use. When end-users onramp funds, Coinbase Onramp charges the following fees:

-   **Spread**: Coinbase Onramp includes a spread in the price when you buy cryptocurrencies. This allows us to temporarily lock in a price for trade execution while you review the transaction details prior to submitting your transaction.
-   **Coinbase fees**: Fees are calculated at the time you place your order and may be determined by a combination of factors, including the selected payment method, the size of the order, and market conditions such as volatility and liquidity. Fees are listed in the order preview screen before you submit your transaction and may differ for similar transactions.

-   **Card and ACH:** There is a 2.5% fee for credit card transactions, and 0.5% fee for ACH
-   **Network fees:** For transactions on cryptocurrency networks (i.e., transfers of cryptocurrency off the Coinbase platform), Coinbase incurs and pays network transaction fees (e.g., miner’s fees). We charge a fee based on our estimate of these network fees. In some cases, the final fee that Coinbase pays may differ from the estimated fee. All fees we charge are disclosed at the time of transaction.

### Can we customize the email sent to the users?

No. Users that make a purchase via Coinbase Onramp receive the standard emails that they receive when purchasing and sending crypto from their Coinbase.com account to a self-custody wallet.

### Can I test my Onramp integration by creating mock buys and sends?

Yes you can. When launching the Onramp experience, at the bottom of the Onramp modal where it says “Secured by Coinbase”, click **10 times** on the word “Secured.” This will navigate you to Onramp’s Debug Menu. There, navigate to the **Actions** tab click on the toggle for “Enable Mocked Buy and Send.” You can now exit the Debug Menu and proceed with mock buys and sends. Remember to click on the “Enable Mocked Buy and Send” toggle once you are done to enable actual buys and sends.

### What is the minimum transaction amount for Apple Pay via Coinbase Onramp?

For [Apple Pay](https://www.coinbase.com/blog/Fiat-to-crypto-in-seconds-with-Apple-Pay) transactions, the minimum amount is approximately $5 USD.

### How do I enable access to zero fee fiat to USDC in my product?

To access zero fee USDC on-and off- ramps, please [apply here](https://www.coinbase.com/developer-platform/developer-interest) to request access. We’ll be in touch shortly. If you need support, please contact us on Discord.

### What are the rate limits for the Buy Quote API?

The **Buy Quote API** is rate limited to prevent abuse that would degrade our ability to maintain consistent API performance for all users. Throttling is enforced per endpoint by app ID at **10 requests per second**. If your requests are being rate limited, HTTP response code 429 is returned with a `rate_limit_exceeded` error. Rate limiting is implemented using a sliding window algorithm.

## Offramp FAQ

### What is the recommended polling strategy for Offramp transaction status?

We recommend to avoid polling immediately after generating the URL; instead, wait until the send transaction is created. Additionally, using exponential backoff to manage polling frequency—starting with short intervals and increasing over time.

### Can users complete ACH withdrawals through the Offramp flow using guest checkout, or is a Coinbase account required?

A Coinbase account with linked bank details is required for Offramp and ACH withdrawals. Guest checkout is not supported for fiat withdrawal.

### What happens if a user sends crypto to the Coinbase-provided address after the 30-minute window in the Offramp flow?

If funds are sent after the 30-minute window, they will still arrive in the users Coinbase account as a crypto balance, but the Offramp transaction will likely move to a TRANSACTION\_STATUS\_FAILED state. The address remains valid and usable by the user, but the original sell flow will not complete automatically.

**See Also:**

-   [Supported Payment Methods](https://developer.chrome.com/onramp/additional-resources/payment-methods)
-   [Layer 2 Networks](https://developer.chrome.com/onramp/additional-resources/layer-2-networks)