# ai mcp setup

## Overview

Think of MCP (Model Context Protocol) as giving your AI assistant a direct hotline to Coinbase’s documentation. Instead of the AI guessing how CDP works, it can instantly look up the correct way to build apps. **Without MCP:**

```
You: "Create a wallet connection component"
AI: *generates generic code that might not work with CDP*

```

**With MCP:**

```
You: "Create a wallet connection component" 
AI: *searches CDP docs automatically*
AI: *generates code using actual CDP components*

```

## How it helps you

When you connect MCP to your AI tools, the AI becomes a CDP expert that can:

-   **Generate accurate code** using our latest components
-   **Answer specific questions** about CDP APIs and authentication
-   **Suggest best practices** for embedded wallets, payments, and transactions
-   **Find the right documentation** when you’re stuck on implementation

## Claude

To use the CDP MCP server with Claude:

## Cursor

To connect the CDP MCP server to Cursor, you can either use the automatic connection or configure it manually:

-   Automatic Connection
    
-   Manual Configuration
    

## VS Code

The CDP MCP server can also be configured with VS Code extensions that support MCP:

1.  Install an MCP-compatible extension
2.  Add the CDP server URL: `https://docs.cdp.coinbase.com/mcp`
3.  Test the connection by querying CDP documentation

## Testing your MCP connection

Once configured, test your MCP connection by asking your AI tool:

```
"What MCP tools do you have available?"

```

You should see the CDP documentation search tool listed. Then try:

```
"Search for information about embedded wallets in the CDP documentation"

```

The AI should be able to search and return relevant CDP documentation.

## Troubleshooting

### Connection issues

**Problem:** MCP server not connecting **Solution:**

-   Verify the URL is exactly: `https://docs.cdp.coinbase.com/mcp`
-   Check your internet connection
-   Restart your AI tool after configuration

**Problem:** Search tool not available **Solution:**

-   Confirm the MCP server was added correctly
-   Try removing and re-adding the server configuration
-   Check the AI tool’s MCP support documentation

### Search issues

**Problem:** Search returns no results **Solution:**

-   Try different search terms
-   Use more general terms (e.g., “wallet” instead of “embedded wallet API”)
-   Verify the MCP connection is working

## Additional resources

-   [Model Context Protocol documentation](https://modelcontextprotocol.io/docs/tutorials/use-remote-mcp-server#connecting-to-a-remote-mcp-server)
-   [Mintlify MCP documentation](https://mintlify.com/docs/ai/model-context-protocol)

## What to read next

With MCP configured, your AI tools now have direct access to CDP documentation. Continue with:

-   **[AI Development Setup](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-development-setup)**: Configure your starter app and development environment for optimal AI-assisted development
-   **[AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)**: Learn day-to-day development practices and core workflow patterns
-   **[AI Prompting Techniques](https://developer.chrome.com/get-started/develop-with-ai/development/ai-prompting-techniques)**: Master effective prompting patterns to leverage your new MCP connection