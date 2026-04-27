# createsolanakeyexportiframe

```
function createSolanaKeyExportIframe(options: CreateKeyExportIframeOptions): Promise<CreateKeyExportIframeResult>;

```

Sets up a secure iframe for exporting Solana private keys. This function handles the communication with a secure iframe that safely exports Solana private keys to the user’s clipboard without exposing them to the JavaScript context. The iframe will be automatically cleaned up when the session expires (status “expired”). The `cleanup` function is idempotent, so it’s safe to call it in your teardown code even if the iframe has already been cleaned up due to expiration.

## Parameters

Parameter

Type

Description

`options`

[`CreateKeyExportIframeOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeOptions)

Configuration options for the Solana key export iframe.

## Returns

`Promise`<[`CreateKeyExportIframeResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeResult)\> An object containing the iframe element, cleanup function, and theme update function.

## Examples

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