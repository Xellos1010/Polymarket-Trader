# mfaerror

Error thrown when an MFA operation fails.

## 

[​](#extends)

Extends

-   `Error`

## 

[​](#constructors)

Constructors

### 

[​](#constructor)

Constructor

```
new MfaError(code: MfaErrorCode, message: string): MfaError;

```

Creates a new MfaError.

#### 

[​](#parameters)

Parameters

Parameter

Type

Description

`code`

[`MfaErrorCode`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/MfaErrorCode)

The error code identifying the type of MFA error.

`message`

`string`

The error message.

#### 

[​](#returns)

Returns

`MfaError`

#### 

[​](#overrides)

Overrides

```
Error.constructor

```

## 

[​](#properties)

Properties

Property

Modifier

Type

Description

`code`

`public`

[`MfaErrorCode`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/MfaErrorCode)

The error code identifying the type of MFA error.