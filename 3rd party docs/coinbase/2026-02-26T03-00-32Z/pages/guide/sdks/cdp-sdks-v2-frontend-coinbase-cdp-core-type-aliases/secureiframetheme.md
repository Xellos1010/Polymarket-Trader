# secureiframetheme

```
type SecureIframeTheme = {
  pageBg: string;
  buttonBg: string;
  buttonBgHover: string;
  buttonBgPressed: string;
  buttonBgFocus: string;
  buttonBorder: string;
  buttonBorderHover: string;
  buttonBorderPressed: string;
  buttonBorderFocus: string;
  buttonBorderFocusInset: string;
  buttonText: string;
  buttonTextHover: string;
  buttonTextPressed: string;
  buttonTextFocus: string;
  buttonBorderRadius: number;
  buttonFontSize: number;
  buttonFontWeight: number;
  buttonSize: "xs" | "sm" | "md" | "lg";
  fontUrl: string;
  fontFamily: string;
};

```

The theme for the secure iframe. Colors should be hex strings (with or without alpha) or the string “transparent”.

## Properties

Property

Type

Description

`pageBg`

`string`

The background color of the page.

`buttonBg`

`string`

The background color of the button.

`buttonBgHover`

`string`

The background color of the button when hovered.

`buttonBgPressed`

`string`

The background color of the button when pressed.

`buttonBgFocus`

`string`

The background color of the button when focused.

`buttonBorder`

`string`

The border color of the button.

`buttonBorderHover`

`string`

The border color of the button when hovered.

`buttonBorderPressed`

`string`

The border color of the button when pressed.

`buttonBorderFocus`

`string`

The ring color of the button when focused.

`buttonBorderFocusInset`

`string`

The inner ring color of the button when focused.

`buttonText`

`string`

The text color of the button.

`buttonTextHover`

`string`

The text color of the button when hovered.

`buttonTextPressed`

`string`

The text color of the button when pressed.

`buttonTextFocus`

`string`

The text color of the button when focused.

`buttonBorderRadius`

`number`

The border radius of the button.

`buttonFontSize`

`number`

The font size of the button.

`buttonFontWeight`

`number`

The font weight of the button.

`buttonSize`

`"xs"` | `"sm"` | `"md"` | `"lg"`

The size of the button.

`fontUrl`

`string`

The URL of the font to use for the button. Must be a google webfont URL.

`fontFamily`

`string`

The font family to use for the button.