# exportwalletcopyaddress

```
function ExportWalletCopyAddress(props: ExportWalletCopyAddressProps): Element;

```

Displays a truncated address with a copy button.

## Parameters

Parameter

Type

Description

`props`

[`ExportWalletCopyAddressProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ExportWalletCopyAddressProps)

The props for the ExportWalletCopyAddress component.

## Returns

`Element` The rendered component.

## See

-   [ExportWallet](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWallet)
-   [CopyAddress](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/CopyAddress)

## Example

```
// Render the ExportWalletCopyAddress component with a custom label
function ExportWalletCopyKeyButtonExample() {
  const { evmAddress } = useEvmAddress();
  if (!evmAddress) return null;
  return (
    <ExportWallet address={evmAddress}>
      <ExportWalletCopyAddress label="My wallet address" />
    </ExportWallet>
  );
}

```