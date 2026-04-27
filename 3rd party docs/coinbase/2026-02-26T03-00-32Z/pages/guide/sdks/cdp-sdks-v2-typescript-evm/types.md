# types

## Interfaces

### CreateServerAccountOptions

Defined in: [src/client/evm/evm.types.ts:339](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L339) Options for creating an EVM server account.

#### Properties

##### accountPolicy?

```
optional accountPolicy: string;

```

Defined in: [src/client/evm/evm.types.ts:343](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L343) The policy ID to apply to the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:345](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L345) The idempotency key.

##### name?

Defined in: [src/client/evm/evm.types.ts:341](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L341) The name of the account.

* * *

### CreateSmartAccountOptions

Defined in: [src/client/evm/evm.types.ts:502](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L502) Options for creating an EVM smart account.

#### Properties

##### enableSpendPermissions?

```
optional enableSpendPermissions: boolean;

```

Defined in: [src/client/evm/evm.types.ts:510](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L510) The flag to enable spend permissions.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:506](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L506) The idempotency key.

##### name?

Defined in: [src/client/evm/evm.types.ts:508](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L508) The name of the account.

##### owner

Defined in: [src/client/evm/evm.types.ts:504](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L504) The owner of the account.

* * *

### CreateSwapQuoteOptions

Defined in: [src/client/evm/evm.types.ts:116](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L116) Options for creating a swap quote between two tokens on an EVM network.

#### Properties

##### fromAmount

Defined in: [src/client/evm/evm.types.ts:124](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L124) The amount to send in atomic units of the token.

##### fromToken

```
fromToken: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:122](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L122) The token to send (source token).

##### gasPrice?

```
optional gasPrice: bigint;

```

Defined in: [src/client/evm/evm.types.ts:132](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L132) The price per unit of gas in wei.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:136](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L136) The idempotency key.

##### network

```
network: EvmSwapsNetwork;

```

Defined in: [src/client/evm/evm.types.ts:118](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L118) The network to create a swap quote on.

##### signerAddress?

```
optional signerAddress: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:128](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L128) The address signing the swap (only needed if taker is a smart contract, i.e. for smart account swaps).

##### slippageBps?

```
optional slippageBps: number;

```

Defined in: [src/client/evm/evm.types.ts:134](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L134) The slippage tolerance in basis points (0-10000).

##### smartAccount?

```
optional smartAccount: {
  address: `0x${string}`;
  fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;
  getUserOperation: (options: Omit<GetUserOperationOptions, "smartAccount">) => Promise<UserOperation>;
  listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;
  name?: string;
  owners: EvmAccount[];
  policies: undefined | string[];
  quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;
  quoteSwap: (options: SmartAccountQuoteSwapOptions) => Promise<SmartAccountQuoteSwapResult>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;
  sendUserOperation: (options: Omit<SendUserOperationOptions<unknown[]>, "smartAccount">) => Promise<SendUserOperationReturnType>;
  signTypedData: (options: Omit<SignTypedDataOptions, "address"> & {
     network: KnownEvmNetworks;
  }) => Promise<`0x${string}`>;
  swap: (options: SmartAccountSwapOptions) => Promise<SmartAccountSwapResult>;
  transfer: (options: SmartAccountTransferOptions) => Promise<SendUserOperationReturnType>;
  type: "evm-smart";
  useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<EvmSmartAccountProperties, "useNetwork"> & { [K in string | number | symbol]: ({ getUserOperation: ...; sendUserOperation: ...; waitForUserOperation: ... } & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)))[K] } & { network: Network })[K] }>;
  useSpendPermission: (options: UseSpendPermissionOptions) => Promise<SendUserOperationReturnType>;
  waitForUserOperation: (options: Omit<WaitForUserOperationOptions, "smartAccountAddress">) => Promise<WaitForUserOperationReturnType>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
};

```

Defined in: [src/client/evm/evm.types.ts:130](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L130) The smart account object (required for smart account execution context only).

###### address

The smart account’s address.

###### fund()

```
fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;

```

Funds an EVM account with the specified token amount.

###### Parameters

###### options

`Omit`<`EvmFundOptions`, `"address"`\> The options for the fund operation.

###### Returns

`Promise`<`FundOperationResult`\> A promise that resolves to the fund operation result containing the transfer details.

###### Example

```
const fundOperation = await account.fund({
  network: "base",
  token: "usdc",
  amount: 1000000n,
});

```

###### getUserOperation()

```
getUserOperation: (options: Omit<GetUserOperationOptions, "smartAccount">) => Promise<UserOperation>;

```

Gets a user operation by its hash.

###### Parameters

###### options

`Omit`<[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions), `"smartAccount"`\> Parameters for getting the user operation.

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\> A promise that resolves to the user operation.

###### Example

```
const userOp = await smartAccount.getUserOperation({
  userOpHash: "0x1234567890123456789012345678901234567890",
});

```

###### listTokenBalances()

```
listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;

```

List the token balances of an account.

###### Parameters

###### options

`Omit`<`ListTokenBalancesOptions`, `"address"`\> The options for the list token balances.

###### Returns

`Promise`<`ListTokenBalancesResult`\> The result of the list token balances.

###### Example

```
const balances = await account.listTokenBalances({
  network: "base-sepolia",
});

```

###### name?

The name of the smart account.

###### owners

Array of accounts that own and can sign for the smart account (currently only supports one owner but will be extended to support multiple owners in the future).

###### policies

```
policies: undefined | string[];

```

The list of policy IDs that apply to the smart account. This will include both the project-level policy and the account-level policy, if one exists.

###### quoteFund()

```
quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;

```

Gets a quote to fund an EVM account.

###### Parameters

###### options

`Omit`<`EvmQuoteFundOptions`, `"address"`\> The options for the quote fund.

###### Returns

`Promise`<`EvmQuote`\> A promise that resolves to a Quote object containing details about the funding operation.

###### Example

```
const quote = await account.quoteFund({
  network: "base",
  token: "usdc",
  amount: 1000000n,
});

```

###### quoteSwap()

```
quoteSwap: (options: SmartAccountQuoteSwapOptions) => Promise<SmartAccountQuoteSwapResult>;

```

Creates a swap quote without executing the transaction. This is useful when you need to get swap details before executing the swap. The taker is automatically set to the smart account’s address.

###### Parameters

###### options

`SmartAccountQuoteSwapOptions` Configuration options for creating the swap quote.

###### Returns

`Promise`<`SmartAccountQuoteSwapResult`\> A promise that resolves to the swap quote or a response indicating that liquidity is unavailable.

###### Example

```
const swapQuote = await smartAccount.quoteSwap({
  network: "base",
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
});
if (swapQuote.liquidityAvailable) {
  console.log(`Can swap for ${swapQuote.toAmount} USDC`);
}

```

###### requestFaucet()

```
requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;

```

Requests funds from an EVM faucet.

###### Parameters

###### options

`Omit`<`RequestFaucetOptions`, `"address"`\> Parameters for requesting funds from the EVM faucet.

###### Returns

`Promise`<`RequestFaucetResult`\> A promise that resolves to the transaction hash.

###### Example

```
const result = await account.requestFaucet({
  network: "base-sepolia",
  token: "eth",
});

```

###### sendUserOperation()

```
sendUserOperation: (options: Omit<SendUserOperationOptions<unknown[]>, "smartAccount">) => Promise<SendUserOperationReturnType>;

```

Sends a user operation.

###### Parameters

###### options

`Omit`<`SendUserOperationOptions`<`unknown`\[\]>, `"smartAccount"`\> Parameters for sending the user operation.

###### Returns

`Promise`<`SendUserOperationReturnType`\> A promise that resolves to an object containing the smart account address, the user operation hash, and the status of the user operation.

###### Example

```
const userOp = await smartAccount.sendUserOperation({
  network: "base-sepolia",
  calls: [
    {
      to: "0x1234567890123456789012345678901234567890",
      value: parseEther("0.000001"),
      data: "0x",
    },
  ],
});

```

###### signTypedData()

```
signTypedData: (options: Omit<SignTypedDataOptions, "address"> & {
  network: KnownEvmNetworks;
}) => Promise<`0x${string}`>;

```

Signs a typed data message.

###### Parameters

###### options

`Omit`<[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions), `"address"`\> & { `network`: `KnownEvmNetworks`; } Configuration options for signing the typed data.

###### Returns

`Promise`<`` `0x${string}` ``\> A promise that resolves to the signature.

###### Example

```
const signature = await smartAccount.signTypedData({
  network: "base-sepolia",
  typedData: {
    domain: {
      name: "Test",
      chainId: 84532,
      verifyingContract: "0x0000000000000000000000000000000000000000",
    },
    types: {
      Test: [{ name: "name", type: "string" }],
    },
    primaryType: "Test",
    message: {
      name: "John Doe",
    },
  },
});

```

###### swap()

```
swap: (options: SmartAccountSwapOptions) => Promise<SmartAccountSwapResult>;

```

Executes a token swap on the specified network via a user operation. This method handles all the steps required for a swap, including Permit2 signatures if needed. The taker is automatically set to the smart account’s address.

###### Parameters

###### options

`SmartAccountSwapOptions` Configuration options for the swap.

###### Returns

`Promise`<`SmartAccountSwapResult`\> A promise that resolves to the user operation result.

###### Throws

If liquidity is not available when using inline options.

###### Examples

```
// First create a swap quote
const swapQuote = await cdp.evm.createSwapQuote({
  network: "base",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
  taker: smartAccount.address,
  signerAddress: smartAccount.owners[0].address
});
// Check if liquidity is available
if (!swapQuote.liquidityAvailable) {
  console.error("Insufficient liquidity for swap");
  return;
}
// Execute the swap
const { userOpHash } = await smartAccount.swap({
  swapQuote: swapQuote
});
console.log(`Swap executed with user op hash: ${userOpHash}`);

```

```
// Create and execute swap in one call
const { userOpHash } = await smartAccount.swap({
  network: "base",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
});
console.log(`Swap executed with user op hash: ${userOpHash}`);

```

###### transfer()

```
transfer: (options: SmartAccountTransferOptions) => Promise<SendUserOperationReturnType>;

```

Transfer an amount of a token from an account to another account.

###### Parameters

###### options

`SmartAccountTransferOptions` The options for the transfer.

###### Returns

`Promise`<`SendUserOperationReturnType`\> The user operation result.

###### Examples

```
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: 10000n, // equivalent to 0.01 USDC
  token: "usdc",
  network: "base-sepolia",
});

```

**Using parseUnits to specify USDC amount**

```
import { parseUnits } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseUnits("0.01", 6), // USDC uses 6 decimal places
  token: "usdc",
  network: "base-sepolia",
});

```

**Transfer ETH**

```
import { parseEther } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseEther("0.000001"),
  token: "eth",
  network: "base-sepolia",
});

```

**Using a contract address**

```
import { parseEther } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseEther("0.000001"),
  token: "0x4200000000000000000000000000000000000006", // WETH on Base Sepolia
  network: "base-sepolia",
});

```

**Transfer to another account**

```
const sender = await cdp.evm.createAccount({ name: "Sender" });
const receiver = await cdp.evm.createAccount({ name: "Receiver" });
const { userOpHash } = await sender.transfer({
  to: receiver,
  amount: 10000n, // equivalent to 0.01 USDC
  token: "usdc",
  network: "base-sepolia",
});

```

###### type

Identifier for the smart account type.

###### useNetwork()

```
useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<EvmSmartAccountProperties, "useNetwork"> & { [K in string | number | symbol]: ({ getUserOperation: ...; sendUserOperation: ...; waitForUserOperation: ... } & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)))[K] } & { network: Network })[K] }>;

```

A function that returns a network-scoped smart account.

###### Type Parameters

###### Network

`Network` *extends* `KnownEvmNetworks`

###### Parameters

###### network

`Network` The network name or RPC URL

###### Returns

`Promise`<{ \[K in string | number | symbol\]: (Omit<EvmSmartAccountProperties, “useNetwork”> & { \[K in string | number | symbol\]: ({ getUserOperation: …; sendUserOperation: …; waitForUserOperation: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>

###### Example

```
// For known networks, type is inferred automatically:
const baseAccount = await smartAccount.useNetwork("base");
// For custom RPC URLs with type hints (requires casting):
const typedAccount = await smartAccount.useNetwork<"base">("https://mainnet.base.org" as "base");
// For custom RPC URLs without type hints (only sendTransaction, transfer and waitForTransactionReceipt methods available):
const customAccount = await smartAccount.useNetwork("https://mainnet.base.org");

```

###### useSpendPermission()

```
useSpendPermission: (options: UseSpendPermissionOptions) => Promise<SendUserOperationReturnType>;

```

Uses a spend permission to execute a transaction via user operation. This allows the smart account to spend tokens that have been approved via a spend permission.

###### Parameters

###### options

`UseSpendPermissionOptions` Configuration options for using the spend permission.

###### Returns

`Promise`<`SendUserOperationReturnType`\> A promise that resolves to the user operation result.

###### Throws

If the network doesn’t support spend permissions via CDP API.

###### Example

```
const spendPermission = {
  account: "0x1234...", // Smart account that owns the tokens
  spender: smartAccount.address, // This smart account that can spend
  token: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE", // ETH
  allowance: parseEther("0.01"),
  period: 86400, // 1 day
  start: 0,
  end: 281474976710655,
  salt: 0n,
  extraData: "0x",
};
const result = await smartAccount.useSpendPermission({
  spendPermission,
  value: parseEther("0.001"), // Spend 0.001 ETH
  network: "base-sepolia",
});

```

###### waitForUserOperation()

```
waitForUserOperation: (options: Omit<WaitForUserOperationOptions, "smartAccountAddress">) => Promise<WaitForUserOperationReturnType>;

```

Waits for a user operation to complete or fail.

###### Parameters

###### options

`Omit`<`WaitForUserOperationOptions`, `"smartAccountAddress"`\> Parameters for waiting for the user operation.

###### Returns

`Promise`<`WaitForUserOperationReturnType`\> A promise that resolves to the transaction receipt.

###### Example

```
// Send a user operation and get the user operation hash
const { userOpHash } = await smartAccount.sendUserOperation({
  network: "base-sepolia",
  calls: [
    {
      to: "0x0000000000000000000000000000000000000000",
      value: parseEther("0.000001"),
      data: "0x",
    },
  ],
});
// Wait for the user operation to complete or fail
const result = await smartAccount.waitForUserOperation({
  userOpHash: userOp.userOpHash,
});

```

###### waitForFundOperationReceipt()

```
waitForFundOperationReceipt(options: WaitForFundOperationOptions): Promise<WaitForFundOperationResult>;

```

Waits for a fund operation to complete and returns the transfer receipt.

###### Parameters

###### options

`WaitForFundOperationOptions` The options for the wait for fund operation.

###### Returns

`Promise`<`WaitForFundOperationResult`\> A promise that resolves to the completed transfer receipt containing details about the funding operation.

###### Example

```
const completedTransfer = await account.waitForFundOperationReceipt({
  transferId: "transfer_123",
});

```

##### taker

Defined in: [src/client/evm/evm.types.ts:126](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L126) The address receiving the output of the swap.

##### toToken

Defined in: [src/client/evm/evm.types.ts:120](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L120) The token to receive (destination token).

* * *

### CreateSwapQuoteResult

Defined in: [src/client/evm/evm.types.ts:224](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L224) Result of creating a swap quote.

#### Properties

##### blockNumber

Defined in: [src/client/evm/evm.types.ts:240](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L240) The block number at which the liquidity conditions were examined.

##### execute()

```
execute: (options?: ExecuteSwapQuoteOptions) => Promise<ExecuteSwapQuoteResult>;

```

Defined in: [src/client/evm/evm.types.ts:269](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L269) Execute the swap using the quote.

###### Parameters

###### options?

[`ExecuteSwapQuoteOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#executeswapquoteoptions) Options for executing the swap.

###### Returns

`Promise`<[`ExecuteSwapQuoteResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#executeswapquoteresult)\> A promise that resolves to the swap execution result.

##### fees

Defined in: [src/client/evm/evm.types.ts:242](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L242) The estimated fees for the swap.

##### fromAmount

Defined in: [src/client/evm/evm.types.ts:234](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L234) The amount to send in atomic units of the token.

##### fromToken

```
fromToken: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:232](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L232) The token to send (source token).

##### issues

Defined in: [src/client/evm/evm.types.ts:244](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L244) Potential issues discovered during validation.

##### liquidityAvailable

```
liquidityAvailable: true;

```

Defined in: [src/client/evm/evm.types.ts:226](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L226) Whether liquidity is available for the swap.

##### minToAmount

Defined in: [src/client/evm/evm.types.ts:238](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L238) The minimum amount to receive after slippage in atomic units of the token.

##### network

```
network: EvmSwapsNetwork;

```

Defined in: [src/client/evm/evm.types.ts:228](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L228) The network for which this swap quote was created.

##### permit2?

```
optional permit2: {
  eip712: EIP712Message;
};

```

Defined in: [src/client/evm/evm.types.ts:259](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L259) Permit2 data if required for the swap.

###### eip712

EIP-712 typed data for signing.

##### toAmount

Defined in: [src/client/evm/evm.types.ts:236](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L236) The amount to receive in atomic units of the token.

##### toToken

Defined in: [src/client/evm/evm.types.ts:230](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L230) The token to receive (destination token).

##### transaction?

```
optional transaction: {
  data: `0x${string}`;
  gas: bigint;
  gasPrice: bigint;
  to: `0x${string}`;
  value: bigint;
};

```

Defined in: [src/client/evm/evm.types.ts:246](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L246) The transaction to execute the swap.

###### data

The transaction data.

###### gas

The gas limit for the transaction.

###### gasPrice

The gas price for the transaction in Wei.

###### to

The contract address to send the transaction to.

###### value

The value to send with the transaction in Wei.

* * *

### EvmCall

Defined in: [src/client/evm/evm.types.ts:299](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L299) A call to be executed in a user operation.

#### Properties

##### data

Defined in: [src/client/evm/evm.types.ts:309](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L309) The call data to send. This is the hex-encoded data of the function call consisting of the method selector and the function arguments.

##### to

Defined in: [src/client/evm/evm.types.ts:303](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L303) The address the call is directed to.

##### value

Defined in: [src/client/evm/evm.types.ts:305](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L305) The amount of ETH to send with the call, in wei.

* * *

### ExecuteSwapQuoteOptions

Defined in: [src/client/evm/evm.types.ts:202](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L202) Options for executing a swap quote.

#### Properties

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:204](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L204) Optional idempotency key for the request.

* * *

### ExecuteSwapQuoteResult

Defined in: [src/client/evm/evm.types.ts:210](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L210) Result of executing a swap quote.

#### Properties

##### smartAccountAddress?

```
optional smartAccountAddress: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:216](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L216) The address of the smart account (for smart account swaps).

##### status?

```
optional status: "broadcast";

```

Defined in: [src/client/evm/evm.types.ts:218](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L218) The status of the user operation (for smart accounts swaps).

##### transactionHash?

```
optional transactionHash: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:212](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L212) The transaction hash of the executed swap (for EOA swaps).

##### userOpHash?

```
optional userOpHash: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:214](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L214) The user operation hash of the executed swap (for smart account swaps).

* * *

### ExportServerAccountOptions

Defined in: [src/client/evm/evm.types.ts:365](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L365) Options for exporting an EVM server account.

#### Properties

##### address?

```
optional address: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:367](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L367) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:371](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L371) The idempotency key.

##### name?

Defined in: [src/client/evm/evm.types.ts:369](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L369) The name of the account.

* * *

### GetOrCreateServerAccountOptions

Defined in: [src/client/evm/evm.types.ts:399](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L399) Options for getting an EVM account, or creating one if it doesn’t exist.

#### Properties

##### name

Defined in: [src/client/evm/evm.types.ts:401](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L401) The name of the account.

* * *

### GetOrCreateSmartAccountOptions

Defined in: [src/client/evm/evm.types.ts:407](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L407) Options for getting an EVM account, or creating one if it doesn’t exist.

#### Properties

##### enableSpendPermissions?

```
optional enableSpendPermissions: boolean;

```

Defined in: [src/client/evm/evm.types.ts:413](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L413) The flag to enable spend permissions.

##### name

Defined in: [src/client/evm/evm.types.ts:409](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L409) The name of the account.

##### owner

Defined in: [src/client/evm/evm.types.ts:411](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L411) The owner of the account.

* * *

### GetServerAccountOptions

Defined in: [src/client/evm/evm.types.ts:377](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L377) Options for getting an EVM account.

#### Properties

##### address?

```
optional address: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:379](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L379) The address of the account.

##### name?

Defined in: [src/client/evm/evm.types.ts:381](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L381) The name of the account.

* * *

### GetSmartAccountOptions

Defined in: [src/client/evm/evm.types.ts:387](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L387) Options for getting an EVM smart account.

#### Properties

##### address?

```
optional address: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:389](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L389) The address of the account.

##### name?

Defined in: [src/client/evm/evm.types.ts:393](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L393) The name of the account.

##### owner

Defined in: [src/client/evm/evm.types.ts:391](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L391) The owner of the account.

* * *

### GetSwapPriceOptions

Defined in: [src/client/evm/evm.types.ts:142](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L142) Options for getting a swap price.

#### Properties

##### fromAmount

Defined in: [src/client/evm/evm.types.ts:150](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L150) The amount to send in atomic units of the token.

##### fromToken

```
fromToken: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:148](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L148) The token to send (source token).

##### gasPrice?

```
optional gasPrice: bigint;

```

Defined in: [src/client/evm/evm.types.ts:156](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L156) The gas price in Wei.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:160](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L160) The idempotency key.

##### network

```
network: EvmSwapsNetwork;

```

Defined in: [src/client/evm/evm.types.ts:144](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L144) The network to get a price from.

##### signerAddress?

```
optional signerAddress: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:154](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L154) The signer address (only needed if taker is a smart contract).

##### slippageBps?

```
optional slippageBps: number;

```

Defined in: [src/client/evm/evm.types.ts:158](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L158) The slippage tolerance in basis points (0-10000).

##### taker

Defined in: [src/client/evm/evm.types.ts:152](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L152) The address that will perform the swap.

##### toToken

Defined in: [src/client/evm/evm.types.ts:146](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L146) The token to receive (destination token).

* * *

### GetSwapPriceResult

Defined in: [src/client/evm/evm.types.ts:166](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L166) Result of getting a swap price.

#### Properties

##### blockNumber

Defined in: [src/client/evm/evm.types.ts:180](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L180) The block number at which the liquidity conditions were examined.

##### fees

Defined in: [src/client/evm/evm.types.ts:182](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L182) The estimated fees for the swap.

##### fromAmount

Defined in: [src/client/evm/evm.types.ts:174](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L174) The amount to send in atomic units of the token.

##### fromToken

```
fromToken: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:172](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L172) The token to send (source token).

##### gas?

Defined in: [src/client/evm/evm.types.ts:186](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L186) The gas estimate for the swap.

##### gasPrice?

```
optional gasPrice: bigint;

```

Defined in: [src/client/evm/evm.types.ts:188](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L188) The gas price in Wei.

##### issues

Defined in: [src/client/evm/evm.types.ts:184](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L184) Potential issues discovered during validation.

##### liquidityAvailable

```
liquidityAvailable: true;

```

Defined in: [src/client/evm/evm.types.ts:168](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L168) Whether liquidity is available for the swap.

##### minToAmount

Defined in: [src/client/evm/evm.types.ts:178](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L178) The minimum amount to receive after slippage in atomic units of the token.

##### toAmount

Defined in: [src/client/evm/evm.types.ts:176](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L176) The amount to receive in atomic units of the token.

##### toToken

Defined in: [src/client/evm/evm.types.ts:170](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L170) The token to receive (destination token).

* * *

### GetUserOperationOptions

Defined in: [src/client/evm/evm.types.ts:275](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L275) Options for getting a user operation.

#### Properties

##### smartAccount

```
smartAccount: 
  | `0x${string}`
  | {
  address: `0x${string}`;
  fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;
  getUserOperation: (options: Omit<GetUserOperationOptions, "smartAccount">) => Promise<UserOperation>;
  listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;
  name?: string;
  owners: EvmAccount[];
  policies: undefined | string[];
  quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;
  quoteSwap: (options: SmartAccountQuoteSwapOptions) => Promise<SmartAccountQuoteSwapResult>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;
  sendUserOperation: (options: Omit<SendUserOperationOptions<unknown[]>, "smartAccount">) => Promise<SendUserOperationReturnType>;
  signTypedData: (options: Omit<SignTypedDataOptions, "address"> & {
     network: KnownEvmNetworks;
  }) => Promise<`0x${string}`>;
  swap: (options: SmartAccountSwapOptions) => Promise<SmartAccountSwapResult>;
  transfer: (options: SmartAccountTransferOptions) => Promise<SendUserOperationReturnType>;
  type: "evm-smart";
  useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<EvmSmartAccountProperties, "useNetwork"> & { [K in string | number | symbol]: ({ getUserOperation: ...; sendUserOperation: ...; waitForUserOperation: ... } & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)))[K] } & { network: Network })[K] }>;
  useSpendPermission: (options: UseSpendPermissionOptions) => Promise<SendUserOperationReturnType>;
  waitForUserOperation: (options: Omit<WaitForUserOperationOptions, "smartAccountAddress">) => Promise<WaitForUserOperationReturnType>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}
  | ReadonlySmartAccount;

```

Defined in: [src/client/evm/evm.types.ts:277](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L277) The smart account.

###### Type declaration

`` `0x${string}` ``

```
{
  address: `0x${string}`;
  fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;
  getUserOperation: (options: Omit<GetUserOperationOptions, "smartAccount">) => Promise<UserOperation>;
  listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;
  name?: string;
  owners: EvmAccount[];
  policies: undefined | string[];
  quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;
  quoteSwap: (options: SmartAccountQuoteSwapOptions) => Promise<SmartAccountQuoteSwapResult>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;
  sendUserOperation: (options: Omit<SendUserOperationOptions<unknown[]>, "smartAccount">) => Promise<SendUserOperationReturnType>;
  signTypedData: (options: Omit<SignTypedDataOptions, "address"> & {
     network: KnownEvmNetworks;
  }) => Promise<`0x${string}`>;
  swap: (options: SmartAccountSwapOptions) => Promise<SmartAccountSwapResult>;
  transfer: (options: SmartAccountTransferOptions) => Promise<SendUserOperationReturnType>;
  type: "evm-smart";
  useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<EvmSmartAccountProperties, "useNetwork"> & { [K in string | number | symbol]: ({ getUserOperation: ...; sendUserOperation: ...; waitForUserOperation: ... } & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)))[K] } & { network: Network })[K] }>;
  useSpendPermission: (options: UseSpendPermissionOptions) => Promise<SendUserOperationReturnType>;
  waitForUserOperation: (options: Omit<WaitForUserOperationOptions, "smartAccountAddress">) => Promise<WaitForUserOperationReturnType>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}

```

###### address

The smart account’s address.

###### fund()

```
fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;

```

Funds an EVM account with the specified token amount.

###### Parameters

###### options

`Omit`<`EvmFundOptions`, `"address"`\> The options for the fund operation.

###### Returns

`Promise`<`FundOperationResult`\> A promise that resolves to the fund operation result containing the transfer details.

###### Example

```
const fundOperation = await account.fund({
  network: "base",
  token: "usdc",
  amount: 1000000n,
});

```

###### getUserOperation()

```
getUserOperation: (options: Omit<GetUserOperationOptions, "smartAccount">) => Promise<UserOperation>;

```

Gets a user operation by its hash.

###### Parameters

###### options

`Omit`<[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions), `"smartAccount"`\> Parameters for getting the user operation.

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\> A promise that resolves to the user operation.

###### Example

```
const userOp = await smartAccount.getUserOperation({
  userOpHash: "0x1234567890123456789012345678901234567890",
});

```

###### listTokenBalances()

```
listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;

```

List the token balances of an account.

###### Parameters

###### options

`Omit`<`ListTokenBalancesOptions`, `"address"`\> The options for the list token balances.

###### Returns

`Promise`<`ListTokenBalancesResult`\> The result of the list token balances.

###### Example

```
const balances = await account.listTokenBalances({
  network: "base-sepolia",
});

```

###### name?

The name of the smart account.

###### owners

Array of accounts that own and can sign for the smart account (currently only supports one owner but will be extended to support multiple owners in the future).

###### policies

```
policies: undefined | string[];

```

The list of policy IDs that apply to the smart account. This will include both the project-level policy and the account-level policy, if one exists.

###### quoteFund()

```
quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;

```

Gets a quote to fund an EVM account.

###### Parameters

###### options

`Omit`<`EvmQuoteFundOptions`, `"address"`\> The options for the quote fund.

###### Returns

`Promise`<`EvmQuote`\> A promise that resolves to a Quote object containing details about the funding operation.

###### Example

```
const quote = await account.quoteFund({
  network: "base",
  token: "usdc",
  amount: 1000000n,
});

```

###### quoteSwap()

```
quoteSwap: (options: SmartAccountQuoteSwapOptions) => Promise<SmartAccountQuoteSwapResult>;

```

Creates a swap quote without executing the transaction. This is useful when you need to get swap details before executing the swap. The taker is automatically set to the smart account’s address.

###### Parameters

###### options

`SmartAccountQuoteSwapOptions` Configuration options for creating the swap quote.

###### Returns

`Promise`<`SmartAccountQuoteSwapResult`\> A promise that resolves to the swap quote or a response indicating that liquidity is unavailable.

###### Example

```
const swapQuote = await smartAccount.quoteSwap({
  network: "base",
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
});
if (swapQuote.liquidityAvailable) {
  console.log(`Can swap for ${swapQuote.toAmount} USDC`);
}

```

###### requestFaucet()

```
requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;

```

Requests funds from an EVM faucet.

###### Parameters

###### options

`Omit`<`RequestFaucetOptions`, `"address"`\> Parameters for requesting funds from the EVM faucet.

###### Returns

`Promise`<`RequestFaucetResult`\> A promise that resolves to the transaction hash.

###### Example

```
const result = await account.requestFaucet({
  network: "base-sepolia",
  token: "eth",
});

```

###### sendUserOperation()

```
sendUserOperation: (options: Omit<SendUserOperationOptions<unknown[]>, "smartAccount">) => Promise<SendUserOperationReturnType>;

```

Sends a user operation.

###### Parameters

###### options

`Omit`<`SendUserOperationOptions`<`unknown`\[\]>, `"smartAccount"`\> Parameters for sending the user operation.

###### Returns

`Promise`<`SendUserOperationReturnType`\> A promise that resolves to an object containing the smart account address, the user operation hash, and the status of the user operation.

###### Example

```
const userOp = await smartAccount.sendUserOperation({
  network: "base-sepolia",
  calls: [
    {
      to: "0x1234567890123456789012345678901234567890",
      value: parseEther("0.000001"),
      data: "0x",
    },
  ],
});

```

###### signTypedData()

```
signTypedData: (options: Omit<SignTypedDataOptions, "address"> & {
  network: KnownEvmNetworks;
}) => Promise<`0x${string}`>;

```

Signs a typed data message.

###### Parameters

###### options

`Omit`<[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions), `"address"`\> & { `network`: `KnownEvmNetworks`; } Configuration options for signing the typed data.

###### Returns

`Promise`<`` `0x${string}` ``\> A promise that resolves to the signature.

###### Example

```
const signature = await smartAccount.signTypedData({
  network: "base-sepolia",
  typedData: {
    domain: {
      name: "Test",
      chainId: 84532,
      verifyingContract: "0x0000000000000000000000000000000000000000",
    },
    types: {
      Test: [{ name: "name", type: "string" }],
    },
    primaryType: "Test",
    message: {
      name: "John Doe",
    },
  },
});

```

###### swap()

```
swap: (options: SmartAccountSwapOptions) => Promise<SmartAccountSwapResult>;

```

Executes a token swap on the specified network via a user operation. This method handles all the steps required for a swap, including Permit2 signatures if needed. The taker is automatically set to the smart account’s address.

###### Parameters

###### options

`SmartAccountSwapOptions` Configuration options for the swap.

###### Returns

`Promise`<`SmartAccountSwapResult`\> A promise that resolves to the user operation result.

###### Throws

If liquidity is not available when using inline options.

###### Examples

```
// First create a swap quote
const swapQuote = await cdp.evm.createSwapQuote({
  network: "base",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
  taker: smartAccount.address,
  signerAddress: smartAccount.owners[0].address
});
// Check if liquidity is available
if (!swapQuote.liquidityAvailable) {
  console.error("Insufficient liquidity for swap");
  return;
}
// Execute the swap
const { userOpHash } = await smartAccount.swap({
  swapQuote: swapQuote
});
console.log(`Swap executed with user op hash: ${userOpHash}`);

```

```
// Create and execute swap in one call
const { userOpHash } = await smartAccount.swap({
  network: "base",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
});
console.log(`Swap executed with user op hash: ${userOpHash}`);

```

###### transfer()

```
transfer: (options: SmartAccountTransferOptions) => Promise<SendUserOperationReturnType>;

```

Transfer an amount of a token from an account to another account.

###### Parameters

###### options

`SmartAccountTransferOptions` The options for the transfer.

###### Returns

`Promise`<`SendUserOperationReturnType`\> The user operation result.

###### Examples

```
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: 10000n, // equivalent to 0.01 USDC
  token: "usdc",
  network: "base-sepolia",
});

```

**Using parseUnits to specify USDC amount**

```
import { parseUnits } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseUnits("0.01", 6), // USDC uses 6 decimal places
  token: "usdc",
  network: "base-sepolia",
});

```

**Transfer ETH**

```
import { parseEther } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseEther("0.000001"),
  token: "eth",
  network: "base-sepolia",
});

```

**Using a contract address**

```
import { parseEther } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseEther("0.000001"),
  token: "0x4200000000000000000000000000000000000006", // WETH on Base Sepolia
  network: "base-sepolia",
});

```

**Transfer to another account**

```
const sender = await cdp.evm.createAccount({ name: "Sender" });
const receiver = await cdp.evm.createAccount({ name: "Receiver" });
const { userOpHash } = await sender.transfer({
  to: receiver,
  amount: 10000n, // equivalent to 0.01 USDC
  token: "usdc",
  network: "base-sepolia",
});

```

###### type

Identifier for the smart account type.

###### useNetwork()

```
useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<EvmSmartAccountProperties, "useNetwork"> & { [K in string | number | symbol]: ({ getUserOperation: ...; sendUserOperation: ...; waitForUserOperation: ... } & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)))[K] } & { network: Network })[K] }>;

```

A function that returns a network-scoped smart account.

###### Type Parameters

###### Network

`Network` *extends* `KnownEvmNetworks`

###### Parameters

###### network

`Network` The network name or RPC URL

###### Returns

`Promise`<{ \[K in string | number | symbol\]: (Omit<EvmSmartAccountProperties, “useNetwork”> & { \[K in string | number | symbol\]: ({ getUserOperation: …; sendUserOperation: …; waitForUserOperation: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>

###### Example

```
// For known networks, type is inferred automatically:
const baseAccount = await smartAccount.useNetwork("base");
// For custom RPC URLs with type hints (requires casting):
const typedAccount = await smartAccount.useNetwork<"base">("https://mainnet.base.org" as "base");
// For custom RPC URLs without type hints (only sendTransaction, transfer and waitForTransactionReceipt methods available):
const customAccount = await smartAccount.useNetwork("https://mainnet.base.org");

```

###### useSpendPermission()

```
useSpendPermission: (options: UseSpendPermissionOptions) => Promise<SendUserOperationReturnType>;

```

Uses a spend permission to execute a transaction via user operation. This allows the smart account to spend tokens that have been approved via a spend permission.

###### Parameters

###### options

`UseSpendPermissionOptions` Configuration options for using the spend permission.

###### Returns

`Promise`<`SendUserOperationReturnType`\> A promise that resolves to the user operation result.

###### Throws

If the network doesn’t support spend permissions via CDP API.

###### Example

```
const spendPermission = {
  account: "0x1234...", // Smart account that owns the tokens
  spender: smartAccount.address, // This smart account that can spend
  token: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE", // ETH
  allowance: parseEther("0.01"),
  period: 86400, // 1 day
  start: 0,
  end: 281474976710655,
  salt: 0n,
  extraData: "0x",
};
const result = await smartAccount.useSpendPermission({
  spendPermission,
  value: parseEther("0.001"), // Spend 0.001 ETH
  network: "base-sepolia",
});

```

###### waitForUserOperation()

```
waitForUserOperation: (options: Omit<WaitForUserOperationOptions, "smartAccountAddress">) => Promise<WaitForUserOperationReturnType>;

```

Waits for a user operation to complete or fail.

###### Parameters

###### options

`Omit`<`WaitForUserOperationOptions`, `"smartAccountAddress"`\> Parameters for waiting for the user operation.

###### Returns

`Promise`<`WaitForUserOperationReturnType`\> A promise that resolves to the transaction receipt.

###### Example

```
// Send a user operation and get the user operation hash
const { userOpHash } = await smartAccount.sendUserOperation({
  network: "base-sepolia",
  calls: [
    {
      to: "0x0000000000000000000000000000000000000000",
      value: parseEther("0.000001"),
      data: "0x",
    },
  ],
});
// Wait for the user operation to complete or fail
const result = await smartAccount.waitForUserOperation({
  userOpHash: userOp.userOpHash,
});

```

###### waitForFundOperationReceipt()

```
waitForFundOperationReceipt(options: WaitForFundOperationOptions): Promise<WaitForFundOperationResult>;

```

Waits for a fund operation to complete and returns the transfer receipt.

###### Parameters

###### options

`WaitForFundOperationOptions` The options for the wait for fund operation.

###### Returns

`Promise`<`WaitForFundOperationResult`\> A promise that resolves to the completed transfer receipt containing details about the funding operation.

###### Example

```
const completedTransfer = await account.waitForFundOperationReceipt({
  transferId: "transfer_123",
});

```

[`ReadonlySmartAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#readonlysmartaccount)

##### userOpHash

```
userOpHash: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:279](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L279) The user operation hash.

* * *

### ImportServerAccountOptions

Defined in: [src/client/evm/evm.types.ts:351](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L351) Options for importing an EVM server account.

#### Properties

##### encryptionPublicKey?

```
optional encryptionPublicKey: string;

```

Defined in: [src/client/evm/evm.types.ts:353](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L353) The public RSA key used to encrypt the private key when importing an EVM account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:357](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L357) The idempotency key.

##### name?

Defined in: [src/client/evm/evm.types.ts:355](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L355) The name of the account.

##### privateKey

```
privateKey: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:359](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L359) The private key of the account.

* * *

### ListServerAccountResult

Defined in: [src/client/evm/evm.types.ts:477](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L477) The result of listing EVM server accounts.

#### Properties

##### accounts

```
accounts: {
  address: `0x${string}`;
  fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;
  listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;
  name?: string;
  policies?: string[];
  quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;
  quoteSwap: (options: AccountQuoteSwapOptions) => Promise<AccountQuoteSwapResult>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;
  sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<TransactionResult>;
  sign: (parameters: {
     hash: `0x${string}`;
  }) => Promise<`0x${string}`>;
  signMessage: (parameters: {
     message: SignableMessage;
  }) => Promise<`0x${string}`>;
  signTransaction: (transaction: TransactionSerializable) => Promise<`0x${string}`>;
  signTypedData: <typedData, primaryType>(parameters: TypedDataDefinition<typedData, primaryType>) => Promise<`0x${string}`>;
  swap: (options: AccountSwapOptions) => Promise<SendSwapTransactionResult>;
  transfer: (options: TransferOptions) => Promise<{
     transactionHash: `0x${string}`;
  }>;
  type: "evm-server";
  useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<{ address: `0x${string}`; fund: (options: Omit<(...), (...)>) => Promise<(...)>; listTokenBalances: (options: Omit<(...), (...)>) => Promise<(...)>; name?: string; policies?: (...)[]; quoteFund: (options: Omit<(...), (...)>) => Promise<(...)>; quoteSwap: (options: AccountQuoteSwapOptions) => Promise<(...)>; requestFaucet: (options: Omit<(...), (...)>) => Promise<(...)>; sendTransaction: (options: Omit<(...), (...)>) => Promise<(...)>; sign: (parameters: { hash: ... }) => Promise<(...)>; signMessage: (parameters: { message: ... }) => Promise<(...)>; signTransaction: (transaction: TransactionSerializable) => Promise<(...)>; signTypedData: (parameters: TypedDataDefinition<(...), (...)>) => Promise<(...)>; swap: (options: AccountSwapOptions) => Promise<(...)>; transfer: (options: TransferOptions) => Promise<(...)>; type: "evm-server"; useNetwork: <Network extends NetworkOrRpcUrl>(network: Network) => Promise<{ [K in keyof (Omit<{ address: `0x${string}`; sign: (parameters: { hash: `0x${string}`; }) => Promise<`0x${string}`>; signMessage: (parameters: { message: SignableMessage; }) => Promise<...>; ... 15 more ...; useNetwork: ...; }, "transfer" | ... 5 more ....; useSpendPermission: (options: UseSpendPermissionOptions) => Promise<(...)>; waitForFundOperationReceipt: any }, "transfer" | "sendTransaction" | "quoteSwap" | "swap" | "useSpendPermission" | "useNetwork" | (keyof Actions)> & { [K in string | number | symbol]: ({ sendTransaction: ...; transfer: ...; waitForTransactionReceipt: ... } & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)))[K] } & { network: Network })[K] }>;
  useSpendPermission: (options: UseSpendPermissionOptions) => Promise<TransactionResult>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}[];

```

Defined in: [src/client/evm/evm.types.ts:479](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L479) The accounts.

###### address

The address of the signer.

###### fund()

```
fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;

```

Funds an EVM account with the specified token amount.

###### Parameters

###### options

`Omit`<`EvmFundOptions`, `"address"`\> The options for the fund operation.

###### Returns

`Promise`<`FundOperationResult`\> A promise that resolves to the fund operation result containing the transfer details.

###### Example

```
const fundOperation = await account.fund({
  network: "base",
  token: "usdc",
  amount: 1000000n,
});

```

###### listTokenBalances()

```
listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;

```

List the token balances of an account.

###### Parameters

###### options

`Omit`<`ListTokenBalancesOptions`, `"address"`\> The options for the list token balances.

###### Returns

`Promise`<`ListTokenBalancesResult`\> The result of the list token balances.

###### Example

```
const balances = await account.listTokenBalances({
  network: "base-sepolia",
});

```

###### name?

Optional name for the server account.

###### policies?

```
optional policies: string[];

```

A list of Policy ID’s that apply to the account.

###### quoteFund()

```
quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;

```

Gets a quote to fund an EVM account.

###### Parameters

###### options

`Omit`<`EvmQuoteFundOptions`, `"address"`\> The options for the quote fund.

###### Returns

`Promise`<`EvmQuote`\> A promise that resolves to a Quote object containing details about the funding operation.

###### Example

```
const quote = await account.quoteFund({
  network: "base",
  token: "usdc",
  amount: 1000000n,
});

```

###### quoteSwap()

```
quoteSwap: (options: AccountQuoteSwapOptions) => Promise<AccountQuoteSwapResult>;

```

Creates a swap quote without executing the transaction. This is useful when you need to get swap details before executing the swap. The taker is automatically set to the account’s address.

###### Parameters

###### options

`AccountQuoteSwapOptions` Configuration options for creating the swap quote.

###### Returns

`Promise`<`AccountQuoteSwapResult`\> A promise that resolves to the swap quote or a response indicating that liquidity is unavailable.

###### Example

```
const swapQuote = await account.quoteSwap({
  network: "base",
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
});
if (swapQuote.liquidityAvailable) {
  console.log(`Can swap for ${swapQuote.toAmount} USDC`);
}

```

###### requestFaucet()

```
requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;

```

Requests funds from an EVM faucet.

###### Parameters

###### options

`Omit`<`RequestFaucetOptions`, `"address"`\> Parameters for requesting funds from the EVM faucet.

###### Returns

`Promise`<`RequestFaucetResult`\> A promise that resolves to the transaction hash.

###### Example

```
const result = await account.requestFaucet({
  network: "base-sepolia",
  token: "eth",
});

```

###### sendTransaction()

```
sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<TransactionResult>;

```

Signs an EVM transaction and sends it to the specified network using the Coinbase API. This method handles nonce management and gas estimation automatically.

###### Parameters

###### options

`Omit`<`SendTransactionOptions`, `"address"`\> Configuration options for sending the transaction.

###### Returns

`Promise`<`TransactionResult`\> A promise that resolves to the transaction hash.

###### Examples

**Sending an RLP-encoded transaction**

```
import { parseEther, serializeTransaction } from "viem";
import { baseSepolia } from "viem/chains";
const { transactionHash } = await account.sendTransaction({
  transaction: serializeTransaction({
    to: "0x4252e0c9A3da5A2700e7d91cb50aEf522D0C6Fe8",
    value: parseEther("0.000001"),
    chainId: baseSepolia.id,
    // Fields below are optional, CDP API will populate them if omitted.
    // nonce
    // maxPriorityFeePerGas
    // maxFeePerGas
    // gas
  }),
  network: "base-sepolia",
});

```

**Sending an EIP-1559 transaction request object**

```
const { transactionHash } = await account.sendTransaction({
  transaction: {
    to: "0x4252e0c9A3da5A2700e7d91cb50aEf522D0C6Fe8",
    value: parseEther("0.000001"),
    // Fields below are optional, CDP API will populate them if omitted.
    // nonce
    // maxPriorityFeePerGas
    // maxFeePerGas
    // gas
  },
  network: "base-sepolia",
});

```

###### sign()

```
sign: (parameters: {
  hash: `0x${string}`;
}) => Promise<`0x${string}`>;

```

Signs a message hash and returns the signature as a hex string.

###### Parameters

###### parameters

###### hash

`` `0x${string}` ``

###### Returns

`Promise`<`` `0x${string}` ``\>

###### signMessage()

```
signMessage: (parameters: {
  message: SignableMessage;
}) => Promise<`0x${string}`>;

```

Signs a message and returns the signature as a hex string.

###### Parameters

###### parameters

###### message

`SignableMessage`

###### Returns

`Promise`<`` `0x${string}` ``\>

###### signTransaction()

```
signTransaction: (transaction: TransactionSerializable) => Promise<`0x${string}`>;

```

Signs a transaction and returns the signed transaction as a hex string.

###### Parameters

###### transaction

`TransactionSerializable`

###### Returns

`Promise`<`` `0x${string}` ``\>

###### signTypedData()

```
signTypedData: <typedData, primaryType>(parameters: TypedDataDefinition<typedData, primaryType>) => Promise<`0x${string}`>;

```

Signs a typed data and returns the signature as a hex string.

###### Type Parameters

###### typedData

`typedData` *extends* | `Record`<`string`, `unknown`\> | { \[`key`: `string`\]: readonly `TypedDataParameter`\[\]; \[`key`: `` `string[${string}]` ``\]: `undefined`; \[`key`: `` `function[${string}]` ``\]: `undefined`; \[`key`: `` `address[${string}]` ``\]: `undefined`; \[`key`: `` `bool[${string}]` ``\]: `undefined`; \[`key`: `` `bytes[${string}]` ``\]: `undefined`; \[`key`: `` `bytes1[${string}]` ``\]: `undefined`; \[`key`: `` `bytes2[${string}]` ``\]: `undefined`; \[`key`: `` `bytes3[${string}]` ``\]: `undefined`; \[`key`: `` `bytes32[${string}]` ``\]: `undefined`; \[`key`: `` `bytes16[${string}]` ``\]: `undefined`; \[`key`: `` `bytes4[${string}]` ``\]: `undefined`; \[`key`: `` `bytes6[${string}]` ``\]: `undefined`; \[`key`: `` `bytes18[${string}]` ``\]: `undefined`; \[`key`: `` `bytes5[${string}]` ``\]: `undefined`; \[`key`: `` `bytes7[${string}]` ``\]: `undefined`; \[`key`: `` `bytes8[${string}]` ``\]: `undefined`; \[`key`: `` `bytes9[${string}]` ``\]: `undefined`; \[`key`: `` `bytes10[${string}]` ``\]: `undefined`; \[`key`: `` `bytes11[${string}]` ``\]: `undefined`; \[`key`: `` `bytes12[${string}]` ``\]: `undefined`; \[`key`: `` `bytes13[${string}]` ``\]: `undefined`; \[`key`: `` `bytes14[${string}]` ``\]: `undefined`; \[`key`: `` `bytes15[${string}]` ``\]: `undefined`; \[`key`: `` `bytes17[${string}]` ``\]: `undefined`; \[`key`: `` `bytes19[${string}]` ``\]: `undefined`; \[`key`: `` `bytes20[${string}]` ``\]: `undefined`; \[`key`: `` `bytes21[${string}]` ``\]: `undefined`; \[`key`: `` `bytes22[${string}]` ``\]: `undefined`; \[`key`: `` `bytes23[${string}]` ``\]: `undefined`; \[`key`: `` `bytes24[${string}]` ``\]: `undefined`; \[`key`: `` `bytes25[${string}]` ``\]: `undefined`; \[`key`: `` `bytes26[${string}]` ``\]: `undefined`; \[`key`: `` `bytes27[${string}]` ``\]: `undefined`; \[`key`: `` `bytes28[${string}]` ``\]: `undefined`; \[`key`: `` `bytes29[${string}]` ``\]: `undefined`; \[`key`: `` `bytes30[${string}]` ``\]: `undefined`; \[`key`: `` `bytes31[${string}]` ``\]: `undefined`; \[`key`: `` `int[${string}]` ``\]: `undefined`; \[`key`: `` `int120[${string}]` ``\]: `undefined`; \[`key`: `` `int64[${string}]` ``\]: `undefined`; \[`key`: `` `int32[${string}]` ``\]: `undefined`; \[`key`: `` `int16[${string}]` ``\]: `undefined`; \[`key`: `` `int8[${string}]` ``\]: `undefined`; \[`key`: `` `int24[${string}]` ``\]: `undefined`; \[`key`: `` `int40[${string}]` ``\]: `undefined`; \[`key`: `` `int48[${string}]` ``\]: `undefined`; \[`key`: `` `int56[${string}]` ``\]: `undefined`; \[`key`: `` `int72[${string}]` ``\]: `undefined`; \[`key`: `` `int80[${string}]` ``\]: `undefined`; \[`key`: `` `int88[${string}]` ``\]: `undefined`; \[`key`: `` `int96[${string}]` ``\]: `undefined`; \[`key`: `` `int104[${string}]` ``\]: `undefined`; \[`key`: `` `int112[${string}]` ``\]: `undefined`; \[`key`: `` `int128[${string}]` ``\]: `undefined`; \[`key`: `` `int136[${string}]` ``\]: `undefined`; \[`key`: `` `int144[${string}]` ``\]: `undefined`; \[`key`: `` `int152[${string}]` ``\]: `undefined`; \[`key`: `` `int160[${string}]` ``\]: `undefined`; \[`key`: `` `int168[${string}]` ``\]: `undefined`; \[`key`: `` `int176[${string}]` ``\]: `undefined`; \[`key`: `` `int184[${string}]` ``\]: `undefined`; \[`key`: `` `int192[${string}]` ``\]: `undefined`; \[`key`: `` `int200[${string}]` ``\]: `undefined`; \[`key`: `` `int208[${string}]` ``\]: `undefined`; \[`key`: `` `int216[${string}]` ``\]: `undefined`; \[`key`: `` `int224[${string}]` ``\]: `undefined`; \[`key`: `` `int232[${string}]` ``\]: `undefined`; \[`key`: `` `int240[${string}]` ``\]: `undefined`; \[`key`: `` `int248[${string}]` ``\]: `undefined`; \[`key`: `` `int256[${string}]` ``\]: `undefined`; \[`key`: `` `uint[${string}]` ``\]: `undefined`; \[`key`: `` `uint120[${string}]` ``\]: `undefined`; \[`key`: `` `uint64[${string}]` ``\]: `undefined`; \[`key`: `` `uint32[${string}]` ``\]: `undefined`; \[`key`: `` `uint16[${string}]` ``\]: `undefined`; \[`key`: `` `uint8[${string}]` ``\]: `undefined`; \[`key`: `` `uint24[${string}]` ``\]: `undefined`; \[`key`: `` `uint40[${string}]` ``\]: `undefined`; \[`key`: `` `uint48[${string}]` ``\]: `undefined`; \[`key`: `` `uint56[${string}]` ``\]: `undefined`; \[`key`: `` `uint72[${string}]` ``\]: `undefined`; \[`key`: `` `uint80[${string}]` ``\]: `undefined`; \[`key`: `` `uint88[${string}]` ``\]: `undefined`; \[`key`: `` `uint96[${string}]` ``\]: `undefined`; \[`key`: `` `uint104[${string}]` ``\]: `undefined`; \[`key`: `` `uint112[${string}]` ``\]: `undefined`; \[`key`: `` `uint128[${string}]` ``\]: `undefined`; \[`key`: `` `uint136[${string}]` ``\]: `undefined`; \[`key`: `` `uint144[${string}]` ``\]: `undefined`; \[`key`: `` `uint152[${string}]` ``\]: `undefined`; \[`key`: `` `uint160[${string}]` ``\]: `undefined`; \[`key`: `` `uint168[${string}]` ``\]: `undefined`; \[`key`: `` `uint176[${string}]` ``\]: `undefined`; \[`key`: `` `uint184[${string}]` ``\]: `undefined`; \[`key`: `` `uint192[${string}]` ``\]: `undefined`; \[`key`: `` `uint200[${string}]` ``\]: `undefined`; \[`key`: `` `uint208[${string}]` ``\]: `undefined`; \[`key`: `` `uint216[${string}]` ``\]: `undefined`; \[`key`: `` `uint224[${string}]` ``\]: `undefined`; \[`key`: `` `uint232[${string}]` ``\]: `undefined`; \[`key`: `` `uint240[${string}]` ``\]: `undefined`; \[`key`: `` `uint248[${string}]` ``\]: `undefined`; \[`key`: `` `uint256[${string}]` ``\]: `undefined`; `address?`: `undefined`; `bool?`: `undefined`; `bytes?`: `undefined`; `bytes1?`: `undefined`; `bytes10?`: `undefined`; `bytes11?`: `undefined`; `bytes12?`: `undefined`; `bytes13?`: `undefined`; `bytes14?`: `undefined`; `bytes15?`: `undefined`; `bytes16?`: `undefined`; `bytes17?`: `undefined`; `bytes18?`: `undefined`; `bytes19?`: `undefined`; `bytes2?`: `undefined`; `bytes20?`: `undefined`; `bytes21?`: `undefined`; `bytes22?`: `undefined`; `bytes23?`: `undefined`; `bytes24?`: `undefined`; `bytes25?`: `undefined`; `bytes26?`: `undefined`; `bytes27?`: `undefined`; `bytes28?`: `undefined`; `bytes29?`: `undefined`; `bytes3?`: `undefined`; `bytes30?`: `undefined`; `bytes31?`: `undefined`; `bytes32?`: `undefined`; `bytes4?`: `undefined`; `bytes5?`: `undefined`; `bytes6?`: `undefined`; `bytes7?`: `undefined`; `bytes8?`: `undefined`; `bytes9?`: `undefined`; `int104?`: `undefined`; `int112?`: `undefined`; `int120?`: `undefined`; `int128?`: `undefined`; `int136?`: `undefined`; `int144?`: `undefined`; `int152?`: `undefined`; `int16?`: `undefined`; `int160?`: `undefined`; `int168?`: `undefined`; `int176?`: `undefined`; `int184?`: `undefined`; `int192?`: `undefined`; `int200?`: `undefined`; `int208?`: `undefined`; `int216?`: `undefined`; `int224?`: `undefined`; `int232?`: `undefined`; `int24?`: `undefined`; `int240?`: `undefined`; `int248?`: `undefined`; `int256?`: `undefined`; `int32?`: `undefined`; `int40?`: `undefined`; `int48?`: `undefined`; `int56?`: `undefined`; `int64?`: `undefined`; `int72?`: `undefined`; `int8?`: `undefined`; `int80?`: `undefined`; `int88?`: `undefined`; `int96?`: `undefined`; `string?`: `undefined`; `uint104?`: `undefined`; `uint112?`: `undefined`; `uint120?`: `undefined`; `uint128?`: `undefined`; `uint136?`: `undefined`; `uint144?`: `undefined`; `uint152?`: `undefined`; `uint16?`: `undefined`; `uint160?`: `undefined`; `uint168?`: `undefined`; `uint176?`: `undefined`; `uint184?`: `undefined`; `uint192?`: `undefined`; `uint200?`: `undefined`; `uint208?`: `undefined`; `uint216?`: `undefined`; `uint224?`: `undefined`; `uint232?`: `undefined`; `uint24?`: `undefined`; `uint240?`: `undefined`; `uint248?`: `undefined`; `uint256?`: `undefined`; `uint32?`: `undefined`; `uint40?`: `undefined`; `uint48?`: `undefined`; `uint56?`: `undefined`; `uint64?`: `undefined`; `uint72?`: `undefined`; `uint8?`: `undefined`; `uint80?`: `undefined`; `uint88?`: `undefined`; `uint96?`: `undefined`; }

###### primaryType

`primaryType` *extends* `string` | `number` | `symbol` = keyof `typedData`

###### Parameters

###### parameters

`TypedDataDefinition`<`typedData`, `primaryType`\>

###### Returns

`Promise`<`` `0x${string}` ``\>

###### swap()

```
swap: (options: AccountSwapOptions) => Promise<SendSwapTransactionResult>;

```

Executes a token swap on the specified network. This method handles all the steps required for a swap, including Permit2 signatures if needed. The taker is automatically set to the account’s address.

###### Parameters

###### options

`AccountSwapOptions` Configuration options for the swap.

###### Returns

`Promise`<`SendSwapTransactionResult`\> A promise that resolves to the transaction hash.

###### Throws

If liquidity is not available when using inline options.

###### Examples

```
// First create a swap quote
const swapQuote = await cdp.evm.createSwapQuote({
  network: "base",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
  taker: account.address
});
// Check if liquidity is available
if (!swapQuote.liquidityAvailable) {
  console.error("Insufficient liquidity for swap");
  return;
}
// Execute the swap
const { transactionHash } = await account.swap({
  swapQuote: swapQuote
});
console.log(`Swap executed with transaction hash: ${transactionHash}`);

```

```
// Create and execute swap in one call
const { transactionHash } = await account.swap({
  network: "base",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
});
console.log(`Swap executed with transaction hash: ${transactionHash}`);

```

###### transfer()

```
transfer: (options: TransferOptions) => Promise<{
  transactionHash: `0x${string}`;
}>;

```

Transfer an amount of a token from an account to another account.

###### Parameters

###### options

`TransferOptions` The options for the transfer.

###### Returns

`Promise`<{ `transactionHash`: `` `0x${string}` ``; }> An object containing the transaction hash.

###### Examples

```
const { transactionHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: 10000n, // equivalent to 0.01 USDC
  token: "usdc",
  network: "base-sepolia",
});

```

**Using parseUnits to specify USDC amount**

```
import { parseUnits } from "viem";
const { transactionHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseUnits("0.01", 6), // USDC uses 6 decimal places
  token: "usdc",
  network: "base-sepolia",
});

```

**Transfer ETH**

```
import { parseEther } from "viem";
const { transactionHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseEther("0.000001"),
  token: "eth",
  network: "base-sepolia",
});

```

**Using a contract address**

```
import { parseEther } from "viem";
const { transactionHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseEther("0.000001"),
  token: "0x4200000000000000000000000000000000000006", // WETH on Base Sepolia
  network: "base-sepolia",
});

```

**Transfer to another account**

```
const sender = await cdp.evm.createAccount({ name: "Sender" });
const receiver = await cdp.evm.createAccount({ name: "Receiver" });
const { transactionHash } = await sender.transfer({
  to: receiver,
  amount: 10000n, // equivalent to 0.01 USDC
  token: "usdc",
  network: "base-sepolia",
});

```

###### type

Indicates this is a server-managed account.

###### useNetwork()

```
useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<{ address: `0x${string}`; fund: (options: Omit<(...), (...)>) => Promise<(...)>; listTokenBalances: (options: Omit<(...), (...)>) => Promise<(...)>; name?: string; policies?: (...)[]; quoteFund: (options: Omit<(...), (...)>) => Promise<(...)>; quoteSwap: (options: AccountQuoteSwapOptions) => Promise<(...)>; requestFaucet: (options: Omit<(...), (...)>) => Promise<(...)>; sendTransaction: (options: Omit<(...), (...)>) => Promise<(...)>; sign: (parameters: { hash: ... }) => Promise<(...)>; signMessage: (parameters: { message: ... }) => Promise<(...)>; signTransaction: (transaction: TransactionSerializable) => Promise<(...)>; signTypedData: (parameters: TypedDataDefinition<(...), (...)>) => Promise<(...)>; swap: (options: AccountSwapOptions) => Promise<(...)>; transfer: (options: TransferOptions) => Promise<(...)>; type: "evm-server"; useNetwork: <Network extends NetworkOrRpcUrl>(network: Network) => Promise<{ [K in keyof (Omit<{ address: `0x${string}`; sign: (parameters: { hash: `0x${string}`; }) => Promise<`0x${string}`>; signMessage: (parameters: { message: SignableMessage; }) => Promise<...>; ... 15 more ...; useNetwork: ...; }, "transfer" | ... 5 more ....; useSpendPermission: (options: UseSpendPermissionOptions) => Promise<(...)>; waitForFundOperationReceipt: any }, "transfer" | "sendTransaction" | "quoteSwap" | "swap" | "useSpendPermission" | "useNetwork" | (keyof Actions)> & { [K in string | number | symbol]: ({ sendTransaction: ...; transfer: ...; waitForTransactionReceipt: ... } & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)) & ((...) extends (...) ? (...) : (...)))[K] } & { network: Network })[K] }>;

```

A function that returns a network-scoped server-managed account.

###### Type Parameters

###### Network

`Network` *extends* `NetworkOrRpcUrl`

###### Parameters

###### network

`Network` The network name or RPC URL

###### Returns

`Promise`<{ \[K in string | number | symbol\]: (Omit<{ address: \`0x\\{string\\}\\\`; fund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; listTokenBalances: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; name?: string; policies?: (...)\\\[\\\]; quoteFund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; quoteSwap: (options: AccountQuoteSwapOptions) =\\> Promise\\<(...)\\>; requestFaucet: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sendTransaction: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sign: (parameters: \\{ hash: ... \\}) =\\> Promise\\<(...)\\>; signMessage: (parameters: \\{ message: ... \\}) =\\> Promise\\<(...)\\>; signTransaction: (transaction: TransactionSerializable) =\\> Promise\\<(...)\\>; signTypedData: (parameters: TypedDataDefinition\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; swap: (options: AccountSwapOptions) =\\> Promise\\<(...)\\>; transfer: (options: TransferOptions) =\\> Promise\\<(...)\\>; type: "evm-server"; useNetwork: \\<Network extends NetworkOrRpcUrl\\>(network: Network) =\\> Promise\\<\\{ \\\[K in keyof (Omit\\<\\{ address: \\\`0x{string}\`; sign: (parameters: { hash: \`0x{string};ˋ})\= Promise\\<0ˋx\\{string\\}\\\`; \\}) =\\> Promise\\<\\\`0x{string}\`>; signMessage: (parameters: { message: SignableMessage; }) => Promise<…>; … 15 more …; useNetwork: …; }, “transfer” | … 5 more …; useSpendPermission: (options: UseSpendPermissionOptions) => Promise<(…)>; waitForFundOperationReceipt: any }, “transfer” | “sendTransaction” | “quoteSwap” | “swap” | “useSpendPermission” | “useNetwork” | (keyof Actions)> & { \[K in string | number | symbol\]: ({ sendTransaction: …; transfer: …; waitForTransactionReceipt: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>

###### Example

```
// For known networks, type is inferred automatically:
const baseAccount = await account.useNetwork("base");
// For custom RPC URLs with type hints (requires casting):
const typedAccount = await account.useNetwork<"base">("https://mainnet.base.org" as "base");
// For custom RPC URLs without type hints (only sendTransaction and waitForTransactionReceipt methods available):
const customAccount = await account.useNetwork("https://mainnet.base.org");

```

###### useSpendPermission()

```
useSpendPermission: (options: UseSpendPermissionOptions) => Promise<TransactionResult>;

```

Uses a spend permission to execute a transaction. This allows the account to spend tokens that have been approved via a spend permission.

###### Parameters

###### options

`UseSpendPermissionOptions` Configuration options for using the spend permission.

###### Returns

`Promise`<`TransactionResult`\> A promise that resolves to the transaction result.

###### Throws

If the network doesn’t support spend permissions via CDP API.

###### Example

```
const spendPermission = {
  account: "0x1234...", // Smart account that owns the tokens
  spender: account.address, // This account that can spend
  token: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE", // ETH
  allowance: parseEther("0.01"),
  period: 86400, // 1 day
  start: 0,
  end: 281474976710655,
  salt: 0n,
  extraData: "0x",
};
const result = await account.useSpendPermission({
  spendPermission,
  value: parseEther("0.001"), // Spend 0.001 ETH
  network: "base-sepolia",
});

```

###### waitForFundOperationReceipt()

```
waitForFundOperationReceipt(options: WaitForFundOperationOptions): Promise<WaitForFundOperationResult>;

```

Waits for a fund operation to complete and returns the transfer receipt.

###### Parameters

###### options

`WaitForFundOperationOptions` The options for the wait for fund operation.

###### Returns

`Promise`<`WaitForFundOperationResult`\> A promise that resolves to the completed transfer receipt containing details about the funding operation.

###### Example

```
const completedTransfer = await account.waitForFundOperationReceipt({
  transferId: "transfer_123",
});

```

##### nextPageToken?

```
optional nextPageToken: string;

```

Defined in: [src/client/evm/evm.types.ts:484](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L484) The next page token to paginate through the accounts. If undefined, there are no more accounts to paginate through.

* * *

### ListServerAccountsOptions

Defined in: [src/client/evm/evm.types.ts:419](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L419) Options for listing EVM accounts.

#### Properties

##### pageSize?

```
optional pageSize: number;

```

Defined in: [src/client/evm/evm.types.ts:421](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L421) The page size to paginate through the accounts.

##### pageToken?

```
optional pageToken: string;

```

Defined in: [src/client/evm/evm.types.ts:423](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L423) The page token to paginate through the accounts.

* * *

### ListSmartAccountResult

Defined in: [src/client/evm/evm.types.ts:464](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L464) The result of listing EVM smart accounts.

#### Properties

##### accounts

```
accounts: ReadonlySmartAccount[];

```

Defined in: [src/client/evm/evm.types.ts:466](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L466) The accounts.

##### nextPageToken?

```
optional nextPageToken: string;

```

Defined in: [src/client/evm/evm.types.ts:471](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L471) The next page token to paginate through the accounts. If undefined, there are no more accounts to paginate through.

* * *

### ListSmartAccountsOptions

Defined in: [src/client/evm/evm.types.ts:490](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L490) Options for listing EVM smart accounts.

#### Properties

##### name?

Defined in: [src/client/evm/evm.types.ts:492](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L492) The name of the account.

##### pageSize?

```
optional pageSize: number;

```

Defined in: [src/client/evm/evm.types.ts:494](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L494) The page size to paginate through the accounts.

##### pageToken?

```
optional pageToken: string;

```

Defined in: [src/client/evm/evm.types.ts:496](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L496) The page token to paginate through the accounts.

* * *

### PrepareUserOperationOptions

Defined in: [src/client/evm/evm.types.ts:285](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L285) Options for preparing a user operation.

#### Properties

##### calls

```
calls: readonly {
  data: `0x${string}`;
  to: `0x${string}`;
  value: bigint;
}[];

```

Defined in: [src/client/evm/evm.types.ts:291](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L291) The calls.

##### network

```
network: EvmUserOperationNetwork;

```

Defined in: [src/client/evm/evm.types.ts:289](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L289) The network.

##### paymasterUrl?

```
optional paymasterUrl: string;

```

Defined in: [src/client/evm/evm.types.ts:293](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L293) The paymaster URL.

##### smartAccount

```
smartAccount: {
  address: `0x${string}`;
  fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;
  getUserOperation: (options: Omit<GetUserOperationOptions, "smartAccount">) => Promise<UserOperation>;
  listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;
  name?: string;
  owners: EvmAccount[];
  policies: undefined | string[];
  quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;
  quoteSwap: (options: SmartAccountQuoteSwapOptions) => Promise<SmartAccountQuoteSwapResult>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;
  sendUserOperation: (options: Omit<SendUserOperationOptions<unknown[]>, "smartAccount">) => Promise<SendUserOperationReturnType>;
  signTypedData: (options: Omit<SignTypedDataOptions, "address"> & {
     network: KnownEvmNetworks;
  }) => Promise<`0x${string}`>;
  swap: (options: SmartAccountSwapOptions) => Promise<SmartAccountSwapResult>;
  transfer: (options: SmartAccountTransferOptions) => Promise<SendUserOperationReturnType>;
  type: "evm-smart";
  useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<EvmSmartAccountProperties, "useNetwork"> & { [K in string | number | symbol]: ({ getUserOperation: (options: ...) => ...; sendUserOperation: (options: ...) => ...; waitForUserOperation: (options: ...) => ... } & (Network extends TransferNetworks ? { transfer: ... } : EmptyObject) & (Network extends ListTokenBalancesNetworks ? { listTokenBalances: ... } : EmptyObject) & (Network extends RequestFaucetNetworks ? { requestFaucet: ... } : EmptyObject) & (Network extends "base" ? { quoteFund: ... } : EmptyObject) & (Network extends "base" ? { fund: ...; waitForFundOperationReceipt: ... } : EmptyObject) & (Network extends QuoteSwapNetworks ? { quoteSwap: ... } : EmptyObject) & (Network extends SwapNetworks ? { swap: ... } : EmptyObject) & (Network extends SpendPermissionNetwork ? { useSpendPermission: ... } : EmptyObject))[K] } & { network: Network })[K] }>;
  useSpendPermission: (options: UseSpendPermissionOptions) => Promise<SendUserOperationReturnType>;
  waitForUserOperation: (options: Omit<WaitForUserOperationOptions, "smartAccountAddress">) => Promise<WaitForUserOperationReturnType>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
};

```

Defined in: [src/client/evm/evm.types.ts:287](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L287) The smart account.

###### address

The smart account’s address.

###### fund()

```
fund: (options: Omit<EvmFundOptions, "address">) => Promise<FundOperationResult>;

```

Funds an EVM account with the specified token amount.

###### Parameters

###### options

`Omit`<`EvmFundOptions`, `"address"`\> The options for the fund operation.

###### Returns

`Promise`<`FundOperationResult`\> A promise that resolves to the fund operation result containing the transfer details.

###### Example

```
const fundOperation = await account.fund({
  network: "base",
  token: "usdc",
  amount: 1000000n,
});

```

###### getUserOperation()

```
getUserOperation: (options: Omit<GetUserOperationOptions, "smartAccount">) => Promise<UserOperation>;

```

Gets a user operation by its hash.

###### Parameters

###### options

`Omit`<[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions), `"smartAccount"`\> Parameters for getting the user operation.

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\> A promise that resolves to the user operation.

###### Example

```
const userOp = await smartAccount.getUserOperation({
  userOpHash: "0x1234567890123456789012345678901234567890",
});

```

###### listTokenBalances()

```
listTokenBalances: (options: Omit<ListTokenBalancesOptions, "address">) => Promise<ListTokenBalancesResult>;

```

List the token balances of an account.

###### Parameters

###### options

`Omit`<`ListTokenBalancesOptions`, `"address"`\> The options for the list token balances.

###### Returns

`Promise`<`ListTokenBalancesResult`\> The result of the list token balances.

###### Example

```
const balances = await account.listTokenBalances({
  network: "base-sepolia",
});

```

###### name?

The name of the smart account.

###### owners

Array of accounts that own and can sign for the smart account (currently only supports one owner but will be extended to support multiple owners in the future).

###### policies

```
policies: undefined | string[];

```

The list of policy IDs that apply to the smart account. This will include both the project-level policy and the account-level policy, if one exists.

###### quoteFund()

```
quoteFund: (options: Omit<EvmQuoteFundOptions, "address">) => Promise<EvmQuote>;

```

Gets a quote to fund an EVM account.

###### Parameters

###### options

`Omit`<`EvmQuoteFundOptions`, `"address"`\> The options for the quote fund.

###### Returns

`Promise`<`EvmQuote`\> A promise that resolves to a Quote object containing details about the funding operation.

###### Example

```
const quote = await account.quoteFund({
  network: "base",
  token: "usdc",
  amount: 1000000n,
});

```

###### quoteSwap()

```
quoteSwap: (options: SmartAccountQuoteSwapOptions) => Promise<SmartAccountQuoteSwapResult>;

```

Creates a swap quote without executing the transaction. This is useful when you need to get swap details before executing the swap. The taker is automatically set to the smart account’s address.

###### Parameters

###### options

`SmartAccountQuoteSwapOptions` Configuration options for creating the swap quote.

###### Returns

`Promise`<`SmartAccountQuoteSwapResult`\> A promise that resolves to the swap quote or a response indicating that liquidity is unavailable.

###### Example

```
const swapQuote = await smartAccount.quoteSwap({
  network: "base",
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
});
if (swapQuote.liquidityAvailable) {
  console.log(`Can swap for ${swapQuote.toAmount} USDC`);
}

```

###### requestFaucet()

```
requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<RequestFaucetResult>;

```

Requests funds from an EVM faucet.

###### Parameters

###### options

`Omit`<`RequestFaucetOptions`, `"address"`\> Parameters for requesting funds from the EVM faucet.

###### Returns

`Promise`<`RequestFaucetResult`\> A promise that resolves to the transaction hash.

###### Example

```
const result = await account.requestFaucet({
  network: "base-sepolia",
  token: "eth",
});

```

###### sendUserOperation()

```
sendUserOperation: (options: Omit<SendUserOperationOptions<unknown[]>, "smartAccount">) => Promise<SendUserOperationReturnType>;

```

Sends a user operation.

###### Parameters

###### options

`Omit`<`SendUserOperationOptions`<`unknown`\[\]>, `"smartAccount"`\> Parameters for sending the user operation.

###### Returns

`Promise`<`SendUserOperationReturnType`\> A promise that resolves to an object containing the smart account address, the user operation hash, and the status of the user operation.

###### Example

```
const userOp = await smartAccount.sendUserOperation({
  network: "base-sepolia",
  calls: [
    {
      to: "0x1234567890123456789012345678901234567890",
      value: parseEther("0.000001"),
      data: "0x",
    },
  ],
});

```

###### signTypedData()

```
signTypedData: (options: Omit<SignTypedDataOptions, "address"> & {
  network: KnownEvmNetworks;
}) => Promise<`0x${string}`>;

```

Signs a typed data message.

###### Parameters

###### options

`Omit`<[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions), `"address"`\> & { `network`: `KnownEvmNetworks`; } Configuration options for signing the typed data.

###### Returns

`Promise`<`` `0x${string}` ``\> A promise that resolves to the signature.

###### Example

```
const signature = await smartAccount.signTypedData({
  network: "base-sepolia",
  typedData: {
    domain: {
      name: "Test",
      chainId: 84532,
      verifyingContract: "0x0000000000000000000000000000000000000000",
    },
    types: {
      Test: [{ name: "name", type: "string" }],
    },
    primaryType: "Test",
    message: {
      name: "John Doe",
    },
  },
});

```

###### swap()

```
swap: (options: SmartAccountSwapOptions) => Promise<SmartAccountSwapResult>;

```

Executes a token swap on the specified network via a user operation. This method handles all the steps required for a swap, including Permit2 signatures if needed. The taker is automatically set to the smart account’s address.

###### Parameters

###### options

`SmartAccountSwapOptions` Configuration options for the swap.

###### Returns

`Promise`<`SmartAccountSwapResult`\> A promise that resolves to the user operation result.

###### Throws

If liquidity is not available when using inline options.

###### Examples

```
// First create a swap quote
const swapQuote = await cdp.evm.createSwapQuote({
  network: "base",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
  taker: smartAccount.address,
  signerAddress: smartAccount.owners[0].address
});
// Check if liquidity is available
if (!swapQuote.liquidityAvailable) {
  console.error("Insufficient liquidity for swap");
  return;
}
// Execute the swap
const { userOpHash } = await smartAccount.swap({
  swapQuote: swapQuote
});
console.log(`Swap executed with user op hash: ${userOpHash}`);

```

```
// Create and execute swap in one call
const { userOpHash } = await smartAccount.swap({
  network: "base",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH in wei
});
console.log(`Swap executed with user op hash: ${userOpHash}`);

```

###### transfer()

```
transfer: (options: SmartAccountTransferOptions) => Promise<SendUserOperationReturnType>;

```

Transfer an amount of a token from an account to another account.

###### Parameters

###### options

`SmartAccountTransferOptions` The options for the transfer.

###### Returns

`Promise`<`SendUserOperationReturnType`\> The user operation result.

###### Examples

```
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: 10000n, // equivalent to 0.01 USDC
  token: "usdc",
  network: "base-sepolia",
});

```

**Using parseUnits to specify USDC amount**

```
import { parseUnits } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseUnits("0.01", 6), // USDC uses 6 decimal places
  token: "usdc",
  network: "base-sepolia",
});

```

**Transfer ETH**

```
import { parseEther } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseEther("0.000001"),
  token: "eth",
  network: "base-sepolia",
});

```

**Using a contract address**

```
import { parseEther } from "viem";
const { userOpHash } = await sender.transfer({
  to: "0x9F663335Cd6Ad02a37B633602E98866CF944124d",
  amount: parseEther("0.000001"),
  token: "0x4200000000000000000000000000000000000006", // WETH on Base Sepolia
  network: "base-sepolia",
});

```

**Transfer to another account**

```
const sender = await cdp.evm.createAccount({ name: "Sender" });
const receiver = await cdp.evm.createAccount({ name: "Receiver" });
const { userOpHash } = await sender.transfer({
  to: receiver,
  amount: 10000n, // equivalent to 0.01 USDC
  token: "usdc",
  network: "base-sepolia",
});

```

###### type

Identifier for the smart account type.

###### useNetwork()

```
useNetwork: <Network>(network: Network) => Promise<{ [K in string | number | symbol]: (Omit<EvmSmartAccountProperties, "useNetwork"> & { [K in string | number | symbol]: ({ getUserOperation: (options: ...) => ...; sendUserOperation: (options: ...) => ...; waitForUserOperation: (options: ...) => ... } & (Network extends TransferNetworks ? { transfer: ... } : EmptyObject) & (Network extends ListTokenBalancesNetworks ? { listTokenBalances: ... } : EmptyObject) & (Network extends RequestFaucetNetworks ? { requestFaucet: ... } : EmptyObject) & (Network extends "base" ? { quoteFund: ... } : EmptyObject) & (Network extends "base" ? { fund: ...; waitForFundOperationReceipt: ... } : EmptyObject) & (Network extends QuoteSwapNetworks ? { quoteSwap: ... } : EmptyObject) & (Network extends SwapNetworks ? { swap: ... } : EmptyObject) & (Network extends SpendPermissionNetwork ? { useSpendPermission: ... } : EmptyObject))[K] } & { network: Network })[K] }>;

```

A function that returns a network-scoped smart account.

###### Type Parameters

###### Network

`Network` *extends* `KnownEvmNetworks`

###### Parameters

###### network

`Network` The network name or RPC URL

###### Returns

`Promise`<{ \[K in string | number | symbol\]: (Omit<EvmSmartAccountProperties, “useNetwork”> & { \[K in string | number | symbol\]: ({ getUserOperation: (options: …) => …; sendUserOperation: (options: …) => …; waitForUserOperation: (options: …) => … } & (Network extends TransferNetworks ? { transfer: … } : EmptyObject) & (Network extends ListTokenBalancesNetworks ? { listTokenBalances: … } : EmptyObject) & (Network extends RequestFaucetNetworks ? { requestFaucet: … } : EmptyObject) & (Network extends “base” ? { quoteFund: … } : EmptyObject) & (Network extends “base” ? { fund: …; waitForFundOperationReceipt: … } : EmptyObject) & (Network extends QuoteSwapNetworks ? { quoteSwap: … } : EmptyObject) & (Network extends SwapNetworks ? { swap: … } : EmptyObject) & (Network extends SpendPermissionNetwork ? { useSpendPermission: … } : EmptyObject))\[K\] } & { network: Network })\[K\] }>

###### Example

```
// For known networks, type is inferred automatically:
const baseAccount = await smartAccount.useNetwork("base");
// For custom RPC URLs with type hints (requires casting):
const typedAccount = await smartAccount.useNetwork<"base">("https://mainnet.base.org" as "base");
// For custom RPC URLs without type hints (only sendTransaction, transfer and waitForTransactionReceipt methods available):
const customAccount = await smartAccount.useNetwork("https://mainnet.base.org");

```

###### useSpendPermission()

```
useSpendPermission: (options: UseSpendPermissionOptions) => Promise<SendUserOperationReturnType>;

```

Uses a spend permission to execute a transaction via user operation. This allows the smart account to spend tokens that have been approved via a spend permission.

###### Parameters

###### options

`UseSpendPermissionOptions` Configuration options for using the spend permission.

###### Returns

`Promise`<`SendUserOperationReturnType`\> A promise that resolves to the user operation result.

###### Throws

If the network doesn’t support spend permissions via CDP API.

###### Example

```
const spendPermission = {
  account: "0x1234...", // Smart account that owns the tokens
  spender: smartAccount.address, // This smart account that can spend
  token: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE", // ETH
  allowance: parseEther("0.01"),
  period: 86400, // 1 day
  start: 0,
  end: 281474976710655,
  salt: 0n,
  extraData: "0x",
};
const result = await smartAccount.useSpendPermission({
  spendPermission,
  value: parseEther("0.001"), // Spend 0.001 ETH
  network: "base-sepolia",
});

```

###### waitForUserOperation()

```
waitForUserOperation: (options: Omit<WaitForUserOperationOptions, "smartAccountAddress">) => Promise<WaitForUserOperationReturnType>;

```

Waits for a user operation to complete or fail.

###### Parameters

###### options

`Omit`<`WaitForUserOperationOptions`, `"smartAccountAddress"`\> Parameters for waiting for the user operation.

###### Returns

`Promise`<`WaitForUserOperationReturnType`\> A promise that resolves to the transaction receipt.

###### Example

```
// Send a user operation and get the user operation hash
const { userOpHash } = await smartAccount.sendUserOperation({
  network: "base-sepolia",
  calls: [
    {
      to: "0x0000000000000000000000000000000000000000",
      value: parseEther("0.000001"),
      data: "0x",
    },
  ],
});
// Wait for the user operation to complete or fail
const result = await smartAccount.waitForUserOperation({
  userOpHash: userOp.userOpHash,
});

```

###### waitForFundOperationReceipt()

```
waitForFundOperationReceipt(options: WaitForFundOperationOptions): Promise<WaitForFundOperationResult>;

```

Waits for a fund operation to complete and returns the transfer receipt.

###### Parameters

###### options

`WaitForFundOperationOptions` The options for the wait for fund operation.

###### Returns

`Promise`<`WaitForFundOperationResult`\> A promise that resolves to the completed transfer receipt containing details about the funding operation.

###### Example

```
const completedTransfer = await account.waitForFundOperationReceipt({
  transferId: "transfer_123",
});

```

* * *

### ReadonlySmartAccount

Defined in: [src/client/evm/evm.types.ts:429](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L429) A smart account that only contains the owner address.

#### Extends

-   `Omit`<[`EvmSmartAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmsmartaccount), | `"owners"` | keyof [`SmartAccountActions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Actions#smartaccountactions) | `"useNetwork"`\>

#### Properties

##### address

Defined in: [src/accounts/evm/types.ts:150](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/accounts/evm/types.ts#L150) The smart account’s address.

###### Inherited from

##### name?

Defined in: [src/accounts/evm/types.ts:152](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/accounts/evm/types.ts#L152) The name of the smart account.

###### Inherited from

##### owners

Defined in: [src/client/evm/evm.types.ts:432](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L432) The owners of the smart account.

##### policies

```
policies: undefined | string[];

```

Defined in: [src/accounts/evm/types.ts:158](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/accounts/evm/types.ts#L158) The list of policy IDs that apply to the smart account. This will include both the project-level policy and the account-level policy, if one exists.

###### Inherited from

##### type

Defined in: [src/accounts/evm/types.ts:156](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/accounts/evm/types.ts#L156) Identifier for the smart account type.

###### Inherited from

* * *

### SignatureResult

Defined in: [src/client/evm/evm.types.ts:570](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L570) A signature result.

#### Properties

##### signature

```
signature: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:572](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L572) The signature.

* * *

### SignHashOptions

Defined in: [src/client/evm/evm.types.ts:516](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L516) Options for signing an EVM hash.

#### Properties

##### address

Defined in: [src/client/evm/evm.types.ts:518](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L518) The address of the account.

##### hash

Defined in: [src/client/evm/evm.types.ts:520](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L520) The hash to sign.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:522](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L522) The idempotency key.

* * *

### SignMessageOptions

Defined in: [src/client/evm/evm.types.ts:528](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L528) Options for signing an EVM message.

#### Properties

##### address

Defined in: [src/client/evm/evm.types.ts:530](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L530) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:534](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L534) The idempotency key.

##### message

Defined in: [src/client/evm/evm.types.ts:532](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L532) The message to sign.

* * *

### SignTransactionOptions

Defined in: [src/client/evm/evm.types.ts:558](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L558) Options for signing an EVM transaction.

#### Properties

##### address

Defined in: [src/client/evm/evm.types.ts:560](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L560) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:564](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L564) The idempotency key.

##### transaction

```
transaction: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:562](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L562) The RLP-encoded transaction to sign, as a 0x-prefixed hex string.

* * *

### SignTypedDataOptions

Defined in: [src/client/evm/evm.types.ts:540](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L540) Options for signing an EVM typed data message.

#### Properties

##### address

Defined in: [src/client/evm/evm.types.ts:542](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L542) The address of the account.

##### domain

Defined in: [src/client/evm/evm.types.ts:544](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L544) The domain of the message.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:552](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L552) The idempotency key.

##### message

```
message: EIP712MessageMessage;

```

Defined in: [src/client/evm/evm.types.ts:550](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L550) The message to sign. The structure of this message must match the `primaryType` struct in the `types` object.

##### primaryType

Defined in: [src/client/evm/evm.types.ts:548](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L548) The primary type of the message. This is the name of the struct in the `types` object that is the root of the message.

##### types

Defined in: [src/client/evm/evm.types.ts:546](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L546) The types of the message.

* * *

### SmartAccountSignAndWrapTypedDataOptions

Defined in: [src/client/evm/evm.types.ts:657](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L657) Options for signing and wrapping EIP-712 typed data with a smart account. This method handles the full smart account signature flow including replay-safe hashing.

#### Properties

##### chainId

Defined in: [src/client/evm/evm.types.ts:659](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L659) The chain ID for the signature (used for replay protection).

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:665](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L665) Optional idempotency key for the signing request.

##### ownerIndex?

```
optional ownerIndex: bigint;

```

Defined in: [src/client/evm/evm.types.ts:663](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L663) The index of the owner to sign with (defaults to 0).

##### typedData

```
typedData: EIP712Message;

```

Defined in: [src/client/evm/evm.types.ts:661](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L661) The EIP-712 typed data message to sign.

* * *

### SwapAllowanceIssue

Defined in: [src/client/evm/evm.types.ts:598](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L598) Details of allowance issues for a swap.

#### Properties

##### currentAllowance

```
currentAllowance: bigint;

```

Defined in: [src/client/evm/evm.types.ts:600](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L600) The current allowance of the fromToken by the taker.

##### spender

Defined in: [src/client/evm/evm.types.ts:602](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L602) The address to set the allowance on.

* * *

### SwapBalanceIssue

Defined in: [src/client/evm/evm.types.ts:608](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L608) Details of balance issues for a swap.

#### Properties

##### currentBalance

Defined in: [src/client/evm/evm.types.ts:612](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L612) The current balance of the fromToken by the taker.

##### requiredBalance

Defined in: [src/client/evm/evm.types.ts:614](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L614) The amount of the token that the taker must hold.

##### token

Defined in: [src/client/evm/evm.types.ts:610](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L610) The contract address of the token.

* * *

### SwapFees

Defined in: [src/client/evm/evm.types.ts:588](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L588) The estimated fees for a swap.

#### Properties

##### gasFee?

```
optional gasFee: TokenFee;

```

Defined in: [src/client/evm/evm.types.ts:590](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L590) The estimated gas fee for the swap.

##### protocolFee?

```
optional protocolFee: TokenFee;

```

Defined in: [src/client/evm/evm.types.ts:592](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L592) The estimated protocol fee for the swap.

* * *

### SwapIssues

Defined in: [src/client/evm/evm.types.ts:620](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L620) Potential issues discovered during swap validation.

#### Properties

##### allowance?

```
optional allowance: SwapAllowanceIssue;

```

Defined in: [src/client/evm/evm.types.ts:622](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L622) Details of the allowances that the taker must set. Null if no allowance is required.

##### balance?

```
optional balance: SwapBalanceIssue;

```

Defined in: [src/client/evm/evm.types.ts:624](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L624) Details of the balance of the fromToken that the taker must hold. Null if sufficient balance.

##### simulationIncomplete

```
simulationIncomplete: boolean;

```

Defined in: [src/client/evm/evm.types.ts:626](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L626) True when the transaction cannot be validated (e.g., insufficient balance).

* * *

### SwapUnavailableResult

Defined in: [src/client/evm/evm.types.ts:194](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L194) Result when liquidity is unavailable for a swap.

#### Properties

##### liquidityAvailable

```
liquidityAvailable: false;

```

Defined in: [src/client/evm/evm.types.ts:196](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L196) Whether liquidity is available for the swap.

* * *

### TokenFee

Defined in: [src/client/evm/evm.types.ts:578](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L578) A fee in a specific token.

#### Properties

##### amount

Defined in: [src/client/evm/evm.types.ts:580](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L580) The amount of the fee in atomic units of the token.

##### token

Defined in: [src/client/evm/evm.types.ts:582](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L582) The contract address of the token that the fee is paid in.

* * *

### UpdateEvmAccountOptions

Defined in: [src/client/evm/evm.types.ts:438](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L438) Options for creating an EVM server account.

#### Properties

##### address

Defined in: [src/client/evm/evm.types.ts:440](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L440) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:444](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L444) The idempotency key.

##### update

```
update: UpdateEvmAccountBody;

```

Defined in: [src/client/evm/evm.types.ts:442](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L442) The updates to apply to the account

* * *

### UpdateEvmSmartAccountOptions

Defined in: [src/client/evm/evm.types.ts:450](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L450) Options for updating an EVM smart account.

#### Properties

##### address

Defined in: [src/client/evm/evm.types.ts:452](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L452) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [src/client/evm/evm.types.ts:456](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L456) The idempotency key.

##### owner

Defined in: [src/client/evm/evm.types.ts:458](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L458) The owner of the account.

##### update

```
update: UpdateEvmSmartAccountBody;

```

Defined in: [src/client/evm/evm.types.ts:454](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L454) The updates to apply to the account

* * *

### UserOperation

Defined in: [src/client/evm/evm.types.ts:315](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L315) A user operation.

#### Properties

##### calls

```
calls: readonly {
  data: `0x${string}`;
  to: `0x${string}`;
  value: bigint;
}[];

```

Defined in: [src/client/evm/evm.types.ts:323](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L323) The list of calls in the user operation.

##### network

```
network: EvmUserOperationNetwork;

```

Defined in: [src/client/evm/evm.types.ts:317](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L317) The network the user operation is for.

##### receipts?

```
optional receipts: UserOperationReceipt[];

```

Defined in: [src/client/evm/evm.types.ts:333](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L333) The receipts associated with the broadcasted user operation.

##### status

```
status: EvmUserOperationStatus;

```

Defined in: [src/client/evm/evm.types.ts:325](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L325) The status of the user operation.

##### transactionHash?

```
optional transactionHash: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:329](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L329) The hash of the transaction that included this particular user operation. This gets set after the user operation is broadcasted and the transaction is included in a block.

##### userOpHash

```
userOpHash: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:321](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L321) The hash of the user operation. This is not the transaction hash, as a transaction consists of multiple user operations. The user operation hash is the hash of this particular user operation which gets signed by the owner of the Smart Account.

* * *

### WaitForUserOperationOptions

Defined in: [src/client/evm/evm.types.ts:632](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L632) Options for waiting for a user operation.

#### Properties

##### smartAccountAddress

```
smartAccountAddress: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:634](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L634) The smart account address.

##### userOpHash

```
userOpHash: `0x${string}`;

```

Defined in: [src/client/evm/evm.types.ts:636](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L636) The user operation hash.

##### waitOptions?

```
optional waitOptions: WaitOptions;

```

Defined in: [src/client/evm/evm.types.ts:638](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L638) The wait options.

## Type Aliases

### CreateSwapOptions

```
type CreateSwapOptions = CreateSwapQuoteOptions;

```

Defined in: [src/client/evm/evm.types.ts:646](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L646) Legacy type aliases for backwards compatibility.

#### Deprecated

Use the new type names instead.

* * *

### CreateSwapResult

```
type CreateSwapResult = CreateSwapQuoteResult;

```

Defined in: [src/client/evm/evm.types.ts:647](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L647)

* * *

### EvmClientInterface

```
type EvmClientInterface = Omit<typeof OpenApiEvmMethods, 
  | "createEvmAccount"
  | "createEvmSmartAccount"
  | "createSpendPermission"
  | "listSpendPermissions"
  | "revokeSpendPermission"
  | "importEvmAccount"
  | "exportEvmAccount"
  | "exportEvmAccountByName"
  | "getEvmAccount"
  | "getEvmAccountByName"
  | "getEvmSmartAccount"
  | "getEvmSmartAccountByName"
  | "getEvmSwapPrice"
  | "createEvmSwapQuote"
  | "getUserOperation"
  | "updateEvmAccount"
  | "listEvmAccounts"
  | "listEvmSmartAccounts"
  | "listEvmTokenBalances"
  | "prepareUserOperation"
  | "requestEvmFaucet"
  | "sendUserOperation"
  | "signEvmHash"
  | "signEvmMessage"
  | "signEvmTransaction"
  | "signEvmTypedData"
  | "sendEvmTransaction"
  | "signEvmTypedData"
  | "updateEvmAccount"
  | "exportEvmAccount"
  | "exportEvmAccountByName"
  | "updateEvmSmartAccount"> & {
  createAccount: (options: CreateServerAccountOptions) => Promise<EvmServerAccount>;
  createSmartAccount: (options: CreateSmartAccountOptions) => Promise<EvmSmartAccount>;
  createSwapQuote: (options: CreateSwapQuoteOptions) => Promise<
     | CreateSwapQuoteResult
    | SwapUnavailableResult>;
  exportAccount: (options: ExportServerAccountOptions) => Promise<string>;
  getAccount: (options: GetServerAccountOptions) => Promise<EvmServerAccount>;
  getOrCreateAccount: (options: GetOrCreateServerAccountOptions) => Promise<EvmServerAccount>;
  getSmartAccount: (options: GetSmartAccountOptions) => Promise<EvmSmartAccount>;
  getSwapPrice: (options: GetSwapPriceOptions) => Promise<
     | GetSwapPriceResult
    | SwapUnavailableResult>;
  getUserOperation: (options: GetUserOperationOptions) => Promise<UserOperation>;
  importAccount: (options: ImportServerAccountOptions) => Promise<EvmServerAccount>;
  listAccounts: (options: ListServerAccountsOptions) => Promise<ListServerAccountResult>;
  listSmartAccounts: (options: ListSmartAccountsOptions) => Promise<ListSmartAccountResult>;
  listSpendPermissions: (options: ListSpendPermissionsOptions) => Promise<ListSpendPermissionsResult>;
  listTokenBalances: (options: ListTokenBalancesOptions) => Promise<ListTokenBalancesResult>;
  prepareUserOperation: (options: PrepareUserOperationOptions) => Promise<UserOperation>;
  requestFaucet: (options: RequestFaucetOptions) => Promise<RequestFaucetResult>;
  sendTransaction: (options: SendTransactionOptions) => Promise<TransactionResult>;
  sendUserOperation: (options: SendUserOperationOptions<unknown[]>) => Promise<SendUserOperationReturnType>;
  signHash: (options: SignHashOptions) => Promise<SignatureResult>;
  signMessage: (options: SignMessageOptions) => Promise<SignatureResult>;
  signTransaction: (options: SignTransactionOptions) => Promise<SignatureResult>;
  signTypedData: (options: SignTypedDataOptions) => Promise<SignatureResult>;
  updateAccount: (options: UpdateEvmAccountOptions) => Promise<EvmServerAccount>;
  updateSmartAccount: (options: UpdateEvmSmartAccountOptions) => Promise<EvmSmartAccount>;
};

```

Defined in: [src/client/evm/evm.types.ts:42](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L42) The EvmClient type, where all OpenApiEvmMethods methods are wrapped.

#### Type declaration

##### createAccount()

```
createAccount: (options: CreateServerAccountOptions) => Promise<EvmServerAccount>;

```

###### Parameters

###### options

[`CreateServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#createserveraccountoptions)

###### Returns

`Promise`<[`EvmServerAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmserveraccount)\>

##### createSmartAccount()

```
createSmartAccount: (options: CreateSmartAccountOptions) => Promise<EvmSmartAccount>;

```

###### Parameters

###### options

[`CreateSmartAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#createsmartaccountoptions)

###### Returns

`Promise`<[`EvmSmartAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmsmartaccount)\>

##### createSwapQuote()

```
createSwapQuote: (options: CreateSwapQuoteOptions) => Promise<
  | CreateSwapQuoteResult
| SwapUnavailableResult>;

```

###### Parameters

###### options

[`CreateSwapQuoteOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#createswapquoteoptions)

###### Returns

`Promise`< | [`CreateSwapQuoteResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#createswapquoteresult) | [`SwapUnavailableResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#swapunavailableresult)\>

##### exportAccount()

```
exportAccount: (options: ExportServerAccountOptions) => Promise<string>;

```

###### Parameters

###### options

[`ExportServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#exportserveraccountoptions)

###### Returns

`Promise`<`string`\>

##### getAccount()

```
getAccount: (options: GetServerAccountOptions) => Promise<EvmServerAccount>;

```

###### Parameters

###### options

[`GetServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getserveraccountoptions)

###### Returns

`Promise`<[`EvmServerAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmserveraccount)\>

##### getOrCreateAccount()

```
getOrCreateAccount: (options: GetOrCreateServerAccountOptions) => Promise<EvmServerAccount>;

```

###### Parameters

###### options

[`GetOrCreateServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getorcreateserveraccountoptions)

###### Returns

`Promise`<[`EvmServerAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmserveraccount)\>

##### getSmartAccount()

```
getSmartAccount: (options: GetSmartAccountOptions) => Promise<EvmSmartAccount>;

```

###### Parameters

###### options

[`GetSmartAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getsmartaccountoptions)

###### Returns

`Promise`<[`EvmSmartAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmsmartaccount)\>

##### getSwapPrice()

```
getSwapPrice: (options: GetSwapPriceOptions) => Promise<
  | GetSwapPriceResult
| SwapUnavailableResult>;

```

###### Parameters

###### options

[`GetSwapPriceOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getswappriceoptions)

###### Returns

`Promise`< | [`GetSwapPriceResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getswappriceresult) | [`SwapUnavailableResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#swapunavailableresult)\>

##### getUserOperation()

```
getUserOperation: (options: GetUserOperationOptions) => Promise<UserOperation>;

```

###### Parameters

###### options

[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions)

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\>

##### importAccount()

```
importAccount: (options: ImportServerAccountOptions) => Promise<EvmServerAccount>;

```

###### Parameters

###### options

[`ImportServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#importserveraccountoptions)

###### Returns

`Promise`<[`EvmServerAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmserveraccount)\>

##### listAccounts()

```
listAccounts: (options: ListServerAccountsOptions) => Promise<ListServerAccountResult>;

```

###### Parameters

###### options

[`ListServerAccountsOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#listserveraccountsoptions)

###### Returns

`Promise`<[`ListServerAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#listserveraccountresult)\>

##### listSmartAccounts()

```
listSmartAccounts: (options: ListSmartAccountsOptions) => Promise<ListSmartAccountResult>;

```

###### Parameters

###### options

[`ListSmartAccountsOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#listsmartaccountsoptions)

###### Returns

`Promise`<[`ListSmartAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#listsmartaccountresult)\>

##### listSpendPermissions()

```
listSpendPermissions: (options: ListSpendPermissionsOptions) => Promise<ListSpendPermissionsResult>;

```

###### Parameters

###### options

`ListSpendPermissionsOptions`

###### Returns

`Promise`<`ListSpendPermissionsResult`\>

##### listTokenBalances()

```
listTokenBalances: (options: ListTokenBalancesOptions) => Promise<ListTokenBalancesResult>;

```

###### Parameters

###### options

`ListTokenBalancesOptions`

###### Returns

`Promise`<`ListTokenBalancesResult`\>

##### prepareUserOperation()

```
prepareUserOperation: (options: PrepareUserOperationOptions) => Promise<UserOperation>;

```

###### Parameters

###### options

[`PrepareUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#prepareuseroperationoptions)

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\>

##### requestFaucet()

```
requestFaucet: (options: RequestFaucetOptions) => Promise<RequestFaucetResult>;

```

###### Parameters

###### options

`RequestFaucetOptions`

###### Returns

`Promise`<`RequestFaucetResult`\>

##### sendTransaction()

```
sendTransaction: (options: SendTransactionOptions) => Promise<TransactionResult>;

```

###### Parameters

###### options

`SendTransactionOptions`

###### Returns

`Promise`<`TransactionResult`\>

##### sendUserOperation()

```
sendUserOperation: (options: SendUserOperationOptions<unknown[]>) => Promise<SendUserOperationReturnType>;

```

###### Parameters

###### options

`SendUserOperationOptions`<`unknown`\[\]>

###### Returns

`Promise`<`SendUserOperationReturnType`\>

##### signHash()

```
signHash: (options: SignHashOptions) => Promise<SignatureResult>;

```

###### Parameters

###### options

[`SignHashOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signhashoptions)

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signatureresult)\>

##### signMessage()

```
signMessage: (options: SignMessageOptions) => Promise<SignatureResult>;

```

###### Parameters

###### options

[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signmessageoptions)

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signatureresult)\>

##### signTransaction()

```
signTransaction: (options: SignTransactionOptions) => Promise<SignatureResult>;

```

###### Parameters

###### options

[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtransactionoptions)

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signatureresult)\>

##### signTypedData()

```
signTypedData: (options: SignTypedDataOptions) => Promise<SignatureResult>;

```

###### Parameters

###### options

[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions)

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signatureresult)\>

##### updateAccount()

```
updateAccount: (options: UpdateEvmAccountOptions) => Promise<EvmServerAccount>;

```

###### Parameters

###### options

[`UpdateEvmAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#updateevmaccountoptions)

###### Returns

`Promise`<[`EvmServerAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmserveraccount)\>

##### updateSmartAccount()

```
updateSmartAccount: (options: UpdateEvmSmartAccountOptions) => Promise<EvmSmartAccount>;

```

###### Parameters

###### options

[`UpdateEvmSmartAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#updateevmsmartaccountoptions)

###### Returns

`Promise`<[`EvmSmartAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmsmartaccount)\>

* * *

### GetSwapQuoteOptions

```
type GetSwapQuoteOptions = GetSwapPriceOptions;

```

Defined in: [src/client/evm/evm.types.ts:648](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L648)

* * *

### GetSwapQuoteResult

```
type GetSwapQuoteResult = GetSwapPriceResult;

```

Defined in: [src/client/evm/evm.types.ts:649](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L649)

* * *

### SwapPriceUnavailableResult

```
type SwapPriceUnavailableResult = SwapUnavailableResult;

```

Defined in: [src/client/evm/evm.types.ts:651](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L651)

* * *

### SwapQuoteUnavailableResult

```
type SwapQuoteUnavailableResult = SwapUnavailableResult;

```

Defined in: [src/client/evm/evm.types.ts:650](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.types.ts#L650)

## References

### ServerAccount

Renames and re-exports [EvmServerAccount](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmserveraccount)

* * *

### SmartAccount

Renames and re-exports [EvmSmartAccount](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmsmartaccount)