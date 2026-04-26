# baseconfig

```
type BaseConfig = {
  projectId: string;
  customAuth?: CustomAuth;
  useMock?: boolean;
  debugging?: boolean;
  basePath?: string;
  secureIframeBasePath?: string;
  disableAnalytics?: boolean;
  nativeOAuthCallback?: string;
};

```

Base configuration for the core package.

## Properties

Property

Type

Description

`projectId`

`string`

The CDP Project ID.

`customAuth?`

[`CustomAuth`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/CustomAuth)

Configuration for custom authentication with third-party identity providers. Use this when integrating with Auth0, Firebase, Cognito, or other IDPs. **Example** `lines import { useAuth0 } from '@auth0/auth0-react'; const { getAccessTokenSilently } = useAuth0(); await initialize({ projectId: 'my-project', customAuth: { getJwt: getAccessTokenSilently } });`

`useMock?`

`boolean`

Whether to use the mock implementation.

`debugging?`

`boolean`

Whether to enable debugging.

`basePath?`

`string`

The base path for the API.

`secureIframeBasePath?`

`string`

The base path for the secure iframe.

`disableAnalytics?`

`boolean`

Whether to disable analytics.

`nativeOAuthCallback?`

`string`

The callback URL for the native OAuth flow.