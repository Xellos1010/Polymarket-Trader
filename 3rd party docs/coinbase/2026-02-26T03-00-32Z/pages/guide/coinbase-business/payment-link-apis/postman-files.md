# postman files

The following Postman collection file is available for download.

-   [Coinbase Business Payment Links Collection](https://developer.chrome.com/coinbase-business/payment-link-apis/files/payment_links.postman_collection.json)
-   [Coinbase Developer Platform Postman Environment](https://developer.chrome.com/coinbase-business/api-architecture/files/coinbase_developer_platform.postman_environment.json)

This collection includes example payloads and automatic JWT token generation for authenticated requests.

## Prerequisites

Before using this collection, ensure you have:

-   Coinbase Business account
-   [CDP API key](https://developer.chrome.com/coinbase-business/introduction/get-started#creating-api-keys) using **ECDSA** signature algorithm
-   Downloaded the following files (we will use these to setup Postman):
    -   **[Payment Links collection](https://developer.chrome.com/coinbase-business/payment-link-apis/files/payment_links.postman_collection.json)**: Pre-configured API requests for creating, listing, and managing payment links
    -   **[Environment configuration](https://developer.chrome.com/coinbase-business/api-architecture/files/coinbase_developer_platform.postman_environment.json)**: Authentication setup and variables for CDP API access

## 1\. Setup Postman

## 2\. Configure your environment

Set up authentication by adding your CDP API key details to Postman.

## 3\. Authenticate and test endpoints

The Postman collection automatically generates a JWT token before each request using your environment variables.

## Available endpoints

The Payment Links collection includes the following endpoints:

-   **Create Payment Link** - Creates a new payment link for accepting cryptocurrency payments
-   **List Payment Links** - Retrieves a paginated list of payment links with optional filtering
-   **Get Payment Link** - Retrieves details of a specific payment link by ID
-   **Deactivate Payment Link** - Deactivates a payment link to prevent further payments

## Troubleshooting

### Common issues and solutions

Issue

Solution

Using Ed25519 key instead of ECDSA

Use ECDSA key - starts with `-----BEGIN EC PRIVATE KEY-----`

Missing quotes in environment values

Include quotes around `name` and `privateKey` values

Invalid JSON

Check the environment dropdown is set to your CDP environment

401 Unauthorized errors

Check JWT generation in Postman Console and verify credentials

### Need additional help?

-   **Documentation**: Review the [Authentication Guide](https://developer.chrome.com/coinbase-business/authentication-authorization/api-key-authentication)
-   **Community**: Join the [CDP Discord](https://discord.com/invite/cdp) for support
-   **Support**: Contact Coinbase Business support with your `X-Request-Id` from failed requests

## What to read next

-   [Payment Link API Reference](https://developer.chrome.com/api-reference/business-api/rest-api/payment-links) - Detailed API documentation with all request/response schemas
-   [Authentication Guide](https://developer.chrome.com/coinbase-business/authentication-authorization/api-key-authentication) - Learn more about CDP API key authentication
-   [Getting Started Guide](https://developer.chrome.com/coinbase-business/introduction/get-started) - Set up your Coinbase Business account and create API keys
-   [Migrate from Commerce](https://developer.chrome.com/coinbase-business/payment-link-apis/migrate/overview) - If you’re moving from the Commerce Charge API