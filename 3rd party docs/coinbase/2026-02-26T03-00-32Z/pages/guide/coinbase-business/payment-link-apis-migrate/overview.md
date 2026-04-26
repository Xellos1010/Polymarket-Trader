# overview

Migrate from Commerce Charge API to Payment Link API for improved reliability, simplified integration, and modern authentication. This guide covers the key changes and provides code examples to help you update your integration.

Migration summary

## Key differences

Component

Charge API (Commerce)

Payment Link API

**Base URL**

`https://api.commerce.coinbase.com`

`https://business.coinbase.com/api/v1`

**Authentication**

`X-CC-Api-Key` header

Bearer token with CDP API key

**Primary resource**

Charge

Payment Link

**ID format**

UUID

24-character hexadecimal

**Payment URL field**

`hosted_url`

`url`

### Status and field mappings

Feature

Charge API

Payment Link API

**Status values**

NEW, SIGNED, PENDING, COMPLETED, EXPIRED, FAILED

ACTIVE, COMPLETED, DEACTIVATED, EXPIRED, FAILED

**Amount structure**

`local_price.amount` and `local_price.currency`

`amount` and `currency` (flat structure)

**Network specification**

Auto-selected

Explicit `network` field

**Redirect URLs**

`redirect_url`, `cancel_url`

`successRedirectUrl`, `failRedirectUrl`

## Migration checklist

### 1\. Update authentication

-   Create CDP API keys
-   Implement JWT Bearer token generation
-   Update API key storage and rotation procedures
-   Test authentication with Payment Link API

### 2\. Update API integration

-   Update base URL to `https://business.coinbase.com/api/v1`
-   Modify request bodies to match new schema
-   Update response parsing to handle new field names
-   Implement idempotency key generation for create requests

### 3\. Handle status changes

-   Map Charge statuses to Payment Link statuses in your system
-   Update order fulfillment logic for new status values
-   Set up webhooks for real-time payment notifications (see [Webhooks documentation](https://developer.chrome.com/coinbase-business/payment-link-apis/webhooks))
-   Update customer-facing status messages

### 4\. Update payment flows

-   Use `url` field instead of `hosted_url` for payment pages
-   Update redirect URL parameters
-   Test full payment flow end-to-end
-   Verify metadata is correctly passed through

### 5\. Handle pagination

-   Update list operations to use `pageSize` and `pageToken`
-   Implement `nextPageToken` handling for multiple pages
-   Test pagination with large datasets

### 6\. Test and validate

-   Consider testing your integration with the [sandbox environment](https://developer.chrome.com/coinbase-business/payment-link-apis/sandbox)
-   Verify amount calculations (ensure proper decimal handling)
-   Test expired payment link scenarios
-   Test deactivation flow
-   Verify metadata preservation
-   Test redirect URLs

### 7\. Monitor and observe

-   Set up monitoring for API errors
-   Track payment success rates
-   Monitor payment link expiration rates
-   Set up alerts for failed payments

## What to read next

-   [API & Schema Mapping](https://developer.chrome.com/coinbase-business/payment-link-apis/migrate/api-schema-mapping) - Detailed endpoint and response comparisons with code examples
-   [FAQ](https://developer.chrome.com/coinbase-business/payment-link-apis/migrate/faq) - Common questions and troubleshooting