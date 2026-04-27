# overview

## Overview

The Invoicing APIs allow you to retrieve and read invoice data for your Coinbase Business account. Access detailed invoice information including payment details, line items, and invoice status.

## Use cases

-   **Accounting integration**: Automatically sync invoice data with accounting software
-   **Custom dashboards**: Build internal dashboards to track invoices and payment status
-   **Automated workflows**: Trigger workflows based on invoice status changes
-   **Financial reporting**: Generate custom reports based on invoice data
-   **Payment tracking**: Monitor invoice payments and track cryptocurrency transactions
-   **Customer insights**: Analyze invoice history and payment patterns

## Key Features

-   **Read invoice data**: Access detailed invoice information via API
-   **Filter and sort**: Query invoices by status, contact email, and date fields
-   **Pagination**: Efficiently retrieve large invoice datasets
-   **Webhooks**: Receive real-time notifications for invoice status changes

## Quick example

```
curl https://business.coinbase.com/api/v1/invoices?status=OPEN \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

```

Returns a list of open invoices with complete details.

## Prerequisites

Before using Invoicing APIs, you’ll need:

-   A [Coinbase Business account](https://www.coinbase.com/business)
-   Basic understanding of REST APIs and JWT authentication

## How it works

1.  **Authenticate** → Generate JWT token with your API key
2.  **List invoices** → Query invoices with optional filters
3.  **Get details** → Retrieve specific invoice by ID
4.  **Track status** → Monitor invoice status and payment information
5.  **Integrate** → Sync data with your systems

## What to read next