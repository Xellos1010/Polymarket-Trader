# verifymfaflow

```
function VerifyMfaFlow(props: VerifyMfaFlowProps): Element;

```

The flow for the VerifyMfa component. Renders the appropriate content based on the current MFA method and step. The “verification” step renders the current method’s content (verification form). The “list” step renders alternate method options.

## Parameters

Parameter

Type

Description

`props`

[`VerifyMfaFlowProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/VerifyMfaFlowProps)

The component props.

## Returns

`Element` The rendered component.

## Example

```
<VerifyMfa>
  <VerifyMfaTitle />
  <VerifyMfaDescription />
  <VerifyMfaFlow />
</VerifyMfa>

```