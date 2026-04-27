# wallet types

There are two types of wallets that can be created using the Server Wallet in CDP SDK, depending on how the private keys are managed: Coinbase-Managed (2-of-2) Wallets and Developer-Managed (1-of-1) Wallets. Looking for more information on the differences between Smart Wallet and Server Wallet? See our [Wallet Comparison](https://developer.chrome.com/server-wallets/comparing-our-wallets). Use the following table to understand the differences between Coinbase-Managed Wallets and Developer-Managed Wallets:

Feature

Coinbase-Managed (2-of-2)

Developer-Managed (1-of-1)

Custody model

Developer custodied

Developer custodied

Key management

Secured by Coinbase and the developer

Secured by the developer

Key structure

2-of-2

1-of-1

Key export

Cannot be exported

Can be exported via CDP SDK

Key storage

Stored securely in developer’s AWS account

Stored by the developer

Developer Experience

Use Coinbase Server-Signer

Developer implements private key storage

Setup time

10 minutes to provision infrastructure

A few seconds to set up the SDK

## Coinbase-Managed (2-of-2) Wallets

Server Wallet offers Coinbased-Managed 2-of-2 Wallets, leveraging advanced cryptographic techniques for enhanced usability and security. These wallets use Multi-Party Computation (MPC) to split private keys into two shares between Coinbase and the developer, ensuring improved security. To use Coinbase-Managed (2-of-2) Wallets, set up your [Server-Signer](https://developer.chrome.com/server-wallets/v1/concepts/server-signer).

## Developer-Managed (1-of-1) Wallets

Developer-Managed wallets are 1-of-1 wallets for which the developer manages the private keys. These wallets are not protected by Coinbase, and a compromised or lost private key could result in loss of funds. You can export the private key for such wallets through the CDP SDK. Secure the exported private keys from CDP SDK using a strong encryption algorithm (e.g., AES-256) before persisting in a database. Ensure you create cloud / offline backups of your encrypted data store. See [Developer-Managed Wallets](https://developer.chrome.com/server-wallets/v1/concepts/wallets#developer-managed-wallets) for more information.