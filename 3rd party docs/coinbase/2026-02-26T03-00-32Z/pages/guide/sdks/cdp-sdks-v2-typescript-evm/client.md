# client

## Classes

### EvmClient

Defined in: [src/client/evm/evm.ts:97](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L97) The namespace containing all EVM methods.

#### Implements

-   [`EvmClientInterface`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#evmclientinterface)

#### Constructors

##### Constructor

```
new EvmClient(): EvmClient;

```

###### Returns

[`EvmClient`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Client#evmclient)

#### Methods

##### createAccount()

```
createAccount(options?: CreateServerAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:132](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L132) Creates a new CDP EVM account.

###### Parameters

###### options?

[`CreateServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#createserveraccountoptions) = `{}` Optional parameters for creating the account.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `policies?`: `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `AccountQuoteSwapOptions`) => `Promise`<`AccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendTransaction`: (`options`: `Omit`<`SendTransactionOptions`, `"address"`\>) => `Promise`<`TransactionResult`\>; `sign`: (`parameters`: { `hash`: `` `0x${string}` ``; }) => `Promise`<`` `0x${string}` ``\>; `signMessage`: (`parameters`: { `message`: `SignableMessage`; }) => `Promise`<`` `0x${string}` ``\>; `signTransaction`: (`transaction`: `TransactionSerializable`) => `Promise`<`` `0x${string}` ``\>; `signTypedData`: <`typedData`, `primaryType`\>(`parameters`: `TypedDataDefinition`<`typedData`, `primaryType`\>) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `AccountSwapOptions`) => `Promise`<`SendSwapTransactionResult`\>; `transfer`: (`options`: `TransferOptions`) => `Promise`<{ `transactionHash`: `` `0x${string}` ``; }>; `type`: `"evm-server"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<{ address: \`0x\\{string\\}\\\`; fund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; listTokenBalances: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; name?: string; policies?: (...)\\\[\\\]; quoteFund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; quoteSwap: (options: AccountQuoteSwapOptions) =\\> Promise\\<(...)\\>; requestFaucet: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sendTransaction: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sign: (parameters: \\{ hash: ... \\}) =\\> Promise\\<(...)\\>; signMessage: (parameters: \\{ message: ... \\}) =\\> Promise\\<(...)\\>; signTransaction: (transaction: TransactionSerializable) =\\> Promise\\<(...)\\>; signTypedData: (parameters: TypedDataDefinition\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; swap: (options: AccountSwapOptions) =\\> Promise\\<(...)\\>; transfer: (options: TransferOptions) =\\> Promise\\<(...)\\>; type: "evm-server"; useNetwork: \\<Network extends NetworkOrRpcUrl\\>(network: Network) =\\> Promise\\<\\{ \\\[K in keyof (Omit\\<\\{ address: \\\`0x{string}\`; sign: (parameters: { hash: \`0x{string};ˋ})\= Promise\\<0ˋx\\{string\\}\\\`; \\}) =\\> Promise\\<\\\`0x{string}\`>; signMessage: (parameters: { message: SignableMessage; }) => Promise<…>; … 15 more …; useNetwork: …; }, “transfer” | … 5 more …; useSpendPermission: (options: UseSpendPermissionOptions) => Promise<(…)>; waitForFundOperationReceipt: any }, “transfer” | “sendTransaction” | “quoteSwap” | “swap” | “useSpendPermission” | “useNetwork” | (keyof Actions)> & { \[K in string | number | symbol\]: ({ sendTransaction: …; transfer: …; waitForTransactionReceipt: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`TransactionResult`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the newly created account.

###### Examples

```
         const account = await cdp.evm.createAccount();

```

```
         const account = await cdp.evm.createAccount({ name: "MyAccount" });

```

```
         const idempotencyKey = uuidv4();
         // First call
         await cdp.evm.createAccount({
           idempotencyKey,
         });
         // Second call with the same idempotency key will return the same account
         await cdp.evm.createAccount({
           idempotencyKey,
         });

```

###### Implementation of

```
EvmClientInterface.createAccount

```

##### createSmartAccount()

```
createSmartAccount(options: CreateSmartAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:339](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L339) Creates a new CDP EVM smart account.

###### Parameters

###### options

[`CreateSmartAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#createsmartaccountoptions) Parameters for creating the smart account.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `getUserOperation`: (`options`: `Omit`<[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions), `"smartAccount"`\>) => `Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `owners`: [`EvmAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmaccount)\[\]; `policies`: `undefined` | `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `SmartAccountQuoteSwapOptions`) => `Promise`<`SmartAccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendUserOperation`: (`options`: `Omit`<`SendUserOperationOptions`<`unknown`\[\]>, `"smartAccount"`\>) => `Promise`<`SendUserOperationReturnType`\>; `signTypedData`: (`options`: `Omit`<[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions), `"address"`\> & { `network`: `KnownEvmNetworks`; }) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `SmartAccountSwapOptions`) => `Promise`<`SmartAccountSwapResult`\>; `transfer`: (`options`: `SmartAccountTransferOptions`) => `Promise`<`SendUserOperationReturnType`\>; `type`: `"evm-smart"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<EvmSmartAccountProperties, “useNetwork”> & { \[K in string | number | symbol\]: ({ getUserOperation: …; sendUserOperation: …; waitForUserOperation: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`SendUserOperationReturnType`\>; `waitForUserOperation`: (`options`: `Omit`<`WaitForUserOperationOptions`, `"smartAccountAddress"`\>) => `Promise`<`WaitForUserOperationReturnType`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the newly created smart account.

###### Examples

```
         const account = await cdp.evm.createAccount();
         const smartAccount = await cdp.evm.createSmartAccount({
           owner: account,
         });

```

```
         // See https://viem.sh/docs/accounts/local/privateKeyToAccount
         const privateKey = generatePrivateKey();
         const account = privateKeyToAccount(privateKey);
         const smartAccount = await client.evm.createSmartAccount({
           owner: account,
         });

```

```
         const idempotencyKey = uuidv4();
         // First call
         await cdp.evm.createSmartAccount({
           owner: account,
           idempotencyKey,
         });
         // Second call with the same idempotency key will return the same smart account
         await cdp.evm.createSmartAccount({
           owner: account,
           idempotencyKey,

```

###### Implementation of

```
EvmClientInterface.createSmartAccount

```

##### createSpendPermission()

```
createSpendPermission(options: CreateSpendPermissionOptions): Promise<UserOperation>;

```

Defined in: [src/client/evm/evm.ts:365](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L365) Creates a spend permission for a smart account.

###### Parameters

###### options

`CreateSpendPermissionOptions` Parameters for creating the spend permission.

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\> A promise that resolves to the spend permission.

###### Example

```
const userOperation = await cdp.evm.createSpendPermission({
  spendPermission,
  network: "base-sepolia",
});

```

##### createSwapQuote()

```
createSwapQuote(options: CreateSwapQuoteOptions): Promise<
  | CreateSwapQuoteResult
| SwapUnavailableResult>;

```

Defined in: [src/client/evm/evm.ts:658](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L658) Creates a quote for a swap between two tokens on an EVM network.

###### Parameters

###### options

[`CreateSwapQuoteOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#createswapquoteoptions) The options for creating a swap quote.

###### Returns

`Promise`< | [`CreateSwapQuoteResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#createswapquoteresult) | [`SwapUnavailableResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#swapunavailableresult)\> A promise that resolves to the swap quote result or a response indicating that liquidity is unavailable.

###### Example

```
const swapQuote = await cdp.evm.createSwapQuote({
  network: "ethereum",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH
  taker: "0x1234567890123456789012345678901234567890"
});

```

###### Implementation of

```
EvmClientInterface.createSwapQuote

```

##### exportAccount()

```
exportAccount(options: ExportServerAccountOptions): Promise<string>;

```

Defined in: [src/client/evm/evm.ts:259](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L259) Exports a CDP EVM account’s private key. It is important to store the private key in a secure place after it’s exported.

###### Parameters

###### options

[`ExportServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#exportserveraccountoptions) Parameters for exporting the account.

###### Returns

`Promise`<`string`\> A promise that resolves to the exported account’s 32-byte private key as a hex string, without the “0x” prefix.

###### Examples

```
const privateKey = await cdp.evm.exportAccount({
  address: "0x1234567890123456789012345678901234567890",
});

```

```
const privateKey = await cdp.evm.exportAccount({
  name: "MyAccount",
});

```

###### Implementation of

```
EvmClientInterface.exportAccount

```

##### getAccount()

```
getAccount(options: GetServerAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:476](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L476) Gets a CDP EVM account.

###### Parameters

###### options

[`GetServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getserveraccountoptions) Parameters for getting the account. Either `address` or `name` must be provided. If both are provided, lookup will be done by `address` and `name` will be ignored.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `policies?`: `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `AccountQuoteSwapOptions`) => `Promise`<`AccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendTransaction`: (`options`: `Omit`<`SendTransactionOptions`, `"address"`\>) => `Promise`<`TransactionResult`\>; `sign`: (`parameters`: { `hash`: `` `0x${string}` ``; }) => `Promise`<`` `0x${string}` ``\>; `signMessage`: (`parameters`: { `message`: `SignableMessage`; }) => `Promise`<`` `0x${string}` ``\>; `signTransaction`: (`transaction`: `TransactionSerializable`) => `Promise`<`` `0x${string}` ``\>; `signTypedData`: <`typedData`, `primaryType`\>(`parameters`: `TypedDataDefinition`<`typedData`, `primaryType`\>) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `AccountSwapOptions`) => `Promise`<`SendSwapTransactionResult`\>; `transfer`: (`options`: `TransferOptions`) => `Promise`<{ `transactionHash`: `` `0x${string}` ``; }>; `type`: `"evm-server"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<{ address: \`0x\\{string\\}\\\`; fund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; listTokenBalances: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; name?: string; policies?: (...)\\\[\\\]; quoteFund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; quoteSwap: (options: AccountQuoteSwapOptions) =\\> Promise\\<(...)\\>; requestFaucet: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sendTransaction: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sign: (parameters: \\{ hash: ... \\}) =\\> Promise\\<(...)\\>; signMessage: (parameters: \\{ message: ... \\}) =\\> Promise\\<(...)\\>; signTransaction: (transaction: TransactionSerializable) =\\> Promise\\<(...)\\>; signTypedData: (parameters: TypedDataDefinition\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; swap: (options: AccountSwapOptions) =\\> Promise\\<(...)\\>; transfer: (options: TransferOptions) =\\> Promise\\<(...)\\>; type: "evm-server"; useNetwork: \\<Network extends NetworkOrRpcUrl\\>(network: Network) =\\> Promise\\<\\{ \\\[K in keyof (Omit\\<\\{ address: \\\`0x{string}\`; sign: (parameters: { hash: \`0x{string};ˋ})\= Promise\\<0ˋx\\{string\\}\\\`; \\}) =\\> Promise\\<\\\`0x{string}\`>; signMessage: (parameters: { message: SignableMessage; }) => Promise<…>; … 15 more …; useNetwork: …; }, “transfer” | … 5 more …; useSpendPermission: (options: UseSpendPermissionOptions) => Promise<(…)>; waitForFundOperationReceipt: any }, “transfer” | “sendTransaction” | “quoteSwap” | “swap” | “useSpendPermission” | “useNetwork” | (keyof Actions)> & { \[K in string | number | symbol\]: ({ sendTransaction: …; transfer: …; waitForTransactionReceipt: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`TransactionResult`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the account.

###### Examples

```
         const account = await cdp.evm.getAccount({
           address: "0x1234567890123456789012345678901234567890",
         });

```

```
         const account = await cdp.evm.getAccount({
           name: "MyAccount",
         });

```

###### Implementation of

```
EvmClientInterface.getAccount

```

##### getOrCreateAccount()

```
getOrCreateAccount(options: GetOrCreateServerAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:530](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L530) Gets a CDP EVM account, or creates one if it doesn’t exist.

###### Parameters

###### options

[`GetOrCreateServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getorcreateserveraccountoptions) Parameters for getting or creating the account.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `policies?`: `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `AccountQuoteSwapOptions`) => `Promise`<`AccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendTransaction`: (`options`: `Omit`<`SendTransactionOptions`, `"address"`\>) => `Promise`<`TransactionResult`\>; `sign`: (`parameters`: { `hash`: `` `0x${string}` ``; }) => `Promise`<`` `0x${string}` ``\>; `signMessage`: (`parameters`: { `message`: `SignableMessage`; }) => `Promise`<`` `0x${string}` ``\>; `signTransaction`: (`transaction`: `TransactionSerializable`) => `Promise`<`` `0x${string}` ``\>; `signTypedData`: <`typedData`, `primaryType`\>(`parameters`: `TypedDataDefinition`<`typedData`, `primaryType`\>) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `AccountSwapOptions`) => `Promise`<`SendSwapTransactionResult`\>; `transfer`: (`options`: `TransferOptions`) => `Promise`<{ `transactionHash`: `` `0x${string}` ``; }>; `type`: `"evm-server"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<{ address: \`0x\\{string\\}\\\`; fund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; listTokenBalances: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; name?: string; policies?: (...)\\\[\\\]; quoteFund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; quoteSwap: (options: AccountQuoteSwapOptions) =\\> Promise\\<(...)\\>; requestFaucet: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sendTransaction: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sign: (parameters: \\{ hash: ... \\}) =\\> Promise\\<(...)\\>; signMessage: (parameters: \\{ message: ... \\}) =\\> Promise\\<(...)\\>; signTransaction: (transaction: TransactionSerializable) =\\> Promise\\<(...)\\>; signTypedData: (parameters: TypedDataDefinition\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; swap: (options: AccountSwapOptions) =\\> Promise\\<(...)\\>; transfer: (options: TransferOptions) =\\> Promise\\<(...)\\>; type: "evm-server"; useNetwork: \\<Network extends NetworkOrRpcUrl\\>(network: Network) =\\> Promise\\<\\{ \\\[K in keyof (Omit\\<\\{ address: \\\`0x{string}\`; sign: (parameters: { hash: \`0x{string};ˋ})\= Promise\\<0ˋx\\{string\\}\\\`; \\}) =\\> Promise\\<\\\`0x{string}\`>; signMessage: (parameters: { message: SignableMessage; }) => Promise<…>; … 15 more …; useNetwork: …; }, “transfer” | … 5 more …; useSpendPermission: (options: UseSpendPermissionOptions) => Promise<(…)>; waitForFundOperationReceipt: any }, “transfer” | “sendTransaction” | “quoteSwap” | “swap” | “useSpendPermission” | “useNetwork” | (keyof Actions)> & { \[K in string | number | symbol\]: ({ sendTransaction: …; transfer: …; waitForTransactionReceipt: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`TransactionResult`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the account.

###### Example

```
const account = await cdp.evm.getOrCreateAccount({
  name: "MyAccount",
});

```

###### Implementation of

```
EvmClientInterface.getOrCreateAccount

```

##### getOrCreateSmartAccount()

```
getOrCreateSmartAccount(options: GetOrCreateSmartAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:579](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L579) Gets a CDP EVM smart account, or creates one if it doesn’t exist. This method first attempts to retrieve an existing smart account with the given parameters. If no account exists, it creates a new one with the specified owner.

###### Parameters

###### options

[`GetOrCreateSmartAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getorcreatesmartaccountoptions) Configuration options for getting or creating the smart account.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `getUserOperation`: (`options`: `Omit`<[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions), `"smartAccount"`\>) => `Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `owners`: [`EvmAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmaccount)\[\]; `policies`: `undefined` | `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `SmartAccountQuoteSwapOptions`) => `Promise`<`SmartAccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendUserOperation`: (`options`: `Omit`<`SendUserOperationOptions`<`unknown`\[\]>, `"smartAccount"`\>) => `Promise`<`SendUserOperationReturnType`\>; `signTypedData`: (`options`: `Omit`<[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions), `"address"`\> & { `network`: `KnownEvmNetworks`; }) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `SmartAccountSwapOptions`) => `Promise`<`SmartAccountSwapResult`\>; `transfer`: (`options`: `SmartAccountTransferOptions`) => `Promise`<`SendUserOperationReturnType`\>; `type`: `"evm-smart"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<EvmSmartAccountProperties, “useNetwork”> & { \[K in string | number | symbol\]: ({ getUserOperation: …; sendUserOperation: …; waitForUserOperation: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`SendUserOperationReturnType`\>; `waitForUserOperation`: (`options`: `Omit`<`WaitForUserOperationOptions`, `"smartAccountAddress"`\>) => `Promise`<`WaitForUserOperationReturnType`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the retrieved or newly created smart account.

###### Example

```
const smartAccount = await cdp.evm.getOrCreateSmartAccount({
  name: "MySmartAccount",
  owner: account,
});

```

##### getSmartAccount()

```
getSmartAccount(options: GetSmartAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:507](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L507) Gets a CDP EVM smart account.

###### Parameters

###### options

[`GetSmartAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getsmartaccountoptions) Parameters for getting the smart account. Either `address` or `name` must be provided. If both are provided, lookup will be done by `address` and `name` will be ignored.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `getUserOperation`: (`options`: `Omit`<[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions), `"smartAccount"`\>) => `Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `owners`: [`EvmAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmaccount)\[\]; `policies`: `undefined` | `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `SmartAccountQuoteSwapOptions`) => `Promise`<`SmartAccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendUserOperation`: (`options`: `Omit`<`SendUserOperationOptions`<`unknown`\[\]>, `"smartAccount"`\>) => `Promise`<`SendUserOperationReturnType`\>; `signTypedData`: (`options`: `Omit`<[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions), `"address"`\> & { `network`: `KnownEvmNetworks`; }) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `SmartAccountSwapOptions`) => `Promise`<`SmartAccountSwapResult`\>; `transfer`: (`options`: `SmartAccountTransferOptions`) => `Promise`<`SendUserOperationReturnType`\>; `type`: `"evm-smart"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<EvmSmartAccountProperties, “useNetwork”> & { \[K in string | number | symbol\]: ({ getUserOperation: …; sendUserOperation: …; waitForUserOperation: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`SendUserOperationReturnType`\>; `waitForUserOperation`: (`options`: `Omit`<`WaitForUserOperationOptions`, `"smartAccountAddress"`\>) => `Promise`<`WaitForUserOperationReturnType`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the smart account.

###### Example

```
const smartAccount = await cdp.evm.getSmartAccount({
  address: "0x1234567890123456789012345678901234567890",
  owner: account,
});

```

###### Implementation of

```
EvmClientInterface.getSmartAccount

```

##### getSwapPrice()

```
getSwapPrice(options: GetSwapPriceOptions): Promise<
  | SwapUnavailableResult
| GetSwapPriceResult>;

```

Defined in: [src/client/evm/evm.ts:627](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L627) Gets the price for a swap between two tokens on an EVM network.

###### Parameters

###### options

[`GetSwapPriceOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getswappriceoptions) The options for getting a swap price.

###### Returns

`Promise`< | [`SwapUnavailableResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#swapunavailableresult) | [`GetSwapPriceResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getswappriceresult)\> A promise that resolves to the swap price result or a response indicating that liquidity is unavailable.

###### Example

```
const price = await cdp.evm.getSwapPrice({
  network: "ethereum-mainnet",
  toToken: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
  fromToken: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", // WETH
  fromAmount: BigInt("1000000000000000000"), // 1 WETH
  taker: "0x1234567890123456789012345678901234567890"
});

```

###### Implementation of

```
EvmClientInterface.getSwapPrice

```

##### getUserOperation()

```
getUserOperation(options: GetUserOperationOptions): Promise<UserOperation>;

```

Defined in: [src/client/evm/evm.ts:688](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L688) Gets a user operation for a smart account by user operation hash.

###### Parameters

###### options

[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions) Parameters for getting the user operation.

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\> A promise that resolves to the user operation.

###### Example

```
const userOp = await cdp.evm.getUserOperation({
  smartAccount,
  userOpHash: "0x1234567890123456789012345678901234567890123456789012345678901234",
});

```

###### Implementation of

```
EvmClientInterface.getUserOperation

```

##### importAccount()

```
importAccount(options: ImportServerAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:183](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L183) Imports a CDP EVM account from an external source.

###### Parameters

###### options

[`ImportServerAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#importserveraccountoptions) Parameters for importing the account.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `policies?`: `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `AccountQuoteSwapOptions`) => `Promise`<`AccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendTransaction`: (`options`: `Omit`<`SendTransactionOptions`, `"address"`\>) => `Promise`<`TransactionResult`\>; `sign`: (`parameters`: { `hash`: `` `0x${string}` ``; }) => `Promise`<`` `0x${string}` ``\>; `signMessage`: (`parameters`: { `message`: `SignableMessage`; }) => `Promise`<`` `0x${string}` ``\>; `signTransaction`: (`transaction`: `TransactionSerializable`) => `Promise`<`` `0x${string}` ``\>; `signTypedData`: <`typedData`, `primaryType`\>(`parameters`: `TypedDataDefinition`<`typedData`, `primaryType`\>) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `AccountSwapOptions`) => `Promise`<`SendSwapTransactionResult`\>; `transfer`: (`options`: `TransferOptions`) => `Promise`<{ `transactionHash`: `` `0x${string}` ``; }>; `type`: `"evm-server"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<{ address: \`0x\\{string\\}\\\`; fund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; listTokenBalances: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; name?: string; policies?: (...)\\\[\\\]; quoteFund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; quoteSwap: (options: AccountQuoteSwapOptions) =\\> Promise\\<(...)\\>; requestFaucet: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sendTransaction: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sign: (parameters: \\{ hash: ... \\}) =\\> Promise\\<(...)\\>; signMessage: (parameters: \\{ message: ... \\}) =\\> Promise\\<(...)\\>; signTransaction: (transaction: TransactionSerializable) =\\> Promise\\<(...)\\>; signTypedData: (parameters: TypedDataDefinition\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; swap: (options: AccountSwapOptions) =\\> Promise\\<(...)\\>; transfer: (options: TransferOptions) =\\> Promise\\<(...)\\>; type: "evm-server"; useNetwork: \\<Network extends NetworkOrRpcUrl\\>(network: Network) =\\> Promise\\<\\{ \\\[K in keyof (Omit\\<\\{ address: \\\`0x{string}\`; sign: (parameters: { hash: \`0x{string};ˋ})\= Promise\\<0ˋx\\{string\\}\\\`; \\}) =\\> Promise\\<\\\`0x{string}\`>; signMessage: (parameters: { message: SignableMessage; }) => Promise<…>; … 15 more …; useNetwork: …; }, “transfer” | … 5 more …; useSpendPermission: (options: UseSpendPermissionOptions) => Promise<(…)>; waitForFundOperationReceipt: any }, “transfer” | “sendTransaction” | “quoteSwap” | “swap” | “useSpendPermission” | “useNetwork” | (keyof Actions)> & { \[K in string | number | symbol\]: ({ sendTransaction: …; transfer: …; waitForTransactionReceipt: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`TransactionResult`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the imported account.

###### Examples

```
         const account = await cdp.evm.importAccount({
           privateKey: "0x123456"
         });

```

```
         const account = await cdp.evm.importAccount({
           privateKey: "0x123456",
           name: "MyAccount"
         });

```

```
         const idempotencyKey = uuidv4();
         // First call
         await cdp.evm.importAccount({
           privateKey: "0x123456",
           idempotencyKey,
         });
         // Second call with the same idempotency key will return the same account
         await cdp.evm.importAccount({
           privateKey: "0x123456"
           idempotencyKey,
         });

```

###### Implementation of

```
EvmClientInterface.importAccount

```

##### listAccounts()

```
listAccounts(options?: ListServerAccountsOptions): Promise<ListServerAccountResult>;

```

Defined in: [src/client/evm/evm.ts:720](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L720) Lists CDP EVM accounts.

###### Parameters

###### options?

[`ListServerAccountsOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#listserveraccountsoptions) = `{}` Optional parameters for listing the accounts.

###### Returns

`Promise`<[`ListServerAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#listserveraccountresult)\> A promise that resolves to an array of accounts, and a token to paginate through the accounts.

###### Examples

```
const accounts = await cdp.evm.listAccounts();

```

```
         let page = await cdp.evm.listAccounts();
         while (page.nextPageToken) {
           page = await cdp.evm.listAccounts({ pageToken: page.nextPageToken });
         }

```

###### Implementation of

```
EvmClientInterface.listAccounts

```

##### listSmartAccounts()

```
listSmartAccounts(options: ListSmartAccountsOptions): Promise<ListSmartAccountResult>;

```

Defined in: [src/client/evm/evm.ts:814](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L814) Lists CDP EVM smart accounts.

###### Parameters

###### options

[`ListSmartAccountsOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#listsmartaccountsoptions) = `{}` Parameters for listing the smart accounts.

###### Returns

`Promise`<[`ListSmartAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#listsmartaccountresult)\> A promise that resolves to an array of smart accounts, and a token to paginate through the smart accounts.

###### Examples

```
const smartAccounts = await cdp.evm.listSmartAccounts();

```

```
         let page = await cdp.evm.listSmartAccounts();
         while (page.nextPageToken) {
           page = await cdp.evm.listSmartAccounts({ pageToken: page.nextPageToken });
         }

```

###### Implementation of

```
EvmClientInterface.listSmartAccounts

```

##### listSpendPermissions()

```
listSpendPermissions(options: ListSpendPermissionsOptions): Promise<ListSpendPermissionsResult>;

```

Defined in: [src/client/evm/evm.ts:845](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L845) Lists the spend permissions for a smart account.

###### Parameters

###### options

`ListSpendPermissionsOptions` Parameters for listing the spend permissions.

###### Returns

`Promise`<`ListSpendPermissionsResult`\> A promise that resolves to the spend permissions.

###### Implementation of

```
EvmClientInterface.listSpendPermissions

```

##### listTokenBalances()

```
listTokenBalances(options: ListTokenBalancesOptions): Promise<ListTokenBalancesResult>;

```

Defined in: [src/client/evm/evm.ts:779](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L779) Lists CDP EVM token balances.

###### Parameters

###### options

`ListTokenBalancesOptions` Parameters for listing the token balances.

###### Returns

`Promise`<`ListTokenBalancesResult`\> A promise that resolves to an array of token balances, and a token to paginate through the token balances.

###### Examples

```
const tokenBalances = await cdp.evm.listTokenBalances({
  address: "0x1234567890123456789012345678901234567890",
  network: "base-sepolia",
});

```

**With pagination**

```
let page = await cdp.evm.listTokenBalances({
  address: "0x1234567890123456789012345678901234567890",
  network: "base-sepolia",
});
while (page.nextPageToken) {
  page = await cdp.evm.listTokenBalances({
    address: "0x1234567890123456789012345678901234567890",
    network: "base-sepolia",
    pageToken: page.nextPageToken,
  });
}
###### Implementation of
```ts
EvmClientInterface.listTokenBalances

```

##### prepareUserOperation()

```
prepareUserOperation(options: PrepareUserOperationOptions): Promise<UserOperation>;

```

Defined in: [src/client/evm/evm.ts:881](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L881) Prepares a user operation for a smart account.

###### Parameters

###### options

[`PrepareUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#prepareuseroperationoptions) Parameters for preparing the user operation.

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\> A promise that resolves to the user operation hash.

###### Example

```
const userOp = await cdp.evm.prepareUserOperation({
  smartAccount,
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

###### Implementation of

```
EvmClientInterface.prepareUserOperation

```

##### requestFaucet()

```
requestFaucet(options: RequestFaucetOptions): Promise<RequestFaucetResult>;

```

Defined in: [src/client/evm/evm.ts:931](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L931) Requests funds from an EVM faucet.

###### Parameters

###### options

`RequestFaucetOptions` Parameters for requesting funds from the EVM faucet.

###### Returns

`Promise`<`RequestFaucetResult`\> A promise that resolves to the transaction hash.

###### Example

```
const result = await cdp.evm.requestFaucet({
  address: "0x1234567890123456789012345678901234567890",
  network: "base-sepolia",
  token: "eth",
});

```

###### Implementation of

```
EvmClientInterface.requestFaucet

```

##### revokeSpendPermission()

```
revokeSpendPermission(options: RevokeSpendPermissionOptions): Promise<UserOperation>;

```

Defined in: [src/client/evm/evm.ts:424](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L424) Revokes a spend permission for a smart account.

###### Parameters

###### options

`RevokeSpendPermissionOptions` Parameters for revoking the spend permission.

###### Returns

`Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\> A promise that resolves to the user operation.

###### Example

```
const userOperation = await cdp.evm.revokeSpendPermission({
  address: "0x1234567890123456789012345678901234567890",
  permissionHash: "0x1234567890123456789012345678901234567890123456789012345678901234",
  network: "base-sepolia",
});

```

##### sendTransaction()

```
sendTransaction(options: SendTransactionOptions): Promise<TransactionResult>;

```

Defined in: [src/client/evm/evm.ts:988](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L988) Signs an EVM transaction and sends it to the specified network using the Coinbase API. This method handles nonce management and gas estimation automatically.

###### Parameters

###### options

`SendTransactionOptions` Configuration options for sending the transaction.

###### Returns

`Promise`<`TransactionResult`\> A promise that resolves to the transaction hash.

###### Examples

**Sending an RLP-encoded transaction**

```
import { parseEther, serializeTransaction } from "viem";
import { baseSepolia } from "viem/chains";
const { transactionHash } = await cdp.evm.sendTransaction({
  address: account.address,
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
const { transactionHash } = await cdp.evm.sendTransaction({
  address: account.address,
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

###### Implementation of

```
EvmClientInterface.sendTransaction

```

##### sendUserOperation()

```
sendUserOperation(options: SendUserOperationOptions<unknown[]>): Promise<SendUserOperationReturnType>;

```

Defined in: [src/client/evm/evm.ts:1027](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L1027) Sends a user operation.

###### Parameters

###### options

`SendUserOperationOptions`<`unknown`\[\]> Parameters for sending the user operation.

###### Returns

`Promise`<`SendUserOperationReturnType`\> A promise that resolves to an object containing the smart account address, the user operation hash, and the status of the user operation.

###### Example

```
const userOp = await cdp.evm.sendUserOperation({
  smartAccount,
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

###### Implementation of

```
EvmClientInterface.sendUserOperation

```

##### signHash()

```
signHash(options: SignHashOptions): Promise<SignatureResult>;

```

Defined in: [src/client/evm/evm.ts:1067](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L1067) Signs an EVM hash.

###### Parameters

###### options

[`SignHashOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signhashoptions) Parameters for signing the hash.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signatureresult)\> A promise that resolves to the signature.

###### Example

```
// Create a new EVM server account to sign with
const ethAccount = await cdp.createEvmServerAccount({});
const signature = await cdp.evm.signHash({
  address: ethAccount.address,
  hash: "0x1234567890123456789012345678901234567890123456789012345678901234",
});

```

###### Implementation of

```
EvmClientInterface.signHash

```

##### signMessage()

```
signMessage(options: SignMessageOptions): Promise<SignatureResult>;

```

Defined in: [src/client/evm/evm.ts:1106](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L1106) Signs an EIP-191 message.

###### Parameters

###### options

[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signmessageoptions) Parameters for signing the message.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signatureresult)\> A promise that resolves to the signature.

###### Example

```
// Create a new EVM server account to sign with
const ethAccount = await cdp.createEvmServerAccount({});
const signature = await cdp.evm.signMessage({
  address: ethAccount.address,
  message: "Hello, world!",
});

```

###### Implementation of

```
EvmClientInterface.signMessage

```

##### signTransaction()

```
signTransaction(options: SignTransactionOptions): Promise<SignatureResult>;

```

Defined in: [src/client/evm/evm.ts:1228](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L1228) Signs an EVM transaction.

###### Parameters

###### options

[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtransactionoptions) Configuration options for signing the transaction.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signatureresult)\> A promise that resolves to the signature.

###### Example

```
import { parseEther, serializeTransaction } from "viem";
import { baseSepolia } from "viem/chains";
// Create a new EVM server account to sign with
const ethAccount = await cdp.createEvmServerAccount({});
const serializedTx = serializeTransaction(
  {
    chainId: baseSepolia.id,
    data: "0x",
    to: "0x4252e0c9A3da5A2700e7d91cb50aEf522D0C6Fe8",
    type: "eip1559",
    value: parseEther("0.000001"),
  },
);
const signature = await cdp.evm.signTransaction({
  address: ethAccount.address,
  transaction: serializedTx,
});

```

###### Implementation of

```
EvmClientInterface.signTransaction

```

##### signTypedData()

```
signTypedData(options: SignTypedDataOptions): Promise<SignatureResult>;

```

Defined in: [src/client/evm/evm.ts:1169](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L1169) Signs an EIP-712 message.

###### Parameters

###### options

[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions) Parameters for signing the EIP-712 message.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signatureresult)\> A promise that resolves to the signature.

###### Example

```
const signature = await cdp.evm.signTypedData({
  address: account.address,
  domain: {
    name: "Permit2",
    chainId: 1,
    verifyingContract: "0x000000000022D473030F116dDEE9F6B43aC78BA3",
  },
  types: {
    EIP712Domain: [
      { name: "name", type: "string" },
      { name: "chainId", type: "uint256" },
      { name: "verifyingContract", type: "address" },
    ],
    PermitTransferFrom: [
      { name: "permitted", type: "TokenPermissions" },
      { name: "spender", type: "address" },
      { name: "nonce", type: "uint256" },
      { name: "deadline", type: "uint256" },
    ],
    TokenPermissions: [
      { name: "token", type: "address" },
      { name: "amount", type: "uint256" },
    ],
  },
  primaryType: "PermitTransferFrom",
  message: {
    permitted: {
      token: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      amount: "1000000",
    },
    spender: "0xFfFfFfFFfFFfFFfFFfFFFFFffFFFffffFfFFFfFf",
    nonce: "0",
    deadline: "1717123200",
  },
});

```

###### Implementation of

```
EvmClientInterface.signTypedData

```

##### updateAccount()

```
updateAccount(options?: UpdateEvmAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:1287](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L1287) Updates a CDP EVM account.

###### Parameters

###### options?

[`UpdateEvmAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#updateevmaccountoptions) Optional parameters for creating the account.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `policies?`: `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `AccountQuoteSwapOptions`) => `Promise`<`AccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendTransaction`: (`options`: `Omit`<`SendTransactionOptions`, `"address"`\>) => `Promise`<`TransactionResult`\>; `sign`: (`parameters`: { `hash`: `` `0x${string}` ``; }) => `Promise`<`` `0x${string}` ``\>; `signMessage`: (`parameters`: { `message`: `SignableMessage`; }) => `Promise`<`` `0x${string}` ``\>; `signTransaction`: (`transaction`: `TransactionSerializable`) => `Promise`<`` `0x${string}` ``\>; `signTypedData`: <`typedData`, `primaryType`\>(`parameters`: `TypedDataDefinition`<`typedData`, `primaryType`\>) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `AccountSwapOptions`) => `Promise`<`SendSwapTransactionResult`\>; `transfer`: (`options`: `TransferOptions`) => `Promise`<{ `transactionHash`: `` `0x${string}` ``; }>; `type`: `"evm-server"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<{ address: \`0x\\{string\\}\\\`; fund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; listTokenBalances: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; name?: string; policies?: (...)\\\[\\\]; quoteFund: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; quoteSwap: (options: AccountQuoteSwapOptions) =\\> Promise\\<(...)\\>; requestFaucet: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sendTransaction: (options: Omit\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; sign: (parameters: \\{ hash: ... \\}) =\\> Promise\\<(...)\\>; signMessage: (parameters: \\{ message: ... \\}) =\\> Promise\\<(...)\\>; signTransaction: (transaction: TransactionSerializable) =\\> Promise\\<(...)\\>; signTypedData: (parameters: TypedDataDefinition\\<(...), (...)\\>) =\\> Promise\\<(...)\\>; swap: (options: AccountSwapOptions) =\\> Promise\\<(...)\\>; transfer: (options: TransferOptions) =\\> Promise\\<(...)\\>; type: "evm-server"; useNetwork: \\<Network extends NetworkOrRpcUrl\\>(network: Network) =\\> Promise\\<\\{ \\\[K in keyof (Omit\\<\\{ address: \\\`0x{string}\`; sign: (parameters: { hash: \`0x{string};ˋ})\= Promise\\<0ˋx\\{string\\}\\\`; \\}) =\\> Promise\\<\\\`0x{string}\`>; signMessage: (parameters: { message: SignableMessage; }) => Promise<…>; … 15 more …; useNetwork: …; }, “transfer” | … 5 more …; useSpendPermission: (options: UseSpendPermissionOptions) => Promise<(…)>; waitForFundOperationReceipt: any }, “transfer” | “sendTransaction” | “quoteSwap” | “swap” | “useSpendPermission” | “useNetwork” | (keyof Actions)> & { \[K in string | number | symbol\]: ({ sendTransaction: …; transfer: …; waitForTransactionReceipt: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`TransactionResult`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the updated account.

###### Examples

```
         const account = await cdp.evm.updateAccount({ address: "0x...", update: { name: "New Name" } });

```

```
         const account = await cdp.evm.updateAccount({ address: "0x...", update: { accountPolicy: "73bcaeeb-d7af-4615-b064-42b5fe83a31e" } });

```

```
         const idempotencyKey = uuidv4();
         // First call
         await cdp.evm.updateAccount({
           address: "0x...",
           update: { accountPolicy: "73bcaeeb-d7af-4615-b064-42b5fe83a31e" },
           idempotencyKey,
         });
         // Second call with the same idempotency key will not update
         await cdp.evm.updateAccount({
           address: '0x...',
           update: { name: "" },
           idempotencyKey,
         });

```

###### Implementation of

```
EvmClientInterface.updateAccount

```

##### updateSmartAccount()

```
updateSmartAccount(options?: UpdateEvmSmartAccountOptions): Promise<{
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
}>;

```

Defined in: [src/client/evm/evm.ts:1319](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L1319) Updates a CDP EVM smart account.

###### Parameters

###### options?

[`UpdateEvmSmartAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#updateevmsmartaccountoptions) Optional parameters for updating the account.

###### Returns

`Promise`<{ `address`: `` `0x${string}` ``; `fund`: (`options`: `Omit`<`EvmFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `getUserOperation`: (`options`: `Omit`<[`GetUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#getuseroperationoptions), `"smartAccount"`\>) => `Promise`<[`UserOperation`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#useroperation)\>; `listTokenBalances`: (`options`: `Omit`<`ListTokenBalancesOptions`, `"address"`\>) => `Promise`<`ListTokenBalancesResult`\>; `name?`: `string`; `owners`: [`EvmAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Accounts#evmaccount)\[\]; `policies`: `undefined` | `string`\[\]; `quoteFund`: (`options`: `Omit`<`EvmQuoteFundOptions`, `"address"`\>) => `Promise`<`EvmQuote`\>; `quoteSwap`: (`options`: `SmartAccountQuoteSwapOptions`) => `Promise`<`SmartAccountQuoteSwapResult`\>; `requestFaucet`: (`options`: `Omit`<`RequestFaucetOptions`, `"address"`\>) => `Promise`<`RequestFaucetResult`\>; `sendUserOperation`: (`options`: `Omit`<`SendUserOperationOptions`<`unknown`\[\]>, `"smartAccount"`\>) => `Promise`<`SendUserOperationReturnType`\>; `signTypedData`: (`options`: `Omit`<[`SignTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#signtypeddataoptions), `"address"`\> & { `network`: `KnownEvmNetworks`; }) => `Promise`<`` `0x${string}` ``\>; `swap`: (`options`: `SmartAccountSwapOptions`) => `Promise`<`SmartAccountSwapResult`\>; `transfer`: (`options`: `SmartAccountTransferOptions`) => `Promise`<`SendUserOperationReturnType`\>; `type`: `"evm-smart"`; `useNetwork`: <`Network`\>(`network`: `Network`) => `Promise`<{ \[K in string | number | symbol\]: (Omit<EvmSmartAccountProperties, “useNetwork”> & { \[K in string | number | symbol\]: ({ getUserOperation: …; sendUserOperation: …; waitForUserOperation: … } & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)) & ((…) extends (…) ? (…) : (…)))\[K\] } & { network: Network })\[K\] }>; `useSpendPermission`: (`options`: `UseSpendPermissionOptions`) => `Promise`<`SendUserOperationReturnType`\>; `waitForUserOperation`: (`options`: `Omit`<`WaitForUserOperationOptions`, `"smartAccountAddress"`\>) => `Promise`<`WaitForUserOperationReturnType`\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the updated account.

###### Implementation of

```
EvmClientInterface.updateSmartAccount

```

##### waitForUserOperation()

```
waitForUserOperation(options: WaitForUserOperationOptions): Promise<WaitForUserOperationReturnType>;

```

Defined in: [src/client/evm/evm.ts:1372](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/evm/evm.ts#L1372) Waits for a user operation to complete or fail.

###### Parameters

###### options

[`WaitForUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/evm/Types#waitforuseroperationoptions) Parameters for waiting for the user operation.

###### Returns

`Promise`<`WaitForUserOperationReturnType`\> A promise that resolves to the transaction receipt.

###### Example

```
// Send a user operation and get the user operation hash
const { userOpHash } = await cdp.evm.sendUserOperation({
  smartAccount,
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
const result = await cdp.evm.waitForUserOperation({
  smartAccountAddress: smartAccount.address,
  userOpHash: userOp.userOpHash,
});

```