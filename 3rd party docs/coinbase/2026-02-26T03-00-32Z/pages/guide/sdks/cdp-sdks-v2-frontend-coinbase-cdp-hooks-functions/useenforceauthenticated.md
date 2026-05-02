# useenforceauthenticated

```
function useEnforceAuthenticated<TArgs, TReturn>(callback: (...args: TArgs) => Promise<TReturn>): (...args: TArgs) => Promise<TReturn>;

```

Higher-order hook that wraps a callback with authentication enforcement. This ensures that the wrapped function can only be called when the user is signed in.

## Type Parameters

Type Parameter

Description

`TArgs` *extends* `unknown`\[\]

Array type representing the arguments of the callback

`TReturn`

Type representing the return value of the callback

## Parameters

Parameter

Type

Description

`callback`

(…`args`: `TArgs`) => `Promise`<`TReturn`\>

The async function to wrap with authentication check

## Returns

A wrapped version of the callback that checks authentication

```
(...args: TArgs): Promise<TReturn>;

```

### Parameters

### Returns

`Promise`<`TReturn`\>

## Throws

Throws an error if the user is not authenticated when the callback is invoked