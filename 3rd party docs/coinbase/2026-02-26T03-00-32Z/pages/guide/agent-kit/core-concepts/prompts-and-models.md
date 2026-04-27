# prompts and models

## Changing Models

AgentKit is model-agnostic, and changing which model depends on which framework you are using.

-   LangChain
    
-   Eliza
    

These LangChain guides in [Python](https://python.langchain.com/docs/integrations/providers/) and [Node.js](https://js.langchain.com/docs/integrations/chat/) will help you change the model for your LangChain agent.For the Node.js version, you can change the model similarly.For any model that can be accessed through the OpenAI standard, you can change the BaseURL as specified by your model provider’s documentation. You can also specify [additional configuration options](https://python.langchain.com/api_reference/openai/llms/langchain_openai.llms.base.OpenAI.html) such as `maxTokens`, `temperature`, and `topP`. Here are some examples:

-   AWS Bedrock (Python)
    
-   Venice (Node.js)
    
-   Hyperbolic (Node.js)
    
-   Claude (Python)
    
-   xAI (Python)
    

```
# requires AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY as env variables
from langchain_aws import ChatBedrockConverse
llm = ChatBedrockConverse(
    model_id="us.amazon.nova-pro-v1:0", # Change to any Bedrock Model ID 
    max_tokens=1024,
    # other params...
    )

```

```
    import { ChatOpenAI } from "@langchain/openai";
    const llm = new ChatOpenAI({
      model: "function_calling_default",
      apiKey: process.env.VENICE_API_KEY, // you can input your API key in plaintext, but this is not recommended 
      configuration: {
        baseURL: "https://api.venice.ai/api/v1",
      },
    });

```

```
import { ChatOpenAI } from "@langchain/openai";
const llm = new ChatOpenAI({
  modelName: "meta-llama/Llama-3.3-70B-Instruct",
  apiKey: process.env.HYPERBOLIC_API_KEY, // you can input your API key in plaintext, but this is not recommended 
  configuration: {
    baseURL: "https://api.hyperbolic.xyz/v1",
    defaultHeaders: {
      "Content-Type": "application/json"
    }
  },
  maxTokens: 2048, // specifies the maximum number of tokens to generate
  temperature: 0.7, // specifies the randomness of the output
  topP: 0.9 // specifies the top-p sampling parameter
});

```

```
# requires ANTHROPIC_API_KEY as env variable
from langchain_anthropic import ChatAnthropic
llm = ChatAnthropic(
    model="claude-3-5-sonnet-20240620", //all below params are optional
    temperature=0,
    max_tokens=1024,
    timeout=None,
    max_retries=2,
    # other params...
)

```

```
# requires XAI_API_KEY as env variable
from langchain_openai import ChatOpenAI
llm = ChatOpenAI(model="grok-2-latest", base_url = "https://api.x.ai/v1")

```

The [Eliza quickstart](https://elizaos.github.io/eliza/docs/quickstart/#choose-your-model) explains how to choose your model. Generally, the steps are to specify model name in your character JSON file and include the appropriate API keys in the `.env` file.

## Prompting Guide

The prompts you use for your agent can significantly impact its personality and performance. Here are some tips to help you get started:

-   **Agent State Modifier:** The agent state modifier is the first thing the agent sees. It is used to set the agent’s initial state and can be used to set the agent’s personality. If there are specific instructions for the agent, they should be included here. However, do not treat it as as a place for guardrails, as a well-written prompt can get around text-based instructions.
-   **Action Prompt:** Every action has an associated prompt that gives the agent context on how to use the action. If there are specific things an agent should know about the parameters, how to ask the user for clarification, or assumptions to make in default cases when the user does not mention a specific parameter, this prompt is a great place to include them.