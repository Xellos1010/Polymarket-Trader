# legacy keys

### Legacy API keys are no longer supported

In February, 2025, we expired all legacy API keys. Create new [CDP API keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) and update your code to continue using Coinbase App APIs.

### Identifying legacy keys

Legacy keys are identifiable by the following characteristics:

-   Key name is 16 characters long
-   Key secret is 32 characters long

Example legacy key:

Example legacy secret:

```
M4009eFnOoPwhNoMwJlsAHgIgleGVFPl

```

Compared to CDP API keys which follow ECDSA format and have the following characteristics:

-   Key name is 64 characters long
-   Key secret is 128 characters long

Example CDP key name:

```
organizations/8f1ac569-ed29-48ae-b989-6798a975afab/apiKeys/87d98ae9-f31f-42ee-9b69-723d3ff9dd77

```

Example CDP key secret:

```
-----BEGIN EC PRIVATE KEY-----
MHcCAQEEIJsB+NpntMgnAHSo16vS6ies3V6nu/liXhPMd7s7+lZ6oAoGCCqGSM49
AwEHoUQDQgAEs0MXQHmufOeRPhjeJOkyNPJjaZv.......Zb5S
FBoh2Je3Rkj3do3+CU6OVOI7MzXPCX33NQ==
-----END EC PRIVATE KEY-----

```