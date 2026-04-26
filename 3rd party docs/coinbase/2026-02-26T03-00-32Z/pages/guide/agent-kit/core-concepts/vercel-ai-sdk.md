# vercel ai sdk

## Introduction to Vercel AI SDK

[Vercel AI SDK](https://sdk.vercel.ai/docs/introduction) is a library for building AI-powered applications with React and JavaScript/TypeScript. It provides a set of tools and utilities that make it easy to build applications that use AI models, including:

-   Streaming responses from AI models
-   Managing AI model state
-   Building chat interfaces
-   Handling tool usage by AI models

[This extension](https://github.com/coinbase/agentkit/tree/main/typescript/framework-extensions/vercel-ai-sdk) allows you to easily integrate AgentKit’s onchain capabilities with Vercel AI SDK, enabling you to build web applications with AI agents that can interact with blockchain networks and a crypto wallet.

## Why Use Vercel AI SDK with AgentKit

Vercel AI SDK is particularly well-suited for building web applications with AI capabilities. When combined with AgentKit, it enables you to:

-   **Build web-based AI agents**: Create web applications with AI agents that can interact onchain
-   **Stream AI responses**: Provide a better user experience with streaming responses from AI models
-   **Leverage React components**: Use Vercel AI SDK’s React components to build chat interfaces
-   **Deploy on Vercel**: Easily deploy your application on Vercel’s platform

## Installation

For a single command to install all necessary dependencies, run:

```
npm install @coinbase/agentkit-vercel-ai-sdk @coinbase/agentkit ai @ai-sdk/openai

```

To break it down, this package is:

```
npm install @coinbase/agentkit-vercel-ai-sdk

```

This package is used alongside AgentKit and AI SDK, so these will need to be installed as well.

```
npm install @coinbase/agentkit ai

```

Finally, install the model provider you want to use. For example, to use OpenAI, install the `@ai-sdk/openai` package. See [here](https://sdk.vercel.ai/docs/foundations/providers-and-models#ai-sdk-providers) for a list of supported model providers.

```
npm install @ai-sdk/openai

```

## Basic Usage

The main export of this package is the `getVercelAITools` function. This function takes an AgentKit instance and returns an object containing the tools for the AgentKit agent. This object can then be passed to AI SDK. Here’s a snippet of code that shows how to use the `getVercelAITools` function to get the tools for the AgentKit agent.

###### chatbot.ts

```
import { getVercelAITools } from "@coinbase/agentkit-vercel-ai-sdk";
import { AgentKit } from "@coinbase/agentkit";
import { generateText } from "ai";
import { openai } from "@ai-sdk/openai";
// Get your Coinbase Developer Platform API key from the Portal: https://portal.cdp.coinbase.com/
// Or, check out one of the other supported wallet providers: https://github.com/coinbase/agentkit/tree/main/typescript/agentkit
const agentKit = await AgentKit.from({
  cdpApiKeyName: process.env.CDP_API_KEY_NAME,
  cdpApiKeyPrivateKey: process.env.CDP_API_KEY_PRIVATE_KEY,
});
const tools = await getVercelAITools(agentKit);
// There are multiple methods to generate text with AI SDK.
// See here for more information: https://sdk.vercel.ai/docs/ai-sdk-core/generating-text
const { text } = await generateText({
  model: openai("gpt-4o-mini"), // Make sure to have OPENAI_API_KEY set in your environment variables
  system: "You are an onchain AI assistant with access to a wallet.",
  prompt: "Print wallet details",
  tools,
  // Allow multi-step tool usage
  // See: https://sdk.vercel.ai/docs/foundations/agents#multi-step-tool-usage
  maxSteps: 10,
});
console.log(text);

```

For a full example, see the [AgentKit AI SDK Chatbot Example](https://github.com/coinbase/agentkit/tree/main/typescript/examples/vercel-ai-sdk-smart-wallet-chatbot).

### Using Different Model Providers

Vercel AI SDK supports multiple model providers. Here’s how to use different providers:

## Troubleshooting

### Common Issues

#### Tool Execution Errors

If you encounter errors during tool execution, check the following:

-   Ensure your CDP API keys are correctly set in environment variables
-   Verify that the network you’re using is supported by the action provider
-   Check that you have sufficient funds for any transactions

#### Model Provider Issues

If you’re having issues with the model provider:

-   Ensure you have the correct API key set for your model provider
-   Check that you’re using a model that supports tool usage
-   Verify that you’ve installed the correct model provider package

### Debugging Tips

-   Set `debug: true` in the options for `getVercelAITools` to see more detailed logs
-   Use the `maxSteps` parameter to limit the number of tool calls the model can make
-   Check the network tab in your browser’s developer tools to see the requests being made