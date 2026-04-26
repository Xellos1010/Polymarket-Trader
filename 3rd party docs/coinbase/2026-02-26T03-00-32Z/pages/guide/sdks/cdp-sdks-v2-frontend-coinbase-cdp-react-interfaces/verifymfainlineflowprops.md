# verifymfainlineflowprops

Props for VerifyMfaInlineFlow component.

## See

[VerifyMfaInlineFlow](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaInlineFlow)

## Properties

Property

Type

Description

`children`

| `ReactNode` | (`props`: | { `view`: `"verify"`; `Content`: `ReactNode`; } | { `view`: `"content"`; `Content`: `null`; }) => `ReactNode`

The content to display. Can be: - A ReactNode: Used as the protected content (shown after MFA verification) - A render function: Receives `{ view, Content }` for full control over rendering When using a render function: - `view` is the current view (“verify” or “content”) - `Content` is the default UI for that view (MFA form for “verify”, null for “content”)

`className?`

`string`

Additional class name for the transition container.

`animateHeight?`

`boolean`

Whether to animate the height during transitions. **Default** `true`

`autoFocus?`

`boolean`

Whether to auto focus forms. **Default** `true`

`transition?`

`"fade"` | `"slide"`

The type of transition to use between views. - “slide”: Slides content left/right (default) - “fade”: Fades content in/out **Default** `"slide"`