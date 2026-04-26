# colors

```
const colors: {
  page: {
     bg: {
        default: {
           value: "{colors.bg.default}";
        };
     };
     border: {
        default: {
           value: "{colors.line.default}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.default}";
        };
        muted: {
           value: "{colors.fg.muted}";
        };
     };
  };
  cta: {
     primary: {
        bg: {
           default: {
              value: "{colors.bg.primary}";
           };
           hover: {
              value: "{colors.bg.primary}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
              };
           };
           pressed: {
              value: "{colors.bg.primary}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
              };
           };
        };
        border: {
           focus: {
              value: "{colors.line.primary}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.onPrimary}";
           };
           hover: {
              value: "{colors.fg.onPrimary}";
           };
        };
     };
     secondary: {
        bg: {
           default: {
              value: "{colors.bg.secondary}";
           };
           hover: {
              value: "{colors.bg.secondary}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
              };
           };
           pressed: {
              value: "{colors.bg.secondary}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
              };
           };
        };
        border: {
           focus: {
              value: "{colors.line.primary}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.onSecondary}";
           };
           hover: {
              value: "{colors.fg.onSecondary}";
           };
        };
     };
  };
  link: {
     primary: {
        text: {
           default: {
              value: "{colors.fg.primary}";
           };
           hover: {
              value: "{colors.fg.primary}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
              };
           };
           pressed: {
              value: "{colors.fg.primary}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
              };
           };
        };
     };
     secondary: {
        text: {
           default: {
              value: "{colors.fg.default}";
           };
           hover: {
              value: "{colors.fg.default}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
              };
           };
           pressed: {
              value: "{colors.fg.default}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
              };
           };
        };
     };
  };
  input: {
     bg: {
        default: {
           value: "{colors.bg.default}";
        };
        readonly: {
           value: "{colors.bg.alternate}";
        };
     };
     border: {
        default: {
           value: "{colors.line.heavy}";
        };
        focus: {
           value: "{colors.line.primary}";
        };
        error: {
           value: "{colors.line.negative}";
        };
        success: {
           value: "{colors.line.positive}";
        };
     };
     label: {
        default: {
           value: "{colors.fg.default}";
        };
     };
     placeholder: {
        default: {
           value: "{colors.fg.muted}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.default}";
        };
        readonly: {
           value: "{colors.fg.muted}";
        };
     };
     errorText: {
        default: {
           value: "{colors.fg.negative}";
        };
     };
     successText: {
        default: {
           value: "{colors.fg.positive}";
        };
     };
  };
  select: {
     label: {
        default: {
           value: "{colors.fg.default}";
        };
     };
     trigger: {
        bg: {
           default: {
              value: "{colors.bg.default}";
           };
           hover: {
              value: "{colors.bg.default}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "5%"]];
              };
           };
           pressed: {
              value: "{colors.bg.default}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "7%"]];
              };
           };
        };
        border: {
           default: {
              value: "{colors.line.default}";
           };
           focus: {
              value: "{colors.line.primary}";
           };
           error: {
              value: "{colors.line.negative}";
           };
           success: {
              value: "{colors.line.positive}";
           };
        };
        placeholder: {
           default: {
              value: "{colors.fg.muted}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.default}";
           };
        };
        errorText: {
           default: {
              value: "{colors.fg.negative}";
           };
        };
        successText: {
           default: {
              value: "{colors.fg.positive}";
           };
        };
     };
     list: {
        bg: {
           default: {
              value: "{colors.bg.default}";
           };
        };
        border: {
           default: {
              value: "{colors.line.default}";
           };
           focus: {
              value: "{colors.line.primary}";
           };
           error: {
              value: "{colors.line.negative}";
           };
           success: {
              value: "{colors.line.positive}";
           };
        };
        item: {
           bg: {
              default: {
                 value: "{colors.bg.default}";
              };
              highlight: {
                 value: "{colors.bg.default}";
                 modify: {
                    type: "color-mix";
                    value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
                 };
              };
           };
           text: {
              default: {
                 value: "{colors.fg.default}";
              };
              muted: {
                 value: "{colors.fg.muted}";
              };
              onHighlight: {
                 value: "{colors.fg.default}";
              };
              mutedOnHighlight: {
                 value: "{colors.fg.muted}";
              };
           };
        };
     };
  };
  code: {
     bg: {
        default: {
           value: "{colors.bg.alternate}";
        };
     };
     border: {
        default: {
           value: "{colors.line.heavy}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.default}";
        };
     };
  };
  badge: {
     primary: {
        bg: {
           default: {
              value: "{colors.bg.primaryWash}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.primary}";
           };
        };
     };
     secondary: {
        bg: {
           default: {
              value: "{colors.bg.secondary}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.onSecondary}";
           };
        };
     };
     warning: {
        bg: {
           default: {
              value: "{colors.bg.warningWash}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.warning}";
           };
        };
     };
  };
  banner: {
     error: {
        bg: {
           default: {
              value: "{colors.bg.negativeWash}";
           };
        };
        icon: {
           default: {
              value: "{colors.fg.negative}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.onNegativeWash}";
           };
        };
     };
     success: {
        bg: {
           default: {
              value: "{colors.bg.positiveWash}";
           };
        };
        icon: {
           default: {
              value: "{colors.fg.positive}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.onPositiveWash}";
           };
        };
     };
     warning: {
        bg: {
           default: {
              value: "{colors.bg.warningWash}";
           };
        };
        icon: {
           default: {
              value: "{colors.fg.warning}";
           };
        };
        text: {
           default: {
              value: "{colors.fg.onWarningWash}";
           };
        };
     };
  };
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

All the color tokens.

## Type declaration

### page

```
readonly page: {
  bg: {
     default: {
        value: "{colors.bg.default}";
     };
  };
  border: {
     default: {
        value: "{colors.line.default}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.default}";
     };
     muted: {
        value: "{colors.fg.muted}";
     };
  };
};

```

#### Type declaration

#### page.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.default}";
  };
};

```

##### Type declaration

#### page.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### page.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### page.border

```
readonly border: {
  default: {
     value: "{colors.line.default}";
  };
};

```

##### Type declaration

#### page.border.default

```
readonly default: {
  value: "{colors.line.default}";
};

```

##### Type declaration

#### page.border.default.value

```
readonly value: "{colors.line.default}" = "{colors.line.default}";

```

#### page.text

```
readonly text: {
  default: {
     value: "{colors.fg.default}";
  };
  muted: {
     value: "{colors.fg.muted}";
  };
};

```

##### Type declaration

#### page.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### page.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### page.text.muted

```
readonly muted: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### page.text.muted.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

### cta

```
readonly cta: {
  primary: {
     bg: {
        default: {
           value: "{colors.bg.primary}";
        };
        hover: {
           value: "{colors.bg.primary}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
           };
        };
        pressed: {
           value: "{colors.bg.primary}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
           };
        };
     };
     border: {
        focus: {
           value: "{colors.line.primary}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.onPrimary}";
        };
        hover: {
           value: "{colors.fg.onPrimary}";
        };
     };
  };
  secondary: {
     bg: {
        default: {
           value: "{colors.bg.secondary}";
        };
        hover: {
           value: "{colors.bg.secondary}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
           };
        };
        pressed: {
           value: "{colors.bg.secondary}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
           };
        };
     };
     border: {
        focus: {
           value: "{colors.line.primary}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.onSecondary}";
        };
        hover: {
           value: "{colors.fg.onSecondary}";
        };
     };
  };
};

```

#### Type declaration

#### cta.primary

```
readonly primary: {
  bg: {
     default: {
        value: "{colors.bg.primary}";
     };
     hover: {
        value: "{colors.bg.primary}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
        };
     };
     pressed: {
        value: "{colors.bg.primary}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
        };
     };
  };
  border: {
     focus: {
        value: "{colors.line.primary}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.onPrimary}";
     };
     hover: {
        value: "{colors.fg.onPrimary}";
     };
  };
};

```

##### Type declaration

#### cta.primary.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.primary}";
  };
  hover: {
     value: "{colors.bg.primary}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
     };
  };
  pressed: {
     value: "{colors.bg.primary}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
     };
  };
};

```

##### Type declaration

#### cta.primary.bg.default

```
readonly default: {
  value: "{colors.bg.primary}";
};

```

##### Type declaration

#### cta.primary.bg.default.value

```
readonly value: "{colors.bg.primary}" = "{colors.bg.primary}";

```

#### cta.primary.bg.hover

```
readonly hover: {
  value: "{colors.bg.primary}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
  };
};

```

##### Type declaration

#### cta.primary.bg.hover.value

```
readonly value: "{colors.bg.primary}" = "{colors.bg.primary}";

```

#### cta.primary.bg.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
};

```

##### Type declaration

#### cta.primary.bg.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### cta.primary.bg.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "15%"]];

```

#### cta.primary.bg.pressed

```
readonly pressed: {
  value: "{colors.bg.primary}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
  };
};

```

##### Type declaration

#### cta.primary.bg.pressed.value

```
readonly value: "{colors.bg.primary}" = "{colors.bg.primary}";

```

#### cta.primary.bg.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
};

```

##### Type declaration

#### cta.primary.bg.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### cta.primary.bg.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "30%"]];

```

#### cta.primary.border

```
readonly border: {
  focus: {
     value: "{colors.line.primary}";
  };
};

```

##### Type declaration

#### cta.primary.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### cta.primary.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### cta.primary.text

```
readonly text: {
  default: {
     value: "{colors.fg.onPrimary}";
  };
  hover: {
     value: "{colors.fg.onPrimary}";
  };
};

```

##### Type declaration

#### cta.primary.text.default

```
readonly default: {
  value: "{colors.fg.onPrimary}";
};

```

##### Type declaration

#### cta.primary.text.default.value

```
readonly value: "{colors.fg.onPrimary}" = "{colors.fg.onPrimary}";

```

#### cta.primary.text.hover

```
readonly hover: {
  value: "{colors.fg.onPrimary}";
};

```

##### Type declaration

#### cta.primary.text.hover.value

```
readonly value: "{colors.fg.onPrimary}" = "{colors.fg.onPrimary}";

```

#### cta.secondary

```
readonly secondary: {
  bg: {
     default: {
        value: "{colors.bg.secondary}";
     };
     hover: {
        value: "{colors.bg.secondary}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
        };
     };
     pressed: {
        value: "{colors.bg.secondary}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
        };
     };
  };
  border: {
     focus: {
        value: "{colors.line.primary}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.onSecondary}";
     };
     hover: {
        value: "{colors.fg.onSecondary}";
     };
  };
};

```

##### Type declaration

#### cta.secondary.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.secondary}";
  };
  hover: {
     value: "{colors.bg.secondary}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
     };
  };
  pressed: {
     value: "{colors.bg.secondary}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
     };
  };
};

```

##### Type declaration

#### cta.secondary.bg.default

```
readonly default: {
  value: "{colors.bg.secondary}";
};

```

##### Type declaration

#### cta.secondary.bg.default.value

```
readonly value: "{colors.bg.secondary}" = "{colors.bg.secondary}";

```

#### cta.secondary.bg.hover

```
readonly hover: {
  value: "{colors.bg.secondary}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
  };
};

```

##### Type declaration

#### cta.secondary.bg.hover.value

```
readonly value: "{colors.bg.secondary}" = "{colors.bg.secondary}";

```

#### cta.secondary.bg.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
};

```

##### Type declaration

#### cta.secondary.bg.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### cta.secondary.bg.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "10%"]];

```

#### cta.secondary.bg.pressed

```
readonly pressed: {
  value: "{colors.bg.secondary}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
  };
};

```

##### Type declaration

#### cta.secondary.bg.pressed.value

```
readonly value: "{colors.bg.secondary}" = "{colors.bg.secondary}";

```

#### cta.secondary.bg.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
};

```

##### Type declaration

#### cta.secondary.bg.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### cta.secondary.bg.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "20%"]];

```

#### cta.secondary.border

```
readonly border: {
  focus: {
     value: "{colors.line.primary}";
  };
};

```

##### Type declaration

#### cta.secondary.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### cta.secondary.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### cta.secondary.text

```
readonly text: {
  default: {
     value: "{colors.fg.onSecondary}";
  };
  hover: {
     value: "{colors.fg.onSecondary}";
  };
};

```

##### Type declaration

#### cta.secondary.text.default

```
readonly default: {
  value: "{colors.fg.onSecondary}";
};

```

##### Type declaration

#### cta.secondary.text.default.value

```
readonly value: "{colors.fg.onSecondary}" = "{colors.fg.onSecondary}";

```

#### cta.secondary.text.hover

```
readonly hover: {
  value: "{colors.fg.onSecondary}";
};

```

##### Type declaration

#### cta.secondary.text.hover.value

```
readonly value: "{colors.fg.onSecondary}" = "{colors.fg.onSecondary}";

```

### link

```
readonly link: {
  primary: {
     text: {
        default: {
           value: "{colors.fg.primary}";
        };
        hover: {
           value: "{colors.fg.primary}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
           };
        };
        pressed: {
           value: "{colors.fg.primary}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
           };
        };
     };
  };
  secondary: {
     text: {
        default: {
           value: "{colors.fg.default}";
        };
        hover: {
           value: "{colors.fg.default}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
           };
        };
        pressed: {
           value: "{colors.fg.default}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
           };
        };
     };
  };
};

```

#### Type declaration

#### link.primary

```
readonly primary: {
  text: {
     default: {
        value: "{colors.fg.primary}";
     };
     hover: {
        value: "{colors.fg.primary}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
        };
     };
     pressed: {
        value: "{colors.fg.primary}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
        };
     };
  };
};

```

##### Type declaration

#### link.primary.text

```
readonly text: {
  default: {
     value: "{colors.fg.primary}";
  };
  hover: {
     value: "{colors.fg.primary}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
     };
  };
  pressed: {
     value: "{colors.fg.primary}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
     };
  };
};

```

##### Type declaration

#### link.primary.text.default

```
readonly default: {
  value: "{colors.fg.primary}";
};

```

##### Type declaration

#### link.primary.text.default.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### link.primary.text.hover

```
readonly hover: {
  value: "{colors.fg.primary}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
  };
};

```

##### Type declaration

#### link.primary.text.hover.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### link.primary.text.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
};

```

##### Type declaration

#### link.primary.text.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### link.primary.text.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "15%"]];

```

#### link.primary.text.pressed

```
readonly pressed: {
  value: "{colors.fg.primary}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
  };
};

```

##### Type declaration

#### link.primary.text.pressed.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### link.primary.text.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
};

```

##### Type declaration

#### link.primary.text.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### link.primary.text.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "30%"]];

```

#### link.secondary

```
readonly secondary: {
  text: {
     default: {
        value: "{colors.fg.default}";
     };
     hover: {
        value: "{colors.fg.default}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
        };
     };
     pressed: {
        value: "{colors.fg.default}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
        };
     };
  };
};

```

##### Type declaration

#### link.secondary.text

```
readonly text: {
  default: {
     value: "{colors.fg.default}";
  };
  hover: {
     value: "{colors.fg.default}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
     };
  };
  pressed: {
     value: "{colors.fg.default}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
     };
  };
};

```

##### Type declaration

#### link.secondary.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### link.secondary.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### link.secondary.text.hover

```
readonly hover: {
  value: "{colors.fg.default}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
  };
};

```

##### Type declaration

#### link.secondary.text.hover.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### link.secondary.text.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
};

```

##### Type declaration

#### link.secondary.text.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### link.secondary.text.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "10%"]];

```

#### link.secondary.text.pressed

```
readonly pressed: {
  value: "{colors.fg.default}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
  };
};

```

##### Type declaration

#### link.secondary.text.pressed.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### link.secondary.text.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
};

```

##### Type declaration

#### link.secondary.text.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### link.secondary.text.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "20%"]];

```

### input

```
readonly input: {
  bg: {
     default: {
        value: "{colors.bg.default}";
     };
     readonly: {
        value: "{colors.bg.alternate}";
     };
  };
  border: {
     default: {
        value: "{colors.line.heavy}";
     };
     focus: {
        value: "{colors.line.primary}";
     };
     error: {
        value: "{colors.line.negative}";
     };
     success: {
        value: "{colors.line.positive}";
     };
  };
  label: {
     default: {
        value: "{colors.fg.default}";
     };
  };
  placeholder: {
     default: {
        value: "{colors.fg.muted}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.default}";
     };
     readonly: {
        value: "{colors.fg.muted}";
     };
  };
  errorText: {
     default: {
        value: "{colors.fg.negative}";
     };
  };
  successText: {
     default: {
        value: "{colors.fg.positive}";
     };
  };
};

```

#### Type declaration

#### input.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.default}";
  };
  readonly: {
     value: "{colors.bg.alternate}";
  };
};

```

##### Type declaration

#### input.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### input.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### input.bg.readonly

```
readonly readonly: {
  value: "{colors.bg.alternate}";
};

```

##### Type declaration

#### input.bg.readonly.value

```
readonly value: "{colors.bg.alternate}" = "{colors.bg.alternate}";

```

#### input.border

```
readonly border: {
  default: {
     value: "{colors.line.heavy}";
  };
  focus: {
     value: "{colors.line.primary}";
  };
  error: {
     value: "{colors.line.negative}";
  };
  success: {
     value: "{colors.line.positive}";
  };
};

```

##### Type declaration

#### input.border.default

```
readonly default: {
  value: "{colors.line.heavy}";
};

```

##### Type declaration

#### input.border.default.value

```
readonly value: "{colors.line.heavy}" = "{colors.line.heavy}";

```

#### input.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### input.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### input.border.error

```
readonly error: {
  value: "{colors.line.negative}";
};

```

##### Type declaration

#### input.border.error.value

```
readonly value: "{colors.line.negative}" = "{colors.line.negative}";

```

#### input.border.success

```
readonly success: {
  value: "{colors.line.positive}";
};

```

##### Type declaration

#### input.border.success.value

```
readonly value: "{colors.line.positive}" = "{colors.line.positive}";

```

#### input.label

```
readonly label: {
  default: {
     value: "{colors.fg.default}";
  };
};

```

##### Type declaration

#### input.label.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### input.label.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### input.placeholder

```
readonly placeholder: {
  default: {
     value: "{colors.fg.muted}";
  };
};

```

##### Type declaration

#### input.placeholder.default

```
readonly default: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### input.placeholder.default.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### input.text

```
readonly text: {
  default: {
     value: "{colors.fg.default}";
  };
  readonly: {
     value: "{colors.fg.muted}";
  };
};

```

##### Type declaration

#### input.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### input.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### input.text.readonly

```
readonly readonly: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### input.text.readonly.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### input.errorText

```
readonly errorText: {
  default: {
     value: "{colors.fg.negative}";
  };
};

```

##### Type declaration

#### input.errorText.default

```
readonly default: {
  value: "{colors.fg.negative}";
};

```

##### Type declaration

#### input.errorText.default.value

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

#### input.successText

```
readonly successText: {
  default: {
     value: "{colors.fg.positive}";
  };
};

```

##### Type declaration

#### input.successText.default

```
readonly default: {
  value: "{colors.fg.positive}";
};

```

##### Type declaration

#### input.successText.default.value

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

### select

```
readonly select: {
  label: {
     default: {
        value: "{colors.fg.default}";
     };
  };
  trigger: {
     bg: {
        default: {
           value: "{colors.bg.default}";
        };
        hover: {
           value: "{colors.bg.default}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "5%"]];
           };
        };
        pressed: {
           value: "{colors.bg.default}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "7%"]];
           };
        };
     };
     border: {
        default: {
           value: "{colors.line.default}";
        };
        focus: {
           value: "{colors.line.primary}";
        };
        error: {
           value: "{colors.line.negative}";
        };
        success: {
           value: "{colors.line.positive}";
        };
     };
     placeholder: {
        default: {
           value: "{colors.fg.muted}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.default}";
        };
     };
     errorText: {
        default: {
           value: "{colors.fg.negative}";
        };
     };
     successText: {
        default: {
           value: "{colors.fg.positive}";
        };
     };
  };
  list: {
     bg: {
        default: {
           value: "{colors.bg.default}";
        };
     };
     border: {
        default: {
           value: "{colors.line.default}";
        };
        focus: {
           value: "{colors.line.primary}";
        };
        error: {
           value: "{colors.line.negative}";
        };
        success: {
           value: "{colors.line.positive}";
        };
     };
     item: {
        bg: {
           default: {
              value: "{colors.bg.default}";
           };
           highlight: {
              value: "{colors.bg.default}";
              modify: {
                 type: "color-mix";
                 value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
              };
           };
        };
        text: {
           default: {
              value: "{colors.fg.default}";
           };
           muted: {
              value: "{colors.fg.muted}";
           };
           onHighlight: {
              value: "{colors.fg.default}";
           };
           mutedOnHighlight: {
              value: "{colors.fg.muted}";
           };
        };
     };
  };
};

```

#### Type declaration

#### select.label

```
readonly label: {
  default: {
     value: "{colors.fg.default}";
  };
};

```

##### Type declaration

#### select.label.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### select.label.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### select.trigger

```
readonly trigger: {
  bg: {
     default: {
        value: "{colors.bg.default}";
     };
     hover: {
        value: "{colors.bg.default}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "5%"]];
        };
     };
     pressed: {
        value: "{colors.bg.default}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "7%"]];
        };
     };
  };
  border: {
     default: {
        value: "{colors.line.default}";
     };
     focus: {
        value: "{colors.line.primary}";
     };
     error: {
        value: "{colors.line.negative}";
     };
     success: {
        value: "{colors.line.positive}";
     };
  };
  placeholder: {
     default: {
        value: "{colors.fg.muted}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.default}";
     };
  };
  errorText: {
     default: {
        value: "{colors.fg.negative}";
     };
  };
  successText: {
     default: {
        value: "{colors.fg.positive}";
     };
  };
};

```

##### Type declaration

#### select.trigger.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.default}";
  };
  hover: {
     value: "{colors.bg.default}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "5%"]];
     };
  };
  pressed: {
     value: "{colors.bg.default}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "7%"]];
     };
  };
};

```

##### Type declaration

#### select.trigger.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### select.trigger.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### select.trigger.bg.hover

```
readonly hover: {
  value: "{colors.bg.default}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "5%"]];
  };
};

```

##### Type declaration

#### select.trigger.bg.hover.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### select.trigger.bg.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "5%"]];
};

```

##### Type declaration

#### select.trigger.bg.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### select.trigger.bg.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "5%"]];

```

#### select.trigger.bg.pressed

```
readonly pressed: {
  value: "{colors.bg.default}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "7%"]];
  };
};

```

##### Type declaration

#### select.trigger.bg.pressed.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### select.trigger.bg.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "7%"]];
};

```

##### Type declaration

#### select.trigger.bg.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### select.trigger.bg.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "7%"]];

```

#### select.trigger.border

```
readonly border: {
  default: {
     value: "{colors.line.default}";
  };
  focus: {
     value: "{colors.line.primary}";
  };
  error: {
     value: "{colors.line.negative}";
  };
  success: {
     value: "{colors.line.positive}";
  };
};

```

##### Type declaration

#### select.trigger.border.default

```
readonly default: {
  value: "{colors.line.default}";
};

```

##### Type declaration

#### select.trigger.border.default.value

```
readonly value: "{colors.line.default}" = "{colors.line.default}";

```

#### select.trigger.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### select.trigger.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### select.trigger.border.error

```
readonly error: {
  value: "{colors.line.negative}";
};

```

##### Type declaration

#### select.trigger.border.error.value

```
readonly value: "{colors.line.negative}" = "{colors.line.negative}";

```

#### select.trigger.border.success

```
readonly success: {
  value: "{colors.line.positive}";
};

```

##### Type declaration

#### select.trigger.border.success.value

```
readonly value: "{colors.line.positive}" = "{colors.line.positive}";

```

#### select.trigger.placeholder

```
readonly placeholder: {
  default: {
     value: "{colors.fg.muted}";
  };
};

```

##### Type declaration

#### select.trigger.placeholder.default

```
readonly default: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### select.trigger.placeholder.default.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### select.trigger.text

```
readonly text: {
  default: {
     value: "{colors.fg.default}";
  };
};

```

##### Type declaration

#### select.trigger.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### select.trigger.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### select.trigger.errorText

```
readonly errorText: {
  default: {
     value: "{colors.fg.negative}";
  };
};

```

##### Type declaration

#### select.trigger.errorText.default

```
readonly default: {
  value: "{colors.fg.negative}";
};

```

##### Type declaration

#### select.trigger.errorText.default.value

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

#### select.trigger.successText

```
readonly successText: {
  default: {
     value: "{colors.fg.positive}";
  };
};

```

##### Type declaration

#### select.trigger.successText.default

```
readonly default: {
  value: "{colors.fg.positive}";
};

```

##### Type declaration

#### select.trigger.successText.default.value

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

#### select.list

```
readonly list: {
  bg: {
     default: {
        value: "{colors.bg.default}";
     };
  };
  border: {
     default: {
        value: "{colors.line.default}";
     };
     focus: {
        value: "{colors.line.primary}";
     };
     error: {
        value: "{colors.line.negative}";
     };
     success: {
        value: "{colors.line.positive}";
     };
  };
  item: {
     bg: {
        default: {
           value: "{colors.bg.default}";
        };
        highlight: {
           value: "{colors.bg.default}";
           modify: {
              type: "color-mix";
              value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
           };
        };
     };
     text: {
        default: {
           value: "{colors.fg.default}";
        };
        muted: {
           value: "{colors.fg.muted}";
        };
        onHighlight: {
           value: "{colors.fg.default}";
        };
        mutedOnHighlight: {
           value: "{colors.fg.muted}";
        };
     };
  };
};

```

##### Type declaration

#### select.list.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.default}";
  };
};

```

##### Type declaration

#### select.list.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### select.list.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### select.list.border

```
readonly border: {
  default: {
     value: "{colors.line.default}";
  };
  focus: {
     value: "{colors.line.primary}";
  };
  error: {
     value: "{colors.line.negative}";
  };
  success: {
     value: "{colors.line.positive}";
  };
};

```

##### Type declaration

#### select.list.border.default

```
readonly default: {
  value: "{colors.line.default}";
};

```

##### Type declaration

#### select.list.border.default.value

```
readonly value: "{colors.line.default}" = "{colors.line.default}";

```

#### select.list.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### select.list.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### select.list.border.error

```
readonly error: {
  value: "{colors.line.negative}";
};

```

##### Type declaration

#### select.list.border.error.value

```
readonly value: "{colors.line.negative}" = "{colors.line.negative}";

```

#### select.list.border.success

```
readonly success: {
  value: "{colors.line.positive}";
};

```

##### Type declaration

#### select.list.border.success.value

```
readonly value: "{colors.line.positive}" = "{colors.line.positive}";

```

#### select.list.item

```
readonly item: {
  bg: {
     default: {
        value: "{colors.bg.default}";
     };
     highlight: {
        value: "{colors.bg.default}";
        modify: {
           type: "color-mix";
           value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
        };
     };
  };
  text: {
     default: {
        value: "{colors.fg.default}";
     };
     muted: {
        value: "{colors.fg.muted}";
     };
     onHighlight: {
        value: "{colors.fg.default}";
     };
     mutedOnHighlight: {
        value: "{colors.fg.muted}";
     };
  };
};

```

##### Type declaration

#### select.list.item.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.default}";
  };
  highlight: {
     value: "{colors.bg.default}";
     modify: {
        type: "color-mix";
        value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
     };
  };
};

```

##### Type declaration

#### select.list.item.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### select.list.item.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### select.list.item.bg.highlight

```
readonly highlight: {
  value: "{colors.bg.default}";
  modify: {
     type: "color-mix";
     value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
  };
};

```

##### Type declaration

#### select.list.item.bg.highlight.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### select.list.item.bg.highlight.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
};

```

##### Type declaration

#### select.list.item.bg.highlight.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### select.list.item.bg.highlight.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "10%"]];

```

#### select.list.item.text

```
readonly text: {
  default: {
     value: "{colors.fg.default}";
  };
  muted: {
     value: "{colors.fg.muted}";
  };
  onHighlight: {
     value: "{colors.fg.default}";
  };
  mutedOnHighlight: {
     value: "{colors.fg.muted}";
  };
};

```

##### Type declaration

#### select.list.item.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### select.list.item.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### select.list.item.text.muted

```
readonly muted: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### select.list.item.text.muted.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### select.list.item.text.onHighlight

```
readonly onHighlight: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### select.list.item.text.onHighlight.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### select.list.item.text.mutedOnHighlight

```
readonly mutedOnHighlight: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### select.list.item.text.mutedOnHighlight.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

### code

```
readonly code: {
  bg: {
     default: {
        value: "{colors.bg.alternate}";
     };
  };
  border: {
     default: {
        value: "{colors.line.heavy}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.default}";
     };
  };
};

```

#### Type declaration

#### code.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.alternate}";
  };
};

```

##### Type declaration

#### code.bg.default

```
readonly default: {
  value: "{colors.bg.alternate}";
};

```

##### Type declaration

#### code.bg.default.value

```
readonly value: "{colors.bg.alternate}" = "{colors.bg.alternate}";

```

#### code.border

```
readonly border: {
  default: {
     value: "{colors.line.heavy}";
  };
};

```

##### Type declaration

#### code.border.default

```
readonly default: {
  value: "{colors.line.heavy}";
};

```

##### Type declaration

#### code.border.default.value

```
readonly value: "{colors.line.heavy}" = "{colors.line.heavy}";

```

#### code.text

```
readonly text: {
  default: {
     value: "{colors.fg.default}";
  };
};

```

##### Type declaration

#### code.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### code.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

### badge

```
readonly badge: {
  primary: {
     bg: {
        default: {
           value: "{colors.bg.primaryWash}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.primary}";
        };
     };
  };
  secondary: {
     bg: {
        default: {
           value: "{colors.bg.secondary}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.onSecondary}";
        };
     };
  };
  warning: {
     bg: {
        default: {
           value: "{colors.bg.warningWash}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.warning}";
        };
     };
  };
};

```

#### Type declaration

#### badge.primary

```
readonly primary: {
  bg: {
     default: {
        value: "{colors.bg.primaryWash}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.primary}";
     };
  };
};

```

##### Type declaration

#### badge.primary.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.primaryWash}";
  };
};

```

##### Type declaration

#### badge.primary.bg.default

```
readonly default: {
  value: "{colors.bg.primaryWash}";
};

```

##### Type declaration

#### badge.primary.bg.default.value

```
readonly value: "{colors.bg.primaryWash}" = "{colors.bg.primaryWash}";

```

#### badge.primary.text

```
readonly text: {
  default: {
     value: "{colors.fg.primary}";
  };
};

```

##### Type declaration

#### badge.primary.text.default

```
readonly default: {
  value: "{colors.fg.primary}";
};

```

##### Type declaration

#### badge.primary.text.default.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### badge.secondary

```
readonly secondary: {
  bg: {
     default: {
        value: "{colors.bg.secondary}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.onSecondary}";
     };
  };
};

```

##### Type declaration

#### badge.secondary.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.secondary}";
  };
};

```

##### Type declaration

#### badge.secondary.bg.default

```
readonly default: {
  value: "{colors.bg.secondary}";
};

```

##### Type declaration

#### badge.secondary.bg.default.value

```
readonly value: "{colors.bg.secondary}" = "{colors.bg.secondary}";

```

#### badge.secondary.text

```
readonly text: {
  default: {
     value: "{colors.fg.onSecondary}";
  };
};

```

##### Type declaration

#### badge.secondary.text.default

```
readonly default: {
  value: "{colors.fg.onSecondary}";
};

```

##### Type declaration

#### badge.secondary.text.default.value

```
readonly value: "{colors.fg.onSecondary}" = "{colors.fg.onSecondary}";

```

#### badge.warning

```
readonly warning: {
  bg: {
     default: {
        value: "{colors.bg.warningWash}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.warning}";
     };
  };
};

```

##### Type declaration

#### badge.warning.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.warningWash}";
  };
};

```

##### Type declaration

#### badge.warning.bg.default

```
readonly default: {
  value: "{colors.bg.warningWash}";
};

```

##### Type declaration

#### badge.warning.bg.default.value

```
readonly value: "{colors.bg.warningWash}" = "{colors.bg.warningWash}";

```

#### badge.warning.text

```
readonly text: {
  default: {
     value: "{colors.fg.warning}";
  };
};

```

##### Type declaration

#### badge.warning.text.default

```
readonly default: {
  value: "{colors.fg.warning}";
};

```

##### Type declaration

#### badge.warning.text.default.value

```
readonly value: "{colors.fg.warning}" = "{colors.fg.warning}";

```

```
readonly banner: {
  error: {
     bg: {
        default: {
           value: "{colors.bg.negativeWash}";
        };
     };
     icon: {
        default: {
           value: "{colors.fg.negative}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.onNegativeWash}";
        };
     };
  };
  success: {
     bg: {
        default: {
           value: "{colors.bg.positiveWash}";
        };
     };
     icon: {
        default: {
           value: "{colors.fg.positive}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.onPositiveWash}";
        };
     };
  };
  warning: {
     bg: {
        default: {
           value: "{colors.bg.warningWash}";
        };
     };
     icon: {
        default: {
           value: "{colors.fg.warning}";
        };
     };
     text: {
        default: {
           value: "{colors.fg.onWarningWash}";
        };
     };
  };
};

```

#### Type declaration

```
readonly error: {
  bg: {
     default: {
        value: "{colors.bg.negativeWash}";
     };
  };
  icon: {
     default: {
        value: "{colors.fg.negative}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.onNegativeWash}";
     };
  };
};

```

##### Type declaration

```
readonly bg: {
  default: {
     value: "{colors.bg.negativeWash}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.bg.negativeWash}";
};

```

##### Type declaration

```
readonly value: "{colors.bg.negativeWash}" = "{colors.bg.negativeWash}";

```

```
readonly icon: {
  default: {
     value: "{colors.fg.negative}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.fg.negative}";
};

```

##### Type declaration

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

```
readonly text: {
  default: {
     value: "{colors.fg.onNegativeWash}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.fg.onNegativeWash}";
};

```

##### Type declaration

```
readonly value: "{colors.fg.onNegativeWash}" = "{colors.fg.onNegativeWash}";

```

```
readonly success: {
  bg: {
     default: {
        value: "{colors.bg.positiveWash}";
     };
  };
  icon: {
     default: {
        value: "{colors.fg.positive}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.onPositiveWash}";
     };
  };
};

```

##### Type declaration

```
readonly bg: {
  default: {
     value: "{colors.bg.positiveWash}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.bg.positiveWash}";
};

```

##### Type declaration

```
readonly value: "{colors.bg.positiveWash}" = "{colors.bg.positiveWash}";

```

```
readonly icon: {
  default: {
     value: "{colors.fg.positive}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.fg.positive}";
};

```

##### Type declaration

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

```
readonly text: {
  default: {
     value: "{colors.fg.onPositiveWash}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.fg.onPositiveWash}";
};

```

##### Type declaration

```
readonly value: "{colors.fg.onPositiveWash}" = "{colors.fg.onPositiveWash}";

```

```
readonly warning: {
  bg: {
     default: {
        value: "{colors.bg.warningWash}";
     };
  };
  icon: {
     default: {
        value: "{colors.fg.warning}";
     };
  };
  text: {
     default: {
        value: "{colors.fg.onWarningWash}";
     };
  };
};

```

##### Type declaration

```
readonly bg: {
  default: {
     value: "{colors.bg.warningWash}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.bg.warningWash}";
};

```

##### Type declaration

```
readonly value: "{colors.bg.warningWash}" = "{colors.bg.warningWash}";

```

```
readonly icon: {
  default: {
     value: "{colors.fg.warning}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.fg.warning}";
};

```

##### Type declaration

```
readonly value: "{colors.fg.warning}" = "{colors.fg.warning}";

```

```
readonly text: {
  default: {
     value: "{colors.fg.onWarningWash}";
  };
};

```

##### Type declaration

```
readonly default: {
  value: "{colors.fg.onWarningWash}";
};

```

##### Type declaration

```
readonly value: "{colors.fg.onWarningWash}" = "{colors.fg.onWarningWash}";

```

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