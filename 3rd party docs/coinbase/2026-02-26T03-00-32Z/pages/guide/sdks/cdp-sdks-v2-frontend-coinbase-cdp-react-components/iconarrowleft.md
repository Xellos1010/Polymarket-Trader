# iconarrowleft

```
function IconArrowLeft(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Arrow Left icon component.

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
  <IconArrowLeft />
  Back
</p>
// Icon with screen-reader accessible label only
<p>
  <IconArrowLeft aria-label="Back" />
</p>

```