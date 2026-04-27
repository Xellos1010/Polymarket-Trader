# automated mass payouts

PaymentsServer Wallet v2

With Server Wallet, you can distribute payments to hundreds or even millions of recipients with just a few lines of code. API Wallets empower you to orchestrate complex financial operations with ease and security, opening the doors to applications that were previously impractical.

![Next.js](https://img.shields.io/badge/-Next.js-000000?style=for-the-badge&logo=next.js&logoColor=white)![TypeScript](https://img.shields.io/badge/-TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)![SCSS](https://img.shields.io/badge/-SCSS-cd6799?style=for-the-badge&logo=SASS&logoColor=white)![Coinbase](https://img.shields.io/badge/Coinbase-0052FF?style=for-the-badge&logo=Coinbase&logoColor=white)

This is a demo application of developer-custodied server wallet payments tool using the new [CDP Wallets v2](https://developer.chrome.com/server-wallets/v2/introduction/welcome). Users can sign in with their Web2 authentication - email (currently powered by Google or Github) and send funds to other users via their emails. All emails are backed by Coinbase Server Wallets running on Base Sepolia. Users who don’t have an existing wallet have one created on send, and they can login and claim their wallet later.

## Acknowledgments

This project leverages the [Gaslite Drop contract](https://github.com/PopPunkLLC/GasliteDrop) from [Pop Punk LLC Gaslite](https://gaslite.xyz/) for mass transfers. Special thanks to [HeimLabs](https://heimlabs.xyz/) for foundational work on the original Paymaker [frontend](https://github.com/heimlabs/paymaker-frontend) and [backend](https://github.com/heimlabs/paymaker-backend) that informed this sample app.

## Prerequisites

-   Git
-   [Bun](https://bun.sh/)
-   [Coinbase Developer Platform (CDP) Account](https://developer.coinbase.com/)
-   Docker for local Neon DB

## Getting Started

**0\. Deploy the Drop Contract** This application uses the Gaslite Drop smart contract for the batch transfers. The `.env.example` file contains the contract address for the Gaslite Drop contract deployed on the Base Sepolia testnet. If you want to deploy your own contract, you can clone the [Gaslite Core](https://github.com/poppunkllc/gaslite-core), and deploy by calling:

```
cd gaslite-core
forge build
forge script script/GasliteDrop.s.sol \
  --rpc-url https://sepolia.base.org \
  --broadcast \
  --chain-id 84532

```

Note: you will need to have your Base Sepolia wallet private key in your env as PRIVATE\_KEY

1.  Clone the repository:

```
git clone https://github.com/coinbase-samples/coinbase-samples/cdp-sdk-mass-payments-ts.git
cd cdp-sdk-mass-payments-ts

```

2.  Install dependencies:

3.  Set up your environment variables - You will need to do the following

-   Copy .env.example to .env.local
-   Get an API Key ID and Secret from [Coinbase Developer Platform portal](https://portal.cdp.coinbase.com/)
-   Generate a WalletSecret from the [CDP Portal Wallet Page](https://portal.cdp.coinbase.com/products/server-wallets)
-   Navigate to the Node page on the CDP Portal to get a Base Sepolia RPC URL for BASE\_SEPOLIA\_NODE\_URL
-   Generate a secret for NEXTAUTH\_SECRET via:

-   Generate a ClientId and Secret for either Sign in with Google or Sign in with Github (or both)

Set that output in the .env.local for JWT and CSRF token signing

4.  Start the local Postgres/Neon database:

5.  Start the development server:

The application will be available at [http://localhost:3000](http://localhost:3000/).