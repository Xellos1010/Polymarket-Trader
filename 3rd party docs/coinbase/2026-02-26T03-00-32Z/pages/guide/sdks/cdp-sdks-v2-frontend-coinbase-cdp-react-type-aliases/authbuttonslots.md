# authbuttonslots

```
type AuthButtonSlots = {
  placeholder?: (props: Pick<HTMLAttributes<HTMLDivElement>, "className">) => ReactNode;
  signOutButton?: (props: Pick<SignOutButtonProps, "onSuccess">) => ReactNode;
  signInModal?: (props: Pick<SignInModalProps, "open" | "setIsOpen" | "onSuccess">) => ReactNode;
};

```

Component slots for the AuthButton.

## Extended by

-   [`AuthButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/AuthButtonProps)

## Properties

Property

Type

Description

`placeholder?`

(`props`: `Pick`<`HTMLAttributes`<`HTMLDivElement`\>, `"className"`\>) => `ReactNode`

The placeholder to render while the CDP SDK is initializing.

`signOutButton?`

(`props`: `Pick`<[`SignOutButtonProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignOutButtonProps), `"onSuccess"`\>) => `ReactNode`

The sign out button, rendered when the user is signed in.

`signInModal?`

(`props`: `Pick`<[`SignInModalProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInModalProps), `"open"` | `"setIsOpen"` | `"onSuccess"`\>) => `ReactNode`

The sign in modal, rendered when the user is signed out.