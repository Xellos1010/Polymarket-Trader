# actions

## Type Aliases

### AccountActions

```
type AccountActions = {
  fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;
  quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;
  sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;
  signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;
  signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;
  transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
};

```

Defined in: [actions/solana/types.ts:25](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L25)

#### Properties

##### fund()

```
fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;

```

Defined in: [actions/solana/types.ts:210](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L210) Funds a Solana account with the specified token amount.

###### Parameters

###### options

`Omit`<`SolanaFundOptions`, `"address"`\> The options for the fund operation.

###### Returns

`Promise`<`FundOperationResult`\> A promise that resolves to the fund operation result containing the transfer details.

###### Example

```
const fundOperation = await account.fund({
  token: "usdc",
  amount: 1000000n,
});

```

##### quoteFund()

```
quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;

```

Defined in: [actions/solana/types.ts:189](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L189) Gets a quote to fund a Solana account.

###### Parameters

###### options

`Omit`<`SolanaQuoteFundOptions`, `"address"`\> The options for the quote fund.

###### Returns

`Promise`<`SolanaQuote`\> A promise that resolves to a Quote object containing details about the funding operation.

###### Example

```
const quote = await account.quoteFund({
  token: "usdc",
  amount: 1000000n,
});

```

##### requestFaucet()

```
requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;

```

Defined in: [actions/solana/types.ts:46](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L46) Requests funds from a Solana faucet.

###### Parameters

###### options

`Omit`<[`RequestFaucetOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#requestfaucetoptions), `"address"`\> Parameters for requesting funds from the Solana faucet.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\> A promise that resolves to the transaction hash.

###### Example

```
// Create a Solana account
const account = await cdp.solana.createAccount();
// Request funds from the Solana faucet
const result = await account.requestFaucet({
  token: "sol",
});

```

##### sendTransaction()

```
sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;

```

Defined in: [actions/solana/types.ts:139](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L139) Sends a transaction.

###### Parameters

###### options

`Omit`<[`SendTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#sendtransactionoptions), `"address"`\> Parameters for sending the transaction.

###### Returns

`Promise`<`SendTransactionResult`\> A promise that resolves to the transaction signature.

###### Example

```
// Create a Solana account
const account = await cdp.solana.createAccount();
// Add your transaction instructions here
const transaction = new Transaction()
// Make sure to set requireAllSignatures to false, since signing will be done through the API
const serializedTransaction = transaction.serialize({
  requireAllSignatures: false,
});
// Base64 encode the serialized transaction
const transaction = Buffer.from(serializedTransaction).toString("base64");
// When you want to sign a transaction, you can do so by address and base64 encoded transaction
const { transactionSignature } = await account.sendTransaction({
  transaction,
});

```

##### signMessage()

```
signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;

```

Defined in: [actions/solana/types.ts:69](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L69) Signs a message.

###### Parameters

###### options

`Omit`<[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signmessageoptions), `"address"`\> Parameters for signing the message.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\> A promise that resolves to the signature.

###### Example

```
// Create a Solana account
const account = await cdp.solana.createAccount();
// Sign a message
const { signature } = await account.signMessage({
  message: "Hello, world!",
});

```

##### signTransaction()

```
signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;

```

Defined in: [actions/solana/types.ts:103](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L103) Signs a transaction.

###### Parameters

###### options

`Omit`<[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signtransactionoptions), `"address"`\> Parameters for signing the transaction.

###### Returns

`Promise`<`SignTransactionResult`\> A promise that resolves to the signature.

###### Example

```
// Create a Solana account
const account = await cdp.solana.createAccount();
// Add your transaction instructions here
const transaction = new Transaction()
// Make sure to set requireAllSignatures to false, since signing will be done through the API
const serializedTransaction = transaction.serialize({
  requireAllSignatures: false,
});
// Base64 encode the serialized transaction
const transaction = Buffer.from(serializedTransaction).toString("base64");
// When you want to sign a transaction, you can do so by address and base64 encoded transaction
const { signedTransaction } = await account.signTransaction({
  transaction,
});

```

##### transfer()

```
transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;

```

Defined in: [actions/solana/types.ts:168](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L168) Transfers SOL or SPL tokens between accounts

###### Parameters

###### options

`Omit`<`TransferOptions`, `"from"`\> Parameters for the transfer.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\> A promise that resolves to the transaction signature, which can be used to wait for the transaction result.

###### Example

```
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
const account = await cdp.solana.getAccount({ name: "Account" });
const { signature } = await account.transfer({
  token: "sol",
  amount: 5 * LAMPORTS_PER_SOL,
  to: "3KzDtddx4i53FBkvCzuDmRbaMozTZoJBb1TToWhz3JfE",
  network: "devnet",
});

```

#### Methods

##### waitForFundOperationReceipt()

```
waitForFundOperationReceipt(options: WaitForFundOperationOptions): Promise<WaitForFundOperationResult>;

```

Defined in: [actions/solana/types.ts:227](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/actions/solana/types.ts#L227) Waits for a fund operation to complete and returns the transfer receipt.

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