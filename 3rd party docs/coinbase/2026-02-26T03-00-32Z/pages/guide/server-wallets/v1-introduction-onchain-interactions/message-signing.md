# message signing

Signing is a cryptographic action that lets a user prove ownership of an address without exposing their private key or initiating an onchain transaction. When a user signs a message, the signed message:

-   Demonstrates control of the address.
-   Can be used to approve a particular action or set of actions.
-   Is a verifiable piece of cryptographic data that can be used onchain or offchain.

In existing apps, signatures are used to authorize transfers of particular assets, prove ownership of an address to sign into an app, and vote in Decentralized Autonomous Organizations (DAOs).

## Example Use Cases

-   **Identity/Sign-on:** Use API Wallets to sign into applications or prove ownership of a wallet.
-   **Multi-signature API Wallets:** Have multiple parties sign partial messages, which are then combined to create a valid transaction or decrypt information.
-   **Scalable, high-frequency interactions:** Have games or trading apps use signatures to verify actions and submit batch updates to the blockchain.

## Examples

### Signing a Payload

You can sign an arbitrary string using the [EIP-191](https://eips.ethereum.org/EIPS/eip-191) standard.

### Signing Typed Structured Data

[EIP-712](https://github.com/ethereum/EIPs/blob/master/EIPS/eip-712.md) is the standard for signing typed structured data, which improves the readability of messages and helps prevent signing opaque, potentially malicious messages.