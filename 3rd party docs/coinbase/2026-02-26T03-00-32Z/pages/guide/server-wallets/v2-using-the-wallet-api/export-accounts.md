# export accounts

## Overview

The CDP Server Wallet offers a secure method for exporting private keys of EVM and Solana accounts, that are compatible with all major wallet providers, such as Coinbase Wallet and Metamask. The export API is end-to-end encrypted between the CDP SDK and the [TEE](https://developer.chrome.com/server-wallets/v2/introduction/security#tee-architecture), ensuring that keys are never exposed outside of the secure enclave while exporting. The private key is encrypted within the TEE by a single-use encryption key generated within the SDK.

Export private keys from your EVM accounts using either the account address or name.

## Exporting Solana accounts

Export private keys from your Solana accounts using either the account address or name.

## API Key Configuration for Private Key Export

In order to programmatically export account private keys, you’ll need to manually configure an API key with a specific scope. When you’re creating a new API Key, first expand the **API restrictions** panel then scroll down to the **API-specific restrictions** section. Ensure that **Export (export private key)** is checked before key creation as seen in the following screenshot.

## What to read next

-   [**v2 Wallet Security**](https://developer.chrome.com/server-wallets/v2/introduction/security): Learn more about the security features of the CDP v2 Server Wallet.
-   [**Policies**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview): Learn more about governing behavior of v2 accounts.
-   [**Importing Accounts**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/import-accounts): Learn more about importing EVM accounts.