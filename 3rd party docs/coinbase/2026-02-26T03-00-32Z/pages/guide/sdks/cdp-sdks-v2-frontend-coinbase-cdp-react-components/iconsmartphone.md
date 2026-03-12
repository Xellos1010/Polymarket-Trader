# iconsmartphone

```
function IconSmartPhone(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Smart phone icon component.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`props`

`Omit`<`SVGProps`<`SVGSVGElement`\>, `"viewBox"`\>

The props for the icon.

## 

[​](#returns)

Returns

`Element` The chat bubble icon.

## 

[​](#example)

Example

```
// Icon is correctly from screen readers
<p>
  <IconSmartPhone />
  Enter code from your authenticator app
</p>
// Icon with screen-reader accessible label only
<p>
  <IconSmartPhone aria-label="Enter code from your authenticator app" />
</p>

```