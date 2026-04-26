# getmfaconfig

```
function getMfaConfig(): Promise<MfaConfig>;

```

Gets the MFA configuration for the current project. This endpoint returns information about whether MFA is enabled for the project and the configuration for TOTP authentication.

## 

[​](#returns)

Returns

`Promise`<`MfaConfig`\> The MFA configuration for the project.

## 

[​](#example)

Example

```
const result = await getMfaConfig();
if (result?.totpConfig?.enabled) {
  console.log("TOTP MFA is enabled for this project");
}

```