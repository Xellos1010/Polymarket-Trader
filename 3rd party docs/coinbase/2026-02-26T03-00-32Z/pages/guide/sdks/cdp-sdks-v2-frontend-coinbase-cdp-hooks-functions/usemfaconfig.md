# usemfaconfig

```
function useMfaConfig(): 
  | {
  mfaConfig: undefined;
}
  | {
  mfaConfig: null | MfaConfig;
};

```

A hook for reading the pre-fetched MFA configuration for the current project. The config is fetched during SDK initialization and cached in core state. Returns null if MFA is not configured (404 during init).

## 

[​](#returns)

Returns

| { `mfaConfig`: `undefined`; } | { `mfaConfig`: `null` | `MfaConfig`; } The MFA configuration, null if not configured, or undefined if not initialized.