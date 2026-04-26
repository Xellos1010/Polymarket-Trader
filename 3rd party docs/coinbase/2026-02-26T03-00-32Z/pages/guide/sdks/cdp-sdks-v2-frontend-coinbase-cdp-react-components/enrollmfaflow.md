# enrollmfaflow

```
function EnrollMfaFlow(props: EnrollMfaFlowProps): Element;

```

The flow for the EnrollMfa component. Renders the appropriate content based on the current MFA method and step. The “list” step is handled directly by this component. The “setup” and “setup-verification” steps are delegated to method-specific content.

## Parameters

Parameter

Type

Description

`props`

[`EnrollMfaFlowProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/EnrollMfaFlowProps)

The component props.

## Returns

`Element` The rendered component.

## Example

```
<EnrollMfa>
  <EnrollMfaTitle />
  <EnrollMfaDescription />
  <EnrollMfaFlow />
</EnrollMfa>

```