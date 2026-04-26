# overview

EVMSolana

## Overview

Custom Social Login allows you to use your own OAuth applications for Google, Apple, X (Twitter), and Telegram authentication instead of CDP’s default OAuth providers. This gives you complete control over the OAuth experience, including branding, rate limits, and compliance requirements. By default, Embedded Wallets uses CDP-managed OAuth applications for social login, allowing users to recognize and trust Coinbase’s brand during authentication. Custom Social Login enables you to replace these defaults with your own OAuth applications while maintaining the same seamless authentication experience.

## Why use custom OAuth?

## How it works

## Critical disclaimers

## Provider comparison

Provider

Setup Complexity

Verification Time

Redirect URL

Special Requirements

**Google**

Moderate

Instant (may require app verification for production)

`https://api.cdp.coinbase.com/platform/v2/end-users/auth/oauth/google/callback`

OAuth consent screen configuration

**Apple**

High

Instant

`https://api.cdp.coinbase.com/platform/v2/end-users/auth/oauth/apple/callback`

Apple Developer account ($99/year), Private key (.p8 file)

**X**

Moderate

Instant (may require approval for additional permissions)

`https://api.cdp.coinbase.com/platform/v2/end-users/auth/oauth/x/callback`

Rate limits apply even on free tier

**Telegram**

Low

Instant

N/A (uses your app domain)

Telegram account, BotFather bot creation. **React Native not supported.**

## Prerequisites

Before configuring custom OAuth, ensure you have:

-   **CDP Project ID**: Available in the CDP Portal
-   **Access to CDP Portal**: Permission to configure Embedded Wallets settings
-   **Developer accounts**: Accounts with each OAuth provider you plan to use:
    -   Google: [Google Cloud Platform account](https://console.cloud.google.com/)
    -   Apple: [Apple Developer account](https://developer.apple.com/) ($99/year required)
    -   X: [X Developer account](https://developer.x.com/) (free tier available)
    -   Telegram: [Telegram account](https://telegram.org/) with access to [BotFather](https://t.me/BotFather)
-   **OAuth 2.0 understanding**: Basic familiarity with OAuth flows and terminology

## Quick start

Choose a provider to get started with custom OAuth configuration:

## What to read next