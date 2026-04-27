# tokens

```
const tokens: {
  borderRadius: {
     badge: {
        value: "{borderRadius.full}";
     };
     banner: {
        value: "{borderRadius.lg}";
     };
     cta: {
        value: "{borderRadius.full}";
     };
     input: {
        value: "{borderRadius.sm}";
     };
     link: {
        value: "{borderRadius.full}";
     };
     modal: {
        value: "{borderRadius.sm}";
     };
     select: {
        trigger: {
           value: "{borderRadius.sm}";
        };
        list: {
           value: "{borderRadius.sm}";
        };
     };
     none: {
        value: 0;
     };
     xs: {
        value: "{font.size.base}";
        modify: {
           type: "multiply";
           value: 0.25;
        };
     };
     sm: {
        value: "{font.size.base}";
        modify: {
           type: "multiply";
           value: 0.5;
        };
     };
     md: {
        value: "{font.size.base}";
        modify: {
           type: "multiply";
           value: 0.75;
        };
     };
     lg: {
        value: "{font.size.base}";
        modify: {
           type: "multiply";
           value: 1;
        };
     };
     xl: {
        value: "{font.size.base}";
        modify: {
           type: "multiply";
           value: 1.5;
        };
     };
     full: {
        value: 99999;
     };
  };
  colors: {
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
  font: {
     family: {
        page: {
           value: "{font.family.body}";
        };
        cta: {
           value: "{font.family.interactive}";
        };
        link: {
           value: "{font.family.interactive}";
        };
        input: {
           value: "{font.family.interactive}";
        };
        select: {
           value: "{font.family.interactive}";
        };
        code: {
           value: "{font.family.mono}";
        };
        iframe: {
           value: "{font.family.interactive}";
        };
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
     url: {
        iframe: {
           value: "";
        };
     };
     size: {
        base: {
           value: 16;
        };
     };
  };
  zIndex: {
     select: {
        list: {
           value: "{zIndex.popup}";
        };
     };
     modal: {
        overlay: {
           value: "{zIndex.scrim}";
        };
        dialog: {
           value: "{zIndex.floating}";
        };
     };
     base: {
        value: 0;
        unit: "none";
     };
     raised: {
        value: 1;
        unit: "none";
     };
     popup: {
        value: 200;
        unit: "none";
     };
     scrim: {
        value: 400;
        unit: "none";
     };
     floating: {
        value: 500;
        unit: "none";
     };
  };
};

```

All the tokens used in the theme.

## Type declaration

### borderRadius

```
borderRadius: {
  badge: {
     value: "{borderRadius.full}";
  };
  banner: {
     value: "{borderRadius.lg}";
  };
  cta: {
     value: "{borderRadius.full}";
  };
  input: {
     value: "{borderRadius.sm}";
  };
  link: {
     value: "{borderRadius.full}";
  };
  modal: {
     value: "{borderRadius.sm}";
  };
  select: {
     trigger: {
        value: "{borderRadius.sm}";
     };
     list: {
        value: "{borderRadius.sm}";
     };
  };
  none: {
     value: 0;
  };
  xs: {
     value: "{font.size.base}";
     modify: {
        type: "multiply";
        value: 0.25;
     };
  };
  sm: {
     value: "{font.size.base}";
     modify: {
        type: "multiply";
        value: 0.5;
     };
  };
  md: {
     value: "{font.size.base}";
     modify: {
        type: "multiply";
        value: 0.75;
     };
  };
  lg: {
     value: "{font.size.base}";
     modify: {
        type: "multiply";
        value: 1;
     };
  };
  xl: {
     value: "{font.size.base}";
     modify: {
        type: "multiply";
        value: 1.5;
     };
  };
  full: {
     value: 99999;
  };
};

```

#### Type declaration

#### borderRadius.badge

```
readonly badge: {
  value: "{borderRadius.full}";
};

```

##### Type declaration

#### borderRadius.badge.value

```
readonly value: "{borderRadius.full}" = "{borderRadius.full}";

```

```
readonly banner: {
  value: "{borderRadius.lg}";
};

```

##### Type declaration

```
readonly value: "{borderRadius.lg}" = "{borderRadius.lg}";

```

#### borderRadius.cta

```
readonly cta: {
  value: "{borderRadius.full}";
};

```

##### Type declaration

#### borderRadius.cta.value

```
readonly value: "{borderRadius.full}" = "{borderRadius.full}";

```

#### borderRadius.input

```
readonly input: {
  value: "{borderRadius.sm}";
};

```

##### Type declaration

#### borderRadius.input.value

```
readonly value: "{borderRadius.sm}" = "{borderRadius.sm}";

```

#### borderRadius.link

```
readonly link: {
  value: "{borderRadius.full}";
};

```

##### Type declaration

#### borderRadius.link.value

```
readonly value: "{borderRadius.full}" = "{borderRadius.full}";

```

#### borderRadius.modal

```
readonly modal: {
  value: "{borderRadius.sm}";
};

```

##### Type declaration

#### borderRadius.modal.value

```
readonly value: "{borderRadius.sm}" = "{borderRadius.sm}";

```

#### borderRadius.select

```
readonly select: {
  trigger: {
     value: "{borderRadius.sm}";
  };
  list: {
     value: "{borderRadius.sm}";
  };
};

```

##### Type declaration

#### borderRadius.select.trigger

```
readonly trigger: {
  value: "{borderRadius.sm}";
};

```

##### Type declaration

#### borderRadius.select.trigger.value

```
readonly value: "{borderRadius.sm}" = "{borderRadius.sm}";

```

#### borderRadius.select.list

```
readonly list: {
  value: "{borderRadius.sm}";
};

```

##### Type declaration

#### borderRadius.select.list.value

```
readonly value: "{borderRadius.sm}" = "{borderRadius.sm}";

```

#### borderRadius.none

```
readonly none: {
  value: 0;
};

```

##### Type declaration

#### borderRadius.none.value

#### borderRadius.xs

```
readonly xs: {
  value: "{font.size.base}";
  modify: {
     type: "multiply";
     value: 0.25;
  };
};

```

##### Type declaration

#### borderRadius.xs.value

```
readonly value: "{font.size.base}" = "{font.size.base}";

```

#### borderRadius.xs.modify

```
readonly modify: {
  type: "multiply";
  value: 0.25;
};

```

##### Type declaration

#### borderRadius.xs.modify.type

```
readonly type: "multiply" = "multiply";

```

#### borderRadius.xs.modify.value

```
readonly value: 0.25 = 0.25;

```

#### borderRadius.sm

```
readonly sm: {
  value: "{font.size.base}";
  modify: {
     type: "multiply";
     value: 0.5;
  };
};

```

##### Type declaration

#### borderRadius.sm.value

```
readonly value: "{font.size.base}" = "{font.size.base}";

```

#### borderRadius.sm.modify

```
readonly modify: {
  type: "multiply";
  value: 0.5;
};

```

##### Type declaration

#### borderRadius.sm.modify.type

```
readonly type: "multiply" = "multiply";

```

#### borderRadius.sm.modify.value

```
readonly value: 0.5 = 0.5;

```

#### borderRadius.md

```
readonly md: {
  value: "{font.size.base}";
  modify: {
     type: "multiply";
     value: 0.75;
  };
};

```

##### Type declaration

#### borderRadius.md.value

```
readonly value: "{font.size.base}" = "{font.size.base}";

```

#### borderRadius.md.modify

```
readonly modify: {
  type: "multiply";
  value: 0.75;
};

```

##### Type declaration

#### borderRadius.md.modify.type

```
readonly type: "multiply" = "multiply";

```

#### borderRadius.md.modify.value

```
readonly value: 0.75 = 0.75;

```

#### borderRadius.lg

```
readonly lg: {
  value: "{font.size.base}";
  modify: {
     type: "multiply";
     value: 1;
  };
};

```

##### Type declaration

#### borderRadius.lg.value

```
readonly value: "{font.size.base}" = "{font.size.base}";

```

#### borderRadius.lg.modify

```
readonly modify: {
  type: "multiply";
  value: 1;
};

```

##### Type declaration

#### borderRadius.lg.modify.type

```
readonly type: "multiply" = "multiply";

```

#### borderRadius.lg.modify.value

#### borderRadius.xl

```
readonly xl: {
  value: "{font.size.base}";
  modify: {
     type: "multiply";
     value: 1.5;
  };
};

```

##### Type declaration

#### borderRadius.xl.value

```
readonly value: "{font.size.base}" = "{font.size.base}";

```

#### borderRadius.xl.modify

```
readonly modify: {
  type: "multiply";
  value: 1.5;
};

```

##### Type declaration

#### borderRadius.xl.modify.type

```
readonly type: "multiply" = "multiply";

```

#### borderRadius.xl.modify.value

```
readonly value: 1.5 = 1.5;

```

#### borderRadius.full

```
readonly full: {
  value: 99999;
};

```

##### Type declaration

#### borderRadius.full.value

```
readonly value: 99999 = 99999;

```

### colors

```
colors: {
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

#### Type declaration

#### colors.page

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

##### Type declaration

#### colors.page.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.default}";
  };
};

```

##### Type declaration

#### colors.page.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### colors.page.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### colors.page.border

```
readonly border: {
  default: {
     value: "{colors.line.default}";
  };
};

```

##### Type declaration

#### colors.page.border.default

```
readonly default: {
  value: "{colors.line.default}";
};

```

##### Type declaration

#### colors.page.border.default.value

```
readonly value: "{colors.line.default}" = "{colors.line.default}";

```

#### colors.page.text

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

#### colors.page.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.page.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.page.text.muted

```
readonly muted: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### colors.page.text.muted.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### colors.cta

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

##### Type declaration

#### colors.cta.primary

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

#### colors.cta.primary.bg

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

#### colors.cta.primary.bg.default

```
readonly default: {
  value: "{colors.bg.primary}";
};

```

##### Type declaration

#### colors.cta.primary.bg.default.value

```
readonly value: "{colors.bg.primary}" = "{colors.bg.primary}";

```

#### colors.cta.primary.bg.hover

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

#### colors.cta.primary.bg.hover.value

```
readonly value: "{colors.bg.primary}" = "{colors.bg.primary}";

```

#### colors.cta.primary.bg.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
};

```

##### Type declaration

#### colors.cta.primary.bg.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.cta.primary.bg.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "15%"]];

```

#### colors.cta.primary.bg.pressed

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

#### colors.cta.primary.bg.pressed.value

```
readonly value: "{colors.bg.primary}" = "{colors.bg.primary}";

```

#### colors.cta.primary.bg.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
};

```

##### Type declaration

#### colors.cta.primary.bg.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.cta.primary.bg.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "30%"]];

```

#### colors.cta.primary.border

```
readonly border: {
  focus: {
     value: "{colors.line.primary}";
  };
};

```

##### Type declaration

#### colors.cta.primary.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### colors.cta.primary.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### colors.cta.primary.text

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

#### colors.cta.primary.text.default

```
readonly default: {
  value: "{colors.fg.onPrimary}";
};

```

##### Type declaration

#### colors.cta.primary.text.default.value

```
readonly value: "{colors.fg.onPrimary}" = "{colors.fg.onPrimary}";

```

#### colors.cta.primary.text.hover

```
readonly hover: {
  value: "{colors.fg.onPrimary}";
};

```

##### Type declaration

#### colors.cta.primary.text.hover.value

```
readonly value: "{colors.fg.onPrimary}" = "{colors.fg.onPrimary}";

```

#### colors.cta.secondary

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

#### colors.cta.secondary.bg

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

#### colors.cta.secondary.bg.default

```
readonly default: {
  value: "{colors.bg.secondary}";
};

```

##### Type declaration

#### colors.cta.secondary.bg.default.value

```
readonly value: "{colors.bg.secondary}" = "{colors.bg.secondary}";

```

#### colors.cta.secondary.bg.hover

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

#### colors.cta.secondary.bg.hover.value

```
readonly value: "{colors.bg.secondary}" = "{colors.bg.secondary}";

```

#### colors.cta.secondary.bg.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
};

```

##### Type declaration

#### colors.cta.secondary.bg.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.cta.secondary.bg.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "10%"]];

```

#### colors.cta.secondary.bg.pressed

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

#### colors.cta.secondary.bg.pressed.value

```
readonly value: "{colors.bg.secondary}" = "{colors.bg.secondary}";

```

#### colors.cta.secondary.bg.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
};

```

##### Type declaration

#### colors.cta.secondary.bg.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.cta.secondary.bg.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "20%"]];

```

#### colors.cta.secondary.border

```
readonly border: {
  focus: {
     value: "{colors.line.primary}";
  };
};

```

##### Type declaration

#### colors.cta.secondary.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### colors.cta.secondary.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### colors.cta.secondary.text

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

#### colors.cta.secondary.text.default

```
readonly default: {
  value: "{colors.fg.onSecondary}";
};

```

##### Type declaration

#### colors.cta.secondary.text.default.value

```
readonly value: "{colors.fg.onSecondary}" = "{colors.fg.onSecondary}";

```

#### colors.cta.secondary.text.hover

```
readonly hover: {
  value: "{colors.fg.onSecondary}";
};

```

##### Type declaration

#### colors.cta.secondary.text.hover.value

```
readonly value: "{colors.fg.onSecondary}" = "{colors.fg.onSecondary}";

```

#### colors.link

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

##### Type declaration

#### colors.link.primary

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

#### colors.link.primary.text

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

#### colors.link.primary.text.default

```
readonly default: {
  value: "{colors.fg.primary}";
};

```

##### Type declaration

#### colors.link.primary.text.default.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### colors.link.primary.text.hover

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

#### colors.link.primary.text.hover.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### colors.link.primary.text.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "15%"]];
};

```

##### Type declaration

#### colors.link.primary.text.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.link.primary.text.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "15%"]];

```

#### colors.link.primary.text.pressed

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

#### colors.link.primary.text.pressed.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### colors.link.primary.text.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "30%"]];
};

```

##### Type declaration

#### colors.link.primary.text.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.link.primary.text.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "30%"]];

```

#### colors.link.secondary

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

#### colors.link.secondary.text

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

#### colors.link.secondary.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.link.secondary.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.link.secondary.text.hover

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

#### colors.link.secondary.text.hover.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.link.secondary.text.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
};

```

##### Type declaration

#### colors.link.secondary.text.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.link.secondary.text.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "10%"]];

```

#### colors.link.secondary.text.pressed

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

#### colors.link.secondary.text.pressed.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.link.secondary.text.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "20%"]];
};

```

##### Type declaration

#### colors.link.secondary.text.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.link.secondary.text.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "20%"]];

```

#### colors.input

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

##### Type declaration

#### colors.input.bg

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

#### colors.input.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### colors.input.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### colors.input.bg.readonly

```
readonly readonly: {
  value: "{colors.bg.alternate}";
};

```

##### Type declaration

#### colors.input.bg.readonly.value

```
readonly value: "{colors.bg.alternate}" = "{colors.bg.alternate}";

```

#### colors.input.border

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

#### colors.input.border.default

```
readonly default: {
  value: "{colors.line.heavy}";
};

```

##### Type declaration

#### colors.input.border.default.value

```
readonly value: "{colors.line.heavy}" = "{colors.line.heavy}";

```

#### colors.input.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### colors.input.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### colors.input.border.error

```
readonly error: {
  value: "{colors.line.negative}";
};

```

##### Type declaration

#### colors.input.border.error.value

```
readonly value: "{colors.line.negative}" = "{colors.line.negative}";

```

#### colors.input.border.success

```
readonly success: {
  value: "{colors.line.positive}";
};

```

##### Type declaration

#### colors.input.border.success.value

```
readonly value: "{colors.line.positive}" = "{colors.line.positive}";

```

#### colors.input.label

```
readonly label: {
  default: {
     value: "{colors.fg.default}";
  };
};

```

##### Type declaration

#### colors.input.label.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.input.label.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.input.placeholder

```
readonly placeholder: {
  default: {
     value: "{colors.fg.muted}";
  };
};

```

##### Type declaration

#### colors.input.placeholder.default

```
readonly default: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### colors.input.placeholder.default.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### colors.input.text

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

#### colors.input.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.input.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.input.text.readonly

```
readonly readonly: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### colors.input.text.readonly.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### colors.input.errorText

```
readonly errorText: {
  default: {
     value: "{colors.fg.negative}";
  };
};

```

##### Type declaration

#### colors.input.errorText.default

```
readonly default: {
  value: "{colors.fg.negative}";
};

```

##### Type declaration

#### colors.input.errorText.default.value

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

#### colors.input.successText

```
readonly successText: {
  default: {
     value: "{colors.fg.positive}";
  };
};

```

##### Type declaration

#### colors.input.successText.default

```
readonly default: {
  value: "{colors.fg.positive}";
};

```

##### Type declaration

#### colors.input.successText.default.value

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

#### colors.select

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

##### Type declaration

#### colors.select.label

```
readonly label: {
  default: {
     value: "{colors.fg.default}";
  };
};

```

##### Type declaration

#### colors.select.label.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.select.label.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.select.trigger

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

#### colors.select.trigger.bg

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

#### colors.select.trigger.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### colors.select.trigger.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### colors.select.trigger.bg.hover

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

#### colors.select.trigger.bg.hover.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### colors.select.trigger.bg.hover.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "5%"]];
};

```

##### Type declaration

#### colors.select.trigger.bg.hover.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.select.trigger.bg.hover.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "5%"]];

```

#### colors.select.trigger.bg.pressed

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

#### colors.select.trigger.bg.pressed.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### colors.select.trigger.bg.pressed.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "7%"]];
};

```

##### Type declaration

#### colors.select.trigger.bg.pressed.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.select.trigger.bg.pressed.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "7%"]];

```

#### colors.select.trigger.border

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

#### colors.select.trigger.border.default

```
readonly default: {
  value: "{colors.line.default}";
};

```

##### Type declaration

#### colors.select.trigger.border.default.value

```
readonly value: "{colors.line.default}" = "{colors.line.default}";

```

#### colors.select.trigger.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### colors.select.trigger.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### colors.select.trigger.border.error

```
readonly error: {
  value: "{colors.line.negative}";
};

```

##### Type declaration

#### colors.select.trigger.border.error.value

```
readonly value: "{colors.line.negative}" = "{colors.line.negative}";

```

#### colors.select.trigger.border.success

```
readonly success: {
  value: "{colors.line.positive}";
};

```

##### Type declaration

#### colors.select.trigger.border.success.value

```
readonly value: "{colors.line.positive}" = "{colors.line.positive}";

```

#### colors.select.trigger.placeholder

```
readonly placeholder: {
  default: {
     value: "{colors.fg.muted}";
  };
};

```

##### Type declaration

#### colors.select.trigger.placeholder.default

```
readonly default: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### colors.select.trigger.placeholder.default.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### colors.select.trigger.text

```
readonly text: {
  default: {
     value: "{colors.fg.default}";
  };
};

```

##### Type declaration

#### colors.select.trigger.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.select.trigger.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.select.trigger.errorText

```
readonly errorText: {
  default: {
     value: "{colors.fg.negative}";
  };
};

```

##### Type declaration

#### colors.select.trigger.errorText.default

```
readonly default: {
  value: "{colors.fg.negative}";
};

```

##### Type declaration

#### colors.select.trigger.errorText.default.value

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

#### colors.select.trigger.successText

```
readonly successText: {
  default: {
     value: "{colors.fg.positive}";
  };
};

```

##### Type declaration

#### colors.select.trigger.successText.default

```
readonly default: {
  value: "{colors.fg.positive}";
};

```

##### Type declaration

#### colors.select.trigger.successText.default.value

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

#### colors.select.list

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

#### colors.select.list.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.default}";
  };
};

```

##### Type declaration

#### colors.select.list.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### colors.select.list.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### colors.select.list.border

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

#### colors.select.list.border.default

```
readonly default: {
  value: "{colors.line.default}";
};

```

##### Type declaration

#### colors.select.list.border.default.value

```
readonly value: "{colors.line.default}" = "{colors.line.default}";

```

#### colors.select.list.border.focus

```
readonly focus: {
  value: "{colors.line.primary}";
};

```

##### Type declaration

#### colors.select.list.border.focus.value

```
readonly value: "{colors.line.primary}" = "{colors.line.primary}";

```

#### colors.select.list.border.error

```
readonly error: {
  value: "{colors.line.negative}";
};

```

##### Type declaration

#### colors.select.list.border.error.value

```
readonly value: "{colors.line.negative}" = "{colors.line.negative}";

```

#### colors.select.list.border.success

```
readonly success: {
  value: "{colors.line.positive}";
};

```

##### Type declaration

#### colors.select.list.border.success.value

```
readonly value: "{colors.line.positive}" = "{colors.line.positive}";

```

#### colors.select.list.item

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

#### colors.select.list.item.bg

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

#### colors.select.list.item.bg.default

```
readonly default: {
  value: "{colors.bg.default}";
};

```

##### Type declaration

#### colors.select.list.item.bg.default.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### colors.select.list.item.bg.highlight

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

#### colors.select.list.item.bg.highlight.value

```
readonly value: "{colors.bg.default}" = "{colors.bg.default}";

```

#### colors.select.list.item.bg.highlight.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.contrast}", "10%"]];
};

```

##### Type declaration

#### colors.select.list.item.bg.highlight.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.select.list.item.bg.highlight.modify.value

```
readonly value: readonly [readonly ["{colors.bg.contrast}", "10%"]];

```

#### colors.select.list.item.text

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

#### colors.select.list.item.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.select.list.item.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.select.list.item.text.muted

```
readonly muted: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### colors.select.list.item.text.muted.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### colors.select.list.item.text.onHighlight

```
readonly onHighlight: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.select.list.item.text.onHighlight.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.select.list.item.text.mutedOnHighlight

```
readonly mutedOnHighlight: {
  value: "{colors.fg.muted}";
};

```

##### Type declaration

#### colors.select.list.item.text.mutedOnHighlight.value

```
readonly value: "{colors.fg.muted}" = "{colors.fg.muted}";

```

#### colors.code

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

##### Type declaration

#### colors.code.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.alternate}";
  };
};

```

##### Type declaration

#### colors.code.bg.default

```
readonly default: {
  value: "{colors.bg.alternate}";
};

```

##### Type declaration

#### colors.code.bg.default.value

```
readonly value: "{colors.bg.alternate}" = "{colors.bg.alternate}";

```

#### colors.code.border

```
readonly border: {
  default: {
     value: "{colors.line.heavy}";
  };
};

```

##### Type declaration

#### colors.code.border.default

```
readonly default: {
  value: "{colors.line.heavy}";
};

```

##### Type declaration

#### colors.code.border.default.value

```
readonly value: "{colors.line.heavy}" = "{colors.line.heavy}";

```

#### colors.code.text

```
readonly text: {
  default: {
     value: "{colors.fg.default}";
  };
};

```

##### Type declaration

#### colors.code.text.default

```
readonly default: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.code.text.default.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.badge

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

##### Type declaration

#### colors.badge.primary

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

#### colors.badge.primary.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.primaryWash}";
  };
};

```

##### Type declaration

#### colors.badge.primary.bg.default

```
readonly default: {
  value: "{colors.bg.primaryWash}";
};

```

##### Type declaration

#### colors.badge.primary.bg.default.value

```
readonly value: "{colors.bg.primaryWash}" = "{colors.bg.primaryWash}";

```

#### colors.badge.primary.text

```
readonly text: {
  default: {
     value: "{colors.fg.primary}";
  };
};

```

##### Type declaration

#### colors.badge.primary.text.default

```
readonly default: {
  value: "{colors.fg.primary}";
};

```

##### Type declaration

#### colors.badge.primary.text.default.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### colors.badge.secondary

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

#### colors.badge.secondary.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.secondary}";
  };
};

```

##### Type declaration

#### colors.badge.secondary.bg.default

```
readonly default: {
  value: "{colors.bg.secondary}";
};

```

##### Type declaration

#### colors.badge.secondary.bg.default.value

```
readonly value: "{colors.bg.secondary}" = "{colors.bg.secondary}";

```

#### colors.badge.secondary.text

```
readonly text: {
  default: {
     value: "{colors.fg.onSecondary}";
  };
};

```

##### Type declaration

#### colors.badge.secondary.text.default

```
readonly default: {
  value: "{colors.fg.onSecondary}";
};

```

##### Type declaration

#### colors.badge.secondary.text.default.value

```
readonly value: "{colors.fg.onSecondary}" = "{colors.fg.onSecondary}";

```

#### colors.badge.warning

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

#### colors.badge.warning.bg

```
readonly bg: {
  default: {
     value: "{colors.bg.warningWash}";
  };
};

```

##### Type declaration

#### colors.badge.warning.bg.default

```
readonly default: {
  value: "{colors.bg.warningWash}";
};

```

##### Type declaration

#### colors.badge.warning.bg.default.value

```
readonly value: "{colors.bg.warningWash}" = "{colors.bg.warningWash}";

```

#### colors.badge.warning.text

```
readonly text: {
  default: {
     value: "{colors.fg.warning}";
  };
};

```

##### Type declaration

#### colors.badge.warning.text.default

```
readonly default: {
  value: "{colors.fg.warning}";
};

```

##### Type declaration

#### colors.badge.warning.text.default.value

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

##### Type declaration

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

#### colors.bg

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

##### Type declaration

#### colors.bg.default

```
readonly default: {
  value: "#ffffff";
};

```

##### Type declaration

#### colors.bg.default.value

```
readonly value: "#ffffff" = colorsBase.white;

```

#### colors.bg.alternate

```
readonly alternate: {
  value: "#eef0f3";
};

```

##### Type declaration

#### colors.bg.alternate.value

```
readonly value: "#eef0f3" = colorsBase.gray100;

```

#### colors.bg.contrast

```
readonly contrast: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.bg.contrast.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.bg.overlay

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

#### colors.bg.overlay.value

```
readonly value: "{colors.bg.alternate}" = "{colors.bg.alternate}";

```

#### colors.bg.overlay.modify

```
readonly modify: {
  type: "color-alpha";
  value: 0.33;
};

```

##### Type declaration

#### colors.bg.overlay.modify.type

```
readonly type: "color-alpha" = "color-alpha";

```

#### colors.bg.overlay.modify.value

```
readonly value: 0.33 = 0.33;

```

#### colors.bg.skeleton

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

#### colors.bg.skeleton.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.bg.skeleton.modify

```
readonly modify: {
  type: "color-alpha";
  value: 0.1;
};

```

##### Type declaration

#### colors.bg.skeleton.modify.type

```
readonly type: "color-alpha" = "color-alpha";

```

#### colors.bg.skeleton.modify.value

```
readonly value: 0.1 = 0.1;

```

#### colors.bg.primary

```
readonly primary: {
  value: "#0052ff";
};

```

##### Type declaration

#### colors.bg.primary.value

```
readonly value: "#0052ff" = colorsBase.blue500;

```

#### colors.bg.secondary

```
readonly secondary: {
  value: "#eef0f3";
};

```

##### Type declaration

#### colors.bg.secondary.value

```
readonly value: "#eef0f3" = colorsBase.gray100;

```

#### colors.bg.primaryWash

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

#### colors.bg.primaryWash.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### colors.bg.primaryWash.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.default}", "92%"]];
};

```

##### Type declaration

#### colors.bg.primaryWash.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.bg.primaryWash.modify.value

```
readonly value: readonly [readonly ["{colors.bg.default}", "92%"]];

```

#### colors.bg.positiveWash

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

#### colors.bg.positiveWash.value

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

#### colors.bg.positiveWash.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.default}", "92%"]];
};

```

##### Type declaration

#### colors.bg.positiveWash.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.bg.positiveWash.modify.value

```
readonly value: readonly [readonly ["{colors.bg.default}", "92%"]];

```

#### colors.bg.negativeWash

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

#### colors.bg.negativeWash.value

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

#### colors.bg.negativeWash.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.default}", "92%"]];
};

```

##### Type declaration

#### colors.bg.negativeWash.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.bg.negativeWash.modify.value

```
readonly value: readonly [readonly ["{colors.bg.default}", "92%"]];

```

#### colors.bg.warningWash

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

#### colors.bg.warningWash.value

```
readonly value: "{colors.fg.warning}" = "{colors.fg.warning}";

```

#### colors.bg.warningWash.modify

```
readonly modify: {
  type: "color-mix";
  value: readonly [readonly ["{colors.bg.default}", "92%"]];
};

```

##### Type declaration

#### colors.bg.warningWash.modify.type

```
readonly type: "color-mix" = "color-mix";

```

#### colors.bg.warningWash.modify.value

```
readonly value: readonly [readonly ["{colors.bg.default}", "92%"]];

```

#### colors.fg

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

##### Type declaration

#### colors.fg.default

```
readonly default: {
  value: "#0a0b0d";
};

```

##### Type declaration

#### colors.fg.default.value

```
readonly value: "#0a0b0d" = colorsBase.black;

```

#### colors.fg.muted

```
readonly muted: {
  value: "#5b616e";
};

```

##### Type declaration

#### colors.fg.muted.value

```
readonly value: "#5b616e" = colorsBase.gray700;

```

#### colors.fg.primary

```
readonly primary: {
  value: "#0052ff";
};

```

##### Type declaration

#### colors.fg.primary.value

```
readonly value: "#0052ff" = colorsBase.blue500;

```

#### colors.fg.onPrimary

```
readonly onPrimary: {
  value: "#ffffff";
};

```

##### Type declaration

#### colors.fg.onPrimary.value

```
readonly value: "#ffffff" = colorsBase.white;

```

#### colors.fg.onSecondary

```
readonly onSecondary: {
  value: "#0a0b0d";
};

```

##### Type declaration

#### colors.fg.onSecondary.value

```
readonly value: "#0a0b0d" = colorsBase.black;

```

#### colors.fg.positive

```
readonly positive: {
  value: "#098551";
};

```

##### Type declaration

#### colors.fg.positive.value

```
readonly value: "#098551" = colorsBase.green500;

```

#### colors.fg.negative

```
readonly negative: {
  value: "#cf202f";
};

```

##### Type declaration

#### colors.fg.negative.value

```
readonly value: "#cf202f" = colorsBase.red500;

```

#### colors.fg.warning

```
readonly warning: {
  value: "#ed702f";
};

```

##### Type declaration

#### colors.fg.warning.value

```
readonly value: "#ed702f" = colorsBase.amber500;

```

#### colors.fg.onPrimaryWash

```
readonly onPrimaryWash: {
  value: "{colors.fg.onPrimary}";
};

```

##### Type declaration

#### colors.fg.onPrimaryWash.value

```
readonly value: "{colors.fg.onPrimary}" = "{colors.fg.onPrimary}";

```

#### colors.fg.onPositiveWash

```
readonly onPositiveWash: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.fg.onPositiveWash.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.fg.onNegativeWash

```
readonly onNegativeWash: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.fg.onNegativeWash.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.fg.onWarningWash

```
readonly onWarningWash: {
  value: "{colors.fg.default}";
};

```

##### Type declaration

#### colors.fg.onWarningWash.value

```
readonly value: "{colors.fg.default}" = "{colors.fg.default}";

```

#### colors.line

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

##### Type declaration

#### colors.line.default

```
readonly default: {
  value: "#dcdfe4";
};

```

##### Type declaration

#### colors.line.default.value

```
readonly value: "#dcdfe4" = colorsBase.gray200;

```

#### colors.line.heavy

```
readonly heavy: {
  value: "#9397a0";
};

```

##### Type declaration

#### colors.line.heavy.value

```
readonly value: "#9397a0" = colorsBase.gray500;

```

#### colors.line.primary

```
readonly primary: {
  value: "{colors.fg.primary}";
};

```

##### Type declaration

#### colors.line.primary.value

```
readonly value: "{colors.fg.primary}" = "{colors.fg.primary}";

```

#### colors.line.positive

```
readonly positive: {
  value: "{colors.fg.positive}";
};

```

##### Type declaration

#### colors.line.positive.value

```
readonly value: "{colors.fg.positive}" = "{colors.fg.positive}";

```

#### colors.line.negative

```
readonly negative: {
  value: "{colors.fg.negative}";
};

```

##### Type declaration

#### colors.line.negative.value

```
readonly value: "{colors.fg.negative}" = "{colors.fg.negative}";

```

### font

```
font: {
  family: {
     page: {
        value: "{font.family.body}";
     };
     cta: {
        value: "{font.family.interactive}";
     };
     link: {
        value: "{font.family.interactive}";
     };
     input: {
        value: "{font.family.interactive}";
     };
     select: {
        value: "{font.family.interactive}";
     };
     code: {
        value: "{font.family.mono}";
     };
     iframe: {
        value: "{font.family.interactive}";
     };
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
  url: {
     iframe: {
        value: "";
     };
  };
  size: {
     base: {
        value: 16;
     };
  };
};

```

#### Type declaration

#### font.family

```
readonly family: {
  page: {
     value: "{font.family.body}";
  };
  cta: {
     value: "{font.family.interactive}";
  };
  link: {
     value: "{font.family.interactive}";
  };
  input: {
     value: "{font.family.interactive}";
  };
  select: {
     value: "{font.family.interactive}";
  };
  code: {
     value: "{font.family.mono}";
  };
  iframe: {
     value: "{font.family.interactive}";
  };
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

##### Type declaration

#### font.family.page

```
readonly page: {
  value: "{font.family.body}";
};

```

##### Type declaration

#### font.family.page.value

```
readonly value: "{font.family.body}" = "{font.family.body}";

```

#### font.family.cta

```
readonly cta: {
  value: "{font.family.interactive}";
};

```

##### Type declaration

#### font.family.cta.value

```
readonly value: "{font.family.interactive}" = "{font.family.interactive}";

```

#### font.family.link

```
readonly link: {
  value: "{font.family.interactive}";
};

```

##### Type declaration

#### font.family.link.value

```
readonly value: "{font.family.interactive}" = "{font.family.interactive}";

```

#### font.family.input

```
readonly input: {
  value: "{font.family.interactive}";
};

```

##### Type declaration

#### font.family.input.value

```
readonly value: "{font.family.interactive}" = "{font.family.interactive}";

```

#### font.family.select

```
readonly select: {
  value: "{font.family.interactive}";
};

```

##### Type declaration

#### font.family.select.value

```
readonly value: "{font.family.interactive}" = "{font.family.interactive}";

```

#### font.family.code

```
readonly code: {
  value: "{font.family.mono}";
};

```

##### Type declaration

#### font.family.code.value

```
readonly value: "{font.family.mono}" = "{font.family.mono}";

```

#### font.family.iframe

```
readonly iframe: {
  value: "{font.family.interactive}";
};

```

##### Type declaration

#### font.family.iframe.value

```
readonly value: "{font.family.interactive}" = "{font.family.interactive}";

```

#### font.family.mono

```
readonly mono: {
  value: "\"DM Mono\", monospace";
};

```

##### Type declaration

#### font.family.mono.value

```
readonly value: "\"DM Mono\", monospace" = '"DM Mono", monospace';

```

#### font.family.sans

```
readonly sans: {
  value: "\"Rethink Sans\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"";
};

```

##### Type declaration

#### font.family.sans.value

```
readonly value: "\"Rethink Sans\", -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif, \"Apple Color Emoji\", \"Segoe UI Emoji\", \"Segoe UI Symbol\"" = '"Rethink Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol"';

```

#### font.family.body

```
readonly body: {
  value: "{font.family.sans}";
};

```

##### Type declaration

#### font.family.body.value

```
readonly value: "{font.family.sans}" = "{font.family.sans}";

```

#### font.family.interactive

```
readonly interactive: {
  value: "{font.family.sans}";
};

```

##### Type declaration

#### font.family.interactive.value

```
readonly value: "{font.family.sans}" = "{font.family.sans}";

```

#### font.url

```
readonly url: {
  iframe: {
     value: "";
  };
};

```

##### Type declaration

#### font.url.iframe

```
readonly iframe: {
  value: "";
};

```

##### Type declaration

#### font.url.iframe.value

#### font.size

```
readonly size: {
  base: {
     value: 16;
  };
};

```

##### Type declaration

#### font.size.base

```
readonly base: {
  value: 16;
};

```

##### Type declaration

#### font.size.base.value

### zIndex

```
zIndex: {
  select: {
     list: {
        value: "{zIndex.popup}";
     };
  };
  modal: {
     overlay: {
        value: "{zIndex.scrim}";
     };
     dialog: {
        value: "{zIndex.floating}";
     };
  };
  base: {
     value: 0;
     unit: "none";
  };
  raised: {
     value: 1;
     unit: "none";
  };
  popup: {
     value: 200;
     unit: "none";
  };
  scrim: {
     value: 400;
     unit: "none";
  };
  floating: {
     value: 500;
     unit: "none";
  };
};

```

#### Type declaration

#### zIndex.select

```
readonly select: {
  list: {
     value: "{zIndex.popup}";
  };
};

```

##### Type declaration

#### zIndex.select.list

```
readonly list: {
  value: "{zIndex.popup}";
};

```

##### Type declaration

#### zIndex.select.list.value

```
readonly value: "{zIndex.popup}" = "{zIndex.popup}";

```

#### zIndex.modal

```
readonly modal: {
  overlay: {
     value: "{zIndex.scrim}";
  };
  dialog: {
     value: "{zIndex.floating}";
  };
};

```

##### Type declaration

#### zIndex.modal.overlay

```
readonly overlay: {
  value: "{zIndex.scrim}";
};

```

##### Type declaration

#### zIndex.modal.overlay.value

```
readonly value: "{zIndex.scrim}" = "{zIndex.scrim}";

```

#### zIndex.modal.dialog

```
readonly dialog: {
  value: "{zIndex.floating}";
};

```

##### Type declaration

#### zIndex.modal.dialog.value

```
readonly value: "{zIndex.floating}" = "{zIndex.floating}";

```

#### zIndex.base

```
readonly base: {
  value: 0;
  unit: "none";
};

```

##### Type declaration

#### zIndex.base.value

#### zIndex.base.unit

```
readonly unit: "none" = "none";

```

#### zIndex.raised

```
readonly raised: {
  value: 1;
  unit: "none";
};

```

##### Type declaration

#### zIndex.raised.value

#### zIndex.raised.unit

```
readonly unit: "none" = "none";

```

```
readonly popup: {
  value: 200;
  unit: "none";
};

```

##### Type declaration

```
readonly value: 200 = 200;

```

```
readonly unit: "none" = "none";

```

#### zIndex.scrim

```
readonly scrim: {
  value: 400;
  unit: "none";
};

```

##### Type declaration

#### zIndex.scrim.value

```
readonly value: 400 = 400;

```

#### zIndex.scrim.unit

```
readonly unit: "none" = "none";

```

#### zIndex.floating

```
readonly floating: {
  value: 500;
  unit: "none";
};

```

##### Type declaration

#### zIndex.floating.value

```
readonly value: 500 = 500;

```

#### zIndex.floating.unit

```
readonly unit: "none" = "none";

```