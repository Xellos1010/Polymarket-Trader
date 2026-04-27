# iconchevrondown

```
function IconChevronDown(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Chevron down icon component.

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

`Element` The chevron down icon.

## 

[​](#example)

Example

```
// Icon is correctly hidden from screen readers
<p>
  <IconChevronDown />
  Expand
</p>
// Icon with screen-reader accessible label only
<p>
  <IconChevronDown aria-label="Expand" />
</p>

```