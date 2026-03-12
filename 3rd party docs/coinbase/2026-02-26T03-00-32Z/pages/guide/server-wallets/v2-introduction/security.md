# security

## Overview

The v2 Server Wallet is secured by CDP’s [**Trusted** Execution Environment](https://en.wikipedia.org/wiki/Trusted_execution_environment) (TEE), a highly isolated compute environment that is used for sensitive cryptographic operations such as private key generation and transaction signing. The TEE is hosted on [AWS Nitro Enclaves](https://aws.amazon.com/ec2/nitro/nitro-enclaves/), an isolated, secure compute environment. The TEE has **no persistent storage**, **no interactive access**, and **no external networking**, ensuring that even a root or admin user cannot access or SSH into the TEE. All operations that take place in the TEE are not visible to CDP, AWS, or the outside world.

## TEE architecture

The following diagram demonstrates the architecture of the TEE:

### How it works

1.  Incoming requests to the v2 Server Wallet are authenticated with the developer’s [Wallet Secret](#wallet-secrets).
2.  After validating the request, it is forwarded to the TEE over [VSOCK](https://man7.org/linux/man-pages/man7/vsock.7.html), which provides the only source of data flow to and from the TEE.
3.  The TEE performs sensitive operations, including verifying the wallet authentication signature, private key generation, and transaction signing. Account private keys are encrypted and decrypted exclusively **within the enclave**, and **never leave the TEE**. An encrypted version of the private keys are stored in CDP’s database and can only be accessed by the developer with the corresponding Wallet Secret.
4.  The resulting payload is sent back to the v2 Server Wallet over VSOCK.
5.  The v2 Server Wallet returns the result to the client.

## Wallet Secrets

Wallet Secrets are used to authenticate requests to the v2 Server Wallet. Wallet Secrets are asymmetric private keys that conform to [ECDSA](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-5.pdf#page=29), a cryptographic technique for creating and verifying digital signatures. They rely on the [**secp256r1**](https://www.secg.org/sec2-v2.pdf#page=13) elliptic curve (also known as P-256), making keys small, fast, and highly secure. Read more about using Wallet Secrets in our [v2 API Reference documentation](https://developer.chrome.com/api-reference/v2/authentication#wallet-secret).

### 2FA

To increase security of your wallet, we recommend [enabling two-factor authentication](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/wallet-secret-rotation#2fa-management) (2FA). We support physical security keys, passkeys, Google or Duo authentication apps, security push notifications, and even trusted contacts.

### Lost access

If you lose access to your Wallet Secret, you can delete the old secret and generate a new one through the CDP Portal. See [Wallet Secret Rotation](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/wallet-secret-rotation) for more information on how to update your secret and manage two factor authentication.

## What to read next

-   [v2 API Reference documentation](https://developer.chrome.com/api-reference/v2/authentication)): Learn how to use Wallet Secrets to authenticate requests to the v2 Server Wallet.
-   [v2 Server Wallet Quickstart](https://developer.chrome.com/quickstart.mdx): Learn how to use the v2 Server Wallet to create a new wallet and perform transactions.
-   [Wallet Secret Rotation](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/wallet-secret-rotation): Learn how to update your Wallet Secret and manage two factor authentication.