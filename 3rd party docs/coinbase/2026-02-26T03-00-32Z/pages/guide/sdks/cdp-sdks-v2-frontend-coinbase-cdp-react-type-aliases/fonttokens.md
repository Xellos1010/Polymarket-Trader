# fonttokens

```
type FontTokens = SemanticFonts & ComponentFonts;

```

Defines all the fonts in the theme. To fully change the theme, you only need to define the [SemanticFonts](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/SemanticFonts), and the rest of the values will inherit from them. For more granular control, individual [ComponentFonts](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ComponentFonts) can be overridden. For example, the `font-family-interactive` semantic font is used to define the default font for interactive elements. The `font-family-cta` component font inherits from the `font-family-interactive` semantic font via CSS variables. If you want to override the just the default CTA font, you can do so by defining the `font-family-cta` token in the theme.

## Example

```
const fontTokens: Partial<FontTokens> = {
  // Change the default monospace font
  "font-family-mono": "'Source Code Pro', monospace",
  // Make buttons use the default monospace font
  "font-family-cta": "var(--cdp-web-font-family-mono)",
  // Make iframed content use the default monospace font
  "font-family-iframe": "var(--cdp-web-font-family-mono)",
  // Provide a custom font URL for iframed content (must be a google webfont URL)
  "font-url-iframe": "https://fonts.googleapis.com/css2?family=Source+Code+Pro:ital,wght@0,200..900;1,200..900&display=swap",
};

```

## See

[font](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Variables/font) for the default token values