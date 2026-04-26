# fetchwithx402

```
function fetchWithX402(options: FetchWithX402Options): FetchWithX402ReturnType;

```

Hook that provides a wrapped fetch function with payment handling.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`options`

[`FetchWithX402Options`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/FetchWithX402Options)

Configuration object for the fetch function

## 

[​](#returns)

Returns

[`FetchWithX402ReturnType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/FetchWithX402ReturnType) A wrapped fetch function with payment handling

## 

[​](#example)

Example

```
const { fetchWithPayment } = fetchWithX402();
const response = await fetchWithPayment("https://x402-resource.com", {
  method: "GET",
});

```