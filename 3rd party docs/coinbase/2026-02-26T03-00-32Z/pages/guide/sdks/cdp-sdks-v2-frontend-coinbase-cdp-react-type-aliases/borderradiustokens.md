# borderradiustokens

```
type BorderRadiusTokens = SemanticBorderRadius & ComponentBorderRadius;

```

Defines all the border radius values in the theme. To change the border radius scale, you can override the [SemanticBorderRadius](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/SemanticBorderRadius) values, and the rest of the values will inherit from them. For more granular control, individual [ComponentBorderRadius](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ComponentBorderRadius) can be overridden. For example, the `borderRadius-sm` semantic border radius is used to define a small amount of border radius. The `borderRadius-input` and `borderRadius-modal` component values inherit from the `borderRadius-sm` semantic border radius via CSS variables. If you want to override the just the default modal border radius, you can do so by defining the `borderRadius-modal` token in the theme.

## See

[borderRadius](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Variables/borderRadius) for the default token values