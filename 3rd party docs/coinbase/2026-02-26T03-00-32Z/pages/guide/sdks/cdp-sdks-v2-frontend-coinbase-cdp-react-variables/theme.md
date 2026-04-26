# theme

```
const theme: Flattened<{
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
                       value: readonly [readonly [..., ...]];
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
}>;

```

The theme is a flattened tokens object with values appropriate for web environments (i.e. CSS properties & CSS Variables). It DOES NOT include the namespace (`--cdp-web-`) in the keys.

## Example

```
const theme: Partial<Theme> = {
  "colors-bg-primary": "#0052ff",
  "colors-cta-primary-bg-default": "var(--cdp-web-colors-bg-primary)",
  "font-size-base": "16px",
};

```