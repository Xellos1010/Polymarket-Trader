# overview

The new [v2 Onramp Order API](https://developer.chrome.com/api-reference/v2/rest-api/onramp/create-an-onramp-order) enables you to build a native feeling onramp experience with Apple Pay where the user never leaves your app. **It’s the fastest onramp experience available anywhere.** Integrating takes only three steps:

## Integration steps

## Requirements

### User verification

In order to provide an API driven native onramp experience, we rely on you, the app developer, to collect and verify the user’s email address and phone number in your request to the [Create Onramp Order API](https://developer.chrome.com/api-reference/v2/rest-api/onramp/create-an-onramp-order). You must verify the user’s ownership of the email address and phone number, this can be done by sending an OTP using a vendor like Twilio or AWS SES. Additionally, the phone number must be re-verified at least every 60 days.

### US only

We currently support the Apple Pay Onramp API for US users with valid US phone numbers. The phone number must be a real cell phone number, not a VoIP phone number.

### Supported platforms

The Apple Pay onramp payment link can be rendered in:

-   **iOS apps**: Via a webview in an iOS app
-   **Web apps**: Via iframe on your website (requires additional setup - see below)

### User gesture required

Apple [requires](https://developer.apple.com/documentation/applepayontheweb/creating-an-apple-pay-session) that an Apple Pay session be created by a user gesture. This means that the user has to physically press the Apple Pay button we render within the webview/iframe. It cannot be programmatically triggered.

### Legal agreements

Your users must accept Coinbase’s [Guest Checkout Terms of Service](https://www.coinbase.com/legal/guest-checkout/us), [User Agreement](https://www.coinbase.com/legal/user_agreement) and [Privacy Policy](https://www.coinbase.com/legal/privacy) prior to using Coinbase Onramp. It is your responsibility to clearly inform users that by proceeding with this payment they are agreeing to these policies.

### Web App Requirements

Rendering the Apple Pay Onramp payment link on your web app in an iframe requires some additional security measures to ensure the safety of your users.

-   Your web app’s domain must be registered on the domain allow list in CDP portal
-   You must pass the domain name to the [create onramp order API](https://developer.chrome.com/api-reference/v2/rest-api/onramp/create-an-onramp-order#body-domain) when creating a payment link
-   You must verify the ownership of your domain by hosting a domain verification file (provided by us)
-   Your domain must not be registered with any other Apple Merchant ID in the Apple Developer Portal
-   You must include the `sandbox="allow-scripts allow-same-origin"` and `referrerpolicy="no-referrer"` attributes on your iframe

To get started with your web app integration, [schedule a call with our team](https://calendar.app.google/BLn6fzaz2aCZGvLu7) who will walk you through the process of verifying your domain. You will also need to consider the different levels of Apple Pay support provided by various browsers. Safari offers native Apple Pay support, but other browsers offer a QR code experience where the user can scan the code and complete payment on their phone.

## Post message events

Payment links returned by the Create Order API are designed to be loaded within a webview so that your app can subscribe to [post message](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage) events emitted by our web component. Events contain an error code and an error message. The message will be localized for the user so it can be displayed directly in your app UI. See the documentation of your webview library for details on how to consume post message events.

Post message event structure

```
{
  eventName: "<EVENT_NAME>",
  data: {
    errorCode: "<ERROR_CODE>",
    errorMessage: "<ERROR_MESSAGE>",
}

```

### Events names

The following events are published by the Apple Pay button payment link.

Emitted when Javascript is initialized and we have started fetching data required to render.

Emitted when the Apple Pay button is successfully rendered and ready for user interaction.

Emitted when an error occurred attempting to initialize the Apple Pay button. See the error message for more details. Some possible error codes are listed below.

Error Code

Description

`ERROR_CODE_INIT`

The payment link is no longer valid, call the Create Onramp Order endpoint to create a new one.

`ERROR_CODE_GUEST_APPLE_PAY_NOT_SUPPORTED`

The user’s browser or device does not support Apple Pay. This error can be safely ignored on web apps as the browser will fall back to rendering an Apple Pay QR code.

`ERROR_CODE_GUEST_APPLE_PAY_NOT_SETUP`

The user has not set up Apple Pay on their device. Prompt the user to setup Apple Pay then try again.

onramp\_api.commit\_success

Emitted after the user presses the Apple Pay button if the transaction was successfully started.

Emitted after the user presses the Apple Pay button if the transaction could not be started. See the error message for more details regarding the payment failure reasons. Some possible error codes are listed below.

Error Code

Description

`ERROR_CODE_GUEST_CARD_SOFT_DECLINED`

The user was declined by the bank. Please contact your bank or try again with a different debit card.

Users attempting to use Apple Cash will also get this error, but we cannot distinguish it from other bank decline cases.

`ERROR_CODE_GUEST_INVALID_CARD`

Invalid card or billing address.

`ERROR_CODE_GUEST_CARD_INSUFFICIENT_BALANCE`

The debit card has an insufficient balance to process the transaction.

`ERROR_CODE_GUEST_CARD_HARD_DECLINED`

The transaction was declined by the issuing bank of the card.

`ERROR_CODE_GUEST_CARD_RISK_DECLINED`

The transaction was flagged by our risk rules and is unable to proceed.

`ERROR_CODE_GUEST_REGION_MISMATCH`

The region the user is located in is not supported.

`ERROR_CODE_GUEST_PERMISSION_DENIED`

The user has been blocked from using onramp.

`ERROR_CODE_GUEST_CARD_PREPAID_DECLINED`

The user tried to pay with a prepaid debit card, which is unsupported.

`ERROR_CODE_GUEST_TRANSACTION_LIMIT`

This transaction would exceed the user’s weekly transaction limit.

`ERROR_CODE_GUEST_TRANSACTION_COUNT`

This transaction would exceed the user’s lifetime transaction count limit (currently 15).

Emitted if the user cancels the Apple Pay popup.

If you keep the webview active in your app after receiving the `onramp_api.commit_success` message, the webview will poll our transaction status API automatically and report success or failure via the following two events.

onramp\_api.polling\_success

Emitted if the transaction completed successfully and funds have been sent to the destination wallet address.

Emitted if there was an error processing the transaction. Some possible error codes are listed below.

Error Code

Description

`ERROR_CODE_GUEST_TRANSACTION_BUY_FAILED`

We were unable to complete the crypto purchase, likely due to a failed risk check. The user’s card will not be charged.

`ERROR_CODE_GUEST_TRANSACTION_SEND_FAILED`

We were unable to send the funds to the user’s destination address, the user’s card will be refunded.

`ERROR_CODE_GUEST_TRANSACTION_TRANSACTION_FAILED`

An internal error has occurred in Coinbase services, the Onramp team will be automatically notified to investigate.

`ERROR_CODE_GUEST_TRANSACTION_AVS_VALIDATION_FAILED`

We were unable to process the transaction due to failure to validate the user’s billing address. Ask the user to verify their billing address with the bank card. The user’s card will not be charged.

## Testing

You can test your integration with the Apple Pay Onramp API by creating sandbox orders. To create a sandbox order, just prefix the `partnerUserRef` parameter in your call to the [Create Onramp Order API](https://developer.chrome.com/api-reference/v2/rest-api/onramp/create-an-onramp-order#body-partner-user-ref) with the string `sandbox-`. Doing so will result in your Apple Pay transaction always succeeding, but your debit card will never be charged. For the `phoneNumber` parameter, you can use any random phone number, as long as it’s in a valid US phone number format (example: +1 international code + US area code + 7 digit number; +12345678901)

### Web app testing

When testing your web app integration, you can append the query parameter `&useApplePaySandbox=true` onto the end of the payment link to use a fake Apple Pay popup, making it easier to test your integration on localhost.

## Troubleshooting

-   When integrating via iframe, make sure to include the `allow=payment` attribute on the iframe element.

## Reference Implementation

To explore our full set of Onramp demo applications across web, backend, and mobile, see the [Onramp demo app collateral](https://developer.chrome.com/get-started/demo-apps/starter/onramp-demo-app). Check out our [Apple Pay web demo](https://onramp-demo-application-git-main-coinbase-vercel.vercel.app/apple-pay) to see the experience in action. This demo shows how Apple Pay can be embedded directly in your web app for a seamless onramp experience. The source code is available [here](https://github.com/coinbase/onramp-demo-application). For a full React Native / Expo mobile reference implementation that showcases the Onramp v2 API, CDP Embedded Wallets, and Apple Pay integration, check out the [Onramp v2 mobile demo app](https://testflight.apple.com/join/s4VZYcej). Source code is available [here](https://github.com/coinbase/onramp-v2-mobile-demo/) For native iOS implementations, see our [iOS WKWebView demo](https://github.com/coinbase/onramp-v2-mobile-demo/tree/master/standalone-sample/ios-native-wkwebview) which shows how to embed the Apple Pay flow in a native app using WKWebView and handle payment events through the `cbOnramp` message handler.