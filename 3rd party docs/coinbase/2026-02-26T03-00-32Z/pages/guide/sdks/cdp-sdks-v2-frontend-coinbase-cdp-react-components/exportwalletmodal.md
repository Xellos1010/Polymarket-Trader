# exportwalletmodal

```
function ExportWalletModal(props: ExportWalletModalProps): Element;

```

A export wallet modal component that wraps the [ExportWallet](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWallet) component.

## Parameters

Parameter

Type

Description

`props`

[`ExportWalletModalProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/ExportWalletModalProps)

The props for the ExportWalletModal component.

## Returns

`Element` The ExportWalletModal component.

## See

-   [ExportWalletModalClose](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletModalClose) for the modal close button.
-   [ExportWalletModalContent](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletModalContent) for the modal content.
-   [ExportWalletModalTitle](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletModalTitle) for the modal title.
-   [ExportWalletModalTrigger](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletModalTrigger) for the trigger button.

## Examples

```
// Render the ExportWalletModal component with an EVM address
function ExportWalletModalExample() {
  const { evmAddress } = useEvmAddress();
  return (
    <ExportWalletModal address={evmAddress} />
  );
}

```

```
// Render the ExportWalletModal component with a Solana address
function ExportWalletModalExample() {
  const { solanaAddress } = useSolanaAddress();
  return (
    <ExportWalletModal address={solanaAddress} />
  );
}

```

```
// Render the ExportWalletModal component with a custom label for the trigger button
function ExportWalletModalExample() {
  const { solanaAddress } = useSolanaAddress();
  return (
    <ExportWalletModal address={solanaAddress}>
      <ExportWalletModalTrigger label="Export Solana wallet" />
    </ExportWalletModal>
  );
}

```

```
// Render the ExportWalletModal component with a custom button as the trigger
function ExportWalletModalExample() {
  const { solanaAddress } = useSolanaAddress();
  return (
    <ExportWalletModal address={solanaAddress}>
      <button type="button">Export Solana wallet</button>
    </ExportWalletModal>
  );
}

```

```
// Render the ExportWalletModal component with customized content
function ExportWalletModalExample() {
  const { solanaAddress } = useSolanaAddress();
  return (
    <ExportWalletModal address={solanaAddress}>
      <ExportWalletModalTrigger />
      <ExportWalletModalContent>
        <ExportWallet address={solanaAddress}>
          <div className="header">
            <ExportWalletModalTitle />
            <ExportWalletModalClose />
          </div>
          <div className="content">
            <ExportWalletWarning />
            <ExportWalletCopyAddress />
            <ExportWalletCopyKeyButton />
            <p className="help-text">
              Your private key gives full control of your wallet.
              Store it safely and never share it with anyone.
            </p>
          </div>
        </ExportWallet>
      </ExportWalletModalContent>
    </ExportWalletModal>
  );
}

```