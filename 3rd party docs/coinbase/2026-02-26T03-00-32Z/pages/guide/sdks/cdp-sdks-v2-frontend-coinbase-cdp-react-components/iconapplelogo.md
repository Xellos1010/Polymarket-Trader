# iconapplelogo

```
function IconAppleLogo(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Apple logo icon component.

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

`Element` The Apple logo icon.

## 

[​](#example)

Example

```
// Icon is correctly hidden from screen readers
<p>
  <IconAppleLogo />
  Apple
</p>
// Icon with screen-reader accessible label only
<p>
  <IconAppleLogo aria-label="Apple" />
</p>

```