# flattened

```
type Flattened<T> = { [K in KebabCasePaths<T>]: string };

```

A flattened representation of the Tokens type, where keys are kebab-cased paths and all values are strings.

## 

[​](#type-parameters)

Type Parameters

Type Parameter

`T` *extends* `Record`<`string`, `unknown`\>

## 

[​](#example)

Example

```
const themeOverrides: Partial<Flattened<typeof tokens>> = {
  'colors-brand-primary': string;
  'fontFamily-sans': string;
}

```