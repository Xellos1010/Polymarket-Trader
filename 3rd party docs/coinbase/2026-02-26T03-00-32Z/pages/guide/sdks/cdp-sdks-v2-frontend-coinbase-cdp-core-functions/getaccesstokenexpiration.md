# getaccesstokenexpiration

```
function getAccessTokenExpiration(): Promise<null | number>;

```

Gets the expiration time of the access token for the current user.

## 

[​](#returns)

Returns

`Promise`<`null` | `number`\> The expiration time of the access token for the current user, or null if no user is signed in.

## 

[​](#example)

Example

```
const accessTokenExpiration = await getAccessTokenExpiration();

```