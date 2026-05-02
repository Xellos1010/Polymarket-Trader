# exportwallet

```
function ExportWallet(props: ExportWalletProps): Element;

```

The ExportWallet component is used to export the private key of an EVM or Solana account. Note that an EVM smart account’s private key cannot be exported. If a smart account address is provided, a warning message will be displayed explaining that the private key cannot be copied, and the copy key button will not be rendered.

## Parameters

Parameter

Type

Description

`props`

[`ExportWalletProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/ExportWalletProps)

The props for the ExportWallet component.

## Returns

`Element` The rendered component.

## See

-   [ExportWalletTitle](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletTitle)
-   [ExportWalletWarning](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletWarning)
-   [ExportWalletCopyAddress](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletCopyAddress)
-   [ExportWalletCopyKeyButton](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletCopyKeyButton)
-   [ExportWalletFooter](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletFooter)

## Examples

```
// Render the ExportWallet component with an EVM address
function ExportWalletExample() {
  const { evmAddress } = useEvmAddress();
  return (
    <ExportWallet address={evmAddress} />
  );
}

```

```
// Render the ExportWallet component with a Solana address
function ExportWalletExample() {
  const { solanaAddress } = useSolanaAddress();
  return (
    <ExportWallet address={solanaAddress} />
  );
}

```

## Further reading

-   [ExportWallet Overview](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWallet.README)