# colortokens

```
type ColorTokens = SemanticColors & ComponentColors;

```

Defines all the colors in the theme. To fully change the theme, you only need to define the [SemanticColors](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/SemanticColors), and the rest of the values will inherit from them. For more granular control, individual [ComponentColors](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ComponentColors) can be overridden. For example, the `colors-bg-primary` semantic color is used to define the background color for a “primary” variant component. The `colors-cta-primary-bg-default` component color inherits from the `colors-bg-primary` semantic color via CSS variables. If you want to override the just the default background color of the primary cta, you can do so by defining the `colors-cta-primary-bg-default` token in the theme.

## Example

```
// Change the primary background color to teal but make the primary cta button black
const theme: Partial<Theme> = {
  "colors-bg-primary": "teal",
  "colors-cta-primary-bg-default": "black",
};

```