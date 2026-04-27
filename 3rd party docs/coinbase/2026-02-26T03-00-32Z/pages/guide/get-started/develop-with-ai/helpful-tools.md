# helpful tools

## Overview

AI-assisted crypto development requires specialized tools for optimal workflow efficiency. This guide focuses on AI-first development environments, local testing essentials, and CDP-specific integrations that enhance your development experience. These tools are specifically chosen to support AI-assisted workflows, local development iterations, and seamless integration with the Coinbase Developer Platform.

## AI-first development environments

**[Cursor IDE](https://cursor.sh/)** ⭐ **Recommended**

-   AI-first code editor built on VS Code
-   Natural language editing and code completion
-   Built-in chat with full project context
-   **When to use:** Primary development environment for AI-assisted coding
-   **Getting started:** Download from cursor.sh and follow [MCP Setup Guide](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-mcp-setup)

**[Claude](https://claude.ai/)**

-   Advanced conversational AI for code review and architecture planning
-   Excellent at creating detailed specifications and breaking down complex problems
-   **When to use:** Architectural planning, code review, detailed specifications
-   **Getting started:** Create account and follow [MCP Setup Guide](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-mcp-setup) for CDP integration

**[ChatGPT](https://chat.openai.com/)**

-   General-purpose AI assistant for development tasks
-   Strong at explaining concepts and providing multiple solution approaches
-   **When to use:** Quick questions, concept explanations, alternative solutions
-   **Getting started:** Create account, or see [AI Development Setup](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-development-setup) for AI tool recommendations

**[GitHub Copilot](https://github.com/features/copilot)**

-   AI pair programmer that works in your existing editor
-   Inline code suggestions and auto-completion
-   **When to use:** If you prefer to stay in your current IDE (VS Code, IntelliJ, etc.)
-   **Getting started:** Install extension and subscribe, or see [AI Development Setup](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-development-setup) for IDE recommendations

## CDP integration

### Documentation

**CDP MCP Server** ⭐ **Essential**

-   Gives AI tools direct access to CDP documentation
-   Enables accurate code generation using latest CDP components
-   **When to use:** Always - dramatically improves AI code quality for CDP development
-   **Getting started:** Follow the [MCP Setup Guide](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-mcp-setup) for step-by-step configuration

### Context

**[CDP Web SDK](https://docs.cdp.coinbase.com/embedded-wallets/quickstart)**

-   JavaScript/TypeScript SDK for embedded wallets
-   Simplifies wallet creation and user authentication
-   **When to use:** Building consumer apps with embedded wallets
-   **Getting started:** See [Embedded Wallets Quickstart](https://developer.chrome.com/embedded-wallets/quickstart)

**[OnchainKit](https://onchainkit.xyz/)**

-   React components and hooks for onchain development
-   Pre-built UI components for wallets, transactions, and DeFi
-   **When to use:** Building DeFi apps or apps requiring external wallet connections
-   **Getting started:** Follow [OnchainKit Getting Started](https://onchainkit.xyz/getting-started) or use [AI Development Setup](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-development-setup)

**[AgentKit](https://docs.cdp.coinbase.com/agent-kit/)**

-   Framework for building AI agents that interact with blockchain networks
-   Built-in CDP SDK integration for automated trading and operations
-   **When to use:** Building AI agents, trading bots, or automated portfolio management
-   **Getting started:** See [AgentKit Quickstart](https://developer.chrome.com/agent-kit/getting-started/quickstart) or use [AI Development Setup](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-development-setup)

**[CDP SDK](https://github.com/coinbase/cdp-sdk)** ⭐ **Essential**

-   Client libraries for managing EVM and Solana wallets with CDP-secured private keys
-   Available in TypeScript, Python, Go, and Rust
-   Programmatic wallet creation, management, and transactions
-   **When to use:** Server-side applications, automation, or any programmatic wallet operations
-   **Getting started:** See [CDP SDK Documentation](https://docs.cdp.coinbase.com/api-v2/docs/welcome) and choose your preferred language

## Local testing essentials

**[ngrok](https://ngrok.com/)** ⭐ **Essential for mobile testing**

-   Creates secure HTTPS tunnels to your local development server
-   Essential for testing mobile wallet integrations and embedded wallet flows
-   **When to use:** Testing on real mobile devices, sharing development builds with stakeholders
-   **Getting started:** See [Testing Strategies](https://developer.chrome.com/get-started/develop-with-ai/development/ai-testing) for mobile testing setup

**Jest/Vitest**

-   Unit testing frameworks for JavaScript/TypeScript (most CDP starter apps come pre-configured)
-   **When to use:** Testing AI-generated components and crypto functions locally
-   **Getting started:** See [Testing Strategies](https://developer.chrome.com/get-started/develop-with-ai/development/ai-testing) for AI-assisted testing approaches

**[Playwright](https://playwright.dev/) / [Cypress](https://cypress.io/)**

-   End-to-end testing frameworks for complete crypto user flows
-   Test wallet connections, transactions, and complex user journeys
-   **When to use:** Testing complete AI-generated user flows from wallet connection to transaction completion
-   **Getting started:** See [Testing Strategies](https://developer.chrome.com/get-started/develop-with-ai/development/ai-testing) for end-to-end testing with AI assistance

## AI debugging and optimization

**Built-in linting tools**

-   ESLint, Prettier, and TypeScript checking (included in most CDP starter apps)
-   Automatically catch common mistakes in AI-generated code
-   **When to use:** Always enabled - essential for validating AI-generated code quality
-   **Getting started:** See [Troubleshooting Guide](https://developer.chrome.com/get-started/develop-with-ai/ai-troubleshooting) for linting setup

**[dotenv](https://www.npmjs.com/package/dotenv)** / **.env files**

-   Standard approach for managing environment variables in local development
-   Secure storage for CDP API keys and configuration (most CDP starter apps include this)
-   **When to use:** Storing CDP API keys, project IDs, and sensitive configuration locally
-   **Getting started:** Check your starter app’s `.env.example` file or see [AI Deployment Guide](https://developer.chrome.com/get-started/develop-with-ai/development/ai-deployment)

## Spec-driven development

**[Spec Kit](https://github.com/github/spec-kit)**

-   Designed specifically for breaking specifications into actionable development tasks
-   Helps structure AI-generated project plans into manageable steps
-   **When to use:** Converting high-level crypto app ideas into specific, testable development tasks
-   **Getting started:** See [AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows) for spec-driven development approach

**Specification documentation**

-   Markdown files, GitBook, or Notion for detailed project specifications
-   Essential for giving AI tools context about your project requirements
-   **When to use:** Before starting any significant crypto application development
-   **Getting started:** Follow the spec-driven development process in [AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)

**Task breakdown tools**

-   Linear, Height, or GitHub Issues for organizing AI-generated development tasks
-   Track progress through specification implementation phases
-   **When to use:** Managing larger projects with multiple development phases and dependencies
-   **Getting started:** Import task lists generated through spec-driven development workflows

### AI-first development starter kit

Essential tools for getting started with AI-assisted crypto development:

1.  **Cursor IDE** - Primary AI development environment
2.  **CDP MCP Server** - AI documentation access for accurate code generation
3.  **ngrok** - Local mobile testing of wallet integrations
4.  **CDP Web SDK or OnchainKit** - Depending on your app type ([embedded wallets](https://developer.chrome.com/embedded-wallets/quickstart) vs [external wallets](https://docs.base.org/onchainkit/wallet/wallet))

### Local testing and iteration

Key tools for efficient local development and testing:

1.  **ngrok** - Test on real mobile devices
2.  **Jest/Vitest** - Unit testing AI-generated components
3.  **dotenv/.env files** - Secure local API key storage

### Advanced AI workflows

For complex projects requiring structured development:

1.  **Claude for planning** - Detailed architecture and specification creation
2.  **Spec Kit** - Break specifications into actionable development tasks
3.  **Cursor IDE** - Implement with full project context and AI assistance
4.  **Playwright + ngrok** - Comprehensive testing including mobile devices

## What to read next

Now that you know which tools are available, learn how to use them effectively:

-   **[AI Development Setup](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-development-setup)**: Configure your starter app and development environment using these tools
-   **[MCP Setup Guide](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-mcp-setup)**: Connect the CDP MCP server to your AI tools for better code generation
-   **[AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)**: Learn day-to-day development practices using your configured tools
-   **[AI Deployment](https://developer.chrome.com/get-started/develop-with-ai/development/ai-deployment)**: Deploy your applications using Vercel and other hosting platforms