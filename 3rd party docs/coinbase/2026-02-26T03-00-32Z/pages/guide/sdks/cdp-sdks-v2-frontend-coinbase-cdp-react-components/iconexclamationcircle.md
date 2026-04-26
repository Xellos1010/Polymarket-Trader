# iconexclamationcircle

```
function IconExclamationCircle(props: Omit<SVGProps<SVGSVGElement>, "viewBox">): Element;

```

Exclamation Circle icon component.

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

`Element` The Exclamation Circle icon.

## 

[​](#example)

Example

```
// Icon is correctly from screen readers
<p>
  <IconExclamationCircle />
  Warning!
</p>
// Icon with screen-reader accessible label only
<p>
  <IconExclamationCircle aria-label="Warning" />
</p>

```