# getaccesstoken

```
function getAccessToken(options: {
  forceRefresh: boolean;
}): Promise<null | string>;

```

Gets the access token for the current user.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`options`

{ `forceRefresh`: `boolean`; }

The options for getting the token.

`options.forceRefresh`

`boolean`

Whether to force a refresh of the token.

## 

[​](#returns)

Returns

`Promise`<`null` | `string`\> The access token for the current user, or null if no user is signed in.

## 

[​](#example)

Example

```
const accessToken = await getAccessToken();

```