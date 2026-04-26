# verifymfainlineprops

Props for the VerifyMfaInline component.

## See

[VerifyMfaInline](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/VerifyMfaInline)

## Extends

-   `HTMLAttributes`<`HTMLDivElement`\>

## Properties

Property

Type

Description

Overrides

`children`

`ReactNode`

The component children. Should include VerifyMfaInlineFlow. Can optionally include VerifyMfaInlineBackButton outside the Flow.

`HTMLAttributes.children`

`verifyFirst?`

`boolean`

If true, forces MFA verification before showing content. Use this when you want users to verify BEFORE seeing the content. If false (default), content is shown first and MFA is triggered automatically when a protected action is called. **Default** `true`

\-

`onVerified?`

() => `void`

Called when MFA verification completes successfully.

\-

`onCancel?`

() => `void`

Called when MFA verification is cancelled. If provided, a back/cancel action in the verify view will trigger this and transition back to content.

\-

`successDelay?`

`number`

The delay in milliseconds before transitioning to content after successful verification. This allows users to see the success state before the transition. **Default** `500`

\-

`className?`

`string`

Additional class name for the container.

`HTMLAttributes.className`