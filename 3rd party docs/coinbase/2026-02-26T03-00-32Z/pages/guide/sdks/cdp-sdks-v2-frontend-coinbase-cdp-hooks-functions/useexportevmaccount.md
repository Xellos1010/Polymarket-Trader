# useexportevmaccount

```
function useExportEvmAccount(): {
  exportEvmAccount: (options: ExportEvmAccountOptions) => Promise<ExportEvmAccountResult>;
};

```

Hook that provides a wrapped function to export EVM account private keys with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to export.

## Returns

```
{
  exportEvmAccount: (options: ExportEvmAccountOptions) => Promise<ExportEvmAccountResult>;
}

```

### exportEvmAccount()

```
exportEvmAccount: (options: ExportEvmAccountOptions) => Promise<ExportEvmAccountResult>;

```

#### Parameters

#### Returns

`Promise`<[`ExportEvmAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/ExportEvmAccountResult)\>

## Example

```
function ExportPrivateKey() {
  const { exportEvmAccount } = useExportEvmAccount();
  const { evmAddress } = useEvmAddress();
  const handleExport = async () => {
    if (!evmAddress) return;
    try {
      const { privateKey } = await exportEvmAccount({
        evmAccount: evmAddress
      });
      console.log("Private Key:", privateKey);
    } catch (error) {
      console.error("Failed to export private key:", error);
    }
  };
  return (
    <button onClick={handleExport}>Export Private Key</button>
  );
}

```

## Deprecated

This function will be removed soon. Use `useEvmKeyExportIframe` instead for a more secure key export experience that never exposes the private key to your application’s JavaScript context.

## See

[useEvmKeyExportIframe](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Functions/useEvmKeyExportIframe)