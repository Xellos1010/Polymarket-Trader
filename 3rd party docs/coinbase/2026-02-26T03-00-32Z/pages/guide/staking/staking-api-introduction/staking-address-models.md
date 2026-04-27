# staking address models

### Choosing the Right Address Model – Wallet Address vs. External Address

The CDP SDK supports two address models: External Addresses and CDP Wallet Addresses. Both models come with distinct advantages and limitations. Below, we explore these models to help you make an informed decision.

### External Addresses

External Addresses represent a self-custody model where private keys are not managed by the CDP SDK. Instead, developers maintain control of the keys, offering flexibility in implementing wallets and managing keys according to their preferences. This model is similar to wallets like MetaMask or Coinbase Wallet, where users manage a private key or seed phrase for recovery. All networks supported by CDP SDK are compatible with External Addresses.

#### External Addresses may be suitable if you:

-   Already use an existing wallet custody provider.
-   Are integrating staking for use cases where end users self-custody.
-   Leverage another key management (e.g. Multi-Party Computation) service.
-   Prefer to generate and manage your own wallet addresses.

#### Benefits of Staking with External Addresses

-   Broader network support (we prioritize network support for external addresses)
-   Complete control over wallet management and key storage.
-   Greater flexibility for advanced users.

#### Limitations of Staking with External Addresses

-   Lack of access to wider features offered by the CDP SDK.
-   Additional development effort required for secure key storage and retrieval.

### CDP Wallet Addresses

CDP Wallet Addresses represent a **developer-custodied model**, where private keys are managed by the CDP SDK. This model supports all actions available within the CDP SDK, although it is compatible with a subset of networks. See [Creating a Wallet](https://developer.chrome.com/server-wallets/v1/concepts/addresses#creating-wallet-addresses) for more details.

#### CDP Wallet Addresses may be suitable if you:

-   Cater to less crypto-savvy users.
-   Want to abstract complex processes from your users.

#### Benefits of Staking with CDP Wallet Addresses

-   Comprehensive feature set with seamless SDK integration.
-   Reduced development time for setup and implementation.
-   Flexible configuration options, such as [1-of-1 or 2-of-2 signer wallet schemes](https://developer.chrome.com/server-wallets/v1/concepts/wallets).

#### Limitations of Staking with CDP Wallet Addresses

-   Limited network support compared to External Addresses.
-   Private keys are still custodied by the developer (in 2-of-2 model one shard is still stored by developer), which may not suit all use cases.

### Choosing the Right Model for Your Use Case

Align your address model with the specific needs of your application and users. For more flexibility, control, and network support, External Addresses may be the best fit. For streamlined functionality and reduced development effort, CDP Wallet Addresses are a strong choice. For further assistance, please consult the CDP SDK documentation or join the conversation on the [CDP Discord](https://discord.com/invite/cdp). Feedback and questions are always welcome.