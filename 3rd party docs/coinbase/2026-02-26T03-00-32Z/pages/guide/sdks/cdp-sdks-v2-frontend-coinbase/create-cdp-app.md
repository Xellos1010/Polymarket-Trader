# create cdp app

This package contains the `@coinbase/create-cdp-app` CLI command, which allows developers to quickly bootstrap a React App using the CDP Web SDK.

## Quickstart

This guide will help you get started with `@coinbase/create-cdp-app`. You’ll learn how to create a new CDP-enabled React application with just a few commands.

### Prerequisites

Before you begin, make sure you have one of the following package managers installed:

-   [pnpm](https://pnpm.io/) (recommended)
-   [npm](https://www.npmjs.com/)
-   [yarn](https://yarnpkg.com/)

Gather your project ID and whitelist your app from the CDP Portal:

1.  Sign in or create an account on the [CDP Portal](https://portal.cdp.coinbase.com/)
2.  Copy your Project ID from the dashboard
3.  Go to the [Embedded Wallets CORS settings](https://portal.cdp.coinbase.com/products/embedded-wallets/cors)
4.  Click add origin and whitelist `http://localhost:3000` (or wherever your app will run)

### Create Your First CDP App

You can create a new CDP app using any of these methods:

```
# Using pnpm
pnpm create @coinbase/cdp-app@latest
# Using npm
npm create @coinbase/cdp-app@latest
# Using yarn
yarn create @coinbase/cdp-app@latest

```

The CLI will guide you through the setup process:

1.  Enter your project name (defaults to “cdp-app”)
2.  Select a template (React, Next.js, or React Native)
3.  Enter your CDP Project ID
4.  Select account type (EVM EOA, EVM Smart Accounts, or Solana)
5.  Configure Onramp (Next.js only)
6.  Confirm you have whitelisted your app domain on the CDP Portal
7.  Confirm directory overwrite if needed

## Available Templates

Currently, `create-cdp-app` offers the following templates:

-   **React** (`react`): A React application template that includes:
    -   Vite for fast development and building
    -   TypeScript for type safety
    -   CDP React components for authentication
    -   Example transaction components
    -   Base Sepolia integration
-   **Next.js** (`nextjs`): A Next.js application template that includes:
    -   Next.js 15 App Router
    -   CDP React components for authentication and wallet management
    -   Example transaction components for Base Sepolia
    -   Built-in TypeScript support
    -   ESLint with Next.js configuration
    -   Viem for type-safe Ethereum interactions
-   **React Native** (`react-native`): A React Native with Expo template that includes:
    -   Expo SDK for cross-platform mobile development
    -   CDP React Native components for authentication
    -   Example transaction components
    -   TypeScript support
    -   Support for both iOS and Android

## Command-Line Arguments

You can also pass command-line arguments to pre-configure the setup process.

Argument

Description

`<directory>`

Optional. The name of the directory to create the project in. Defaults to `cdp-app`.

`--template <name>`

Specifies the project template to use. Options: `react`, `nextjs`, `react-native`.

`--project-id <id>`

Your CDP Project ID from the portal.

`--account-type <type>`

Specifies the account type to configure. Options: `evm-eoa` (default), `evm-smart`, `solana`.

`--onramp`

Enables Coinbase Onramp.  
**Note:** This is only compatible with the `nextjs` template. If no template is specified, `nextjs` will be used automatically. If an incompatible template (e.g., `react`) is specified, this flag will be ignored and Onramp will be disabled.

`--no-onramp`

Disables Coinbase Onramp.

### Examples

```
# Create a Next.js app with EVM Smart Accounts and Onramp enabled
npm create @coinbase/cdp-app@latest my-next-app --template nextjs --project-id YOUR_PROJECT_ID --account-type evm-smart --onramp
# Create a React app with Solana support
npm create @coinbase/cdp-app@latest my-solana-app --template react --project-id YOUR_PROJECT_ID --account-type solana
# Create a React Native app with regular EVM accounts (default)
npm create @coinbase/cdp-app@latest my-mobile-app --template react-native --project-id YOUR_PROJECT_ID --account-type evm-eoa

```