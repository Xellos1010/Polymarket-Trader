# introduction

## Introduction

The Invoicing API enables developers to retrieve and read invoice data for their Coinbase Business account. Access detailed invoice information including payment details, line items, and invoice status.

## Base URL

```
https://business.coinbase.com/api/v1/invoices

```

## Prerequisites

Before using the Invoicing API, you’ll need:

-   A [Coinbase Business account](https://www.coinbase.com/business)
-   Basic understanding of REST APIs and JWT authentication

## Authentication

All Invoicing API endpoints require authentication using a JWT Bearer token. See the [Authentication guide](https://developer.chrome.com/coinbase-business/authentication-authorization/api-key-authentication) for details on generating your token.

## Available Endpoints

-   **[List Invoices](https://developer.chrome.com/api-reference/business-api/rest-api/invoicing/list-invoices)** - Retrieves a paginated list of invoices
-   **[Get Invoice](https://developer.chrome.com/api-reference/business-api/rest-api/invoicing/get-invoice)** - Retrieves details of a specific invoice by ID

## Invoice Status

Invoices can have the following statuses:

-   `DRAFT` - Invoice is in draft state and has not been sent
-   `OPEN` - Invoice has been sent and is awaiting payment
-   `SCHEDULED` - Invoice is scheduled to be sent at a future date
-   `PAID` - Invoice has been paid in full
-   `VOID` - Invoice has been voided and is no longer valid
-   `OVERDUE` - Invoice is past its due date and has not been paid