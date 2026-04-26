# componentzindex

```
type ComponentZIndex = Flattened<{
  zIndex: typeof zIndexComponents;
}>;

```

Component z-index values for individual UI components. They inherit from the SemanticZIndex via CSS variables.

## 

[​](#example)

Example

```
const theme: Partial<ComponentZIndex> = {
  "zIndex-modal-overlay": "1000",
  "zIndex-modal-dialog": "1001",
};

```

## 

[​](#see)

See

-   [zIndexComponents](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Variables/zIndexComponents) for the default token values
-   [SemanticZIndex](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/SemanticZIndex) for the semantic z-index values that component values inherit from