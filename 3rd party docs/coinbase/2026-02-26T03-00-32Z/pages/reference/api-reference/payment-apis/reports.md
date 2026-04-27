# reports

## Overview

Access and download reports from your project in the [CDP Portal](https://portal.cdp.coinbase.com/). Reports provide detailed insights into your project’s transfers and activity.

Currently, Portal Reports support:

-   **Transfer Reports** only — a detailed view of all [transfers](https://developer.chrome.com/api-reference/payment-apis/rest-api/transfers/transfers) processed through your project
-   **Recurring schedules** — daily, weekly, or monthly (no one-time reports)
-   **SFTP delivery** — reports are delivered as CSV files to your SFTP server

Navigate to **Reports** in [CDP Portal](https://portal.cdp.coinbase.com/). Click the **Create recurring report** button.

## 2\. Enter report data

Name your report and select the columns to include. Add filters to narrow down the data if needed. For example, for a Transfers report:

### Available columns

The available columns are organized into the following categories:

### Available filters

Narrow down which transfers to include in your report:

Filter

Description

Status

Filter by transfer status: `quoted`, `processing`, `completed`, `failed`, etc.

Asset

Filter by source asset or destination asset (e.g., `USDC`, `ETH`).

Amount

Filter by source amount or destination amount ranges.

Once finished, click **Continue** in the lower-right corner.

## 2\. Setup schedule and delivery

### Schedule frequency

Select a start date and frequency for your recurring report:

Frequency

Runs

Data window

Daily

Every day

Previous full day (midnight to midnight UTC)

Weekly

Same day each week as your start date

Previous full week (up to midnight UTC)

Monthly

1st of each month

Previous full month (e.g., a report on August 1st includes all of July)

All scheduled reports run between **00:00 and 02:00 UTC**. You can optionally set an end date to stop generating reports after a specific date.

### Configure SFTP delivery

Reports are delivered as CSV files via SFTP. To configure delivery:

1.  Enter your SFTP host, port, username, and remote path.
2.  Copy the SSH public key provided by CDP and add the public key to your SFTP server’s authorized keys file.
3.  Click **Test connection** to verify the setup.

## 3\. Review and create report

On the final step, review your report configuration and click **Create report** to set up your recurring report.

## 4\. Check status and manage reports

On the **Reports** dashboard, you can see all recurring reports you have created and their completion status. Select a report to:

-   View report details and history
-   Modify schedule, filters, or delivery settings
-   Enable or disable the schedule
-   Delete the report