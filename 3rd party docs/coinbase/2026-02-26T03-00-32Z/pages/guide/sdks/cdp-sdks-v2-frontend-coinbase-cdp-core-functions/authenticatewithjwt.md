# authenticatewithjwt

```
function authenticateWithJWT(): Promise<AuthenticateWithJWTResult>;

```

Authenticates an end user using a developer-issued JWT from a third-party identity provider (e.g., Auth0, Firebase, Cognito, custom OIDC). This function validates the JWT, auto-creates the user if they don’t exist, and provisions wallets based on your configuration. **Important**: Before using this function, you must:

1.  Configure custom auth in the CDP Portal (JWKS endpoint, issuer, etc.)
2.  Provide a `customAuth.getJwt` callback in your config to handle JWT refresh

## Returns

`Promise`<[`AuthenticateWithJWTResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/AuthenticateWithJWTResult)\> The authentication result with user and wallet information

## Examples

```
// React - With Auth0
import { useAuth0 } from '@auth0/auth0-react';
import { initialize, authenticateWithJWT } from '@coinbase/cdp-core';
const { getAccessTokenSilently } = useAuth0();
await initialize({
  projectId: 'my-project',
  customAuth: {
    getJwt: getAccessTokenSilently
  },
  ethereum: { createOnLogin: 'eoa' }
});
const result = await authenticateWithJWT();
console.log(result.user); // Authenticated user with wallet

```

```
// React Native - With Auth0
import { useAuth0 } from 'react-native-auth0';
import { initialize, authenticateWithJWT } from '@coinbase/cdp-core';
const { authorize, getCredentials } = useAuth0();
await initialize({
  projectId: 'my-project',
  customAuth: {
    getJwt: async () => {
      const creds = await getCredentials();
      return creds?.accessToken;
    }
  },
  ethereum: { createOnLogin: 'eoa' }
});
// User clicks sign in - triggers Auth0 login
await authorize();
// Then authenticate with CDP
const result = await authenticateWithJWT();
console.log(result.user); // Authenticated user with wallet

```

## Throws

If customAuth.getJwt callback is not configured or JWT retrieval fails