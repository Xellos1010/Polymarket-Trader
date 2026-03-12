# google

EVMSolana

## Overview

Configure your own Google OAuth application to enable custom branding and control over the Google Sign-In experience for Embedded Wallets. This guide walks you through creating a Google Cloud project, configuring OAuth credentials, and integrating with CDP. **Time to complete**: ~15-20 minutes

## Prerequisites

Before you begin, ensure you have:

-   **Google Cloud Platform account**: Sign up at [console.cloud.google.com](https://console.cloud.google.com/)
-   **CDP Project ID**: Available in your CDP Portal dashboard
-   **Access to CDP Portal**: Permission to configure Embedded Wallets settings

## Step 1: Create a Google Cloud project

1.  Navigate to the [Google Cloud Console](https://console.cloud.google.com/)
2.  Click the project dropdown at the top of the page
3.  Click **New Project**
4.  Enter a project name (e.g., “My App - Embedded Wallets”)
5.  Click **Create**
6.  Once created, select your new project from the project dropdown

## Step 2: Configure OAuth consent screen

1.  In the Google Cloud Console, navigate to **Google Auth Platform** > **Audience**
2.  Choose a user type:
    -   **Internal**: Only users within your Google Workspace organization can sign in (requires Google Workspace)
    -   **External**: Any user with a Google account can sign in
3.  In the **Test users** section (if using External user type):
    -   Add test users if your app is still in testing mode
4.  Click **Branding**
5.  Fill in the required fields:
    -   **App name**: Your application’s name (shown to users during OAuth)
    -   **User support email**: Your support email address
    -   **App logo**: (Optional) Upload your app’s logo
    -   **Application home page**: Your application’s URL
    -   **Application privacy policy link**: Link to your privacy policy
    -   **Application terms of service link**: Link to your terms of service
    -   **Authorized domains**: Add your application’s domain(s)
    -   **Developer contact information**: Your email address
6.  On the **Data Access** screen:
    -   Click **Add or Remove Scopes**
    -   Add the following scopes:
        -   `.../auth/userinfo.email`
        -   `.../auth/userinfo.profile`
        -   `openid`
    -   Click **Update** then **Save**
7.  Review your configuration and click **Back to Dashboard**

## Step 3: Create OAuth 2.0 credentials

1.  In the Google Cloud Console, navigate to **Google Auth Platform** > **Clients**
2.  Click **Create client**
3.  Choose **Web application** as the application type
4.  Enter a name for the OAuth client (e.g., “CDP Embedded Wallets”)
5.  Under **Authorized redirect URIs**, click **Add URI**
6.  Enter the exact redirect URI:
    
    ```
    https://api.cdp.coinbase.com/platform/v2/end-users/auth/oauth/google/callback
    
    ```
    

7.  Click **Create**
8.  A dialog will appear with your **Client ID** and **Client Secret**
9.  Copy both values and store them securely - you’ll need them in the next step

## Step 4: Configure CDP Portal

Now that you have your Google OAuth credentials, configure them in the CDP Portal:

## Step 5: Verification

Test your Google OAuth configuration:

1.  **In your application**, ensure you’re calling the Google sign-in method (see [SDK Integration](#sdk-integration) below)
2.  **Click** “Sign in with Google”
3.  **Verify** that the OAuth consent screen shows your application name (not “Coinbase” or “CDP”)
4.  **Complete** the authentication flow
5.  **Check** that the user successfully signs in and their wallet is accessible

## SDK Integration

Integrate Google authentication in your application using CDP’s SDK:

## Production considerations

## Troubleshooting

## What to read next