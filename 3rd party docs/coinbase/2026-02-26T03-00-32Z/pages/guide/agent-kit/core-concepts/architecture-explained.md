# architecture explained

AgentKit is a modular toolkit that allows you to build AI agents that can take actions onchain. It is designed to be:

-   Wallet provider agnostic
-   Framework agnostic
-   Model agnostic

## Core Components

The major components of AgentKit are:

### 1\. AgentKit Core (`@coinbase/agentkit`/ `coinbase-agentkit`)

The heart of AgentKit is the `@coinbase/agentkit` (or `coinbase-agentkit` in Python) package, which provides:

-   Common interfaces and classes for “Action Providers” (small units of functionality) and “Wallet Providers” (providers of wallets)
-   Simple configuration layer for API keys and environment variables
-   Shared logic for interacting with CDP Server Wallet or any wallet provider
-   Exposed interfaces/abstract classes for “Actions” that define:
    -   Method signatures
    -   Input schemas
    -   Operation logic
-   To create an agent, you will need to configure AgentKit with API keys and environment variables, and create the AgentKit instance that associates a wallet provider and action providers.

```
const agentKit = await AgentKit.from({
  walletProvider,
  actionProviders: [cdp, erc721, pyth, wallet],
});

```

### 2\. Framework Extensions

Built on top of AgentKit Core, these optional packages adapt core actions to specific agent frameworks or workflows. These are primarily what developers will use to build their agents. These frameworks typically wrap core actions with framework-specific tools, adapt to AI framework plugin interfaces, add “agentic” behaviors for AI model interaction, and handle action discovery and execution.

#### Available Extensions

-   **agentkit-langchain**: Deep integration with LangChain Tools (Python and TypeScript)
-   **farcaster-langchain**: Farcaster interaction capabilities
-   **twitter-langchain**: Twitter (X) interaction capabilities

```
// Integrate with AI framework (e.g., LangChain)
const tools = await getLangChainTools(agentKit);

```

### 3\. Wallet Providers

#### Default Implementation

-   Uses Server Wallet from Coinbase Developer Platform
-   Handles address management, transaction signing, and onchain interactions

#### Configuration Examples

**TypeScript:**

```
const agentKit = await AgentKit.from({
  walletProvider,
  actionProviders: [cdp, erc721, pyth, wallet],
});

```

#### Extensibility

-   Support for alternative wallet providers:
    -   Hardware wallets
    -   Browser extension wallets
    -   Hybrid approaches
-   Flexible wallet data import/export for state persistence

### 4\. Action Providers

Actions are the core unit of functionality in AgentKit.

#### Action Components

Each action includes:

-   **Name**: e.g., “transfer”, “deploy\_token”, “farcaster\_post\_cast”
-   **Input Schema**: Required arguments for the action
-   **Method**: Execution logic, typically interfacing with CDP SDK

#### Organization

-   Actions are grouped into logical namespaces:
    -   `actions/cdp/defi`: DeFi operations
    -   `actions/cdp/social`: Social platform interactions