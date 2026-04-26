# uselinkauthflow

```
function useLinkAuthFlow(): {
  back: () => void;
  direction?: "left" | "right";
  link: (method: AuthMethod) => void;
  linkSuccess: () => void;
};

```

A hook to get the link auth flow context value.

## Returns

The link auth flow context value.

### back()

A function to call when the back button is clicked.

#### Returns

`void`

### direction?

```
optional direction: "left" | "right";

```

The direction of the flow transition.

### link()

```
link: (method: AuthMethod) => void;

```

A function to call when the user links an auth method.

#### Parameters

Parameter

Type

`method`

[`AuthMethod`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/AuthMethod)

#### Returns

`void`

### linkSuccess()

A function to call when an auth method is successfully linked.

#### Returns

`void`

## See

-   [LinkAuthFlow](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuthFlow)
-   [LinkAuth](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuth)