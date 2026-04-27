# sandbox

A public sandbox is available for testing API connectivity and web trading.

## Sandbox URLs

Use the following URLs to test your API connectivity. See the [Runbook](https://developer.chrome.com/exchange/introduction/systems-operations) for Production URLs.

API

URL

REST API

`https://api-public.sandbox.exchange.coinbase.com`

Websocket Feed

`wss://ws-feed-public.sandbox.exchange.coinbase.com`

Websocket Direct Feed

`wss://ws-direct.sandbox.exchange.coinbase.com`

FIX 5.0 API - Order Entry

`tcp+ssl://fix-ord.sandbox.exchange.coinbase.com:6121`

FIX 5.0 API - Market Data Snapshot Enabled

`tcp+ssl://fix-md.sandbox.exchange.coinbase.com:6121`

FIX 5.0 API - Market Data Snapshot Disabled

`tcp+ssl://fix-md.sandbox.exchange.coinbase.com:6122`

FIX 5.0 API - Dedicated Drop Copy

`tcp+ssl://fix-dc.sandbox.exchange.coinbase.com:6122`

## MiCA Sandbox URLs

MiCA clients must use the following URLs to test your API connectivity. See the [Runbook](https://developer.chrome.com/exchange/introduction/systems-operations) for Production URLs.

API

URL

REST API

`https://api-us.dma.sandbox.prime.coinbase.com`

Websocket Feed

`wss://ws-us.dma.sandbox.prime.coinbase.com`

Websocket Direct Feed

`wss://ws-us-direct.dma.sandbox.prime.coinbase.com`

FIX 5.0 API - Order Entry

`tcp+ssl://fix-us.dma.sandbox.prime.coinbase.com:7110`

FIX 5.0 API - Market Data Snapshot Enabled

`tcp+ssl://fix-us.dma.sandbox.prime.coinbase.com:7120`

FIX 5.0 API - Market Data Snapshot Disabled

`tcp+ssl://fix-us.dma.sandbox.prime.coinbase.com:7121`

FIX 5.0 API - Dedicated Drop Copy

`tcp+ssl://fix-dc.dma.sandbox.prime.coinbase.com:7122`

## Sandbox SSL Certificate

Your FIX SSL client must validate the following sandbox FIX server SSL certificate:

```
-----BEGIN CERTIFICATE-----
MIIEdDCCA1ygAwIBAgIQD03L1cHVypYSDFuvcnpAHzANBgkqhkiG9w0BAQsFADBG
MQswCQYDVQQGEwJVUzEPMA0GA1UEChMGQW1hem9uMRUwEwYDVQQLEwxTZXJ2ZXIg
Q0EgMUIxDzANBgNVBAMTBkFtYXpvbjAeFw0yMjAzMjcwMDAwMDBaFw0yMzA0MjUy
MzU5NTlaMCoxKDAmBgNVBAMMHyouc2FuZGJveC5leGNoYW5nZS5jb2luYmFzZS5j
b20wggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQC8LYRdqMoVNa/0M4MF
+Wkr8SiybZ95JycTE+0ZVmf92DKo4I8m/8fBtOrH0jgrhvamVSJ0lI6VFiAzlTd1
doUbliQ9Xm1aE/YHQO9J64AIP97peysgHBd+g3/Vhz33aaaU2vyHH5kPHiekU8n/
ObXPPoFd/Awul8uxxlXsVFx8oBWL2MeMjLNLLWNiGWq+lQloGKsQYVR/fQZizvpP
vyZO6pCLRId6+Wq3Tcb7NHQZc6+tePVi+5fovE+lm/yQrhjGqDzI7P4rWjJqCPrA
sYJeYFcVJhdSuFY2Ngm8eKeDP14TVEs9pkIWvyMGmB17QBPbRJipdoKu1N6fsx54
N9JDAgMBAAGjggF4MIIBdDAfBgNVHSMEGDAWgBRZpGYGUqB7lZI8o5QHJ5Z0W/k9
0DAdBgNVHQ4EFgQUa5RZ0yvv71YteSuqO1VRvmGGKv0wKgYDVR0RBCMwIYIfKi5z
YW5kYm94LmV4Y2hhbmdlLmNvaW5iYXNlLmNvbTAOBgNVHQ8BAf8EBAMCBaAwHQYD
VR0lBBYwFAYIKwYBBQUHAwEGCCsGAQUFBwMCMD0GA1UdHwQ2MDQwMqAwoC6GLGh0
dHA6Ly9jcmwuc2NhMWIuYW1hem9udHJ1c3QuY29tL3NjYTFiLTEuY3JsMBMGA1Ud
IAQMMAowCAYGZ4EMAQIBMHUGCCsGAQUFBwEBBGkwZzAtBggrBgEFBQcwAYYhaHR0
cDovL29jc3Auc2NhMWIuYW1hem9udHJ1c3QuY29tMDYGCCsGAQUFBzAChipodHRw
Oi8vY3J0LnNjYTFiLmFtYXpvbnRydXN0LmNvbS9zY2ExYi5jcnQwDAYDVR0TAQH/
BAIwADANBgkqhkiG9w0BAQsFAAOCAQEATpjyCMwAOSFKFTA67UaVkDCjz/ULBY6P
L4JwTJ+7kmT+HMvGimx15CsVjne64bT5twWlzqA/l4h25HGj0hD0TU2ktqmFhfAm
DpjGVp4KgIcZpvv7oRIU4e5I422Y++2UVuATwLWdELgpnm4AVq1aqI10XrQlJeHL
gRVfV5qkr9Vsc+fk7HY7YwbNQk2jXbRaj22f6GxiJ/6VmUcCD7zZ1GZtUipv0JEy
PtWD/BbSKNx1GJnLZ6L+QytPs+MW+FEetlU/oqPuyYRhmJUBUiwKkm6yKWRj9tQf
sq0a4uLI3SUgsBv/CQ/Qa9LnRdNjvlWSKLzeIX2LU9rE/3F3oQh7HQ==
-----END CERTIFICATE-----

```

## Unsupported Features

The Transfer endpoints are *not* available for testing in the Sandbox:

-   [Withdraw to payment](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/withdraw-to-payment-method)
-   [Deposit from payment](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/deposit-from-payment-method)
-   [Deposit from Coinbase account](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/deposit-from-coinbase-account)
-   [Withdraw to crypto address](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/withdraw-to-crypto-address)
-   [Withdraw to Coinbase Account](https://developer.chrome.com/api-reference/exchange-api/rest-api/transfers/withdraw-to-coinbase-account)

## Creating API Keys

To create an API key in the sandbox web interface:

1.  Go to the [sandbox web interface](https://public.sandbox.exchange.coinbase.com/)
2.  Create an account or sign in.
3.  Go to **API** in your profile dropdown menu.
4.  Click **New API Key**.

## Managing Funds

To add or remove funds in the sandbox web interface:

1.  Go to the **Portfolios** tab.
2.  Click the **Deposit** and **Withdraw** buttons as you would on the production web interface.