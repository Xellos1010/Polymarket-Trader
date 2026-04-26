# useexportsolanaaccount

```
function useExportSolanaAccount(): {
  exportSolanaAccount: (options: ExportSolanaAccountOptions) => Promise<ExportSolanaAccountResult>;
};

```

Hook that provides a wrapped function to export Solana account private keys with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to export.

## Returns

```
{
  exportSolanaAccount: (options: ExportSolanaAccountOptions) => Promise<ExportSolanaAccountResult>;
}

```

### exportSolanaAccount()

```
exportSolanaAccount: (options: ExportSolanaAccountOptions) => Promise<ExportSolanaAccountResult>;

```

#### Parameters

#### Returns

`Promise`<[`ExportSolanaAccountResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/ExportSolanaAccountResult)\>

## Example

```
function ExportSolanaPrivateKey() {
  const { exportSolanaAccount } = useExportSolanaAccount();
  const { solanaAddress } = useSolanaAddress();
  const handleExport = async () => {
    if (!solanaAddress) return;
    try {
      const { privateKey } = await exportSolanaAccount({
        solanaAccount: solanaAddress
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

This function will be removed soon. Use `useSolanaKeyExportIframe` instead for a more secure key export experience that never exposes the private key to your application’s JavaScript context.

## See

[useSolanaKeyExportIframe](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Functions/useSolanaKeyExportIframe)