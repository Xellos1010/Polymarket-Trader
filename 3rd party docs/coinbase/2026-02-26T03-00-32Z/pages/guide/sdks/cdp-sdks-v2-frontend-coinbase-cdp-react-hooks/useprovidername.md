# useprovidername

```
function useProviderName(): string;

```

Hook to access the provider name from a component.

## 

[​](#returns)

Returns

`string` The provider name.

## 

[​](#example)

Example

```
const MyComponent = () => {
  const providerName = useProviderName();
  return <div>{providerName}</div>;
}
function App() {
  return (
    <CDPReactProvider name="provider-instance" config={config}>
      <MyComponent />
    </CDPReactProvider>
  );
}

```