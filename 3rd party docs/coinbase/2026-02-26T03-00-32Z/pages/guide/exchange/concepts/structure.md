# structure

## Profiles

**Profiles** are the top-level organizational units in Coinbase Exchange that allow you to create multiple, isolated trading portfolios within a single Exchange account, and can be used for different trading strategies, teams, or purposes. Profiles are the equivalent of portfolios on the Coinbase Exchange website. The maximum number of profiles is 100.

### Default Profile

Every Exchange account has a **default profile** that serves as:

-   **Primary trading interface**: The main profile used for trading activities
-   **Crypto on/off-ramp**: The entry and exit point for cryptocurrency deposits and withdrawals
-   **Initial balance holder**: Where your initial funds are stored when you first fund your Exchange account

### Managing Profiles

You can [retrieve all profiles](https://developer.chrome.com/exchange/reference/exchangerestapi_getprofiles) associated with your Exchange account:

-   Go
    

```
credentials, err := credentials.ReadEnvCredentials("EXCHANGE_CREDENTIALS")
httpClient, err := core.DefaultHttpClient()
client := client.NewRestClient(credentials, httpClient)
profilesSvc := profiles.NewProfilesService(client)
request := &profiles.ListProfilesRequest{}
response, err := profilesSvc.ListProfiles(context.Background(), request)

```

To learn more about this SDK, please visit the [Exchange Go SDK](https://github.com/coinbase-samples/exchange-sdk-go).

### Creating Additional Profiles

You can [create additional profiles via the API](https://developer.chrome.com/exchange/reference/exchangerestapi_postprofile) to organize your trading activities. Common use cases include:

-   **Strategy separation**: Different profiles for different trading strategies (e.g., long-term holdings vs. day trading)
-   **Risk management**: Isolated profiles for different risk tolerances or investment goals

Profiles are designed for institutional trading and portfolio management. They are not intended for:

-   Tracking individual end-user balances
-   Implementing retail-style send/receive flows

### Profile Transfers

You can transfer balances between profiles (e.g., from the default profile to a strategy-specific one). This enables flexible portfolio management and fund allocation. For detailed information, see the [Transfers Concepts](#) section. Here’s how to create a new profile:

-   Go
    

```
credentials, err := credentials.ReadEnvCredentials("EXCHANGE_CREDENTIALS")
httpClient, err := core.DefaultHttpClient()
client := client.NewRestClient(credentials, httpClient)
profilesSvc := profiles.NewProfilesService(client)
request := &profiles.CreateProfileRequest{
    Name: profileName,
}
response, err := profilesSvc.CreateProfile(context.Background(), request)

```

### API Keys

An API key is scoped to a specific profile. An API key can only view and create data that belongs to its own profile, unless otherwise noted. This is true for the REST API, FIX API and Websocket Feed. To access data or actions on a different profile, create a new API key on the Coinbase Exchange website.

### Deleted Profiles

Profiles can be deleted on the Coinbase Exchange website. The permissions of an API key associated with a deleted profile are automatically set to “View.”

## Accounts

**Accounts** represent individual asset balances within a specific profile. Each account holds a single cryptocurrency or fiat currency and provides the foundation for trading activities on the Exchange.

### Account Characteristics

-   **Profile-specific**: Each profile has its own unique set of accounts
-   **Asset-specific**: Each account holds only one type of asset (e.g., BTC, ETH, USD)
-   **Independent balances**: The same asset in different profiles will have separate account IDs and balances
-   **Trading access**: Accounts enable you to place orders and execute trades

For example, if you have BTC in both Profile A and Profile B, they will have different `account_id`s, separate balances, and independent transaction histories.

### Listing Accounts

To retrieve a [list of all accounts](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getaccounts) for a given profile:

-   Go
    

```
credentials, err := credentials.ReadEnvCredentials("EXCHANGE_CREDENTIALS")
httpClient, err := core.DefaultHttpClient()
client := client.NewRestClient(credentials, httpClient)
accountsSvc := accounts.NewAccountsService(client)
request := &accounts.ListAccountsRequest{}
response, err := accountsSvc.ListAccounts(context.Background(), request)

```

### Account Ledger

Each account maintains a comprehensive historical ledger that tracks all financial events, including:

-   **Transfers**: Incoming and outgoing transfers between accounts or profiles
-   **Order matches**: Completed trades and their impact on balances
-   **Fees and rebates**: Trading fees, maker rebates, and other charges
-   **Conversions**: Asset conversions and their associated costs
-   **Deposits and withdrawals**: Funding and withdrawal activities

### Querying Account History

To [query the account ledger](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getaccountledger) for a specific time period, you can include a `start_date` parameter. This is useful for generating reports, tracking performance, or reconciling transactions. Example: Query ledger entries since the start of the year:

-   Go
    

```
credentials, err := credentials.ReadEnvCredentials("EXCHANGE_CREDENTIALS")
httpClient, err := core.DefaultHttpClient()
client := client.NewRestClient(credentials, httpClient)
accountsSvc := accounts.NewAccountsService(client)
request := &accounts.GetAccountLedgerRequest{
    AccountId: accountId,
    StartDate: "2025-01-01T00:00:00Z",
}
response, err := accountsSvc.GetAccountLedger(context.Background(), request)

```

## Coinbase Accounts

**Coinbase Accounts** represent the bridge between your Exchange account and your Coinbase retail account. These accounts are tied together via the same email address, meaning a single user identity spans both platforms. This connection allows you to easily transfer funds between Exchange and retail through the API.

### Key Features

-   **Direct integration**: Access to wallets managed via Coinbase.com
-   **Fund transfer capability**: Move funds between Exchange and Coinbase retail accounts
-   **Unified experience**: Manage both institutional trading and retail wallet activities
-   **Enhanced liquidity**: Leverage both platforms for optimal fund management

### Listing Coinbase Accounts

To retrieve all Coinbase Account wallets associated with your Exchange account:

-   Go
    

```
credentials, err := credentials.ReadEnvCredentials("EXCHANGE_CREDENTIALS")
httpClient, err := core.DefaultHttpClient()
client := client.NewRestClient(credentials, httpClient)
accountsSvc := accounts.NewAccountsService(client)
request := &accounts.ListCoinbaseAccountsRequest{}
response, err := accountsSvc.ListCoinbaseAccounts(context.Background(), request)

```

To learn more about this SDK, please visit the [Exchange Go SDK](https://github.com/coinbase-samples/exchange-sdk-go).

## Summary

The Coinbase Exchange account structure provides a flexible and powerful framework for institutional trading:

1.  **Profiles** organize your trading activities into separate portfolios
2.  **Accounts** manage individual asset balances within each profile
3.  **Coinbase Accounts** connect your Exchange activities to your retail Coinbase account

This hierarchical structure enables sophisticated portfolio management, risk control, and operational efficiency for institutional traders and organizations.