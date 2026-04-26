# registermfalistener

```
function registerMfaListener(listener: MfaListener, options?: MfaListenerOptions): () => void;

```

Registers an MFA verification listener. Listeners can be global (no scope) or scoped to a specific DOM element. The event bubbles up from the trigger element through the DOM:

-   Scoped listeners listen on their container and catch events from within
-   Global listeners listen on document and catch anything that bubbles up

## Parameters

Parameter

Type

Description

`listener`

[`MfaListener`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/MfaListener)

The listener function to call when MFA is required.

`options?`

[`MfaListenerOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/MfaListenerOptions)

Optional configuration including scope element.

## Returns

A function to unregister the listener.

### Returns

`void`

## Example

```
// Global listener (responds to any MFA trigger not handled by a scoped listener)
const unregisterGlobal = registerMfaListener(({ methods }) => {
  openMfaModal(methods);
});
// Scoped listener (only responds to triggers inside the container)
const container = document.getElementById('my-flow');
if (container) {
  const unregisterScoped = registerMfaListener(
    ({ methods }) => showInlineMfa(methods),
    { scope: container }
  );
  // Later, when done:
  unregisterScoped();
}
// Clean up global listener when done:
unregisterGlobal();

```