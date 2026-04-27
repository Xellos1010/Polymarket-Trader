# smart contract interactions

Server Wallet supports smart contract [writes](#smart-contract-writes), [reads](#smart-contract-reads), [smart contract token deployments](https://developer.chrome.com/server-wallets/v1/introduction/onchain-interactions/smart-contract-deployments), and [message signing](https://developer.chrome.com/server-wallets/v1/introduction/onchain-interactions/message-signing). This allows developers to unlock a wide range of onchain activities while abstracting complexity from end-users.

## Example Use Cases

-   **Telegram bots for anything:** Beyond swapping and sending, developers can create Telegram bots for any smart contract functionality.
-   **Multi-step smart contract interactions:** Create complex, programmatic DeFi applications that abstract difficult steps from your user. Truly programmatic money legos.

## Smart Contract Writes

Smart contracts live on a blockchain and programmatically enforce and execute the terms of an agreement. Integrating smart contracts with API Wallets enables:

-   Complex, multi-step operations triggered by a single user action.
-   Offchain actions with onchain verification and execution.
-   Limitless interactions with any onchain app or protocol.

## Examples

### Invoking (Calling) a Contract

We natively support the following standard contract interfaces, without you providing an ABI:

-   [ERC-20](https://docs.openzeppelin.com/contracts/5.x/erc20) (fungible tokens)
-   [ERC-721](https://docs.openzeppelin.com/contracts/5.x/erc721) (NFTs)
-   [ERC-1155](https://docs.openzeppelin.com/contracts/3.x/erc1155) (Multi token standard)

However, any other contracts require providing the ABI. If you are interested in cached ABIs for more common smart contracts, reach out to us in the **#wallet-api** channel of the [CDP Discord](https://discord.com/invite/cdp). A contract [ABI](https://docs.soliditylang.org/en/latest/abi-spec.html) (Application Binary Interface) is the way to interact with a smart contract, both externally and contract-to-contract. It is a file defining the various methods and parameters of the contract. The SDK also supports payable transactions, which are transactions that send ETH to the contract.

### Transfer an ERC-721 NFT

Here’s an example of using `invokeContract` to transfer an ERC-721 NFT:

### Invoking an arbitrary smart contract

This is an example of using `invokeContract` on an arbitrary contract, where the ABI must be explicitly provided.

### Viewing transaction receipts and logs

The CDP SDK supports parsing transaction receipts and logs from contract invocations. Here’s an example.

## Smart Contract Reads

Smart contract support being queried and returning state data without explicitly modifying their state. Readable functions are useful for retrieving current contract state, calculating values, or checking conditions before performing transactions. Including an ABI is optional, but without it, the CDP SDK will not be able to determine the return type of the function, and the return value will not be parsed. With an ABI, Node.js types are inferred and can be used directly. In Python, the SDK will automatically convert to the expected type at runtime.

### Examples

Here’s an example of reading from a smart contract using the CDP SDK: