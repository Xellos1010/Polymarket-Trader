# eip1193providererror

EIP-1193 provider error.

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
new EIP1193ProviderError(code: EIP1193ErrorCode, message: string): EIP1193ProviderError;

```

Creates a new EIP-1193 Provider error.

#### 

[​](#parameters)

Parameters

Parameter

Type

Description

`code`

[`EIP1193ErrorCode`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EIP1193ErrorCode)

The error code from EIP1193ErrorCode enum.

`message`

`string`

The error message.

#### 

[​](#returns)

Returns

`EIP1193ProviderError`

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

[`EIP1193ErrorCode`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/EIP1193ErrorCode)

The error code from EIP1193ErrorCode enum.