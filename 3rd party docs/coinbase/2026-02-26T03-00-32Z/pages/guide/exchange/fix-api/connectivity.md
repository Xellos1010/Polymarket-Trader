# connectivity

[Financial Information eXchange](http://en.wikipedia.org/wiki/Financial_Information_eXchange), or FIX, is a standard protocol which can be used to enter orders, submit cancel requests, and receive fills. FIX API users typically have existing software that runs FIX for order management. The baseline specification for the Exchange FIX API:

-   Order Entry & Market Data: [FIX 5.0 SP2](https://www.onixs.biz/fix-dictionary/5.0/index.html)

## Supported Endpoints

## FIX Gateway

Before logging onto a FIX session, clients must establish a secure connection to the FIX gateway. See the [available endpoints](#supported-endpoints) above. **TCP SSL** If your FIX implementation does not support establishing a **native TCP SSL connection**, you must setup a local proxy such as [stunnel](https://www.stunnel.org/) to establish a secure connection to the FIX gateway. **Static IP** Coinbase Exchange **does not** support static IP addresses. If your firewall rules require a static IP address, you must create a TCP proxy server with a static IP address which is capable of resolving an IP address using DNS. **AWS IP** If connecting from servers **outside of AWS** which require firewall rules, use the [AWS provided resources](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html) to determine how best to whitelist AWS IP ranges.

## Ciphers

Coinbase Exchange supports **TLSv1.2** with the following server ciphers:

Recommend

Length

Cipher Suite

Elliptic Curve

Preferred

128 bits

`ECDHE-RSA-AES128-GCM-SHA256`

Curve P-256 DHE 256

Accepted

128 bits

`ECDHE-RSA-AES128-SHA256`

Curve P-256 DHE 256

Accepted

256 bits

`ECDHE-RSA-AES256-GCM-SHA384`

Curve P-256 DHE 256

Accepted

256 bits

`ECDHE-RSA-AES256-SHA384`

Curve P-256 DHE 256

## SSL Tunnels

[Exchange FIX API endpoints](https://developer.chrome.com/exchange/fix-api/connectivity#supported-endpoints) only accept TCP connections secured by SSL. If your FIX client library cannot establish an SSL connection natively, you must run a local proxy that establishes a secure connection and allows unencrypted local connections.