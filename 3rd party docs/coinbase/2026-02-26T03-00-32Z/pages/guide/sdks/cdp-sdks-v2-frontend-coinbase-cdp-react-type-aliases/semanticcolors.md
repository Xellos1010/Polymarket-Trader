# semanticcolors

```
type SemanticColors = Flattened<{
  colors: typeof colorsSemantic;
}>;

```

Semantic colors are the base colors for the theme. They are typically not used directly in the components, but are used to define the base colors for the components.

## Example

```
const theme: Partial<SemanticColors> = {
  "colors-bg-default": "#ffffff",
  "colors-bg-alternate": "#eef0f3",
  "colors-bg-overlay": "color(from var(--cdp-web-colors-bg-alternate) srgb r g b / 0.33)",
};

```

## See

-   [colorsSemantic](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Variables/colorsSemantic) for the default token values
-   [ComponentColors](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ComponentColors) for the component colors that inherit from the SemanticColors