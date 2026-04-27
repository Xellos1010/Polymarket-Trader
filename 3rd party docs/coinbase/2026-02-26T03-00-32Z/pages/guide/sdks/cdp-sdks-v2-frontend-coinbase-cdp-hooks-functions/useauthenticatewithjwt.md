# useauthenticatewithjwt

```
function useAuthenticateWithJWT(): {
  authenticateWithJWT: () => Promise<AuthenticateWithJWTResult>;
};

```

Hook that provides access to authenticate end users with developer-issued JWTs. Use this when integrating with third-party identity providers (Auth0, Firebase, Cognito, etc.). **Important**: Before using this hook, you must:

1.  Configure custom auth in the CDP Portal (JWKS endpoint, issuer, etc.)
2.  Provide a `customAuth.getJwt` callback in your CDP initialization config

## Returns

```
{
  authenticateWithJWT: () => Promise<AuthenticateWithJWTResult>;
}

```

### authenticateWithJWT()

```
authenticateWithJWT: () => Promise<AuthenticateWithJWTResult>;

```

#### Returns

`Promise`<`AuthenticateWithJWTResult`\>

## Examples

```
// React - With Auth0
import { useAuth0 } from '@auth0/auth0-react';
import { useAuthenticateWithJWT } from '@coinbase/cdp-hooks';
function AuthButton() {
  const { getAccessTokenSilently } = useAuth0();
  const { authenticateWithJWT } = useAuthenticateWithJWT();
  const [error, setError] = useState(null);
  const handleLogin = async () => {
    try {
      // Authenticate with CDP (uses customAuth.getJwt callback)
      const { user, isNewUser } = await authenticateWithJWT();
      console.log('Authenticated user:', user);
      if (isNewUser) {
        console.log('New user created with wallet!');
      }
    } catch (err) {
      setError(err.message);
    }
  };
  return (
    <div>
      <button onClick={handleLogin}>Sign In with Auth0</button>
      {error && <p>Error: {error}</p>}
    </div>
  );
}

```

```
// React Native - With Auth0
import { useAuth0 } from 'react-native-auth0';
import { useAuthenticateWithJWT } from '@coinbase/cdp-hooks';
import { Button, Text, View } from 'react-native';
function AuthScreen() {
  const { authorize } = useAuth0();
  const { authenticateWithJWT } = useAuthenticateWithJWT();
  const [error, setError] = useState(null);
  const handleLogin = async () => {
    try {
      // Step 1: Trigger Auth0 login (opens browser)
      await authorize();
      // Step 2: Authenticate with CDP
      const { user, isNewUser } = await authenticateWithJWT();
      console.log('Authenticated user:', user);
      if (isNewUser) {
        console.log('New user created with wallet!');
      }
    } catch (err) {
      setError(err.message);
    }
  };
  return (
    <View>
      <Button title="Sign In with Auth0" onPress={handleLogin} />
      {error && <Text>Error: {error}</Text>}
    </View>
  );
}

```