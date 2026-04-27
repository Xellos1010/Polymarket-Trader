# types

## Interfaces

### CreateAccountOptions

Defined in: [client/solana/solana.types.ts:59](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L59) Options for creating a Solana account.

#### Properties

##### accountPolicy?

```
optional accountPolicy: string;

```

Defined in: [client/solana/solana.types.ts:63](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L63) The policy ID to apply to the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [client/solana/solana.types.ts:65](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L65) The idempotency key.

##### name?

Defined in: [client/solana/solana.types.ts:61](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L61) The name of the account.

* * *

### ExportAccountOptions

Defined in: [client/solana/solana.types.ts:71](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L71) Options for exporting a Solana account.

#### Properties

##### address?

```
optional address: string;

```

Defined in: [client/solana/solana.types.ts:73](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L73) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [client/solana/solana.types.ts:77](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L77) The idempotency key.

##### name?

Defined in: [client/solana/solana.types.ts:75](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L75) The name of the account.

* * *

### GetAccountOptions

Defined in: [client/solana/solana.types.ts:83](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L83) Options for getting a Solana account.

#### Properties

##### address?

```
optional address: string;

```

Defined in: [client/solana/solana.types.ts:85](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L85) The address of the account.

##### name?

Defined in: [client/solana/solana.types.ts:87](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L87) The name of the account.

* * *

### GetOrCreateAccountOptions

Defined in: [client/solana/solana.types.ts:93](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L93) Options for getting a Solana account.

#### Properties

##### name

Defined in: [client/solana/solana.types.ts:95](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L95) The name of the account.

* * *

### ImportAccountOptions

Defined in: [client/solana/solana.types.ts:171](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L171) Options for importing a Solana account.

#### Properties

##### encryptionPublicKey?

```
optional encryptionPublicKey: string;

```

Defined in: [client/solana/solana.types.ts:173](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L173) The public RSA key used to encrypt the private key when importing a Solana account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [client/solana/solana.types.ts:177](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L177) The idempotency key.

##### name?

Defined in: [client/solana/solana.types.ts:175](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L175) The name of the account.

##### privateKey

```
privateKey: string | Uint8Array<ArrayBufferLike>;

```

Defined in: [client/solana/solana.types.ts:179](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L179) The private key of the account - can be a base58 encoded string or raw bytes.

* * *

### ListAccountsOptions

Defined in: [client/solana/solana.types.ts:113](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L113) Options for listing Solana accounts.

#### Properties

##### pageSize?

```
optional pageSize: number;

```

Defined in: [client/solana/solana.types.ts:115](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L115) The page size.

##### pageToken?

```
optional pageToken: string;

```

Defined in: [client/solana/solana.types.ts:117](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L117) The page token.

* * *

### ListAccountsResult

Defined in: [client/solana/solana.types.ts:123](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L123) The result of listing Solana accounts.

#### Properties

##### accounts

```
accounts: {
  fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;
  quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;
  sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;
  signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;
  signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;
  transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}[];

```

Defined in: [client/solana/solana.types.ts:125](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L125) The accounts.

###### fund()

```
fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;

```

Funds a Solana account with the specified token amount.

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

###### quoteFund()

```
quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;

```

Gets a quote to fund a Solana account.

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

###### requestFaucet()

```
requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;

```

Requests funds from a Solana faucet.

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

###### sendTransaction()

```
sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;

```

Sends a transaction.

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

###### signMessage()

```
signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;

```

Signs a message.

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

###### signTransaction()

```
signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;

```

Signs a transaction.

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

###### transfer()

```
transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;

```

Transfers SOL or SPL tokens between accounts

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

Defined in: [client/solana/solana.types.ts:129](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L129) The token for the next page of accounts, if any.

* * *

### ListTokenBalancesOptions

Defined in: [client/solana/solana.types.ts:185](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L185) Options for listing Solana token balances.

#### Properties

##### address

Defined in: [client/solana/solana.types.ts:187](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L187) The address of the account.

##### network?

```
optional network: ListSolanaTokenBalancesNetwork;

```

Defined in: [client/solana/solana.types.ts:189](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L189) The network to list token balances for.

##### pageSize?

```
optional pageSize: number;

```

Defined in: [client/solana/solana.types.ts:191](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L191) The page size.

##### pageToken?

```
optional pageToken: string;

```

Defined in: [client/solana/solana.types.ts:193](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L193) The page token.

* * *

### ListTokenBalancesResult

Defined in: [client/solana/solana.types.ts:245](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L245) The result of listing Solana token balances.

#### Properties

##### balances

```
balances: SolanaTokenBalance[];

```

Defined in: [client/solana/solana.types.ts:247](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L247) The token balances.

##### nextPageToken?

```
optional nextPageToken: string;

```

Defined in: [client/solana/solana.types.ts:252](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L252) The next page token to paginate through the token balances. If undefined, there are no more token balances to paginate through.

* * *

### RequestFaucetOptions

Defined in: [client/solana/solana.types.ts:135](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L135) Options for requesting funds from a Solana faucet.

#### Properties

##### address

Defined in: [client/solana/solana.types.ts:137](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L137) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [client/solana/solana.types.ts:141](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L141) The idempotency key.

##### token

Defined in: [client/solana/solana.types.ts:139](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L139) The token to request funds for.

* * *

### SendTransactionOptions

Defined in: [client/solana/solana.types.ts:199](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L199) Options for sending a Solana transaction.

#### Properties

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [client/solana/solana.types.ts:205](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L205) The idempotency key.

##### network

```
network: SendSolanaTransactionBodyNetwork;

```

Defined in: [client/solana/solana.types.ts:201](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L201) The network to send the transaction to.

##### transaction

Defined in: [client/solana/solana.types.ts:203](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L203) The base64 encoded transaction to send.

* * *

### SignatureResult

Defined in: [client/solana/solana.types.ts:51](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L51) A Solana signature result.

#### Properties

##### signature

Defined in: [client/solana/solana.types.ts:53](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L53) The signature.

* * *

### SignMessageOptions

Defined in: [client/solana/solana.types.ts:147](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L147) Options for signing a Solana message.

#### Properties

##### address

Defined in: [client/solana/solana.types.ts:149](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L149) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [client/solana/solana.types.ts:153](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L153) The idempotency key.

##### message

Defined in: [client/solana/solana.types.ts:151](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L151) The message to sign.

* * *

### SignTransactionOptions

Defined in: [client/solana/solana.types.ts:159](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L159) Options for signing a Solana transaction.

#### Properties

##### address

Defined in: [client/solana/solana.types.ts:161](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L161) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [client/solana/solana.types.ts:165](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L165) The idempotency key.

##### transaction

Defined in: [client/solana/solana.types.ts:163](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L163) The base64 encoded transaction to sign.

* * *

### SolanaToken

Defined in: [client/solana/solana.types.ts:223](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L223)

#### Properties

##### mintAddress

Defined in: [client/solana/solana.types.ts:225](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L225) The token address.

##### name?

Defined in: [client/solana/solana.types.ts:227](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L227) The token name.

##### symbol?

Defined in: [client/solana/solana.types.ts:229](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L229) The token symbol.

* * *

### SolanaTokenAmount

Defined in: [client/solana/solana.types.ts:216](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L216)

#### Properties

##### amount

Defined in: [client/solana/solana.types.ts:218](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L218) The amount of the token.

##### decimals

Defined in: [client/solana/solana.types.ts:220](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L220) The number of decimals in the token.

* * *

### SolanaTokenBalance

Defined in: [client/solana/solana.types.ts:235](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L235) A Solana token balance.

#### Properties

##### amount

```
amount: SolanaTokenAmount;

```

Defined in: [client/solana/solana.types.ts:237](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L237) The amount of the token.

##### token

Defined in: [client/solana/solana.types.ts:239](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L239) The token.

* * *

### TransactionResult

Defined in: [client/solana/solana.types.ts:211](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L211) The result of sending a Solana transaction.

#### Properties

##### signature

Defined in: [client/solana/solana.types.ts:213](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L213) The signature of the transaction base58 encoded.

* * *

### UpdateSolanaAccountOptions

Defined in: [client/solana/solana.types.ts:101](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L101) Options for creating a SOL server account.

#### Properties

##### address

Defined in: [client/solana/solana.types.ts:103](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L103) The address of the account.

##### idempotencyKey?

```
optional idempotencyKey: string;

```

Defined in: [client/solana/solana.types.ts:107](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L107) The idempotency key.

##### update

```
update: UpdateSolanaAccountBody;

```

Defined in: [client/solana/solana.types.ts:105](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L105) The updates to apply to the account

## Type Aliases

### SolanaClientInterface

```
type SolanaClientInterface = Omit<typeof OpenApiSolanaMethods, 
  | "createSolanaAccount"
  | "getSolanaAccount"
  | "getSolanaAccountByName"
  | "updateSolanaAccount"
  | "listSolanaAccounts"
  | "requestSolanaFaucet"
  | "signSolanaMessage"
  | "signSolanaTransaction"
  | "updateSolanaAccount"
  | "exportSolanaAccount"
  | "exportSolanaAccountByName"
  | "importSolanaAccount"
  | "listSolanaTokenBalances"
  | "sendSolanaTransaction"> & {
  createAccount: (options: CreateAccountOptions) => Promise<Account>;
  exportAccount: (options: ExportAccountOptions) => Promise<string>;
  getAccount: (options: GetAccountOptions) => Promise<Account>;
  getOrCreateAccount: (options: GetOrCreateAccountOptions) => Promise<Account>;
  importAccount: (options: ImportAccountOptions) => Promise<SolanaAccount>;
  listAccounts: (options: ListAccountsOptions) => Promise<ListAccountsResult>;
  listTokenBalances: (options: ListTokenBalancesOptions) => Promise<ListTokenBalancesResult>;
  requestFaucet: (options: RequestFaucetOptions) => Promise<SignatureResult>;
  sendTransaction: (options: SendSolanaTransactionBody) => Promise<SignatureResult>;
  signMessage: (options: SignMessageOptions) => Promise<SignatureResult>;
  signTransaction: (options: SignTransactionOptions) => Promise<SignatureResult>;
  updateAccount: (options: UpdateSolanaAccountOptions) => Promise<Account>;
};

```

Defined in: [client/solana/solana.types.ts:17](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.types.ts#L17) The SolanaClient type, where all OpenApiSolanaMethods methods are wrapped.

#### Type declaration

##### createAccount()

```
createAccount: (options: CreateAccountOptions) => Promise<Account>;

```

###### Parameters

###### options

[`CreateAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#createaccountoptions)

###### Returns

`Promise`<`Account`\>

##### exportAccount()

```
exportAccount: (options: ExportAccountOptions) => Promise<string>;

```

###### Parameters

###### options

[`ExportAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#exportaccountoptions)

###### Returns

`Promise`<`string`\>

##### getAccount()

```
getAccount: (options: GetAccountOptions) => Promise<Account>;

```

###### Parameters

###### options

[`GetAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#getaccountoptions)

###### Returns

`Promise`<`Account`\>

##### getOrCreateAccount()

```
getOrCreateAccount: (options: GetOrCreateAccountOptions) => Promise<Account>;

```

###### Parameters

###### options

[`GetOrCreateAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#getorcreateaccountoptions)

###### Returns

`Promise`<`Account`\>

##### importAccount()

```
importAccount: (options: ImportAccountOptions) => Promise<SolanaAccount>;

```

###### Parameters

###### options

[`ImportAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#importaccountoptions)

###### Returns

`Promise`<[`SolanaAccount`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Accounts#solanaaccount)\>

##### listAccounts()

```
listAccounts: (options: ListAccountsOptions) => Promise<ListAccountsResult>;

```

###### Parameters

###### options

[`ListAccountsOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#listaccountsoptions)

###### Returns

`Promise`<[`ListAccountsResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#listaccountsresult)\>

##### listTokenBalances()

```
listTokenBalances: (options: ListTokenBalancesOptions) => Promise<ListTokenBalancesResult>;

```

###### Parameters

###### options

[`ListTokenBalancesOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#listtokenbalancesoptions)

###### Returns

`Promise`<[`ListTokenBalancesResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#listtokenbalancesresult)\>

##### requestFaucet()

```
requestFaucet: (options: RequestFaucetOptions) => Promise<SignatureResult>;

```

###### Parameters

###### options

[`RequestFaucetOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#requestfaucetoptions)

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>

##### sendTransaction()

```
sendTransaction: (options: SendSolanaTransactionBody) => Promise<SignatureResult>;

```

###### Parameters

###### options

`SendSolanaTransactionBody`

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>

##### signMessage()

```
signMessage: (options: SignMessageOptions) => Promise<SignatureResult>;

```

###### Parameters

###### options

[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signmessageoptions)

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>

##### signTransaction()

```
signTransaction: (options: SignTransactionOptions) => Promise<SignatureResult>;

```

###### Parameters

###### options

[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signtransactionoptions)

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>

##### updateAccount()

```
updateAccount: (options: UpdateSolanaAccountOptions) => Promise<Account>;

```

###### Parameters

###### options

[`UpdateSolanaAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#updatesolanaaccountoptions)

###### Returns

`Promise`<`Account`\>