# eip 191 signing

## Overview

Message signing allows you to apply a unique cryptographic signature to verify your identity on EVM networks. Using the [CDP-SDK](https://github.com/coinbase/cdp-sdk), developers can enable signing of messages with the EIP-191 standard, which prepends messages with a standard prefix before signing. This ensures messages are easily distinguishable from transaction data and provides a secure way to prove ownership of an address. In this guide, you will learn how to:

-   Create an EVM account
-   Sign a message using EIP-191 standard

## Prerequisites

It is assumed you have already completed the [Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) guide.

## 1\. Create an account

To create an account, see below:

After running the above snippet, you should see output similar to the following:

```
Successfully created account: 0x6aAEc7535706ce8B36ea184B2236D53702c1F06A

```

## 2\. Sign message

The v2 Server Wallet supports [EIP-191](https://developer.chrome.com/api-reference/v2/rest-api/evm-accounts/sign-an-eip-191-message) message signing, which provides a standardized way to sign messages on Ethereum. Per the specification, the message is prepended with `\x19Ethereum Signed Message:\n` followed by the message length before being signed.

Expand to learn more about EIP-191

### What is EIP-191?

EIP-191 is a standard for signing messages in Ethereum that helps prevent signed messages from being confused with signed transactions. It defines a structured format for signed data that includes:

-   A magic byte `0x19` to ensure the data could never be a valid RLP-encoded transaction
-   The string “Ethereum Signed Message:\\n”
-   The length of the message
-   The actual message content

### Use Cases

EIP-191 signed messages are commonly used for:

-   **Authentication**: Proving ownership of an address without making a transaction
-   **Off-chain agreements**: Signing terms of service or other agreements
-   **Message verification**: Allowing others to verify that a specific address signed a message
-   **Login systems**: Implementing “Sign in with Ethereum” functionality

### Security Benefits

-   Messages cannot be replayed as transactions
-   Clear separation between transaction signing and message signing
-   Human-readable messages instead of raw hashes
-   Protection against phishing attacks by showing users what they’re signing

Here is a complete example showing how to sign a message using EIP-191:

After running the above snippet, you should see output similar to the following:

```
Created account: 0x2Ae896e791c9596c7beDeCC3E06Fa6DA9aE2B193  
Message: Hello, Coinbase Developer Platform!
Signature: 0x1b0c9cf8cd4554c6c6d9e7311e88f1be075d7f25b418a044f4bf2c0a42a93e212ad0a8b54de9e0b5f7e3812de3f2c6cc79aa8c3e1c02e7ad14b4a8f42012c2c01b    

```

To summarize, in the example above, we:

-   Created an EVM account
-   Signed a plain text message using EIP-191 standard
-   The message was automatically prepended with the EIP-191 prefix before signing
-   Generated a cryptographic signature that can be used to verify the signer’s identity

The EIP-191 standard ensures that signed messages cannot be confused with transaction data, providing a secure way to prove ownership of an address.

## Verifying Signatures

Once you have a signature, it can be verified by anyone to confirm that the message was signed by the claimed address. This is useful for authentication systems, proving ownership, or validating off-chain agreements.

## What to read next

-   [**EIP-712 Signing**](https://developer.chrome.com/server-wallets/v2/evm-features/eip-712-signing): Learn about signing structured typed data with EIP-712.
-   [**v2 Security**](https://developer.chrome.com/server-wallets/v2/introduction/security): Learn about the security features of v2 Server Wallet.
-   [**API Reference**](https://developer.chrome.com/api-reference/v2/rest-api/evm-accounts/sign-an-eip-191-message): Explore the complete API reference for EIP-191 message signing.