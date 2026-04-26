# security requirements

## Overview

To prevent unauthorized usage of onramp sessions, all applications must implement proper security measures when integrating with CDP APIs. This ensures that onramp experiences can only be accessed through approved, compliant applications with authenticated users.

## CORS Protection Requirements

Any APIs that call our authenticated endpoints for **web based clients** must implement CORS headers:

-   Only allow specific, approved origins that should have access to your integration. Do not set `Access-Control-Allow-Origin` to `*`.
-   This prevents malicious websites from hijacking your APIs to create unauthorized onramp sessions

**Example CORS Configuration:**

```
Access-Control-Allow-Origin: https://yourapprovedomain.com
Access-Control-Allow-Methods: POST
Access-Control-Allow-Headers: Content-Type, Authorization

```

## Strongly Recommended Security Measures

While CORS provides essential protection, implementing additional authentication layers significantly enhances the security of your onramp integration.

### Authenticated Endpoints

**Require user authentication before creating onramp sessions.** This ensures that only legitimate, authenticated users can initiate onramp experiences.

#### Wallet Signature Authentication

Implement wallet signature verification where users sign a unique message (containing wallet address and timestamp) with their private key. This proves wallet ownership without exposing sensitive keys, making it cryptographically impossible for attackers to impersonate legitimate users.

#### Traditional User Login

For applications with existing user accounts, require standard JWT or session-based authentication before allowing session token creation. This leverages your existing authentication infrastructure and user management systems.

## Additional Resources

-   [CORS MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) — Comprehensive CORS implementation guide