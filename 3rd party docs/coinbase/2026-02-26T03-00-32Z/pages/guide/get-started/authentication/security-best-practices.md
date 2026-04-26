# security best practices

## Overview

Securing your CDP API keys is crucial when using the Coinbase Developer Platform. Exposed credentials can lead to compromised accounts and financial loss. Follow these best practices to keep your CDP API keys secure.

## Universal security practices

### 1\. Never embed keys in code

Embedding API keys in code increases the risk of accidental exposure. When sharing code, you might forget to remove embedded keys. **Instead:** Store keys in environment variables or files outside your application’s source tree.

### 2\. Never store keys inside your source tree

Keep API key files outside your application’s source tree to prevent them from being committed to version control systems like GitHub.

Limiting key access to specific sources reduces the impact of compromised credentials. Use the [allowlist feature](#ip-allowlist-configuration) to specify IPs or CIDRs, ensuring API requests are only honored from your defined origins.

### 4\. Restrict signatures to specific APIs

When multiple APIs are enabled in your project, restrict JWT token usage to specific APIs to prevent replay attacks. Include the API request path in the signing body to ensure signatures work only for their intended API.

### 5\. Delete unused keys

Remove API keys you no longer need to minimize attack surface.

### 6\. Rotate keys periodically

Regular key rotation reduces the risk of long-term key compromise. Since Coinbase Developer Platform uses **asymmetric cryptography**, key rotation requires creating new keys and deleting old ones.

## Secret API key security

Secret API keys are used for server-side authentication and must be kept private for maximum security. These keys should never be exposed in client-side code or public repositories.

### IP allowlist configuration

Restrict the use of your Secret API key to specific IPs (IPv4 and IPv6 supported). This provides an additional layer of security by ensuring requests can only originate from your authorized servers. **Steps to configure IP allowlist:**

1.  Click **API keys** on the left side navigation bar.
2.  Navigate to the API key you’d like to edit and click **Manage**.
3.  Click **Edit Key** and expand **API restrictions**.
4.  Add an IP or CIDR under **IP allowlist** and save the changes.

**IP Format Examples:**

-   IPv4: `192.168.45.123, 10.54.213.87`
-   IPv6: `2001:db8:1234:5678::1, 2001:db8:abcd:9876::42`

### Storage security

-   Store your private/public key pair in a secure location
-   Use environment variables for production deployments
-   Consider using secret management services (e.g., AWS Secrets Manager, HashiCorp Vault)
-   Never commit keys to version control

## Client API key security

Client API keys are used in client-side applications and have different security considerations than secret keys.

### Domain allowlist

Restrict the use of your Client API key to specific domains by validating the `Origin` header in API requests. This prevents unauthorized websites from using your key.

### Key rotation

Client API keys support rotation with configurable expiration times to ensure you can replace keys without downtime:

1.  Navigate to the Coinbase Developer Platform and select **Client API Key** under the **API Keys** tab
2.  Click the **Rotate** button
3.  Select an expiration time for your previous key
4.  Replace all references to your previous key, including in your RPC endpoint

## JWT authentication security

When using JWTs for API authentication:

-   JWTs expire after 2 minutes - regenerate before expiration to ensure uninterrupted access
-   Include nonces in JWT headers for added security against replay attacks
-   Use the Ed25519 signature algorithm for new keys (better performance and security than ECDSA)
-   Validate all JWT components including signatures and expiration times

## OAuth client security

When using OAuth clients for user authentication:

-   Store your client ID and client secret securely
-   Never expose client secrets in client-side code
-   Use secure state parameters (at least 8 characters) to prevent CSRF attacks
-   Store OAuth access and refresh tokens encrypted, with encryption keys in environment variables
-   Require all redirect URIs to use SSL (https://)
-   Always validate SSL certificates to prevent man-in-the-middle attacks
-   Implement proper token management (access tokens expire after 1 hour)
-   Use refresh tokens appropriately (they can only be used once)

## Additional recommendations

-   Monitor API key usage for suspicious activity
-   Implement rate limiting on your endpoints
-   Use HTTPS for all API communications
-   Always validate SSL certificates when connecting over HTTPS
-   Log and audit API key usage
-   Have an incident response plan for compromised keys
-   Regularly review and update your security practices
-   Consider using hardware security modules (HSMs) or secure enclaves for key storage in production
-   Follow the principle of least privilege when granting API permissions
-   Use separate keys for development, testing, and production environments