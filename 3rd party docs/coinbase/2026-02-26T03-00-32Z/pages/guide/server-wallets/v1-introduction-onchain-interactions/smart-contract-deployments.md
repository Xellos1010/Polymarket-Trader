# smart contract deployments

The CDP SDK supports deploying custom contracts alongside ERC-20s (fungible tokens), ERC-721s (NFTs), and ERC-1155s (MultiTokens). If you’d like to see support for additional contracts, contact us in the [CDP Discord](https://discord.com/invite/cdp).

-   **Agent-generated Contracts**: Allow your agents to independently create and programmatically deploy smart contracts, enabling features like:
    -   Escrow contracts
    -   Single-use contracts for complex DeFi operations
    -   Hyper-personalized apps
-   **Expansive Gaming Ecosystem**: Create autonomous in-game currencies and items.
-   **NFT-based Verification for Photos**: Create a camera app that mints an NFT upon photo capture to prove its provenance.
-   **Loyalty Programs**: Introduce a loyalty program for your users that can be used across your platform and on others.
-   **AI-Generated Game Assets**: Use fungible and non-fungible tokens alongside artificial intelligence to allow users to build their own characters and assets.

## Examples

### Deploying an Arbitrary Contract

To deploy an arbitrary contract, first write a Solidity smart contract, then get the contract in the [input JSON format](https://docs.soliditylang.org/en/v0.8.20/using-the-compiler.html#input-description). There are three ways to do this:

1.  **Remix (web-based, recommended):** Use the web-based [Remix IDE](https://remix.ethereum.org/) to compile your Solidity contract (Ctrl + S within the .sol file), then in the file explorer navigate to the ‘build-info’ folder to find your Solidity input JSON format. Grab the first portion of the JSON file, going from line 1 to the end bracket before the “output” property.
2.  **Foundry (best for complex, production-ready contracts):** In traditional smart contract development, toolchains like [Foundry](https://book.getfoundry.sh/) are great for production-ready testing and development. Foundry has a command to build and compile contracts in the JSON format:

```
forge build --print-compiler-input > compiler_input.json

```

3.  **AI-generated input JSON (simplest contracts):** Have AI generate Solidity code in the input JSON format; [AgentKit](https://developer.chrome.com/agent-kit/welcome) agents are prompted with some additional context on how to build them, and is generally accurate on simple smart contracts. Due to limitations with current models, AI may not be able to generate complex multi-file architectures.

Next, execute the following command to compile and deploy the contract:

The contract code will be automatically verified and available for viewing on [Etherscan’s](https://etherscan.io/) family of blockchain explorers.

### Deploying an ERC-20

[ERC-20 tokens](https://docs.openzeppelin.com/contracts/5.x/erc20) are the most common type of fungible token on Ethereum. Interacting with the contract is done through the [Transfer API](https://developer.chrome.com/server-wallets/v1/concepts/transfers) for simple transfers, or with the `invokeContract` function for other calls. All [standard ERC-20 functions](https://docs.openzeppelin.com/contracts/5.x/api/token/erc20#ERC20) are supported. Below is an example of how to call a deployed ERC20 contract. Tokens can be created and interacted with by doing the following:

### Deploying an ERC-721

[ERC-721](https://docs.openzeppelin.com/contracts/5.x/erc721) is the standard for non-fungible tokens on Ethereum. The URI is the location of the metadata for the NFT. To properly interact with marketplaces, the URI must be a [valid JSON file](https://docs.opensea.io/docs/metadata-standards). The creation and interaction process is similar to fungible tokens:

### Deploying an ERC-1155

[ERC-1155](https://docs.openzeppelin.com/contracts/5.x/erc1155) is the standard for multi-token fungible tokens on Ethereum. Instead of minting one token at a time, you can mint multiple tokens in a single transaction with the same metadata. [This guide](https://github.com/ethereum/ercs/blob/master/ERCS/erc-1155.md#erc-1155-metadata-uri-json-schema) explains how to define the metadata properly. Two mint methods are supported: `mint` and `mintBatch`. `mint` requires a single `to` address and `mintBatch` requires an array of `to` addresses and an array of `values` of the same length.

## Supported Methods

### ERC-20 Functions

**Inherits:** [ERC20](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/release-v5.0/contracts/token/ERC20/ERC20.sol)

#### constructor

Constructor to initialize the ERC20 token with a name, symbol, and initial supply. The entire initial supply is assigned to the deployer of the contract.

```
constructor(string memory name, string memory symbol, uint256 amount) ERC20(name, symbol);

```

### ERC-721 Functions

**Inherits:** [ERC721AQueryable](https://github.com/chiru-labs/ERC721A/blob/main/contracts/extensions/ERC721AQueryable.sol), [Ownable2Step](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/release-v5.0/contracts/access/Ownable2Step.sol)

#### constructor

Constructor to initialize the token with name, symbol, base URI, and deployer address

```
constructor(string memory name, string memory symbol, string memory baseURI)
    ERC721A(name, symbol)
    Ownable(msg.sender);

```

**Parameters**

Name

Type

Description

`name`

`string`

The name of the token

`symbol`

`string`

The symbol of the token

`baseURI`

`string`

The base URI for the token metadata

#### mint

Mint a single token to a specified address *Only the contract owner can call this function*

```
function mint(address to) external onlyOwner;

```

**Parameters**

Name

Type

Description

`to`

`address`

The address to mint the token to

#### mint

Mint a single token to a specified address with data *Only the contract owner can call this function*

```
function mint(address to, bytes memory data) external onlyOwner;

```

**Parameters**

Name

Type

Description

`to`

`address`

The address to mint the token to

`data`

`bytes`

The data to pass to the minted token

#### mint

Mint a specified quantity of tokens to a specified address *Only the contract owner can call this function*

```
function mint(address to, uint8 quantity) external onlyOwner;

```

**Parameters**

Name

Type

Description

`to`

`address`

The address to mint tokens to

`quantity`

`uint8`

The number of tokens to mint

#### mint

Mint a specified quantity of tokens to a specified address with data *Only the contract owner can call this function*

```
function mint(address to, uint8 quantity, bytes memory data) external onlyOwner;

```

**Parameters**

Name

Type

Description

`to`

`address`

The address to mint tokens to

`quantity`

`uint8`

The number of tokens to mint

`data`

`bytes`

The data to pass to the minted tokens

### ERC-1155 Functions

**Inherits:** [ERC1155Supply](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/release-v5.0/contracts/token/ERC1155/extensions/ERC1155Supply.sol), [Ownable2Step](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/release-v5.0/contracts/access/Ownable2Step.sol)

#### constructor

Constructs an ERC1155 token with a URI, owned by the deployer of the contract

```
constructor(string memory uri) ERC1155(uri) Ownable(msg.sender);

```

**Parameters**

Name

Type

Description

`uri`

`string`

The URI for all the token metadata, should be of the format “[https://token-cdn-domain/{id}.json](https://token-cdn-domain/%7Bid%7D.json)”

#### mint

Mint a new token which can be fungible or non-fungible. Non-fungible tokens have a unique ID with a total supply of 1

```
function mint(address to, uint256 id, uint256 value) external onlyOwner;

```

**Parameters**

Name

Type

Description

`to`

`address`

The address to receive the minted tokens

`id`

`uint256`

The ID of the token to mint

`value`

`uint256`

The amount of tokens to mint

#### mint

Mint a new token which can be fungible or non-fungible. Non-fungible tokens have a unique ID with a total supply of 1

```
function mint(address to, uint256 id, uint256 value, bytes memory data) external onlyOwner;

```

**Parameters**

Name

Type

Description

`to`

`address`

The address to receive the minted tokens

`id`

`uint256`

The ID of the token to mint

`value`

`uint256`

The amount of tokens to mint

`data`

`bytes`

Additional data with no specified format, to be passed to the receiver contract

#### mintBatch

Mint a batch of new tokens which can be fungible or non-fungible. Non-fungible tokens have a unique ID with a total supply of 1

```
function mintBatch(address to, uint256[] memory ids, uint256[] memory values) external onlyOwner;

```

**Parameters**

Name

Type

Description

`to`

`address`

The address to receive the minted tokens

`ids`

`uint256[]`

The IDs of the tokens to mint

`values`

`uint256[]`

The amounts of tokens to mint, must be the same length as `ids`

#### mintBatch

Mint a batch of new tokens which can be fungible or non-fungible. Non-fungible tokens have a unique ID with a total supply of 1

```
function mintBatch(address to, uint256[] memory ids, uint256[] memory values, bytes memory data) external onlyOwner;

```

**Parameters**

Name

Type

Description

`to`

`address`

The address to receive the minted tokens

`ids`

`uint256[]`

The IDs of the tokens to mint

`values`

`uint256[]`

The amounts of tokens to mint, must be the same length as `ids`

`data`

`bytes`

Additional data with no specified format, to be passed to the receiver contract