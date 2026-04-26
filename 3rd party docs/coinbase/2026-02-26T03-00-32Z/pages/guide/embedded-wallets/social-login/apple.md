# apple

EVMSolana

## Overview

Configure Apple Sign In with your own Apple Developer credentials to enable custom branding for the Apple authentication experience in Embedded Wallets. This guide walks you through creating an App ID, Services ID, private key, and integrating with CDP. **Time to complete**: ~20-30 minutes

## Prerequisites

Before you begin, ensure you have:

-   **Apple Developer account**: Enrollment costs $99/year at [developer.apple.com](https://developer.apple.com/)
-   **CDP Project ID**: Available in your CDP Portal dashboard
-   **Access to CDP Portal**: Permission to configure Embedded Wallets settings

## Step 1: Create an App ID

1.  Navigate to the [Apple Developer Portal](https://developer.apple.com/account/)
2.  Go to **Certificates, Identifiers & Profiles**
3.  Select **Identifiers** from the sidebar
4.  Click the **+** button to create a new identifier
5.  Select **App IDs** and click **Continue**
6.  Select **App** as the type and click **Continue**
7.  Fill in the App ID details:
    -   **Description**: A descriptive name (e.g., “My App - Embedded Wallets”)
    -   **Bundle ID**: Choose “Explicit” and enter a reverse-domain identifier (e.g., `com.yourcompany.app`)
8.  Under **Capabilities**, enable **Sign in with Apple**
9.  Click **Continue**, then **Register**

## Step 2: Create a Services ID

1.  Still in **Identifiers**, click the **+** button again
2.  Select **Services IDs** and click **Continue**
3.  Fill in the Services ID details:
    -   **Description**: “CDP Embedded Wallets Service” (or custom description)
    -   **Identifier**: Enter a unique identifier, different from your App ID (e.g., `com.yourcompany.app.services`)
4.  Check **Sign in with Apple**
5.  Click **Continue**, then **Register**
6.  Back in the Identifiers list, click on your newly created Services ID
7.  Check **Sign in with Apple** and click **Configure**
8.  In the configuration dialog:
    -   **Primary App ID**: Select the App ID you created in Step 1
    -   **Domains and Subdomains**: Enter `api.cdp.coinbase.com`
    -   **Return URLs**: Click the **+** button and add:
        
        ```
        https://api.cdp.coinbase.com/platform/v2/end-users/auth/oauth/apple/callback
        
        ```
        

9.  Click **Next**, then **Done**
10.  Click **Continue**, then **Save**
11.  **Copy your Services ID** (e.g., `com.yourcompany.app.services`) - this is your **Client ID**

## Step 3: Create a private key

1.  In the Apple Developer Portal, select **Keys** from the sidebar
2.  Click the **+** button to create a new key
3.  Configure the key:
    -   **Key Name**: “CDP Embedded Wallets Key” (or custom name)
    -   Check **Sign in with Apple**
    -   Click **Configure** next to “Sign in with Apple”
4.  In the configuration dialog:
    -   **Primary App ID**: Select the App ID from Step 1
    -   Click **Save**
5.  Click **Continue**, then **Register**
6.  **Download the key**:
    -   Click **Download** to save the `.p8` file
    -   **Copy the Key ID** (10-character alphanumeric string displayed on the page)

7.  Click **Done**

## Step 4: Gather required information

Before configuring CDP Portal, collect all the required values:

Field

Description

Example

Where to find it

**Client ID**

Your Services ID

`com.yourcompany.app.services`

Step 2, item 11

**Team ID**

Your Apple Developer Team ID

`A1B2C3D4E5`

Top right of Apple Developer Portal

**Key ID**

ID of the private key you created

`X9Y8Z7W6V5`

Step 3, item 6

**Private Key**

Contents of the `.p8` file

`-----BEGIN PRIVATE KEY-----\n...`

Open the `.p8` file in a text editor

## Step 5: Configure CDP Portal

Now that you have all the required credentials, configure them in the CDP Portal:

## Step 6: Verification

Test your Apple Sign In configuration:

1.  **In your application**, ensure you’re calling the Apple sign-in method (see [SDK Integration](#sdk-integration) below)
2.  **Click** “Sign in with Apple”
3.  **Verify** that users can complete the Apple authentication flow
4.  **Check** that the user successfully signs in and their wallet is accessible

## SDK Integration

Integrate Apple authentication in your application using CDP’s SDK:

## Production considerations

## Troubleshooting

## What to read next