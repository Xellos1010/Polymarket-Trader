# quickstart

## Prerequisites

Before integrating OAuth2, ensure you have the following:

## Overview

This guide walks you through integrating OAuth2 to access Coinbase user accounts:

-   Register an OAuth application
-   Redirect users to authorize
-   Exchange the authorization code for tokens
-   Make authenticated API requests

## 1\. Register your OAuth application

Go to [CDP Portal OAuth settings](https://portal.cdp.coinbase.com/projects/api-keys/oauth) and create a new OAuth2 application. You’ll receive:

-   **Client ID**: Public identifier for your application
-   **Client Secret**: Keep this secure—never expose in client-side code

```
GET https://login.coinbase.com/oauth2/auth
  ?response_type=code
  &client_id=YOUR_CLIENT_ID
  &redirect_uri=https://your-app.com/callback
  &scope=wallet:accounts:read,wallet:transactions:send
  &state=SECURE_RANDOM_STRING

```

## 3\. Exchange code for tokens

```
curl -X POST https://login.coinbase.com/oauth2/token \
  -d "grant_type=authorization_code" \
  -d "code=AUTHORIZATION_CODE" \
  -d "client_id=YOUR_CLIENT_ID" \
  -d "client_secret=YOUR_CLIENT_SECRET" \
  -d "redirect_uri=https://your-app.com/callback"

```

## 4\. Make authenticated requests

```
curl https://api.coinbase.com/v2/accounts \
  -H "Authorization: Bearer ACCESS_TOKEN"

```

## Token lifecycle

Token Type

Lifetime

Usage

Access Token

1 hour

Authenticate API requests

Refresh Token

1.5 years

Obtain new access tokens

## What to read next