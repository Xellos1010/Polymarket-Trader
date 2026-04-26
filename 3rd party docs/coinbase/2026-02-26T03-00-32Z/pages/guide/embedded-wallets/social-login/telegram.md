# telegram

EVMSolana

## Overview

Configure a Telegram bot to enable Telegram authentication for Embedded Wallets. This guide walks you through creating a bot via BotFather, configuring its domain, and integrating with CDP. **Time to complete**: ~10 minutes

## Prerequisites

Before you begin, ensure you have:

-   **Telegram account**: An active Telegram account
-   **CDP Project ID**: Available in your CDP Portal dashboard
-   **Access to CDP Portal**: Permission to configure Embedded Wallets settings

## Step 1: Create a Telegram bot

1.  Open Telegram and search for **@BotFather**, or navigate directly to [BotFather](https://t.me/BotFather)
2.  Start a conversation and send `/newbot`
3.  Follow the prompts:
    -   **Name**: Enter a display name for your bot (e.g., “My App Login”)
    -   **Username**: Enter a unique username ending in `bot` (e.g., `myapp_auth_bot`)
4.  BotFather will respond with your **bot token** — copy and save it immediately

## Step 2: Configure bot branding

The bot’s profile picture and name are displayed to users during the Telegram login flow. Configure these to match your application’s branding:

1.  In BotFather, send `/setuserpic` and select your bot
2.  Upload your application logo or brand image
3.  Optionally, send `/setdescription` to set a description users see when they open the bot

## Step 3: Set the bot domain

Link your bot to the CDP callback domain so Telegram can route authentication requests:

1.  In BotFather, send `/setdomain`
2.  Select your bot from the list
3.  Enter the domain of your application

## Step 4: Configure CDP Portal

Now that your bot is configured, add the credentials to CDP Portal:

## Step 5: Verification

Test your Telegram configuration:

1.  **In your application**, ensure you’re calling the Telegram sign-in method (see [SDK Integration](#sdk-integration) below)
2.  **Click** “Sign in with Telegram”
3.  **Verify** that the Telegram login widget shows your bot’s name and profile picture
4.  **Complete** the authentication flow
5.  **Check** that the user successfully signs in and their wallet is accessible

## SDK Integration

Integrate Telegram authentication in your application using CDP’s SDK:

## Production considerations

## Troubleshooting

## What to read next