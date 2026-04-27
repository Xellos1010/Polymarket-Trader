# icongooglelogo

```
function IconGoogleLogo(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Google logo icon component.

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

`Element` The Google logo icon.

## 

[​](#example)

Example

```
// Icon is correctly hidden from screen readers
<p>
  <IconGoogleLogo />
  Google
</p>
// Icon with screen-reader accessible label only
<p>
  <IconGoogleLogo aria-label="Google" />
</p>

```