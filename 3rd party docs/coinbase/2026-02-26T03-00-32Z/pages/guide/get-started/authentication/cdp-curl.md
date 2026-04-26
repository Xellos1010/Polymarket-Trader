# cdp curl

[CDP cURL](https://github.com/coinbase/cdpcurl) is a command line tool that allows you to make HTTP requests to all Coinbase API endpoints that leverage CDP API keys.

### How to use CDP cURL

1.  Install [cdpcurl](https://github.com/coinbase/cdpcurl) by running:

```
brew tap coinbase/cdpcurl
brew install cdpcurl

```

Or Install via Go

```
go install github.com/coinbase/cdpcurl@latest

```

2.  Use cdpcurl through the command line by passing in the filepath to your CDP API Key JSON file

### Examples

##### Get account balance of BTC with Coinbase App API

```
cdpcurl -k ~/Downloads/cdp_api_key.json 'https://api.coinbase.com/v2/accounts/BTC'

```

##### Get the latest price of BTC with Advanced Trading API

```
cdpcurl -k ~/Downloads/cdp_api_key.json 'https://api.coinbase.com/api/v3/brokerage/products/BTC-USDC'

```

##### Create a wallet on Base Sepolia with CDP SDK

```
cdpcurl -k ~/Downloads/cdp_api_key.json -X POST -d '{"wallet": {"network_id": "base-sepolia"}}' 'https://api.developer.coinbase.com/platform/v1/wallets'

```

##### Create an Onramp user token

```
cdpcurl -X POST 'https://api.developer.coinbase.com/onramp/v1/token' \
-k /tmp/cdp_api_key.json \
-d '{"addresses": [{"address":"0x750EF1D7a0b4Ab1c97B7A623D7917CcEb5ea779C", "blockchains": ["ethereum"]}]}'

```

##### List products with Advanced Trading API

```
cdpcurl -k ~/Downloads/cdp_api_key.json 'https://api.coinbase.com/api/v3/brokerage/products'

```

##### Get specific product details with Advanced Trading API

```
cdpcurl -k ~/Downloads/cdp_api_key.json 'https://api.coinbase.com/api/v3/brokerage/products/BTC-USD'

```

##### Get exchange rates with Coinbase App API

```
cdpcurl -k ~/Downloads/cdp_api_key.json 'https://api.coinbase.com/v2/exchange-rates?currency=USD'

```

##### Get best bid/ask prices with Advanced Trading API

```
cdpcurl -k ~/Downloads/cdp_api_key.json 'https://api.coinbase.com/api/v3/brokerage/best_bid_ask?product_ids=ETH-USD'

```

##### Get list of supported currencies with Coinbase App API

```
cdpcurl -k ~/Downloads/cdp_api_key.json 'https://api.coinbase.com/v2/currencies'

```