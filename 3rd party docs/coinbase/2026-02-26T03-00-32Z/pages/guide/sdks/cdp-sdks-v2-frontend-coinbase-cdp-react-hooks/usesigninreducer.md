# usesigninreducer

```
function useSignInReducer(initialState: SignInState): [SignInState, ActionDispatch<[SignInAction]>];

```

A reducer hook for the SignIn component.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`initialState`

[`SignInState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInState)

The initial state of the component.

## 

[​](#returns)

Returns

\[[`SignInState`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/SignInState), `ActionDispatch`<\[[`SignInAction`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/SignInAction)\]>\] The current state and dispatcher to perform actions on the state.