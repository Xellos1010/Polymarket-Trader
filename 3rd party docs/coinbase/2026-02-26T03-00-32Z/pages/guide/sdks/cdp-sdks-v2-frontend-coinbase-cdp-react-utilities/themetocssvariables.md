# themetocssvariables

```
function themeToCssVariables(theme: Record<string, string>): CDPWebCSSVariables;

```

Converts a theme object to a CSS variables object for the CDP web component library.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`theme`

`Record`<`string`, `string`\>

The theme object to convert.

## 

[​](#returns)

Returns

[`CDPWebCSSVariables`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/CDPWebCSSVariables) A CSS variables object.

## 

[​](#example)

Example

```
const themeOverrides: Partial<Theme> = {
  "color-bg-primary": "red",
};
// { "--cdp-web-color-bg-primary": "red" }
const cssVariables = themeToCssVariables(themeOverrides);

```