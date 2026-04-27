# useappconfig

```
function useAppConfig(): Required<AppConfig>;

```

Hook to access the app config from a component.

## 

[​](#returns)

Returns

`Required`<[`AppConfig`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/AppConfig)\> The app config.

## 

[​](#example)

Example

```
const MyComponent = () => {
  // Access the app config from a child component
  const appConfig = useAppConfig();
  return <div>{appConfig.appName}</div>;
}
function App() {
  return (
    <CDPReactProvider config={config}>
      <MyComponent />
    </CDPReactProvider>
  );
}

```