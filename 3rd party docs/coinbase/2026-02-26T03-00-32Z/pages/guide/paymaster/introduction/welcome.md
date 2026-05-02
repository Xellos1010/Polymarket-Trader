# welcome

On Ethereum and EVM-compatible blockchains, every transaction requires **gas**—a fee paid in ETH to compensate the network for processing your transaction. This creates friction for new users who need to acquire ETH before they can do anything onchain. **Gas sponsorship** solves this by letting developers pay gas fees on behalf of their users. Instead of users needing ETH in their wallet, your application covers the cost. This creates a seamless, “gasless” experience similar to traditional web apps.

## How Paymasters Work

A **Paymaster** is a smart contract that sponsors gas fees for user transactions. It’s a core component of [ERC-4337 Account Abstraction](https://eips.ethereum.org/EIPS/eip-4337), which enables programmable transaction flows beyond what traditional EOA (Externally Owned Account) wallets can do. Here’s the flow:

1.  **User initiates a transaction** from a smart account (not an EOA)
2.  **Bundler receives the userOperation** and checks if a Paymaster will sponsor it
3.  **Paymaster evaluates the request** against your configured policies (allowlisted contracts, spend limits, etc.)
4.  **If approved, the Paymaster pays the gas** and the transaction executes
5.  **You’re billed** for the sponsored gas plus a 7% fee through your CDP account (monthly invoicing)

The Coinbase Developer Platform Paymaster provides a fully-managed gas sponsorship service that includes:

-   **Paymaster + Bundler in one endpoint** — A single API endpoint handles both paymaster signing and transaction bundling
-   **Policy controls** — Configure contract allowlists, per-user limits, and global spend caps
-   **Analytics & logs** — Monitor sponsored transactions, gas usage, and costs
-   **ERC-7677 compliant** — Works with any compatible SDK or wallet (does not have to be paired with CDP Wallets)

### Supported Networks

Network

Status

Base Mainnet

✅ Supported

Base Sepolia

✅ Supported

Other EVM networks

Use third-party ERC-7677 paymasters

## Get Started

## Example Repository

See [our examples on GitHub](https://github.com/coinbase/paymaster-bundler-examples) for integrations with popular SDKs including Viem, Permissionless, and more.

## Need Help?

Reach out in the `#paymaster` channel in the [CDP Discord](https://discord.com/invite/cdp).