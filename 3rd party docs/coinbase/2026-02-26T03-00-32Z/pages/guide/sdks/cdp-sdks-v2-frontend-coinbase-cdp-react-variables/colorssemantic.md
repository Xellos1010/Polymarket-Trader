# colorssemantic

```
const colorsSemantic: {
  bg: {
     default: {
        value: "#ffffff";
     };
     alternate: {
        value: "#eef0f3";
     };
     contrast: {
        value: "{colors.fg.default}";
     };
     overlay: {
        value: "{colors.bg.alternate}";
        modify: {
           type: "color-alpha";
           value: 0.33;
        };
     };
     skeleton: {
        value: "{colors.fg.default}";
        modify: {
           type: "color-alpha";
           value: 0.1;
        };
     };
     primary: {
        value: "#0052ff";
     };
     secondary: {
        value: "#eef0f3";
     };
     primaryWash: {
        value: "{colors.fg.primary}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.default}", "92%"]];
        };
     };
     positiveWash: {
        value: "{colors.fg.positive}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.default}", "92%"]];
        };
     };
     negativeWash: {
        value: "{colors.fg.negative}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.default}", "92%"]];
        };
     };
     warningWash: {
        value: "{colors.fg.warning}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.default}", "92%"]];
        };
     };
  };
  fg: {
     default: {
        value: "#0a0b0d";
     };
     muted: {
        value: "#5b616e";
     };
     primary: {
        value: "#0052ff";
     };
     onPrimary: {
        value: "#ffffff";
     };
     onSecondary: {
        value: "#0a0b0d";
     };
     positive: {
        value: "#098551";
     };
     negative: {
        value: "#cf202f";
     };
     warning: {
        value: "#ed702f";
     };
     onPrimaryWash: {
        value: "{colors.fg.onPrimary}";
     };
     onPositiveWash: {
        value: "{colors.fg.default}";
     };
     onNegativeWash: {
        value: "{colors.fg.default}";
     };
     onWarningWash: {
        value: "{colors.fg.default}";
     };
  };
  line: {
     default: {
        value: "#dcdfe4";
     };
     heavy: {
        value: "#9397a0";
     };
     primary: {
        value: "{colors.fg.primary}";
     };
     positive: {
        value: "{colors.fg.positive}";
     };
     negative: {
        value: "{colors.fg.negative}";
     };
  };
};

```

These are the default values for the semantic color tokens. Semantic colors are the base colors for the theme. They are typically not used directly in the components, but are used to define the base colors for the components.

## Type declaration

### bg

```
readonly bg: {
  default: {
     value: "#ffffff";
  };
  alternate: {
     value: "#eef0f3";
  };
  contrast: {
     value: "{colors.fg.default}";
  };
  overlay: {
     value: "{colors.bg.alternate}";
     modify: {
        type: "color-alpha";
        value: 0.33;
     };
  };
  skeleton: {
     value: "{colors.fg.default}";
     modify: {
        type: "color-alpha";
        value: 0.1;
     };
  };
  primary: {
     value: "#0052ff";
  };
  secondary: {
     value: "#eef0f3";
  };
  primaryWash: {
     value: "{colors.fg.primary}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.default}", "92%"]];
     };
  };
  positiveWash: {
     value: "{colors.fg.positive}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.default}", "92%"]];
     };
  };
  negativeWash: {
     value: "{colors.fg.negative}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.default}", "92%"]];
     };
  };
  warningWash: {
     value: "{colors.fg.warning}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.default}", "92%"]];
     };
  };
};

```

#### Type declaration

#### bg.default

```
readonly default: {
  value: "#ffffff";
};

```

##### Type declaration

#### bg.default.value

```
readonly value: "#ffffff" = colorsBase.white;

```

#### bg.alternate

```
readonly alternate: {
  value: "#eef0f3";
};

```

##### Type declaration

#### bg.alternate.value

```
readonly value: "#eef0f3" = colorsBase.gray100;

```

#### bg.contrast

```
readonly contrast: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### bg.contrast.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### bg.overlay

```
readonly overlay: {
  value: "{colors.bg.alternate}";
  modify: {
     type: "color-alpha";
     value: 0.33;
  };
};

```

##### Type declaration

#### bg.overlay.value

```
readonly value: "{colors.bg.alternate}" = "{colors.bg.alternate}";

```

#### bg.overlay.modify

```
readonly modify: {
  type: "color-alpha";
  value: 0.33;
};

```

##### Type declaration

#### bg.overlay.modify.type

```
readonly type: "color-alpha" = "color-alpha";

```

#### bg.overlay.modify.value

```
readonly value: 0.33 = 0.33;

```

#### bg.skeleton

```
readonly skeleton: {
  value: "{colors.fg.default}";
  modify: {
     type: "color-alpha";
     value: 0.1;
  };
};

```

##### Type declaration

#### bg.skeleton.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### bg.skeleton.modify

```
readonly modify: {
  type: "color-alpha";
  value: 0.1;
};

```

##### Type declaration

#### bg.skeleton.modify.type

```
readonly type: "color-alpha" = "color-alpha";

```

#### bg.skeleton.modify.value

```
readonly value: 0.1 = 0.1;

```

#### bg.primary

```
readonly primary: {
  value: "#0052ff";
};

```

##### Type declaration

#### bg.primary.value

```
readonly value: "#0052ff" = colorsBase.blue500;

```

#### bg.secondary

```
readonly secondary: {
  value: "#eef0f3";
};

```

##### Type declaration

#### bg.secondary.value

```
readonly value: "#eef0f3" = colorsBase.gray100;

```

#### bg.primaryWash

```
readonly primaryWash: {
  value: "{colors.fg.primary}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.default}", "92%"]];
  };
};

```

##### Type declaration

#### bg.primaryWash.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### bg.primaryWash.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.default}", "92%"]];
};

```

##### Type declaration

#### bg.primaryWash.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### bg.primaryWash.modify.value

```
readonly value: readonly [readonly ["{colors.bg.default}", "92%"]];

```

#### bg.positiveWash

```
readonly positiveWash: {
  value: "{colors.fg.positive}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.default}", "92%"]];
  };
};

```

##### Type declaration

#### bg.positiveWash.value

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

#### bg.positiveWash.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.default}", "92%"]];
};

```

##### Type declaration

#### bg.positiveWash.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### bg.positiveWash.modify.value

```
readonly value: readonly [readonly ["{colors.bg.default}", "92%"]];

```

#### bg.negativeWash

```
readonly negativeWash: {
  value: "{colors.fg.negative}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.default}", "92%"]];
  };
};

```

##### Type declaration

#### bg.negativeWash.value

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

#### bg.negativeWash.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.default}", "92%"]];
};

```

##### Type declaration

#### bg.negativeWash.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### bg.negativeWash.modify.value

```
readonly value: readonly [readonly ["{colors.bg.default}", "92%"]];

```

#### bg.warningWash

```
readonly warningWash: {
  value: "{colors.fg.warning}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.default}", "92%"]];
  };
};

```

##### Type declaration

#### bg.warningWash.value

```
readonly value: "{colors.fg.warning}" = "{colors.fg.warning}";

```

#### bg.warningWash.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.default}", "92%"]];
};

```

##### Type declaration

#### bg.warningWash.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### bg.warningWash.modify.value

```
readonly value: readonly [readonly ["{colors.bg.default}", "92%"]];

```

### fg

```
readonly fg: {
  default: {
     value: "#0a0b0d";
  };
  muted: {
     value: "#5b616e";
  };
  primary: {
     value: "#0052ff";
  };
  onPrimary: {
     value: "#ffffff";
  };
  onSecondary: {
     value: "#0a0b0d";
  };
  positive: {
     value: "#098551";
  };
  negative: {
     value: "#cf202f";
  };
  warning: {
     value: "#ed702f";
  };
  onPrimaryWash: {
     value: "{colors.fg.onPrimary}";
  };
  onPositiveWash: {
     value: "{colors.fg.default}";
  };
  onNegativeWash: {
     value: "{colors.fg.default}";
  };
  onWarningWash: {
     value: "{colors.fg.default}";
  };
};

```

#### Type declaration

#### fg.default

```
readonly default: {
  value: "#0a0b0d";
};

```

##### Type declaration

#### fg.default.value

```
readonly value: "#0a0b0d" = colorsBase.black;

```

#### fg.muted

```
readonly muted: {
  value: "#5b616e";
};

```

##### Type declaration

#### fg.muted.value

```
readonly value: "#5b616e" = colorsBase.gray700;

```

#### fg.primary

```
readonly primary: {
  value: "#0052ff";
};

```

##### Type declaration

#### fg.primary.value

```
readonly value: "#0052ff" = colorsBase.blue500;

```

#### fg.onPrimary

```
readonly onPrimary: {
  value: "#ffffff";
};

```

##### Type declaration

#### fg.onPrimary.value

```
readonly value: "#ffffff" = colorsBase.white;

```

#### fg.onSecondary

```
readonly onSecondary: {
  value: "#0a0b0d";
};

```

##### Type declaration

#### fg.onSecondary.value

```
readonly value: "#0a0b0d" = colorsBase.black;

```

#### fg.positive

```
readonly positive: {
  value: "#098551";
};

```

##### Type declaration

#### fg.positive.value

```
readonly value: "#098551" = colorsBase.green500;

```

#### fg.negative

```
readonly negative: {
  value: "#cf202f";
};

```

##### Type declaration

#### fg.negative.value

```
readonly value: "#cf202f" = colorsBase.red500;

```

#### fg.warning

```
readonly warning: {
  value: "#ed702f";
};

```

##### Type declaration

#### fg.warning.value

```
readonly value: "#ed702f" = colorsBase.amber500;

```

#### fg.onPrimaryWash

```
readonly onPrimaryWash: {
  value: "{colors.fg.onPrimary}";
};

```

##### Type declaration

#### fg.onPrimaryWash.value

```
readonly value: "{colors.fg.onPrimary}" = "{colors.fg.onPrimary}";

```

#### fg.onPositiveWash

```
readonly onPositiveWash: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### fg.onPositiveWash.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### fg.onNegativeWash

```
readonly onNegativeWash: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### fg.onNegativeWash.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### fg.onWarningWash

```
readonly onWarningWash: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### fg.onWarningWash.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

### line

```
readonly line: {
  default: {
     value: "#dcdfe4";
  };
  heavy: {
     value: "#9397a0";
  };
  primary: {
     value: "{colors.fg.primary}";
  };
  positive: {
     value: "{colors.fg.positive}";
  };
  negative: {
     value: "{colors.fg.negative}";
  };
};

```

#### Type declaration

#### line.default

```
readonly default: {
  value: "#dcdfe4";
};

```

##### Type declaration

#### line.default.value

```
readonly value: "#dcdfe4" = colorsBase.gray200;

```

#### line.heavy

```
readonly heavy: {
  value: "#9397a0";
};

```

##### Type declaration

#### line.heavy.value

```
readonly value: "#9397a0" = colorsBase.gray500;

```

#### line.primary

```
readonly primary: {
  value: "{colors.fg.primary}";
};

```

##### Type declaration

#### line.primary.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### line.positive

```
readonly positive: {
  value: "{colors.fg.positive}";
};

```

##### Type declaration

#### line.positive.value

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

#### line.negative

```
readonly negative: {
  value: "{colors.fg.negative}";
};

```

##### Type declaration

#### line.negative.value

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

## See

[colorsComponents](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Variables/colorsComponents) for the component colors that inherit from these