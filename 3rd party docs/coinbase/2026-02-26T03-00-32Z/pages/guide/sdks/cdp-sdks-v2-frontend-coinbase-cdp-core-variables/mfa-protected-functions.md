# mfa protected functions

```
const MFA_PROTECTED_FUNCTIONS: {
  signEvmHash: (options: SignEvmHashOptions) => Promise<SignEvmHashResult>;
  signEvmTransaction: (options: SignEvmTransactionOptions) => Promise<SignEvmTransactionResult>;
  signSolanaTransaction: (options: SignSolanaTransactionOptions) => Promise<SignSolanaTransactionResult>;
  sendEvmTransaction: (options: SendEvmTransactionOptions) => Promise<SendEvmTransactionResult>;
  sendSolanaTransaction: (options: SendSolanaTransactionOptions) => Promise<SendSolanaTransactionResult>;
  signEvmMessage: (options: SignEvmMessageOptions) => Promise<SignEvmHashResult>;
  signSolanaMessage: (options: SignSolanaMessageOptions) => Promise<SignSolanaMessageResult>;
  signEvmTypedData: (options: SignEvmTypedDataOptions) => Promise<SignEvmTypedDataResult>;
  sendUserOperation: (options: SendUserOperationOptions) => Promise<SendUserOperationResult>;
  exportEvmAccount: (options: ExportEvmAccountOptions) => Promise<ExportEvmAccountResult>;
  exportSolanaAccount: (options: ExportSolanaAccountOptions) => Promise<ExportSolanaAccountResult>;
  createEvmKeyExportIframe: (options: CreateKeyExportIframeOptions) => Promise<CreateKeyExportIframeResult>;
  createSolanaKeyExportIframe: (options: CreateKeyExportIframeOptions) => Promise<CreateKeyExportIframeResult>;
};

```

**`Internal`** Map of MFA-protected functions to ensure type safety. If a function is renamed, the import will fail and this file won’t compile.

## Type declaration

### signEvmHash()

```
signEvmHash: (options: SignEvmHashOptions) => Promise<SignEvmHashResult>;

```

Signs a hash with an EVM account.

#### Parameters

Parameter

Type

Description

`options`

[`SignEvmHashOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmHashOptions)

The options for the signing.

#### Returns

`Promise`<[`SignEvmHashResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmHashResult)\> The result of the signing.

#### Example

```
const result = await signEvmHash({
  evmAccount: "0x1234...",
  hash: "0xabcd..." // 32-byte hex string to sign
});

```

### signEvmTransaction()

```
signEvmTransaction: (options: SignEvmTransactionOptions) => Promise<SignEvmTransactionResult>;

```

Signs a hash with an EVM account.

#### Parameters

Parameter

Type

Description

`options`

[`SignEvmTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmTransactionOptions)

The options for the signing.

#### Returns

`Promise`<[`SignEvmTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmTransactionResult)\> The result of the signing.

#### Example

```
const user = await getCurrentUser();
const evmAccount = user?.evmAccountObjects[0]?.address;
const result = await signEvmTransaction({
  evmAccount,
  transaction: {
    to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    value: 100000000000000n, // 0.0001 ETH in wei
    nonce: 0,
    gas: 21000n,
    maxFeePerGas: 30000000000n,
    maxPriorityFeePerGas: 1000000000n,
    chainId: 84532, // Base Sepolia
    type: "eip1559",
  }
});

```

### signSolanaTransaction()

```
signSolanaTransaction: (options: SignSolanaTransactionOptions) => Promise<SignSolanaTransactionResult>;

```

Signs a Solana transaction.

#### Parameters

Parameter

Type

Description

`options`

[`SignSolanaTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignSolanaTransactionOptions)

The options for the signing.

#### Returns

`Promise`<[`SignSolanaTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignSolanaTransactionResult)\> The result of the signing.

#### Example

```
const user = await getCurrentUser();
const solanaAccount = user?.solanaAccountObjects[0]?.address;
const result = await signSolanaTransaction({
  solanaAccount,
  transaction: "base64-encoded-transaction"
});

```

### sendEvmTransaction()

```
sendEvmTransaction: (options: SendEvmTransactionOptions) => Promise<SendEvmTransactionResult>;

```

Sends an EVM transaction.

#### Parameters

Parameter

Type

Description

`options`

[`SendEvmTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmTransactionOptions)

The options for the sending.

#### Returns

`Promise`<[`SendEvmTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendEvmTransactionResult)\> The transaction hash of the sent transaction.

#### Example

```
const user = await getCurrentUser();
const evmAccount = user?.evmAccountObjects[0]?.address;
const result = await sendEvmTransaction({
  evmAccount,
  network: "base-sepolia",
  transaction: {
    to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    value: 100000000000000n, // 0.0001 ETH in wei
    nonce: 0,
    gas: 21000n,
    maxFeePerGas: 30000000000n,
    maxPriorityFeePerGas: 1000000000n,
    chainId: 84532, // Base Sepolia
    type: "eip1559",
  }
});

```

### sendSolanaTransaction()

```
sendSolanaTransaction: (options: SendSolanaTransactionOptions) => Promise<SendSolanaTransactionResult>;

```

Send a Solana transaction.

#### Parameters

Parameter

Type

Description

`options`

[`SendSolanaTransactionOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendSolanaTransactionOptions)

The options for sending the Solana transaction.

#### Returns

`Promise`<[`SendSolanaTransactionResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendSolanaTransactionResult)\> The transaction signature.

### signEvmMessage()

```
signEvmMessage: (options: SignEvmMessageOptions) => Promise<SignEvmHashResult>;

```

Signs an EVM message.

#### Parameters

Parameter

Type

Description

`options`

[`SignEvmMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmMessageOptions)

The options for the signing.

#### Returns

`Promise`<[`SignEvmHashResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmHashResult)\> The result of the signing.

#### Example

```
const user = await getCurrentUser();
const evmAccount = user?.evmAccountObjects[0]?.address;
const result = await signEvmMessage({
  evmAccount,
  message: "Hello World" // Message to sign
});

```

### signSolanaMessage()

```
signSolanaMessage: (options: SignSolanaMessageOptions) => Promise<SignSolanaMessageResult>;

```

Signs a message with a Solana account.

#### Parameters

Parameter

Type

Description

`options`

[`SignSolanaMessageOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignSolanaMessageOptions)

The options for the signing.

#### Returns

`Promise`<[`SignSolanaMessageResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignSolanaMessageResult)\> The result of the signing.

#### Example

```
const user = await getCurrentUser();
const solanaAccount = user?.solanaAccountObjects[0]?.address;
const message = Buffer.from("Hello, Solana!", "utf8").toString("base64");
const result = await signSolanaMessage({
  solanaAccount,
  message // Base64 encoded message to sign
});

```

### signEvmTypedData()

```
signEvmTypedData: (options: SignEvmTypedDataOptions) => Promise<SignEvmTypedDataResult>;

```

Signs EIP-712 typed data with an EVM account.

#### Parameters

Parameter

Type

Description

`options`

[`SignEvmTypedDataOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmTypedDataOptions)

The options for the signing.

#### Returns

`Promise`<[`SignEvmTypedDataResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SignEvmTypedDataResult)\> The result of the signing.

#### Example

```
const user = await getCurrentUser();
const evmAccount = user?.evmAccountObjects[0]?.address;
const result = await signEvmTypedData({
  evmAccount,
  typedData: {
    domain: {
      name: "USDC",
      version: "2",
      chainId: 84532,
      verifyingContract: "0x036CbD53842c5426634e7929541eC2318f3dCF7e", // Base Sepolia USDC
    },
    types: {
      EIP712Domain: [
        { name: "name", type: "string" },
        { name: "version", type: "string" },
        { name: "chainId", type: "uint256" },
        { name: "verifyingContract", type: "address" },
      ],
      TransferWithAuthorization: [
        { name: "from", type: "address" },
        { name: "to", type: "address" },
        { name: "value", type: "uint256" },
        { name: "validAfter", type: "uint256" },
        { name: "validBefore", type: "uint256" },
        { name: "nonce", type: "bytes32" },
      ],
    },
    primaryType: "TransferWithAuthorization",
    message: {
      from: evmAccount,
      to: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
      value: "1000000", // 1 USDC (6 decimals)
      validAfter: 0, // Valid immediately
      validBefore: 2524604400, // Valid until 2050
      nonce: 0
    },
  },
});

```

### sendUserOperation()

```
sendUserOperation: (options: SendUserOperationOptions) => Promise<SendUserOperationResult>;

```

Sends a user operation from a smart account.

#### Parameters

Parameter

Type

Description

`options`

[`SendUserOperationOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendUserOperationOptions)

The options for sending the user operation.

#### Returns

`Promise`<[`SendUserOperationResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SendUserOperationResult)\> Promise that resolves to the user operation hash.

#### Example

```
const user = await getCurrentUser();
const smartAccount = user?.evmSmartAccountObjects[0]?.address;
const result = await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [{
    to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    value: 0n,
    data: "0x",
  }],
  dataSuffix: "0x62617365617070070080218021802180218021802180218021", // ERC-8021 attribution
});
console.log("User Operation Hash:", result.userOperationHash);

```

### exportEvmAccount()

```
exportEvmAccount: (options: ExportEvmAccountOptions) => Promise<ExportEvmAccountResult>;

```

Exports an EVM account’s private key.

#### Parameters

Parameter

Type

Description

`options`

[`ExportEvmAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ExportEvmAccountOptions)

The options for the exporting.

#### Returns

`Promise`<[`ExportEvmAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ExportEvmAccountResult)\> The result of the export.

#### Example

```
const result = await exportEvmAccount({
  evmAccount: "0x1234..."
});

```

#### Deprecated

This function will be removed soon. Use `createEvmKeyExportIframe` instead for a more secure key export experience that never exposes the private key to your application’s JavaScript context.

#### See

[createEvmKeyExportIframe](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Functions/createEvmKeyExportIframe)

### exportSolanaAccount()

```
exportSolanaAccount: (options: ExportSolanaAccountOptions) => Promise<ExportSolanaAccountResult>;

```

Exports the private key of a Solana account.

#### Parameters

Parameter

Type

Description

`options`

[`ExportSolanaAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ExportSolanaAccountOptions)

The options for exporting the account.

#### Returns

`Promise`<[`ExportSolanaAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ExportSolanaAccountResult)\> The result of the export.

#### Example

```
const result = await exportSolanaAccount({
  solanaAccount: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"
});

```

#### Deprecated

This function will be removed soon. Use `createSolanaKeyExportIframe` instead for a more secure key export experience that never exposes the private key to your application’s JavaScript context.

#### See

[createSolanaKeyExportIframe](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Functions/createSolanaKeyExportIframe)

### createEvmKeyExportIframe()

```
createEvmKeyExportIframe: (options: CreateKeyExportIframeOptions) => Promise<CreateKeyExportIframeResult>;

```

Sets up a secure iframe for exporting EVM private keys. This function handles the communication with a secure iframe that safely exports EVM private keys to the user’s clipboard without exposing them to the JavaScript context. The iframe will be automatically cleaned up when the session expires (status “expired”). The `cleanup` function is idempotent, so it’s safe to call it in your teardown code even if the iframe has already been cleaned up due to expiration.

#### Parameters

Parameter

Type

Description

`options`

[`CreateKeyExportIframeOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeOptions)

Configuration options for the EVM key export iframe.

#### Returns

`Promise`<[`CreateKeyExportIframeResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeResult)\> An object containing the iframe element, cleanup function, and theme update function.

#### Examples

```
// Using a CSS selector for the container
const { iframe, cleanup } = await createEvmKeyExportIframe({
  address: "0x1234...",
  target: "#key-export-container",
  projectId: "your-project-id",
  label: "Copy Private Key",
  onStatusUpdate: (status, message) => {
    if (status === "success") {
      console.log("Key copied!");
    } else if (status === "error") {
      console.error("Error:", message);
    }
  }
});
// Later, clean up. Iframe will auto cleanup when it expires.
cleanup();

```

```
// Using an HTMLElement container directly
const container = document.getElementById("my-container") as HTMLElement;
const { cleanup, updateTheme } = await createEvmKeyExportIframe({
  address: "0x1234...",
  target: container,
  projectId: "your-project-id",
  theme: {
    buttonBg: "#0052ff",
    buttonText: "#ffffff"
  }
});
// Update theme later
updateTheme({ buttonBg: "#ff0000" });

```

### createSolanaKeyExportIframe()

```
createSolanaKeyExportIframe: (options: CreateKeyExportIframeOptions) => Promise<CreateKeyExportIframeResult>;

```

Sets up a secure iframe for exporting Solana private keys. This function handles the communication with a secure iframe that safely exports Solana private keys to the user’s clipboard without exposing them to the JavaScript context. The iframe will be automatically cleaned up when the session expires (status “expired”). The `cleanup` function is idempotent, so it’s safe to call it in your teardown code even if the iframe has already been cleaned up due to expiration.

#### Parameters

Parameter

Type

Description

`options`

[`CreateKeyExportIframeOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeOptions)

Configuration options for the Solana key export iframe.

#### Returns

`Promise`<[`CreateKeyExportIframeResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeResult)\> An object containing the iframe element, cleanup function, and theme update function.

#### Examples

```
// Using a CSS selector for the container
const { iframe, cleanup } = await createSolanaKeyExportIframe({
  address: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
  target: "#key-export-container",
  projectId: "your-project-id",
  label: "Copy Private Key",
  onStatusUpdate: (status, message) => {
    if (status === "success") {
      console.log("Key copied!");
    } else if (status === "error") {
      console.error("Error:", message);
    }
  }
});
// Later, clean up. Iframe will auto cleanup when it expires.
cleanup();

```

```
// Using an HTMLElement container directly
const container = document.getElementById("my-container") as HTMLElement;
const { cleanup, updateTheme } = await createSolanaKeyExportIframe({
  address: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
  target: container,
  projectId: "your-project-id",
  theme: {
    buttonBg: "#9945ff",
    buttonText: "#ffffff"
  }
});
// Update theme later
updateTheme({ buttonBg: "#ff0000" });

```