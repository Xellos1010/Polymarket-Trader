# ai docs integration

## Overview

CDP documentation includes a contextual menu that provides quick access to AI-optimized content and direct integrations with popular AI tools. This feature makes it easy to get AI assistance without leaving the documentation.

On any CDP documentation page, select the **Copy page** dropdown found next to the document header to access AI integration options:

## Available options

### Page content options

**Copy page**

-   Copy the current page content as standard markdown
-   **When to use:** Sharing content or basic text copying

**Copy page as Markdown for LLMs**

-   Copy page content optimized specifically for AI consumption
-   Pre-formatted context that helps AI tools understand CDP concepts
-   **When to use:** Feeding content to AI tools for analysis or questions

**View as Markdown**

-   View the current page rendered as plain text markdown
-   **When to use:** Previewing content in markdown format

### Direct AI tool integration

**Open in ChatGPT**

-   Directly open the page content in ChatGPT with pre-filled context
-   **When to use:** Quick questions about the current page

**Open in Claude**

-   Directly open the page content in Claude with pre-filled context
-   **When to use:** Detailed analysis or architectural questions about the current page

**Open in Perplexity**

-   Directly open the page content in Perplexity with pre-filled context
-   **When to use:** Research-oriented questions about the current page

### MCP server integration

**Copy MCP Server**

-   Quick access to copy the MCP server URL for manual configuration
-   **When to use:** Custom setups or other AI tools that support MCP

**Connect to Cursor**

-   One-click installation of the CDP MCP server in Cursor IDE
-   **When to use:** Setting up ongoing AI development workflow in Cursor

**Connect to VS Code**

-   One-click installation of the CDP MCP server in VS Code
-   **When to use:** Setting up ongoing AI development workflow in VS Code

### Additional context

**Full Documentation Context**

-   Access `llms-full.txt` - the entire CDP documentation in a single file optimized for AI tools
-   URL: `https://docs.cdp.coinbase.com/llms-full.txt`
-   **When to use:** When AI tools need comprehensive context about the entire CDP platform

## How to use

### For direct AI tool access

1.  Navigate to any CDP documentation page
2.  Click the **Copy page** dropdown
3.  Select **Open in ChatGPT**, **Open in Claude**, or **Open in Perplexity**
4.  The AI tool will open with the page content pre-loaded

### For manual AI tool usage

1.  Navigate to any CDP documentation page
2.  Click the **Copy page** dropdown
3.  Select **Copy page as Markdown for LLMs**
4.  Paste the content into your preferred AI tool and ask questions

### For comprehensive context

-   Navigate to `https://docs.cdp.coinbase.com/llms-full.txt`
-   Copy the URL or content for comprehensive CDP documentation context
-   Use when questions span multiple areas of CDP documentation

### For MCP server setup

1.  Navigate to any CDP documentation page
2.  Click the **Copy page** dropdown
3.  Select **Connect to Cursor**, **Connect to VSCode**, or **Copy MCP Server URL**
4.  For detailed setup instructions, see the **[MCP Setup Guide](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-mcp-setup)**

## Best practices

**Provide Context**

-   Include relevant code snippets when asking AI questions
-   Mention your specific use case (embedded wallets, DeFi, etc.)
-   Reference the CDP components you’re working with
-   For complex questions spanning multiple topics, consider using `llms-full.txt` for comprehensive context

**Combine with MCP Server**

-   Use the context menu for quick AI questions and one-off tasks
-   Use the [MCP Setup Guide](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-mcp-setup) for ongoing development with full AI tool integration
-   Both approaches work together for optimal AI assistance

## What to read next

-   **[MCP Setup Guide](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-mcp-setup)**: Set up direct AI tool access to CDP documentation for enhanced code generation
-   **[AI Development Setup](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-development-setup)**: Configure your starter app and development environment
-   **[AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)**: Learn day-to-day development practices with AI assistance