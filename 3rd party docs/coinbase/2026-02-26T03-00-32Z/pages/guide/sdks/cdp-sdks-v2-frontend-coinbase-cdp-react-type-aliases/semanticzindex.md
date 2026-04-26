# semanticzindex

```
type SemanticZIndex = Flattened<{
  zIndex: typeof zIndexSemantic;
}>;

```

Semantic z-index values define stacking context purposes. They describe the purpose of each layer rather than arbitrary numbers:

-   `base`: Default layer (0)
-   `raised`: Slight elevation (1)
-   `popup`: Emerges from content, like dropdowns (200)
-   `scrim`: Blocks what’s beneath, like modal overlays (400)
-   `floating`: Above blocking layer, like content inside modals (500)

## Example

```
const theme: Partial<SemanticZIndex> = {
  "zIndex-scrim": "1000",
  "zIndex-floating": "1001",
};

```

## See

-   [zIndexSemantic](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Variables/zIndexSemantic) for the default token values
-   [ComponentZIndex](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ComponentZIndex) for the component z-index values that inherit from SemanticZIndex