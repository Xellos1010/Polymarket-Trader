# cli setup

This Quick Start guide uses the [Prime CLI](https://github.com/coinbase-samples/prime-cli), which offers an easy way to test currently available REST endpoints without additional coding. This section explains how to set up and use the Prime CLI on your own machine. For instance, to create a new market order, you can run the following simple command:

```
primectl create-order -i ETH-USD -t MARKET -s BUY -b 0.1

```

## Quick start guide

1.  In a terminal, run the following command to ensure you have Go installed on your machine (because the Prime CLI is written in Go). If not, visit the [official Go website](https://go.dev/doc/install) to download the latest Go version.

2.  Clone the Prime CLI repository in a location of your choice, then navigate inside the new directory.

```
git clone https://github.com/coinbase-samples/prime-cli
cd prime-cli

```

3.  Pass your Prime credentials. While this logic is handled with each request, you still need to specify your credentials at the start of the application.

You can create Coinbase Prime credentials in the Prime web console under Settings -> APIs. `PRIME_CREDENTIALS` should match the following format, and the following command should be submitted from the terminal window in which you plan to run the Prime CLI:

```
export PRIME_CREDENTIALS='{
"accessKey":"ACCESSKEY_HERE",
"passphrase":"PASSPHRASE_HERE",
"signingKey":"SIGNINGKEY_HERE",
"portfolioId":"PORTFOLIOID_HERE",
"svcAccountId":"SVCACCOUNTID_HERE",
"entityId":"ENTITYID_HERE"
}'

```

4.  Ensure your project dependencies are up-to-date:

5.  Build the application binary and specify an output name that you will use to call the Prime CLI, e.g., `primectl`:

6.  Move your new binary to a directory that’s in your system’s PATH to make the application accessible from any location. For example, this is how you would move `primectl` to `/usr/local/bin`, as well as set permissions to reduce risk:

```
sudo mv primectl /usr/local/bin/
chmod 755 /usr/local/bin/primectl

```

7.  You are now ready to use the Prime CLI! Here are some example commands to help you get started.

-   Listing portfolios (which include your `EntityId`):

-   Listing flags for creating an order:

```
primectl create-order --help

```

-   Previewing an order for 0.001 ETH at the current market USD rate:

```
primectl create-order-preview -b 0.001 -i ETH-USD -s BUY -t MARKET

```