# postman

## Overview

[Postman](https://www.postman.com/) is a popular API testing tool that lets you send requests and inspect responses without writing code. We provide a pre-configured collection with all Payment API endpoints ready to use.

## Prerequisites

-   Postman account ([sign up](https://www.postman.com/) or use the web version)
-   Sandbox API credentials from the [CDP Portal Sandbox](https://portal.cdp.coinbase.com/v2/sandbox) (covered in the [Quickstart](https://developer.chrome.com/api-reference/payment-apis/sandbox/quickstart))

## 1\. Download files

Download both files below:

-   [CDP Payments Collection](https://developer.chrome.com/api-reference/payment-apis/CDP%20Payments%20Sandbox.postman_collection-docs.json): Pre-built requests for all Payment API endpoints with the correct HTTP methods, headers, and request body templates
-   [CDP Payments Environment](https://developer.chrome.com/api-reference/payment-apis/CDP%20Payments%20Sandbox.postman_environment-docs.json): Variables for the Sandbox base URL and your API keys

## 2\. Import into Postman

## 3\. Configure API keys

## 4\. Test requests

The collection handles JWT authentication automatically using your configured API keys.

## Available endpoints

The Postman collection includes all Sandbox endpoints:

-   **[Accounts](https://developer.chrome.com/api-reference/payment-apis/sandbox/guides/accounts)** - Create and list accounts
-   **[Deposit Destinations](https://developer.chrome.com/api-reference/payment-apis/sandbox/guides/deposit-destinations)** - Create and list deposit destinations
-   **[Payment Methods](https://developer.chrome.com/api-reference/payment-apis/sandbox/guides/payment-methods)** - List payment methods and test withdrawals
-   **[Transfers](https://developer.chrome.com/api-reference/payment-apis/sandbox/guides/transfers)** - Create, execute, and list transfers (onchain, email, payment method)

## What to read next