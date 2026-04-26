# ai development setup

This guide walks you through the process of setting up a sample app with source code to help you get started with AI-assisted crypto development with Coinbase Developer Platform (CDP).

## Prerequisites

-   [Coinbase Developer Platform account](https://portal.cdp.coinbase.com/) for access to our APIs
-   AI development environment: [Cursor IDE](https://cursor.sh/) (recommended), [GitHub Copilot](https://github.com/features/copilot), [Claude](https://claude.ai/), or [ChatGPT](https://chat.openai.com/)
-   `npm` for setting up starter apps

## 1\. Choose a starter app

Start with the right starter app depending on your needs. Use this quick mapping:

-   If you want seamless onboarding with embedded wallets for consumers → choose **Consumer-based apps**.
-   If you need external wallet connections and DeFi protocol integrations → choose **Trading apps (DeFi)**.
-   If your core is automation, bots, or 24/7 server-side operations → choose **Automation and AI Agents**.

### Comparison

App type

Best for

Why it works with AI

Starter prompt

Consumer-based apps

-   Payment apps
-   Social crypto features
-   Consumer wallets
-   Tipping systems

-   Embedded-wallet patterns
-   Common flows
-   Simpler security
-   Rapid UI iteration

I’m building a consumer app with embedded wallets. Help me add \[feature\] that works with email auth and no seed phrases.

Trading apps (DeFi)

-   DEX interfaces
-   Portfolio trackers
-   Yield farming apps
-   Advanced trading tools

-   OnchainKit patterns
-   External wallet integration
-   Rich DeFi ecosystem
-   AI helps with complex state

I’m building a DeFi app with OnchainKit and wagmi. Help me create \[feature\] integrating with \[protocol\].

Automation and AI Agents

-   Trading bots
-   Automated portfolio management
-   Cross-chain operations
-   Data analysis

-   AgentKit AI-native patterns
-   Built-in CDP SDK
-   AI-assisted automation design
-   Server-side for 24/7 ops

I’m building an AI agent with AgentKit and CDP SDK. Help me automate \[goal\] with robust error handling and monitoring.

## 2\. Scaffold your project

-   Consumer App
    
-   DeFi App
    
-   AI Agent
    

```
npm create @coinbase/cdp-app@latest my-consumer-app

```

You will be prompted to choose a template type and configure your project. This starter app works with the CDP Web SDK for embedded wallets, email authentication, and no seed phrases required.Find more detailed instructions on its setup in the [Embedded Wallet Guide](https://developer.chrome.com/embedded-wallets/quickstart).

```
npm create onchain@latest my-defi-app

```

You will be prompted to choose a template type and configure your project. This starter app works with OnchainKit for external wallet connections, DeFi protocol integrations, and advanced trading features.Find more detailed instructions on its setup in the [OnchainKit Getting Started Guide](https://onchainkit.xyz/getting-started).

```
npm create onchain-agent@latest my-agent

```

You will be prompted to choose a template type and configure your project. This starter app works with AgentKit for building AI agents that can interact with blockchain networks, including trading, automated portfolio management, and 24/7 server-side operations.Find more detailed instructions on its setup in the [AgentKit Quickstart Guide](https://developer.chrome.com/agent-kit/getting-started/quickstart).

### Integrating starter apps

For complex projects that need features from multiple templates, you can start with one and use AI to integrate patterns from others: **Consumer + DeFi Integration:**

```
# Start with embedded wallet consumer app
npm create @coinbase/cdp-app@latest my-hybrid-app
cd my-hybrid-app
# Then ask AI to add DeFi features
# "Add OnchainKit's Swap and Transaction components to enable DeFi trading while keeping embedded wallet authentication"

```

**DeFi + AI Agent Integration:**

```
# Start with DeFi app
npm create onchain@latest my-trading-app
cd my-trading-app
# Then ask AI to add automation
# "Add AgentKit patterns to automate trading strategies and portfolio rebalancing"

```

**Consumer + AI Agent Integration:**

```
# Start with consumer app
npm create @coinbase/cdp-app@latest my-smart-wallet
cd my-smart-wallet
# Then ask AI to add intelligent features
# "Add AgentKit to create AI-powered spending insights and automated savings features"

```

**Custom Modifications:** If no template fits perfectly, start with the closest match and modify:

```
# Example: DeFi app with embedded wallets instead of external wallet connections
npm create onchain@latest my-custom-app
# Then ask AI to restructure
# "Convert this OnchainKit app to use embedded wallets instead of external wallet connections, keeping all DeFi components but changing the authentication layer"

```

## Re-configuring your app

Once you have scaffolded your project, it should have come equipped with a hidden `.env` file. This file is used to store environment-specific variables for your development setup. It allows you to configure settings like API keys, project IDs, and network configurations without hardcoding them into your application code. This file is crucial for maintaining security and flexibility, as it keeps sensitive information out of your codebase and allows for easy changes to configuration settings.

## What to read next

Your environment is now configured for AI-first development. Continue with:

-   **[AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)**: Learn day-to-day development practices, advanced configuration, debugging strategies, and best practices
-   **[AI-Assisted Deployment Guide](https://developer.chrome.com/get-started/develop-with-ai/development/ai-deployment)**: Detailed deployment setup and environment variable configuration for production releases