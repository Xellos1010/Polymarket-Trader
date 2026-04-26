# iconexclamationtriangle

```
function IconExclamationTriangle(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Exclamation Triangle icon component.

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

`Element` The Exclamation Triangle icon.

## 

[​](#example)

Example

```
// Icon is correctly hidden from screen readers
<p>
  <IconExclamationTriangle />
  Warning!
</p>
// Icon with screen-reader accessible label only
<p>
  <IconExclamationTriangle aria-label="Warning" />
</p>

```