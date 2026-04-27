# customauth

```
type CustomAuth = {
  getJwt: () => Promise<string | undefined>;
};

```

Configuration for custom authentication with third-party identity providers.

## Properties

Property

Type

Description

`getJwt`

() => `Promise`<`string` | `undefined`\>

Callback to retrieve a fresh JWT from your identity provider. Called automatically whenever CDP needs to authenticate requests. **Examples** `lines // With Auth0 (React) const { getAccessTokenSilently } = useAuth0(); customAuth: { getJwt: getAccessTokenSilently }` `lines // With Auth0 (React Native) const { getCredentials } = useAuth0(); customAuth: { getJwt: async () => { const creds = await getCredentials(); return creds?.accessToken; } }`