# changelog

These release notes list upcoming and recent changes to the Coinbase App API.

## 2025

### 2025-SEP-25

-   Adding scaled order type to allow users to create scaled orders. Scaled order is a type of limit order where the user can customize the price and size distribution of its sub-orders.

### 2025-SEP-15

-   Adding attached order configuration Edit Orders endpoint to allow users to modify attached orders.

### 2025-Aug-14

-   Starting from mid-October 2025, a maximum length requirement of 128 characters will be enforced for some client generated IDs. Invalid argument errors will be raised if the IDs are longer than this limit. The affected fields currently are:
    -   Create Order: [client\_order\_id](https://developer.chrome.com/api-reference/advanced-trade-api/rest-api/orders/create-order#body-client-order-id)

### 2025-JUL-30

-   Get Public Market Trades & Market Trades specify side as the maker side of the trade

### 2025-MAY-14

-   List Products now implicitly filters for futures products when expiry-related parameters are passed unless explicitly specified to return spot products.

### 2025-MAY-13

-   Advanced Trade APIs enforce portfolio account-level trade access for [OAuth connections](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/oauth-access)
-   Removed from documentation unsupported OAuth layout and promo parameters and updated Authorization and API key docs pages.

### 2025-MAY-01

-   Brought in Advanced Trade API documentation and reorganized Coinbase App API docs.

### 2025-APR-11

-   Updated [Deposits](https://developer.chrome.com/coinbase-app/transfer-apis/deposit-fiat) and [Withdrawals](https://developer.chrome.com/coinbase-app/transfer-apis/withdraw-fiat) response formats and quote endpoints no longer default to commit.

### 2025-APR-03

-   Added a new error code `resource_exhausted` to the list of [known error codes](https://developer.chrome.com/coinbase-app/api-architecture/error-messages) returned by Coinbase App APIs.

### 2025-FEB-24

-   Added a new error code `unauthorized` to the list of [known error codes](https://developer.chrome.com/coinbase-app/api-architecture/error-messages) returned by Coinbase App APIs.

### 2025-FEB-03

-   Outbound notifications will cease starting Feb 17, 2025. If you’re receiving these, migrate to polling the [Transactions API](https://developer.chrome.com/coinbase-app/track-apis/transactions) instead.

### 2025-JAN-28

-   Updated [Travel Rule Requirements for Crypto Sends](https://developer.chrome.com/coinbase-app/transfer-apis/travel-rule) to include travel rules information required for EU users.

### 2025-JAN-22

-   [Legacy API keys](https://developer.chrome.com/coinbase-app/authentication-authorization/legacy-keys) will be expired starting Feb 5, 2025. Users will need to migrate to CDP API keys to continue using the Coinbase App API.

### 2025-JAN-09

-   Updated the [Send Money API’s arguments](https://developer.chrome.com/coinbase-app/transfer-apis/send-crypto#arguments) to remove deprecated parameters (`financial_institution_website`, `to_financial_institution`). These were incorrectly added to the docs, and have been removed as they are no longer supported.

## 2024

### 2024-NOV-22

-   Updated the OAuth2 docs to correctly reflect that refresh tokens expire after 1.5 years.

### 2024-OCT-22

-   Starting Oct 30, 2024, Transactions API will be updated to enable multichain sends and phone number support.
-   Transfer Money and several other fields are being deprecated.
-   These changes are relevant to all countries except Singapore, Netherlands, Canada, France, and Bermuda.

### 2024-SEP-23

-   Accounts API updated to return Loan Collateral accounts for users that have them, even if 0 notional balance. Accounts are of type `collateral`.

### 2024-SEP-13

-   Announced Notifications feature deprecation, v2/transactions endpoint will be removed Oct. 1, 2024.

### 2024-AUG-16

-   “Sign in with Coinbase” is now known as Coinbase App API. No changes to API functionality occur with this rename. CDP Portal and documentation pages have been updated.

### 2024-AUG-07

-   Updated the OAuth2 docs to link to [Coinbase Developer Platform (CDP)](https://portal.cdp.coinbase.com/) for creating and managing OAuth applications.

### 2024-JUN-12

-   Updated Transactions documentation to correct that Advanced Trade Fill commission is per fill and not per order.

### 2024-APR-18 *(duplicate entries combined)*

-   Updated the docs to reflect:
    -   Coinbase Cloud is now [Coinbase Developer Platform (CDP)](https://portal.cdp.coinbase.com/)
    -   “Sign in with Coinbase” now supports API keys created on CDP
    -   Legacy Keys are now optional

### 2024-APR-05

-   Updated the OAuth2 docs:
    -   Update revoke example to match requirements.

### 2024-APR-02

-   “Sign in with Coinbase” API can now be accessed using Cloud API Keys.

### 2024-MAR-28

-   Updated the OAuth2 docs:
    -   OAuth2 URLs: Changed URLs to new Unified Login versions.
    -   Revoke requirements: Revoke now requires client credentials to be passed in.

### 2024-MAR-07

-   Announced the replacement of v2 Payment Method endpoints with v3 Payment Method endpoints.

### 2024-MAR-06

-   Removed support for the Show User endpoint.

### 2024-MAR-05

-   Added new transaction types, including `tx` as a new default and catch-call type that will be refactored into dedicated types.
-   Updated the `send` type so that it is no longer a catch-all type but restricted to send only.

### 2024-FEB-13

-   Updated OAuth2 docs:
    -   Token expiration: Access tokens expire in 1 hour (not 2).
    -   State requirements: The `state` parameter must be at least 8 characters long.
    -   Account access: Access to user wallets now defined by a drop-down on the consent page.
    -   Send limit flow: `wallet:transactions:send` requires additional OAuth2 parameters and 2FA.
    -   Scopes: The scope `offline_access` returns a refresh token.

### 2024-FEB-09

-   Removed the deprecated endpoint, Update Current User.

### 2024-FEB-02

-   Announced multiple changes to the Transaction APIs:
    -   Feb 7: Select parameters are being deprecated, added, or updated.
    -   Mar 31: Deprecated parameters are being removed.

### 2024-JAN-23

-   Announced the upcoming Feb 15 deprecation of select Account endpoints.

### 2023-OCT-23

-   Announced the upcoming Nov 30 deprecation of select Buys and Sells APIs.