# getcurrentuser

```
function getCurrentUser(): Promise<
  | null
| User>;

```

Gets the currently signed-in user, if any.

## 

[​](#returns)

Returns

`Promise`< | `null` | [`User`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/User)\> The currently signed-in user, or null if no user is signed in.

## 

[​](#example)

Example

```
const user = await getCurrentUser();

```