# x

EVMSolana

## Overview

Configure your own X (formerly Twitter) OAuth application to enable custom branding for the X authentication experience in Embedded Wallets. This guide walks you through creating an X Developer account, configuring an OAuth 2.0 app, and integrating with CDP. **Time to complete**: ~15-20 minutes

## Prerequisites

Before you begin, ensure you have:

-   **X Developer account**: Free tier available at [developer.x.com](https://developer.x.com/)
-   **CDP Project ID**: Available in your CDP Portal dashboard
-   **Access to CDP Portal**: Permission to configure Embedded Wallets settings

## Step 1: Create X Developer account

1.  Navigate to [developer.x.com](https://developer.x.com/)
2.  Click **Sign In** and authenticate with your X account
3.  If you don’t have developer access yet, click **Apply for a developer account**
4.  Complete the developer application:
    -   **What is your primary reason for using the X API?**: Select the most appropriate option (e.g., “Building tools for X users”)
    -   **What are you planning to do with the X API?**: Describe your application (e.g., “Enable users to authenticate and access web3 wallets using their X account”)
    -   **Will your app use Tweet, Retweet, Like, Follow, or Direct Message functionality?**: Select “No” (unless you need these features)
    -   Complete any additional required fields
5.  Review and accept the X Developer Agreement
6.  Click **Submit** and wait for approval (usually instant for basic access)

## Step 2: Create X app

1.  Once your developer account is approved, navigate to the [X Developer Portal](https://developer.x.com/en/portal/dashboard)
2.  Click **\+ Create Project** or **\+ Add App**
3.  Fill in the app details:
    -   **App name**: Your application name (visible to users during OAuth, e.g., “My App”)
    -   **App description**: Brief description of your application
    -   **Website URL**: Your application’s website
    -   **Callback URL**: Leave blank for now (we’ll configure this in Step 3)
4.  Click **Create** or **Next**

## Step 3: Configure OAuth 2.0 settings

1.  In your X app dashboard, navigate to **Settings** or **App settings**
2.  Scroll to **User authentication settings** section
3.  Click **Set up** or **Edit**
4.  Configure OAuth 2.0:
    -   **App permissions**: Select **Read** (minimum required for authentication)
    -   **Type of App**: Select **Web App**
    -   **App info**:
        -   **Callback URI / Redirect URL**: Enter the exact URL:
            
            ```
            https://api.cdp.coinbase.com/platform/v2/end-users/auth/oauth/x/callback
            
            ```
            
        -   **Website URL**: Your application’s website URL

5.  Click **Save**
6.  After saving, you’ll see your OAuth 2.0 credentials:
    -   **Client ID**: Copy this value
    -   **Client Secret**: Click **Generate** or **Regenerate** to create a secret, then copy it

## Step 4: Enable OAuth 2.0

Ensure OAuth 2.0 is properly enabled for your app:

1.  In your app settings, verify that **OAuth 2.0** is enabled
2.  Confirm the **Read** permission is granted (minimum requirement)
3.  Check that the Callback URI is correctly configured

## Step 5: Configure CDP Portal

Now that you have your X OAuth credentials, configure them in the CDP Portal:

## Step 6: Verification

Test your X OAuth configuration:

1.  **In your application**, ensure you’re calling the X sign-in method (see [SDK Integration](#sdk-integration) below)
2.  **Click** “Sign in with X”
3.  **Verify** that the OAuth consent screen shows your application name
4.  **Complete** the authentication flow
5.  **Check** that the user successfully signs in and their wallet is accessible

## SDK Integration

Integrate X authentication in your application using CDP’s SDK:

## Production considerations

## Troubleshooting

## What to read next