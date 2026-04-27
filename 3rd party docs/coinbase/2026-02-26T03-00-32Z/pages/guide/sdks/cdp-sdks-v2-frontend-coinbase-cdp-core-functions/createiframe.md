# createiframe

```
function createIframe(target: string | HTMLElement): HTMLIFrameElement;

```

Creates an iframe element inside the target container.

## Parameters

Parameter

Type

Description

`target`

`string` | `HTMLElement`

Either a CSS selector string or an HTMLElement container. If a string, will query for an element and create an iframe inside it. If an HTMLElement, will create an iframe inside it.

## Returns

`HTMLIFrameElement` The iframe element.

## Throws

InputValidationError if target is an iframe element, selector resolves to an iframe, or selector doesn’t match any element.