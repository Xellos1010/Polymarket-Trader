# message signing

## Overview

Message signing allows you to apply a unique cryptographic signature to messages, ensuring authenticity and integrity. This is particularly useful for verifying ownership of accounts or authorizing actions without sending a transaction. Using the [CDP-SDK](https://github.com/coinbase/cdp-sdk), developers can sign messages for Solana. In this guide, you will learn how to:

-   Sign a message using the CDP v2 Server Wallet

## Prerequisites

It is assumed you have already completed the [Quickstart](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) guide.

## Sign message

Input a message to sign. The CDP v2 Server Wallet will return a signature that can be used to verify the message.