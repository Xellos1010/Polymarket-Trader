# client

## Classes

### SolanaClient

Defined in: [client/solana/solana.ts:53](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L53) The namespace containing all Solana methods.

#### Implements

-   [`SolanaClientInterface`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#solanaclientinterface)

#### Constructors

##### Constructor

```
new SolanaClient(): SolanaClient;

```

###### Returns

[`SolanaClient`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Client#solanaclient)

#### Methods

##### createAccount()

```
createAccount(options: CreateAccountOptions): Promise<{
  fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;
  quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;
  sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;
  signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;
  signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;
  transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}>;

```

Defined in: [client/solana/solana.ts:84](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L84) Creates a new Solana account.

###### Parameters

###### options

[`CreateAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#createaccountoptions) = `{}` Parameters for creating the Solana account.

###### Returns

`Promise`<{ `fund`: (`options`: `Omit`<`SolanaFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `quoteFund`: (`options`: `Omit`<`SolanaQuoteFundOptions`, `"address"`\>) => `Promise`<`SolanaQuote`\>; `requestFaucet`: (`options`: `Omit`<[`RequestFaucetOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#requestfaucetoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `sendTransaction`: (`options`: `Omit`<[`SendTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#sendtransactionoptions), `"address"`\>) => `Promise`<`SendTransactionResult`\>; `signMessage`: (`options`: `Omit`<[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signmessageoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `signTransaction`: (`options`: `Omit`<[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signtransactionoptions), `"address"`\>) => `Promise`<`SignTransactionResult`\>; `transfer`: (`options`: `Omit`<`TransferOptions`, `"from"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the newly created account.

###### Examples

```
         const account = await cdp.solana.createAccount();

```

```
         const account = await cdp.solana.createAccount({ name: "MyAccount" });

```

```
         const idempotencyKey = uuidv4();
         // First call
         await cdp.solana.createAccount({ idempotencyKey });
         // Second call with the same idempotency key will return the same account
         await cdp.solana.createAccount({ idempotencyKey });

```

###### Implementation of

```
SolanaClientInterface.createAccount

```

##### exportAccount()

```
exportAccount(options: ExportAccountOptions): Promise<string>;

```

Defined in: [client/solana/solana.ts:117](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L117) Exports a CDP Solana account’s private key. It is important to store the private key in a secure place after it’s exported.

###### Parameters

###### options

[`ExportAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#exportaccountoptions) Parameters for exporting the Solana account.

###### Returns

`Promise`<`string`\> A promise that resolves to the exported account’s full 64-byte private key as a base58 encoded string.

###### Examples

```
const privateKey = await cdp.solana.exportAccount({
  address: "1234567890123456789012345678901234567890",
});

```

```
const privateKey = await cdp.solana.exportAccount({
  name: "MyAccount",
});

```

###### Implementation of

```
SolanaClientInterface.exportAccount

```

##### getAccount()

```
getAccount(options: GetAccountOptions): Promise<{
  fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;
  quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;
  sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;
  signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;
  signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;
  transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}>;

```

Defined in: [client/solana/solana.ts:266](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L266) Gets a Solana account by its address.

###### Parameters

###### options

[`GetAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#getaccountoptions) Parameters for getting the Solana account. Either `address` or `name` must be provided. If both are provided, lookup will be done by `address` and `name` will be ignored.

###### Returns

`Promise`<{ `fund`: (`options`: `Omit`<`SolanaFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `quoteFund`: (`options`: `Omit`<`SolanaQuoteFundOptions`, `"address"`\>) => `Promise`<`SolanaQuote`\>; `requestFaucet`: (`options`: `Omit`<[`RequestFaucetOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#requestfaucetoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `sendTransaction`: (`options`: `Omit`<[`SendTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#sendtransactionoptions), `"address"`\>) => `Promise`<`SendTransactionResult`\>; `signMessage`: (`options`: `Omit`<[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signmessageoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `signTransaction`: (`options`: `Omit`<[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signtransactionoptions), `"address"`\>) => `Promise`<`SignTransactionResult`\>; `transfer`: (`options`: `Omit`<`TransferOptions`, `"from"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the account.

###### Examples

```
         const account = await cdp.solana.getAccount({
           address: "1234567890123456789012345678901234567890",
         });

```

```
         const account = await cdp.solana.getAccount({
           name: "MyAccount",
         });

```

###### Implementation of

```
SolanaClientInterface.getAccount

```

##### getOrCreateAccount()

```
getOrCreateAccount(options: GetOrCreateAccountOptions): Promise<{
  fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;
  quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;
  sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;
  signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;
  signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;
  transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}>;

```

Defined in: [client/solana/solana.ts:290](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L290) Gets a Solana account by its address.

###### Parameters

###### options

[`GetOrCreateAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#getorcreateaccountoptions) Parameters for getting or creating the Solana account.

###### Returns

`Promise`<{ `fund`: (`options`: `Omit`<`SolanaFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `quoteFund`: (`options`: `Omit`<`SolanaQuoteFundOptions`, `"address"`\>) => `Promise`<`SolanaQuote`\>; `requestFaucet`: (`options`: `Omit`<[`RequestFaucetOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#requestfaucetoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `sendTransaction`: (`options`: `Omit`<[`SendTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#sendtransactionoptions), `"address"`\>) => `Promise`<`SendTransactionResult`\>; `signMessage`: (`options`: `Omit`<[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signmessageoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `signTransaction`: (`options`: `Omit`<[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signtransactionoptions), `"address"`\>) => `Promise`<`SignTransactionResult`\>; `transfer`: (`options`: `Omit`<`TransferOptions`, `"from"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the account.

###### Example

```
const account = await cdp.solana.getOrCreateAccount({
  name: "MyAccount",
});

```

###### Implementation of

```
SolanaClientInterface.getOrCreateAccount

```

##### importAccount()

```
importAccount(options: ImportAccountOptions): Promise<{
  fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;
  quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;
  sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;
  signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;
  signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;
  transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}>;

```

Defined in: [client/solana/solana.ts:191](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L191) Imports a Solana account using a private key. The private key will be encrypted before being stored securely.

###### Parameters

###### options

[`ImportAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#importaccountoptions) Parameters for importing the Solana account.

###### Returns

`Promise`<{ `fund`: (`options`: `Omit`<`SolanaFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `quoteFund`: (`options`: `Omit`<`SolanaQuoteFundOptions`, `"address"`\>) => `Promise`<`SolanaQuote`\>; `requestFaucet`: (`options`: `Omit`<[`RequestFaucetOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#requestfaucetoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `sendTransaction`: (`options`: `Omit`<[`SendTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#sendtransactionoptions), `"address"`\>) => `Promise`<`SendTransactionResult`\>; `signMessage`: (`options`: `Omit`<[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signmessageoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `signTransaction`: (`options`: `Omit`<[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signtransactionoptions), `"address"`\>) => `Promise`<`SignTransactionResult`\>; `transfer`: (`options`: `Omit`<`TransferOptions`, `"from"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the imported account.

###### Examples

```
         const account = await cdp.solana.importAccount({
           privateKey: "3Kzjw8qSxx8bQkV7EHrVFWYiPyNLbBVxtVe1Q5h2zKZY8DdcuT2dKxyz9kU5vQrP",
         });

```

```
         const account = await cdp.solana.importAccount({
           privateKey: "3Kzjw8qSxx8bQkV7EHrVFWYiPyNLbBVxtVe1Q5h2zKZY8DdcuT2dKxyz9kU5vQrP",
           name: "ImportedAccount",
         });

```

```
         const idempotencyKey = uuidv4();
         const account = await cdp.solana.importAccount({
           privateKey: "3Kzjw8qSxx8bQkV7EHrVFWYiPyNLbBVxtVe1Q5h2zKZY8DdcuT2dKxyz9kU5vQrP",
           name: "ImportedAccount",
           idempotencyKey,
         });

```

###### Implementation of

```
SolanaClientInterface.importAccount

```

##### listAccounts()

```
listAccounts(options: ListAccountsOptions): Promise<ListAccountsResult>;

```

Defined in: [client/solana/solana.ts:349](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L349) Lists all Solana accounts.

###### Parameters

###### options

[`ListAccountsOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#listaccountsoptions) = `{}` Parameters for listing the Solana accounts.

###### Returns

`Promise`<[`ListAccountsResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#listaccountsresult)\> A promise that resolves to an array of Solana account instances.

###### Examples

```
         const accounts = await cdp.solana.listAccounts();

```

```
         let page = await cdp.solana.listAccounts();
         while (page.nextPageToken) {
           page = await cdp.solana.listAccounts({ pageToken: page.nextPageToken });
         }
         page.accounts.forEach(account => console.log(account));

```

}

```

###### Implementation of
```ts
SolanaClientInterface.listAccounts

```

##### listTokenBalances()

```
listTokenBalances(options: ListTokenBalancesOptions): Promise<ListTokenBalancesResult>;

```

Defined in: [client/solana/solana.ts:583](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L583) Lists the token balances for a Solana account.

###### Parameters

###### options

[`ListTokenBalancesOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#listtokenbalancesoptions) Parameters for listing the Solana token balances.

###### Returns

`Promise`<[`ListTokenBalancesResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#listtokenbalancesresult)\> A promise that resolves to an array of Solana token balance instances.

###### Example

```
const balances = await cdp.solana.listTokenBalances({ address: "...", network: "solana-devnet" });

```

###### Implementation of

```
SolanaClientInterface.listTokenBalances

```

##### requestFaucet()

```
requestFaucet(options: RequestFaucetOptions): Promise<SignatureResult>;

```

Defined in: [client/solana/solana.ts:392](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L392) Requests funds from a Solana faucet.

###### Parameters

###### options

[`RequestFaucetOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#requestfaucetoptions) Parameters for requesting funds from the Solana faucet.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\> A promise that resolves to the transaction signature.

###### Example

```
         const signature = await cdp.solana.requestFaucet({
           address: "1234567890123456789012345678901234567890",
           token: "sol",
         });

```

###### Implementation of

```
SolanaClientInterface.requestFaucet

```

##### sendTransaction()

```
sendTransaction(options: SendTransactionOptions): Promise<SendTransactionResult>;

```

Defined in: [client/solana/solana.ts:554](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L554) Sends a Solana transaction using the Coinbase API.

###### Parameters

###### options

[`SendTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#sendtransactionoptions) Parameters for sending the Solana transaction.

###### Returns

`Promise`<`SendTransactionResult`\> A promise that resolves to the transaction result.

###### Example

```
const signature = await cdp.solana.sendTransaction({
  network: "solana-devnet",
  transaction: "...",
});

```

###### Implementation of

```
SolanaClientInterface.sendTransaction

```

##### signMessage()

```
signMessage(options: SignMessageOptions): Promise<SignatureResult>;

```

Defined in: [client/solana/solana.ts:423](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L423) Signs a message.

###### Parameters

###### options

[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signmessageoptions) Parameters for signing the message.

###### Returns

`Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\> A promise that resolves to the signature.

###### Example

```
// Create a Solana account
const account = await cdp.solana.createAccount();
// When you want to sign a message, you can do so by address
const signature = await cdp.solana.signMessage({
  address: account.address,
  message: "Hello, world!",
});

```

###### Implementation of

```
SolanaClientInterface.signMessage

```

##### signTransaction()

```
signTransaction(options: SignTransactionOptions): Promise<SignTransactionResult>;

```

Defined in: [client/solana/solana.ts:465](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L465) Signs a transaction.

###### Parameters

###### options

[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signtransactionoptions) Parameters for signing the transaction.

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
const signature = await cdp.solana.signTransaction({
  address: account.address,
  transaction,
});

```

###### Implementation of

```
SolanaClientInterface.signTransaction

```

##### updateAccount()

```
updateAccount(options?: UpdateSolanaAccountOptions): Promise<{
  fund: (options: Omit<SolanaFundOptions, "address">) => Promise<FundOperationResult>;
  quoteFund: (options: Omit<SolanaQuoteFundOptions, "address">) => Promise<SolanaQuote>;
  requestFaucet: (options: Omit<RequestFaucetOptions, "address">) => Promise<SignatureResult>;
  sendTransaction: (options: Omit<SendTransactionOptions, "address">) => Promise<SendTransactionResult>;
  signMessage: (options: Omit<SignMessageOptions, "address">) => Promise<SignatureResult>;
  signTransaction: (options: Omit<SignTransactionOptions, "address">) => Promise<SignTransactionResult>;
  transfer: (options: Omit<TransferOptions, "from">) => Promise<SignatureResult>;
  waitForFundOperationReceipt: Promise<WaitForFundOperationResult>;
}>;

```

Defined in: [client/solana/solana.ts:515](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/solana/solana.ts#L515) Updates a CDP Solana account.

###### Parameters

###### options?

[`UpdateSolanaAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#updatesolanaaccountoptions) Optional parameters for creating the account.

###### Returns

`Promise`<{ `fund`: (`options`: `Omit`<`SolanaFundOptions`, `"address"`\>) => `Promise`<`FundOperationResult`\>; `quoteFund`: (`options`: `Omit`<`SolanaQuoteFundOptions`, `"address"`\>) => `Promise`<`SolanaQuote`\>; `requestFaucet`: (`options`: `Omit`<[`RequestFaucetOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#requestfaucetoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `sendTransaction`: (`options`: `Omit`<[`SendTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#sendtransactionoptions), `"address"`\>) => `Promise`<`SendTransactionResult`\>; `signMessage`: (`options`: `Omit`<[`SignMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signmessageoptions), `"address"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `signTransaction`: (`options`: `Omit`<[`SignTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signtransactionoptions), `"address"`\>) => `Promise`<`SignTransactionResult`\>; `transfer`: (`options`: `Omit`<`TransferOptions`, `"from"`\>) => `Promise`<[`SignatureResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/solana/Types#signatureresult)\>; `waitForFundOperationReceipt`: `Promise`<`WaitForFundOperationResult`\>; }> A promise that resolves to the updated account.

###### Examples

```
         const account = await cdp.sol.updateAccount({ address: "...", update: { name: "New Name" } });

```

```
         const account = await cdp.sol.updateAccount({ address: "...", update: { accountPolicy: "73bcaeeb-d7af-4615-b064-42b5fe83a31e" } });

```

```
         const idempotencyKey = uuidv4();
         // First call
         await cdp.sol.updateAccount({
           address: "0x...",
           update: { accountPolicy: "73bcaeeb-d7af-4615-b064-42b5fe83a31e" },
           idempotencyKey,
         });
         // Second call with the same idempotency key will not update
         await cdp.sol.updateAccount({
           address: '0x...',
           update: { name: "" },
           idempotencyKey,
         });

```

###### Implementation of

```
SolanaClientInterface.updateAccount

```