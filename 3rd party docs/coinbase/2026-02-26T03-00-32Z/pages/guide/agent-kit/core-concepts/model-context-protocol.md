# model context protocol

The [Anthropic Model Context Protocol (MCP)](https://github.com/modelcontextprotocol/) is a standardized protocol designed to facilitate structured interactions between AI models and external tools or APIs. [This extension](https://github.com/coinbase/agentkit/blob/main/typescript/framework-extensions/model-context-protocol/README.md) integrates MCP with AgentKit, enabling AI agents to seamlessly perform onchain actions.

## Why Use MCP with AgentKit?

Integrating MCP with AgentKit provides several key benefits:

-   **Standardized Protocol**: MCP provides a structured, standardized way for AI models to interact with external tools, ensuring consistency and reliability.
-   **Onchain Capabilities**: Enables AI agents to perform blockchain operations such as token transfers, smart contract interactions, and more.
-   **Flexible Integration**: Easily integrates with existing agent workflows and infrastructure.

## Installation

Install the required packages using npm:

```
npm install @coinbase/agentkit-model-context-protocol @coinbase/agentkit @modelcontextprotocol/sdk

```

## Prerequisites

Before you begin, ensure you have the following:

-   [CDP API Key](https://portal.cdp.coinbase.com/access/api)
-   Node.js 18 or higher ([install using nvm](https://github.com/nvm-sh/nvm))

Check your Node.js version:

If needed, install the latest Node.js version:

## Basic Usage

Here’s a basic example demonstrating how to set up an MCP server integrated with AgentKit:

```
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { CallToolRequestSchema, ListToolsRequestSchema } from "@modelcontextprotocol/sdk/types.js";
import { getMcpTools } from "@coinbase/agentkit-model-context-protocol";
import { AgentKit } from "@coinbase/agentkit";
// Initialize AgentKit with your CDP API keys
const agentKit = await AgentKit.from({
  cdpApiKeyName: process.env.CDP_API_KEY_NAME,
  cdpApiKeyPrivateKey: process.env.CDP_API_KEY_PRIVATE_KEY,
});
// Retrieve MCP-compatible tools from AgentKit
const { tools, toolHandler } = await getMcpTools(agentKit);
// Create MCP server instance
const server = new Server(
  {
    name: "agentkit",
    version: "0.1.0",
  },
  {
    capabilities: {
      tools: {},
    },
  },
);
// Handle requests to list available tools
server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools,
  };
});
// Handle requests to execute specific tools
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  try {
    return toolHandler(request.params.name, request.params.arguments);
  } catch (error) {
    throw new Error(`Tool ${request.params.name} failed: ${error}`);
  }
});
// Set up standard input/output transport for the server
const transport = new StdioServerTransport();
// Connect the server to the transport
await server.connect(transport);

```

## Troubleshooting

### Common Issues

#### Tool Execution Errors

If you encounter errors during tool execution, verify the following:

-   Ensure your CDP API keys are correctly set in environment variables.
-   Confirm that the requested tool is supported by your AgentKit instance.
-   Check for sufficient funds or permissions for blockchain transactions.

#### MCP Server Connection Issues

If the MCP server fails to connect:

-   Verify that the transport layer (stdio, HTTP, etc.) is correctly configured.
-   Check for port conflicts or permissions issues if using network-based transports.

### Debugging Tips

-   Enable verbose logging in your MCP server setup to gain insights into request handling and errors.
-   Use environment variables to toggle debug modes for detailed logs.