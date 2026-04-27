# exportsolanaaccount

```
function exportSolanaAccount(options: ExportSolanaAccountOptions): Promise<ExportSolanaAccountResult>;

```

Exports the private key of a Solana account.

## Parameters

Parameter

Type

Description

`options`

[`ExportSolanaAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ExportSolanaAccountOptions)

The options for exporting the account.

## Returns

`Promise`<[`ExportSolanaAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ExportSolanaAccountResult)\> The result of the export.

## Example

```
const result = await exportSolanaAccount({
  solanaAccount: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"
});

```

## Deprecated

This function will be removed soon. Use `createSolanaKeyExportIframe` instead for a more secure key export experience that never exposes the private key to your application’s JavaScript context.

## See

[createSolanaKeyExportIframe](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Functions/createSolanaKeyExportIframe)