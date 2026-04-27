# componentfonts

```
type ComponentFonts = Flattened<{
  font: typeof fontComponents;
}>;

```

Component fonts are the fonts for the individual UI components. They inherit values from the SemanticFonts via CSS variables.

## Example

```
const theme: Partial<ComponentFonts> = {
  // Change the font for ctas to monospace font
  "font-family-cta": "var(--cdp-web-font-family-mono)",
  // Change the font for iframed content to monospace font
  "font-family-iframe": "var(--cdp-web-font-family-mono)",
  // Provide a custom font URL for iframed content (must be a google webfont URL)
  "font-url-iframe": "https://fonts.googleapis.com/css2?family=DM+Mono:ital,wght@0,300;0,400;0,500;1,300;1,400;1,500&display=swap",
};

```

## See

-   [fontComponents](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Variables/fontComponents) for the default token values
-   [SemanticFonts](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/SemanticFonts) for the semantic fonts that the component fonts inherit from