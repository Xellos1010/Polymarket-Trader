# oauth access

### What should be done to prepare for this change?

For OAuth connections, you must specify the portfolio ID when submitting spot orders to the [Create Order API](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/orders/create-order). This change is only required for OAuth connections and spot products. API key connections should not specify a portfolio ID, as it is derived from the API key itself. Set request body property `retail_portfolio_id` to the portfolio available when making POST calls to Create Order on spot products. You can retrieve portfolio IDs from the [List Portfolios API](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/portfolios/list-portfolios) and then see which accounts you were granted access to from the [List Accounts API](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/accounts/list-accounts). If all accounts belong to the same portfolio ID then you can use it for all OAuth based Create Order calls or you can present portfolio selection to your users or deduce the correct portfolio based on the accounts available. Example request body on Create Order API for OAuth connections only:

```
{
  "client_order_id": "SET_CLIENT_ORDER_ID",
  "product_id": "ETH-USDC",
  "side": "BUY",
  "retail_portfolio_id": "SET_PORTFOLIO_ID",
  "order_configuration": {…}
 }

```

### What APIs are impacted by this change?

Advanced Trade APIs (V3 endpoints) only, V2 endpoints are not impacted. Most V3 endpoints will respect portfolio account-level authorization and will error or not return data for accounts not granted. This only applies to connections via OAuth. API key authorization already respects portfolio account access. Notable Advanced Trade APIs are List Accounts and Show Account which will no longer return accounts in portfolios the user has not granted a third-party access to; and Create Order which will no longer accept spot orders without a specified portfolio ID if the connection is via OAuth (see Create Order error question below).

### What does Create Order error response “Target Account Not Tradable” or “Source Account Not Tradeable” mean?

An error response on the Create Order API containing messages “Target Account Not Tradable” or “Source Account Not Tradeable” is an indication that the user has not granted your application access to the fiat or crypto accounts specified by the currencies of the product. This is only relevant for spot products. Example error response on Create Order API

```
{
  "error": "PERMISSION_DENIED",
  "error_details": "Target Account Not Tradable",
  "message": "Target Account Not Tradable"
}

```

### Do users need to do anything?

No, users of your application do not have to do anything. If they do experience any issues after May 13th they can reconnect their Coinbase accounts via OAuth. Third-party OAuth application access can be reviewed on Coinbase at [https://accounts.coinbase.com/security/connections](https://accounts.coinbase.com/security/connections).