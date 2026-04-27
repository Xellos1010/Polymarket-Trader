# tokenvalue

```
type TokenValue = {
  value: string | number;
  unit?: "px" | "none";
  modify?:   | {
     type: "color-alpha";
     value: number | string;
   }
     | {
     type: "color-hsl";
     value: [number, number, number];
   }
     | {
     type: "color-mix";
     value: ReadonlyArray<string | readonly [string, string]>;
   }
     | {
     type: "multiply";
     value: number | string;
   };
};

```

Represents a single theme value, which can be a direct value or a reference with modifications.

## Properties

Property

Type

Description

`value`

`string` | `number`

The value of the token.

`unit?`

`"px"` | `"none"`

Unit for numeric values. Defaults to “px” if not specified. Use “none” for unitless values like z-index.

`modify?`

| { `type`: `"color-alpha"`; `value`: `number` | `string`; } | { `type`: `"color-hsl"`; `value`: \[`number`, `number`, `number`\]; } | { `type`: `"color-mix"`; `value`: `ReadonlyArray`<`string` | readonly \[`string`, `string`\]>; } | { `type`: `"multiply"`; `value`: `number` | `string`; }

Modifications to the value.