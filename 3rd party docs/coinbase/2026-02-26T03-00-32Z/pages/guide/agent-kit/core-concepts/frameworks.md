# frameworks

AgentKit integrates with several popular AI frameworks, enabling you to build blockchain-capable agents using your preferred development tools.

The following frameworks are currently supported:

-   [Agents SDK by OpenAI](#agents-sdk-by-openai)
-   [LangChain](#langchain)
-   [Eliza](#eliza-framework)
-   [Vercel AI SDK](#vercel-ai-sdk)
-   [Model Context Protocol (MCP)](#model-context-protocol-mcp)

## Agents SDK by OpenAI

The [OpenAI Agents SDK](https://platform.openai.com/docs/guides/agents) is a lightweight, Python-first framework that enables you to build production-ready AI agents with minimal abstractions. It’s designed to be easy to learn while providing powerful capabilities for real-world applications. You can find our implementation in our [Replit template](https://replit.com/t/coinbase-developer-platform/repls/CDP-AgentKit-Agents-SDK-Quickstart/view#README.md) or the [AgentKit repository](https://github.com/coinbase/agentkit/).

-   Replit
    
-   Local Environment (repository)
    

#### Step 1: Set Up Your Development Environment

1.  Fork the [Python](https://replit.com/t/coinbase-developer-platform/repls/CDP-AgentKit-Agents-SDK-Quickstart/view#README.md) template
2.  Once forked, you’ll have your own version of the project to modify

#### Step 2: Configure Environment Variables

1.  Click on “Tools” in the left sidebar
2.  Select “Secrets”
3.  Add the following secrets:

```
CDP_API_KEY_NAME=your_cdp_key_name # From cdp.coinbase.com
CDP_API_KEY_PRIVATE_KEY=your_cdp_private_key
OPENAI_API_KEY=your_openai_key # from platform.openai.com
NETWORK_ID="base-sepolia" # Optional, defaults to base-sepolia.

```

#### Step 3: Run the Agent

You can start this chatbot by clicking the “Run” button.

#### Step 1: Clone the repository

Ensure that you have Python 3.10+ and Poetry installed:

```
python --version  # Should be 3.10+
poetry --version  # Make sure Poetry is installed

```

Clone and navigate to the example directory:

```
# Clone the repository
git clone https://github.com/coinbase/agentkit.git
# Navigate to the chatbot-python example
cd agentkit/python/examples/openai-agents-sdk-cdp-chatbot

```

#### Step 2: Configure Environment Variables

Copy the example environment file and configure your variables:

```
# Copy the example environment file
cp .env.local .env
# Edit the .env file with your credentials:
CDP_API_KEY_NAME=your_cdp_key_name # From cdp.coinbase.com
CDP_API_KEY_PRIVATE_KEY=your_cdp_private_key
OPENAI_API_KEY=your_openai_key # from platform.openai.com
NETWORK_ID="base-sepolia" # Optional, defaults to base-sepolia.

```

#### Step 3: Run the Agent

```
# Install dependencies
poetry install
# Run the chatbot
poetry run python chatbot.py

```

## LangChain

[LangChain](https://www.langchain.com/) is a framework for developing applications powered by language models. Our implementation is available in our [Replit templates](https://replit.com/t/coinbase-developer-platform/profile) and the [AgentKit repository](https://github.com/coinbase/agentkit/).

-   Replit
    
-   Local Environment
    

#### Step 1: Set Up Your Development Environment

1.  Fork the template from [NodeJS (EVM)](https://replit.com/t/coinbase-developer-platform/repls/AgentKitjs-Quickstart-020-EVM-CDP-Wallet/view), [Python (EVM)](https://replit.com/t/coinbase-developer-platform/repls/AgentKitpy-012-EVM/view), or [NodeJS (Solana)](https://replit.com/t/coinbase-developer-platform/repls/AgentKitjs-Solana-Quickstart-v020/view) Replit templates
2.  Once forked, you’ll have your own version of the project to modify

#### Step 2: Configure Environment Variables

1.  Click on “Tools” in the left sidebar
2.  Select “Secrets”
3.  Add the following secrets:

#### Step 3: Run the Agent

You can start this chatbot by clicking the “Run” button.

-   EVM
    
-   Solana
    

For Solana, note that the private key is not stored in a file but rather in the environment variable `SOLANA_PRIVATE_KEY`. For Replit to work with an address, you must add the secret `SOLANA_PRIVATE_KEY` with your private key as the value.If you’re using the devnet or testnet, you can get test SOL from the [Solana Faucet](https://faucet.solana.com/).

#### Step 1: Set Up Your Development Environment

-   Typescript
    
-   Python
    

Ensure that you have Node.js 18+ installed:

```
node --version  # Should be 18+
npm --version   # Should be 9.7.2+

```

Clone and set up the repository:

```
# Clone the repository
git clone https://github.com/coinbase/agentkit.git
# Navigate to the root of the typescript monorepo
cd agentkit/typescript
# Install dependencies
npm install
# Build the packages locally
npm run build
# Navigate to the langchain-cdp-chatbot example or the langchain-solana-chatbot
cd examples/langchain-cdp-chatbot

```

Ensure that you have Python 3.10+ and Poetry installed:

```
python --version  # Should be 3.10+
poetry --version  # Make sure Poetry is installed

```

Clone and navigate to the example directory:

```
# Clone the repository
git clone https://github.com/coinbase/agentkit.git
# Navigate to the chatbot-python example
cd agentkit/python/examples/langchain-cdp-chatbot

```

#### Step 2: Configure Environment Variables

-   Typescript
    
-   Python
    

Copy the example environment file and configure your variables:

Copy the example environment file and configure your variables:

```
# Copy the example environment file
cp .env.example .env
# Edit the .env file with your credentials:
# CDP_API_KEY_NAME=your_cdp_key_name
# CDP_API_KEY_PRIVATE_KEY=your_cdp_private_key
# OPENAI_API_KEY=your_openai_key
# NETWORK_ID=base-sepolia  # Optional, defaults to base-sepolia. On Solana, this can be "solana-mainnet", "solana-devnet" (default), or "solana-testnet"

```

#### Step 3: Run the Agent

-   Typescript
    
-   Python
    

```
# Run the chatbot
npm run start

```

```
# Install dependencies
poetry install
# Run the chatbot
make run

```

**Common Issues**

-   If you’re trying to switch networks and your agent will not change, try renaming the `wallet_data.txt` file. Each network requires a new wallet, and if the program identifies a previously-created wallet it will not create the new one on the new network.

### Adding Agent Functionality

Extend your agent with chat capabilities. To add more functionality, see the [agent actions](https://developer.chrome.com/agent-kit/core-concepts/agents-actions) guide.

### Testing Your Agent

Try these example interactions:

```
You: What is your wallet address?
You: transfer .001 ETH to 0x4c8bbcfc6DaE447228FcbB220C1DD4cae623EaaF
You: Register a basename for yourself that represents your identity

```

## Eliza Framework

[Eliza](https://github.com/elizaOS/eliza) is a framework for building AI agents with a focus on simplicity and extensibility. For a detailed walkthrough, see our [video tutorial](https://www.youtube.com/live/DlRR1focAiw).

```
npx create-agentkit-app my-agent
cd my-agent
cp .env.example .env
# edit .env file with your own values
pnpm install
pnpm start

```

## Vercel AI SDK

[Vercel AI SDK](https://sdk.vercel.ai/docs/introduction) is a library for building AI-powered applications with React and JavaScript/TypeScript. Our implementation demonstrates creating a terminal-style chatbot with access to CDP AgentKit actions.

### Prerequisites

#### Checking Node Version

Before using the example, ensure that you have Node.js 18 or higher installed. You can check your Node version by running:

If you don’t have the correct version, you can install it using [nvm](https://github.com/nvm-sh/nvm):

#### API Keys

You’ll need the following API keys:

-   [CDP API Key](https://portal.cdp.coinbase.com/access/api)
-   [OpenAI API Key](https://platform.openai.com/docs/quickstart#create-and-export-an-api-key)

Once you have them, rename the `.env-local` file to `.env` and set the API keys to their corresponding environment variables:

-   `CDP_API_KEY_NAME`
-   `CDP_API_KEY_PRIVATE_KEY`
-   `OPENAI_API_KEY`

### Setting Up the Example

Clone the repository and navigate to the example directory:

```
# Clone the repository
git clone https://github.com/coinbase/agentkit.git
cd agentkit
# Install dependencies and build packages
npm install
npm run build
# Navigate to the example directory
cd typescript/examples/vercel-ai-sdk-cdp-chatbot
# Start the chatbot
npm start

```

### Testing Your Agent

Try these example interactions:

```
You: What is your wallet address?
You: Transfer a portion of your ETH to a random address
You: What is the price of BTC?
You: Deploy an NFT that will go super viral!
You: Deploy an ERC-20 token with total supply 1 billion

```

For more detailed documentation on using Vercel AI SDK with AgentKit, see the [Vercel AI SDK integration guide](https://developer.chrome.com/agent-kit/core-concepts/vercel-ai-sdk).

## Model Context Protocol (MCP)

The [Anthropic Model Context Protocol (MCP)](https://github.com/modelcontextprotocol/sdk) is a standardized protocol designed to facilitate structured interactions between AI models and external tools or APIs. This example demonstrates how to set up an MCP server integrated with AgentKit, allowing Claude Desktop to access the full set of CDP AgentKit actions.

### Prerequisites

#### Checking Node Version

Before using the example, ensure that you have Node.js 18 or higher installed. You can check your Node version by running:

If you don’t have the correct version, you can install it using [nvm](https://github.com/nvm-sh/nvm):

#### API Keys

You’ll need the following API key:

-   [CDP API Key](https://portal.cdp.coinbase.com/access/api)

You’ll need to configure the Claude Desktop config file with your CDP API keys. Copy the contents from `claude_desktop_config.json` to your Claude Desktop config file and update the following:

1.  Update the `args` path to match the location of your built `index.js` file.
2.  Set your CDP API keys in the `env` section:
    -   `CDP_API_KEY_NAME`
    -   `CDP_API_KEY_PRIVATE_KEY`

Then, navigate to the `claude_desktop_config.json` file found in your Claude Desktop app’s settings and update its contents to match the contents of our provided `claude_desktop_config.json` file.

### Setting Up the Example

Clone the repository and navigate to the example directory:

```
# Clone the repository
git clone https://github.com/coinbase/agentkit.git
cd agentkit
# Install dependencies and build packages
npm install
npm run build
# Navigate to the MCP example directory
cd typescript/examples/model-context-protocol-cdp-server

```

Configure your Claude Desktop by updating the `claude_desktop_config.json` file with your CDP API keys and the correct path to your built `index.js` file. To use the chatbot, simply open Claude Desktop after configuring your API keys. The MCP server will run automatically when you interact with Claude.

### Testing Your Agent

Try these example interactions in Claude Desktop:

```
Transfer a portion of your ETH to a random address
What is the price of BTC?
Deploy an NFT that will go super viral!
Deploy an ERC-20 token with total supply 1 billion

```

For more detailed documentation on using MCP with AgentKit, see the [Model Context Protocol extension information](https://developer.chrome.com/agent-kit/core-concepts/model-context-protocol).