# agents actions

Actions are grouped into action providers, which may have specific dependencies like API keys. You can find all of the action providers and actions supported by AgentKit at the following links:

-   [Python version](https://github.com/coinbase/agentkit/tree/main/python/coinbase-agentkit/coinbase_agentkit/action_providers)
-   [Node.js version](https://github.com/coinbase/agentkit/tree/master/typescript/agentkit/src/action-providers)

Generally, Node.js supports more crypto-specific actions than the Python version. By default, AgentKit supports the following actions in the ‘wallet’ action provider:

-   `get_wallet_details` - Get details about the Wallet, like the address
-   `native_transfer` - Transfer native asset between addresses
-   `get_balance` - Get the balance of the native asset

## Adding Action Provider Groupings

Adding an existing action provider to your agent is a two-step process:

1.  Import the action provider to your file
2.  Add the action provider to your AgentKit instance

## Creating an Action Provider

Action providers define the actions that an agent can take. They are created by subclassing the `ActionProvider` abstract class.

## Adding Actions to an Action Provider

-   Node.js
    
-   Python
    

Actions are defined as instance methods on the action provider class with the `@CreateAction` decorator. Actions can use a wallet provider or not and always return a Promise that resolves to a string.**Required Typescript Compiler Options**Creating actions with the @CreateAction decorator requires the following compilerOptions to be included in your project’s tsconfig.json.

```
{
    "compilerOptions": {
        "experimentalDecorators": true,
        "emitDecoratorMetadata": true
    }
} 

```

**Steps to create an action**

1.  Define the action schema. Action schemas are defined using the zod library.

```
import { z } from "zod";
export const MyActionSchema = z.object({
  myField: z.string(),
});

```

2.  Define the action implementation.

```
import { ActionProvider, WalletProvider, Network, CreateAction } from "@coinbase/agentkit";
class MyActionProvider extends ActionProvider<WalletProvider> {
    constructor() {
        super("my-action-provider", []);
    }
    @CreateAction({
        name: "my-action",
        description: "My action description",
        schema: MyActionSchema,
    })
    async myAction(args: z.infer<typeof MyActionSchema>): Promise<string> {
        return args.myField;
    }
    supportsNetwork = (network: Network) => true;
}
export const myActionProvider = () => new MyActionProvider();

```

**Adding Actions to your Action Provider that use a Wallet Provider**Actions that use a wallet provider can be defined as instance methods on the action provider class with the `@CreateAction` decorator that have a `WalletProvider` as the first parameter.

```
class MyActionProvider extends ActionProvider<WalletProvider> {
    constructor() {
        super("my-action-provider", []);
    }
    @CreateAction({
        name: "my-action",
        description: "My action description",
        schema: MyActionSchema,
    })
    async myAction(walletProvider: WalletProvider, args: z.infer<typeof MyActionSchema>): Promise<string> {
        return walletProvider.signMessage(args.myField);
    }
    supportsNetwork = (network: Network) => true;
}

```

**Adding an Action Provider to your AgentKit instance**

```
const agentKit = new AgentKit({
  cdpApiKeyName: "CDP API KEY NAME",
  cdpApiKeyPrivate: "CDP API KEY PRIVATE KEY",
  actionProviders: [myActionProvider()],
});

```

Actions are defined using the `@create_action` decorator. They can optionally use a wallet provider and must return a string.**Steps to create an action**

1.  Define the action schema using Pydantic:

```
from pydantic import BaseModel
class MyActionSchema(BaseModel):
    my_field: str

```

2.  Define the action:

```
from coinbase_agentkit import ActionProvider, WalletProvider, Network, create_action
class MyActionProvider(ActionProvider[TWalletProvider]):
    def __init__(self):
        super().__init__("my-action-provider", [])
    @create_action(
        name="my-action",
        description="My action description",
        schema=MyActionSchema
    )
    def my_action(self, args: dict[str, Any]) -> str:
        return args["my_field"]
    def supports_network(self, network: Network) -> bool:
        return True
def my_action_provider():
    return MyActionProvider()

```

**Adding Actions that use a Wallet Provider**Actions that need access to a wallet provider can include it as their first parameter:

```
class MyActionProvider(ActionProvider[TWalletProvider]):
    @create_action(
        name="my-action",
        description="My action description",
        schema=MyActionSchema
    )
    def my_action(self, wallet_provider: WalletProvider, args: dict[str, Any]) -> str:
        return wallet_provider.sign_message(args["my_field"])

```

**Adding an Action Provider to your AgentKit instance**

```
agent_kit = AgentKit.from_options(AgentKitOptions(
    cdp_api_key_name="CDP API KEY NAME",
    cdp_api_key_private="CDP API KEY PRIVATE KEY",
    action_providers=[my_action_provider()]
))

```

For actions to be made available for any agent, we welcome open-source contributions to AgentKit for adding more actions! Please see our [contribution guide](https://github.com/coinbase/agentkit/blob/main/CONTRIBUTING.md) for more information.