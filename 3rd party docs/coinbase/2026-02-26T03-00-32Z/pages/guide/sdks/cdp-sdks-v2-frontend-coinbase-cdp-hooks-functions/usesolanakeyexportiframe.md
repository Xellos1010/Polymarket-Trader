# usesolanakeyexportiframe

```
function useSolanaKeyExportIframe(options: UseKeyExportIframeOptions): UseKeyExportIframeResult;

```

A hook for creating a secure iframe to export Solana private keys. This hook handles the communication with a secure iframe that safely exports Solana private keys to the user’s clipboard without exposing them to the JavaScript context. The iframe will be automatically cleaned up when the component unmounts or when the session expires.

## Parameters

Parameter

Type

Description

`options`

[`UseKeyExportIframeOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/UseKeyExportIframeOptions)

Configuration options for the Solana key export iframe.

## Returns

[`UseKeyExportIframeResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/UseKeyExportIframeResult) An object containing the iframe status and control functions.

## Example

```
import { useSolanaKeyExportIframe } from '@coinbase/cdp-hooks';
import { useRef } from 'react';
function SolanaKeyExportButton() {
  const containerRef = useRef<HTMLDivElement>(null);
  const { status } = useSolanaKeyExportIframe({
    address: "ABC123...",
    containerRef,
    label: "Copy Private Key",
  });
  return (
    <div>
      <p>Status: {status ?? 'initializing'}</p>
      <div ref={containerRef} />
    </div>
  );
}

```