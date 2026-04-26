# onrampbuyoptionsresponse

```
type OnrampBuyOptionsResponse = {
  paymentCurrencies: OnrampPaymentCurrency[];
  purchaseCurrencies: OnrampPurchaseCurrency[];
};

```

The response from the Onramp Buy Options API

## Properties

Property

Type

Description

`paymentCurrencies`

[`OnrampPaymentCurrency`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampPaymentCurrency)\[\]

List of supported fiat currencies that can be exchanged for crypto on Onramp in the given location. Each currency contains a list of available payment methods, with min and max transaction limits for that currency.

`purchaseCurrencies`

[`OnrampPurchaseCurrency`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/OnrampPurchaseCurrency)\[\]

List of available crypto assets that can be bought on Onramp in the given location.