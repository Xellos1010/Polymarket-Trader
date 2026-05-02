# authbuttonprops

The props for the AuthButton component.

## Extends

-   [`AuthButtonSlots`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthButtonSlots)

## Properties

Property

Type

Description

Inherited from

`children?`

(`props`: { `Placeholder?`: `ReactNode`; `SignOutButton?`: `ReactNode`; `SignInModal?`: `ReactNode`; } & { `isInitialized`: `boolean`; `isSignedIn`: `boolean`; }) => `ReactNode`

\-

\-

`closeOnSuccessDelay?`

`null` | `number`

The delay in milliseconds before the sign in modal is closed and the sign out button is shown after the sign in is successful. If null, the sign in modal will not be closed automatically. If 0, the sign in modal will be closed immediately.

\-

`onSignInSuccess?`

() => `void`

A function to call when the sign in is successful.

\-

`onSignInSuccessTransitionEnd?`

() => `void`

A function to call after the sign in success, when the dialog close transition ends.

\-

`onSignOutSuccess?`

() => `void`

A function to call when the sign out is successful.

\-

`placeholder?`

(`props`: `Pick`<`HTMLAttributes`<`HTMLDivElement`\>, `"className"`\>) => `ReactNode`

The placeholder to render while the CDP SDK is initializing.

[`AuthButtonSlots`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthButtonSlots).[`placeholder`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthButtonSlots#placeholder)

`signOutButton?`

(`props`: `Pick`<[`SignOutButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignOutButtonProps), `"onSuccess"`\>) => `ReactNode`

The sign out button, rendered when the user is signed in.

[`AuthButtonSlots`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthButtonSlots).[`signOutButton`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthButtonSlots#signoutbutton)

`signInModal?`

(`props`: `Pick`<[`SignInModalProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInModalProps), `"open"` | `"setIsOpen"` | `"onSuccess"`\>) => `ReactNode`

The sign in modal, rendered when the user is signed out.

[`AuthButtonSlots`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthButtonSlots).[`signInModal`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthButtonSlots#signinmodal)