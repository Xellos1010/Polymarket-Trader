# apierror

## Extends

-   `Error`

## Constructors

### Constructor

```
new APIError(
   statusCode: number, 
   errorType: APIErrorType, 
   errorMessage: string, 
   correlationId?: string, 
   errorLink?: string, 
   cause?: Error): APIError;

```

#### Parameters

Parameter

Type

`statusCode`

`number`

`errorType`

[`APIErrorType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/APIErrorType)

`errorMessage`

`string`

`correlationId?`

`string`

`errorLink?`

`string`

`cause?`

`Error`

#### Returns

`APIError`

#### Overrides

## Methods

### toJSON()

```
toJSON(): {
  errorLink?: string;
  correlationId?: string;
  name: string;
  statusCode: number;
  errorType: APIErrorType;
  errorMessage: string;
};

```

#### Returns

```
{
  errorLink?: string;
  correlationId?: string;
  name: string;
  statusCode: number;
  errorType: APIErrorType;
  errorMessage: string;
}

```

##### errorLink?

```
optional errorLink: string;

```

##### correlationId?

```
optional correlationId: string;

```

##### name

##### statusCode

##### errorType

##### errorMessage

## Properties

Property

Type

`statusCode`

`number`

`errorType`

[`APIErrorType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/APIErrorType)

`errorMessage`

`string`

`correlationId?`

`string`

`errorLink?`

`string`