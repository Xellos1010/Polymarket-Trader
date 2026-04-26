# issignedin

```
function isSignedIn(): Promise<boolean>;

```

Returns whether the user is currently signed in.

## 

[​](#returns)

Returns

`Promise`<`boolean`\> Whether the user is currently signed in.

## 

[​](#example)

Example

```
const signedIn = await isSignedIn();
if (signedIn) {
  console.log("User is signed in");
}

```