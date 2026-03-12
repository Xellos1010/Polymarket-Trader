# verifymfainlinebackbutton

```
function VerifyMfaInlineBackButton(props: VerifyMfaInlineBackButtonProps): null | Element;

```

Back button for navigating within the MFA verification flow or back to content. In `verifyFirst` mode, it only renders when the inner MFA flow has back navigation. Otherwise, it renders on the verify view and navigates back to content when the inner MFA flow can’t go back.

## Parameters

Parameter

Type

Description

`props`

[`VerifyMfaInlineBackButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/VerifyMfaInlineBackButtonProps)

The props for the component.

## Returns

`null` | `Element` The back button, or null when there is nothing to navigate back to.