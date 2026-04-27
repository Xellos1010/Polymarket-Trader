# overview

Connect to Coinbase’s 100M+ users without sharing their credentials. With OAuth2, users can securely authorize your app to access their accounts, send payments, and trade crypto.

## How it works

Endpoint

Purpose

`login.coinbase.com/oauth2/auth`

User authorization

`login.coinbase.com/oauth2/token`

Token exchange & refresh

`login.coinbase.com/oauth2/revoke`

Disconnect user (optional)

## Before you integrate

## When to use OAuth2

I want to…

Use

Access **other users’** Coinbase accounts

**OAuth2** (this guide)

Access **my own** CDP resources (server wallets, etc.)

[CDP API Keys](https://developer.chrome.com/get-started/authentication/cdp-api-keys)

Access **my own** Coinbase account

[Coinbase App API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication)

Use a legacy OAuth 1.0 integration

**OAuth2** — OAuth 1.0 endpoints are deprecated

## Use cases

### Payouts to Coinbase users

Send payments directly to users’ Coinbase accounts—payroll, creator payments, rewards. **Required scopes:** `wallet:accounts:read`, `wallet:transactions:send`

### Pay with Coinbase

Let users pay for goods and services using their Coinbase balance. **Required scopes:** `wallet:accounts:read`, `wallet:transactions:send`

### Trading integration

Allow users to trade crypto directly from your platform using their Coinbase account. **Required scopes:** `wallet:accounts:read`, `wallet:trades:create`, `wallet:trades:read`