# themeproviderprops

```
type ThemeProviderProps = {
  theme?: Partial<Theme>;
  data-cdp-provider?: string;
} & HTMLAttributes<HTMLDivElement>;

```

Props for the ThemeProvider component.

## 

[​](#type-declaration)

Type declaration

### 

[​](#theme)

theme?

```
optional theme: Partial<Theme>;

```

Theme overrides

### 

[​](#data-cdp-provider)

data-cdp-provider?

```
optional data-cdp-provider: string;

```

Provider name for multi-provider coordination. Used internally by CDPReactProvider to identify which provider owns focused elements. If not provided, a unique ID is generated automatically.

## 

[​](#see)

See

[ThemeProvider](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ThemeProvider)