# exportevmaccount

```
function exportEvmAccount(options: ExportEvmAccountOptions): Promise<ExportEvmAccountResult>;

```

Exports an EVM account’s private key.

## Parameters

Parameter

Type

Description

`options`

[`ExportEvmAccountOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ExportEvmAccountOptions)

The options for the exporting.

## Returns

`Promise`<[`ExportEvmAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/ExportEvmAccountResult)\> The result of the export.

## Example

```
const result = await exportEvmAccount({
  evmAccount: "0x1234..."
});

```

## Deprecated

This function will be removed soon. Use `createEvmKeyExportIframe` instead for a more secure key export experience that never exposes the private key to your application’s JavaScript context.

## See

[createEvmKeyExportIframe](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Functions/createEvmKeyExportIframe)