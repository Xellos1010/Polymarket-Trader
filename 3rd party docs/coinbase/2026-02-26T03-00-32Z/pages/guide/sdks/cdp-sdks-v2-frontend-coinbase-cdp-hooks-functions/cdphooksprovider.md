# cdphooksprovider

```
function CDPHooksProvider(props: CDPHooksProviderProps): Element;

```

Provider component that initializes the CDP SDK and manages authentication state. This must be placed at the root of your application or above any components that need CDP functionality.

## Parameters

Parameter

Type

Description

`props`

[`CDPHooksProviderProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/CDPHooksProviderProps)

The component props

## Returns

`Element` A React context provider wrapping the children with CDP functionality

## Example

```
const cdpConfig = {
  projectId: "your-project-id" // Your project ID from the CDP Portal
}
function App() {
  return (
    <CDPHooksProvider config={cdpConfig}>
      <YourApp />
    </CDPHooksProvider>
  );
}

```