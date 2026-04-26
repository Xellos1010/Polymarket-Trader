# fontsemantic

```
const fontSemantic: {
  family: {
     mono: {
        value: "\"DM Mono\", monospace";
     };
     sans: {
        value: "\"Rethink Sans\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"";
     };
     body: {
        value: "{font.family.sans}";
     };
     interactive: {
        value: "{font.family.sans}";
     };
  };
  size: {
     base: {
        value: 16;
     };
  };
};

```

Semantic font tokens.

## Type declaration

### family

```
readonly family: {
  mono: {
     value: "\"DM Mono\", monospace";
  };
  sans: {
     value: "\"Rethink Sans\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"";
  };
  body: {
     value: "{font.family.sans}";
  };
  interactive: {
     value: "{font.family.sans}";
  };
};

```

#### Type declaration

#### family.mono

```
readonly mono: {
  value: "\"DM Mono\", monospace";
};

```

##### Type declaration

#### family.mono.value

```
readonly value: "\"DM Mono\", monospace" = '"DM Mono", monospace';

```

#### family.sans

```
readonly sans: {
  value: "\"Rethink Sans\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"";
};

```

##### Type declaration

#### family.sans.value

```
readonly value: "\"Rethink Sans\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"" = '"Rethink Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol"';

```

#### family.body

```
readonly body: {
  value: "{font.family.sans}";
};

```

##### Type declaration

#### family.body.value

```
readonly value: "{font.family.sans}" = "{font.family.sans}";

```

#### family.interactive

```
readonly interactive: {
  value: "{font.family.sans}";
};

```

##### Type declaration

#### family.interactive.value

```
readonly value: "{font.family.sans}" = "{font.family.sans}";

```

### size

```
readonly size: {
  base: {
     value: 16;
  };
};

```

#### Type declaration

#### size.base

```
readonly base: {
  value: 16;
};

```

##### Type declaration

#### size.base.value