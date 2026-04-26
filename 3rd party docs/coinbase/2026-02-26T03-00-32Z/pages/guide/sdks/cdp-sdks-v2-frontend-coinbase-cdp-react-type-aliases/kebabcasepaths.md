# kebabcasepaths

```
type KebabCasePaths<T> = T extends Record<string, unknown> ? { [K in keyof T]: T[K] extends { value: unknown } ? K & string : T[K] extends Record<string, unknown> ? `${K & string}-${KebabCasePaths<T[K]> & string}` : K & string }[keyof T] : never;

```

A type that recursively converts a nested object to a flattened object with kebab-case keys.

## 

[​](#type-parameters)

Type Parameters

Type Parameter

`T`

## 

[​](#example)

Example

```
type MyObject = {
  a: {
    b: {
      cKey: string;
    };
  };
};
type Flattened = Flattened<MyObject>;
// { 'a-b-cKey': string }

```