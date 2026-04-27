# semanticborderradius

```
type SemanticBorderRadius = Flattened<{
  borderRadius: typeof borderRadiusSemantic;
}>;

```

Semantic border radii are the base border radius definitions for the theme. They are typically not used directly in the components, but are used to define the border radius for the components. Semantic border radius values are defind in a scale (none, xs - xl, full). The default values are derived from the base font size using calc(). This is to mimic the way rems function but instead of using the font-size applied to the root document, it uses the base font size provided by the theme. Rems can be used, but it’s recommended to set the base font size to a value that uses rems as well (most applications will use 16px or 1rem).

## Example

```
const theme: Partial<SemanticBorderRadius> = {
  "border-radius-lg": "1.5rem", // make sure to set base font size to a rem value if you use rems
  "border-radius-xl": "2rem",
};

```

## See

-   [borderRadiusSemantic](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Variables/borderRadiusSemantic) for the default semantic border radius token values
-   [ComponentBorderRadius](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/ComponentBorderRadius) for the component border radius values that inherit from the SemanticBorderRadius