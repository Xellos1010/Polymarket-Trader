# authorization

## API Scopes and Permissions

Coinbase App APIs employ two distinct permission schemes to ensure granular access to user data and functionality. The scheme in use depends on the authentication method you choose for your application.

Authentication method

Permission scheme

Description

**API Keys**

Simple permission list

When creating API keys, users explicitly grant one or more of these high-level permissions. Your API key will only be able to perform actions within the scope of the permissions selected by the user.

**OAuth**

`service-name:resource:action` pattern

OAuth utilizes a more fine-grained permission model based on a colon-separated pattern. This allows for precise control over the specific resources and actions your application can access on behalf of the user.

### API Keys: Simple permissions

[API keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) offer a straightforward permission model. When a user generates an API key, they can select from the following basic permissions:

-   **View:** Allows your application to read user data such as account balances, transaction history, and profile information.
-   **Trade:** Grants permission to execute buy and sell orders on the user’s behalf.
-   **Transfer:** Enables your application to send and receive funds, on and off platform.

### OAuth: Granular scopes

[OAuth](https://developer.chrome.com/coinbase-app/oauth2-integration/overview) employs a more detailed scoping system, [see detailed list](https://developer.chrome.com/coinbase-app/oauth2-integration/scopes#supported-scopes). With OAuth2, scopes should be considered as grants. Users can select which scopes they grant access to for the application. The application might need to request new scopes over the lifecycle of the authorization. In general, only ask for the scopes that your application needs, and avoid asking for access to unnecessary ones.