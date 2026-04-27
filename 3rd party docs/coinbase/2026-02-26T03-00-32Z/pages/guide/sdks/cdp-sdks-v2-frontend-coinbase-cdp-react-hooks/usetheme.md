# usetheme

Hooks

```
function useTheme(): ThemeContextValue;

```

Hook to access the theme from a component.

## 

[​](#returns)

Returns

[`ThemeContextValue`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/ThemeContextValue) The theme.

## 

[​](#see)

See

[ThemeProvider](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ThemeProvider)

## 

[​](#example)

Example

```
function App() {
  // Style a paragraph with the secondary text color
  const { theme } = useTheme();
  return <p style={{ color: theme["colors-fg-muted"] }}>Secondary text</p>;
}

```

Was this page helpful?

[

useSignInReducer

Previous

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useSignInReducer)[

useVerifyMfaContext

Next

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useVerifyMfaContext)

⌘I