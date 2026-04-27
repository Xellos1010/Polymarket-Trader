# isemailinvalid

```
function isEmailInvalid(value: string): boolean;

```

Check if an email address is invalid.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`value`

`string`

The email address to validate.

## 

[​](#returns)

Returns

`boolean` `true` if the email address is invalid, `false` otherwise.

## 

[​](#example)

Example

```
if (isEmailInvalid("test@example")) {
  console.log("Invalid email address");
}

```