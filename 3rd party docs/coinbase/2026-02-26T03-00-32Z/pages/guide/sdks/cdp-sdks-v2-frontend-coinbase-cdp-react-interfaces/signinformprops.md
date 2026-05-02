# signinformprops

Props for the SignInForm component.

## See

[SignInForm](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignInForm)

## Extends

-   `Omit`<`HTMLAttributes`<`HTMLElement`\>, `"children"`\>

## Properties

Property

Type

Description

Overrides

`as?`

`ElementType`

The element type to render the form as.

\-

`autoFocus?`

`boolean`

If set, will auto focus the form when the component mounts and after transitions.

`Omit.autoFocus`

`onSuccess?`

() => `void`

The function to call when the sign in is successful.

\-

`step?`

`"verification"` | `"credentials"`

If set, will render the form for this step of the sign in flow, regardless of the context value.

\-

`children?`

(`props`: { `step`: `"verification"` | `"credentials"`; `authMethod`: [`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod); `Form`: `ReactNode`; }) => `ReactNode`

The children of the component.

\-