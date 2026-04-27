# connectivity

[Financial Information eXchange](http://en.wikipedia.org/wiki/Financial_Information_eXchange), or FIX, is a standard protocol that can be used to enter orders, submit cancel requests, and receive fills. FIX API users typically have existing software that runs FIX for order management. The baseline specification for the Prime FIX API:

-   Order Entry: [FIX 4.2](https://www.fixtrading.org/standards/fix-4-2/)

This page explains how to connect to the Prime FIX protocol.

## Supported Endpoints

## Getting Started

To get started quickly with FIX, take a look at the sample [FIX application](https://github.com/coinbase-samples/prime-scripts-py/tree/main/FIX) on Coinbase Samples.

## FIX Gateway

Before logging onto a FIX session, clients must establish a secure connection to the FIX gateway. See the [available endpoints](#supported-endpoints) above. **TCP SSL** If your FIX implementation does not support establishing a **native TCP SSL connection**, you must setup a local proxy such as [stunnel](https://www.stunnel.org/) to establish a secure connection to the FIX gateway. See the [SSL Tunnels](#ssl-tunnels) section for more details and examples.

## Maintenance Window

Sessions are forcibly logged out every day between 5pm ET and 5:05pm ET for a maintenance window. All users are required to restart their sessions during this time and reset sequence numbers to 1. During this period, orders will execute normally and clients will receive all Execution Reports upon reconnecting via FIX API, after the maintenance window completes. Upon reconnecting, all FIX clients are required to restart their sessions and reset sequence numbers to 1. It is recommended that clients leverage the Websocket [‘orders’](https://developer.chrome.com/prime/websocket-feed/channels) channel and REST [/orders](https://developer.chrome.com/api-reference/prime-api/rest-api/orders/list-portfolio-orders) endpoint to ensure clients have the ability to reconcile trading activity during this maintenance period. For any GTD or ‘day-type’ orders, it’s recommended to set ExpireTime to 5 minutes before daily maintenance window (e.g. 4:55pm ET)

## Replay

Our server supports message replay and sequence number renegotiation as specified in FIX Session Layer Online, under [Initial synchronization of messages in a FIX connection](https://www.fixtrading.org/standards/fix-session-layer-online/#establishing-a-fix-connection:~:text=Initial%20synchronization%20of%20messages%20in%20a%20FIX%20connection). **For active sessions using replay** — replay files are available for 24 hours, starting at 5pm ET and ending 5pm ET the next day (between the sequence number reset times). **For inactive sessions** — Prime FIX servers have an additional message queuing mechanism that holds a rolling 24 hrs worth of messages that are delivered to you once a FIX connection for a portfolio is re-established. **This is outside of replay** and is not impacted by the sequence number reset times.

## Session Limits

You can have up to 7 concurrent FIX sessions per portfolio, with a maximum of 1 session per API key.

## Rate Limiting

FIX API Rate limits are scoped at both the session-level and portfolio-level

-   FIX requests are limited to 50 messages/second per session
-   FIX orders are limited to 50 orders/second per portfolio
-   Users can have up to 7 concurrent FIX sessions per portfolio, with a maximum of 1 session per API key.

Take the following example, which occurs over a 1 second window:

-   Session 1 - 50 messages - OrderStatusRequest (H) —> Not Rate Limited
-   Session 2 - 50 messages - NewOrderSingle (D) — Not Rate Limited
-   Session 3 - 50 messages - OrderStatusRequest (H) — Not Rate Limited
-   Session 4 - 50 messages - NewOrderSingle (D) — Rejected, Exceeds Rate Limit of 50 orders/sec per portfolio

For more information, see [Rate Limits](https://developer.chrome.com/prime/rest-api/rate-limits).

## SSL Tunnels

`fix.prime.coinbase.com:4198` only accepts TCP connections secured by SSL. If your FIX client library cannot establish an SSL connection natively, you must run a local proxy that establishes a secure connection and allow unencrypted local connections.

#### Configure Stunnel

Example of stunnel configuration (stunnel.conf):

```
foreground = yes
debug = info
[Coinbase]
client = yes
accept = 4198
connect = fix.prime.coinbase.com:4198
verify = 4
CAfile = resources/fix-prime.coinbase.com.pem

```

#### Pull Down Prime Certificate

Command to pull down Prime Certificate for CAfile:

```
openssl s_client -showcerts -connect fix.prime.coinbase.com:4198 < /dev/null | openssl x509 -outform PEM > fix-prime.coinbase.com.pem

```

#### Run stunnel.conf

Command to run your stunnel.conf file:

```
stunnel resources/stunnel.conf

```

#### Configure \[SESSION\]

Sample \[Session\] Section in FIX configuration file:

```
[SESSION]
BeginString=FIX.4.2
SenderCompID=3e74e5c7-56a1-556a-b044-19936c5a728a
TargetCompID=COIN
HeartBtInt=30
SocketConnectPort=4198
SocketConnectHost=127.0.0.1
FileStorePath=./Sessions/

```