# copyevmkeybutton

```
function CopyEvmKeyButton(props: CopyEvmKeyButtonProps): null | Element;

```

The CopyEvmKeyButton component is used to copy the private key of an EVM account.

## Parameters

Parameter

Type

Description

`props`

[`CopyEvmKeyButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/CopyEvmKeyButtonProps)

The props for the CopyEvmKeyButton component.

## Returns

`null` | `Element` The CopyEvmKeyButton component.

## Examples

```
// Render the CopyEvmKeyButton component
function CopyEvmKeyButtonExample() {
  const { currentUser } = useCurrentUser();
  const evmAddress = currentUser?.evmAccountObjects?.[0]?.address;
  if (!evmAddress) return null;
  return (
    <CopyEvmKeyButton address={evmAddress} />
  );
}

```

```
// Render the CopyEvmKeyButton component with custom labels
function CopyEvmKeyButtonExample() {
  const { currentUser } = useCurrentUser();
  const evmAddress = currentUser?.evmAccountObjects?.[0]?.address;
  if (!evmAddress) return null;
  return (
    <CopyEvmKeyButton address={evmAddress} label="Copy private key" copiedLabel="Private key copied" />
  );
}

```

```
// Render the CopyEvmKeyButton component with a different variant
function CopyEvmKeyButtonExample() {
  const { currentUser } = useCurrentUser();
  const evmAddress = currentUser?.evmAccountObjects?.[0]?.address;
  if (!evmAddress) return null;
  return (
    <CopyEvmKeyButton address={evmAddress} variant="secondary" />
  );
}

```

```
// Render the CopyEvmKeyButton component with theme overrides
function CopyEvmKeyButtonExample() {
  const { currentUser } = useCurrentUser();
  const evmAddress = currentUser?.evmAccountObjects?.[0]?.address;
  if (!evmAddress) return null;
  return (
    <CopyEvmKeyButton address={evmAddress} theme={{ fontUrl: "https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap", fontFamily: '"Roboto", sans-serif' }} />
  );
}

```