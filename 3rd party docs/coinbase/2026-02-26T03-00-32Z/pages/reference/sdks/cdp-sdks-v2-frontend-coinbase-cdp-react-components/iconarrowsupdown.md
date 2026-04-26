# iconarrowsupdown

```
function IconArrowsUpDown(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Arrows Up/Down icon component.

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

`Element` The Arrow Left icon.

## 

[​](#example)

Example

```
// Icon is correctly hidden from screen readers
<p>
  <IconArrowsUpDown />
  Swap
</p>
// Icon with screen-reader accessible label only
<p>
  <IconArrowsUpDown aria-label="Swap" />
</p>

```