# iconcheck

```
function IconCheck(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Check Circle icon component.

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

`Element` The Check Circle icon.

## 

[​](#example)

Example

```
// Icon is correctly from screen readers
<p>
  <IconCheck />
  Success!
</p>
// Icon with screen-reader accessible label only
<p>
  <IconCheck aria-label="Success" />
</p>

```