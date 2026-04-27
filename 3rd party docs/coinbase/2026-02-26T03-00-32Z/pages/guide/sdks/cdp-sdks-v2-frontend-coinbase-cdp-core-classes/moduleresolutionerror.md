# moduleresolutionerror

Error thrown when required native modules are not available in the React Native environment.

## Extends

-   `Error`

## Constructors

### Constructor

```
new ModuleResolutionError(
   moduleName: string, 
   requiredFor: string, 
   message?: string): ModuleResolutionError;

```

Creates a new ModuleResolutionError.

#### Parameters

Parameter

Type

Description

`moduleName`

`string`

The name of the missing module.

`requiredFor`

`string`

The API or feature that requires the module.

`message?`

`string`

Optional custom error message.

#### Returns

`ModuleResolutionError`

#### Overrides

## Properties

Property

Modifier

Type

Description

`moduleName`

`public`

`string`

The name of the missing module.

`requiredFor`

`public`

`string`

The API or feature that requires the missing module.