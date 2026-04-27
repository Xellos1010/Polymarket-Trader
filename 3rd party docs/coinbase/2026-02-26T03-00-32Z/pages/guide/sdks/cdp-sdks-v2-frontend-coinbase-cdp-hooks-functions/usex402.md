# usex402

```
function useX402(options: FetchWithX402Options): FetchWithX402ReturnType;

```

Hook that provides a fetch function with X402 payment handling.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`options`

`FetchWithX402Options`

Configuration object for the fetch function

## 

[​](#returns)

Returns

`FetchWithX402ReturnType` A fetch function with X402 payment handling

## 

[​](#example)

Example

```
const { fetchWithPayment } = useX402();
const response = await fetchWithPayment("https://x402-resource.com", {
  method: "GET",
});

```