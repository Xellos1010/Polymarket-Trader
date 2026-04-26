# linkauthitemprops

The props for the LinkAuthItem component.

## See

[LinkAuthItem](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthItem)

## Properties

Property

Type

Description

`authMethod`

[`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod)

A user-readable label for the auth method

`isLinked`

`boolean`

Whether the auth method is linked to the current user’s account

`isPending?`

`boolean`

Whether the auth method linking is pending

`icon?`

`ReactNode`

An icon to display

`label`

`string`

A label for the auth method

`userAlias?`

`string`

The user alias for the auth method (i.e. email address, phone number, etc.)

`onLink`

`MouseEventHandler`<`HTMLButtonElement`\>

A function to call when the auth method is linked.