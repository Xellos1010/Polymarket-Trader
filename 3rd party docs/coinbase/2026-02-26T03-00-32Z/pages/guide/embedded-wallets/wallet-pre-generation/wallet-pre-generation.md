# wallet pre generation

EVMSolana

## Overview

Pre-generate embedded wallets for your users before they sign in, enabling you to fund accounts with assets upfront for a seamless first-time experience.

## Why pre-generate wallets?

-   **Pre-load assets**: Fund wallets with loyalty points, gas, or welcome NFTs before users sign in
-   **Zero-friction onboarding**: Users see a ready-to-use wallet on first login instead of an empty account
-   **Targeted campaigns**: Prepare wallets for specific users (by email, phone, or JWT) before launching marketing campaigns

## Prerequisites

Before pre-generating wallets, ensure you have:

1.  **CDP API Key** - Create one in the [CDP Portal → API Keys](https://portal.cdp.coinbase.com/projects/api-keys)
2.  **Wallet Secret** - Generate one in the [CDP Portal → Server Wallet → Accounts](https://portal.cdp.coinbase.com/products/server-wallets)
3.  **CDP SDK** - Install the [CDP SDK](https://developer.chrome.com/sdks) in your project

### Getting your credentials

For more details, see the [Authentication documentation](https://developer.chrome.com/api-reference/v2/authentication) and [Wallet Secret documentation](https://developer.chrome.com/server-wallets/v2/introduction/security#wallet-secrets).

## Usage

Use the CDP SDK to create an end user with a specific authentication method. Once created, you can fund the wallet address before the user ever signs in.

### Creating an end user

The `createEndUser` method creates a new end user with an associated wallet. You specify the authentication method (email, SMS, or JWT) that the user will use to sign in later.

#### Email authentication

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  // Create an end user with an email authentication method 
  // and both EVM and Solana accounts.
  const endUser = await cdp.endUser.createEndUser({
    authenticationMethods: [
      { type: "email", email: "user@example.com" }
    ],
    evmAccount: { createSmartAccount: false },
    solanaAccount: { createSmartAccount: false }
  });
  console.log("Created end user:", endUser);
  // The end user's wallet addresses are now available.
  // You can fund these addresses before the user signs in.
  console.log("EVM address:", endUser.evmAccounts?.[0]);
  console.log("Solana address:", endUser.solanaAccounts?.[0]);
} catch (error) {
  console.error("Error creating end user:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.create_end_user_request_evm_account import (
    CreateEndUserRequestEvmAccount,
)
from cdp.openapi_client.models.create_end_user_request_solana_account import (
    CreateEndUserRequestSolanaAccount,
)
from cdp.openapi_client.models.email_authentication import EmailAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            # Create an end user with an email authentication method
            # and both EVM and Solana accounts.
            end_user = await cdp.end_user.create_end_user(
                authentication_methods=[
                    AuthenticationMethod(EmailAuthentication(type="email", email="user@example.com"))
                ],
                evm_account=CreateEndUserRequestEvmAccount(create_smart_account=False),
                solana_account=CreateEndUserRequestSolanaAccount(create_smart_account=False),
            )
            print("Created end user:", end_user)
            # The end user's wallet addresses are now available.
            # You can fund these addresses before the user signs in.
        except Exception as e:
            print(f"Error creating end user: {e}")
            raise e
asyncio.run(main())

```

#### SMS authentication

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  // Create an end user with an SMS authentication method
  // and both EVM and Solana accounts.
  const endUser = await cdp.endUser.createEndUser({
    authenticationMethods: [
      { type: "sms", phoneNumber: "+12055555555" }
    ],
    evmAccount: { createSmartAccount: false },
    solanaAccount: { createSmartAccount: false }
  });
  console.log("Created end user:", endUser);
  // The end user's wallet addresses are now available.
  // You can fund these addresses before the user signs in.
  console.log("EVM address:", endUser.evmAccounts?.[0]);
  console.log("Solana address:", endUser.solanaAccounts?.[0]);
} catch (error) {
  console.error("Error creating end user:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.create_end_user_request_evm_account import (
    CreateEndUserRequestEvmAccount,
)
from cdp.openapi_client.models.create_end_user_request_solana_account import (
    CreateEndUserRequestSolanaAccount,
)
from cdp.openapi_client.models.sms_authentication import SmsAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            # Create an end user with an SMS authentication method
            # and both EVM and Solana accounts.
            end_user = await cdp.end_user.create_end_user(
                authentication_methods=[
                    AuthenticationMethod(SmsAuthentication(type="sms", phone_number="+12055555555"))
                ],
                evm_account=CreateEndUserRequestEvmAccount(create_smart_account=False),
                solana_account=CreateEndUserRequestSolanaAccount(create_smart_account=False),
            )
            print("Created end user:", end_user)
            # The end user's wallet addresses are now available.
            # You can fund these addresses before the user signs in.
        except Exception as e:
            print(f"Error creating end user: {e}")
            raise e
asyncio.run(main())

```

#### Custom (JWT) Authentication

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  // Create an end user with a JWT authentication method
  // and both EVM and Solana accounts.
  const endUser = await cdp.endUser.createEndUser({
    authenticationMethods: [
      { type: "jwt", sub: "auth0|69387f18541e0e673845c6b6", kid: "1234567890" }
    ],
    evmAccount: { createSmartAccount: false },
    solanaAccount: { createSmartAccount: false }
  });
  console.log("Created end user:", endUser);
  // The end user's wallet addresses are now available.
  // You can fund these addresses before the user signs in.
  console.log("EVM address:", endUser.evmAccounts?.[0]);
  console.log("Solana address:", endUser.solanaAccounts?.[0]);
} catch (error) {
  console.error("Error creating end user:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.create_end_user_request_evm_account import (
    CreateEndUserRequestEvmAccount,
)
from cdp.openapi_client.models.create_end_user_request_solana_account import (
    CreateEndUserRequestSolanaAccount,
)
from cdp.openapi_client.models.developer_jwt_authentication import DeveloperJWTAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            # Create an end user with a JWT authentication method
            # and both EVM and Solana accounts.
            end_user = await cdp.end_user.create_end_user(
                authentication_methods=[
                    AuthenticationMethod(DeveloperJWTAuthentication(type="jwt", sub="auth0|69387f18541e0e673845c6b6", kid="1234567890"))
                ],
                evm_account=CreateEndUserRequestEvmAccount(create_smart_account=False),
                solana_account=CreateEndUserRequestSolanaAccount(create_smart_account=False),
            )
            print("Created end user:", end_user)
            # The end user's wallet addresses are now available.
            # You can fund these addresses before the user signs in.
        except Exception as e:
            print(f"Error creating end user: {e}")
            raise e
asyncio.run(main())

```

### Pre-generate wallet with an existing private key

If you already have access to your end users’ private keys, you can import them directly into Embedded Wallets. This is useful when migrating users from other wallet solutions—such as [Server Wallets](https://developer.chrome.com/server-wallets/v2/introduction/welcome)—to Embedded Wallets.

#### Email authentication

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  // Import an end user with an existing private key.
  // For Solana: use keyType: "solana" with a base58-encoded private key.
  const endUser = await cdp.endUser.importEndUser({
    authenticationMethods: [
      { type: "email", email: "user@example.com" }
    ],
    privateKey: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    keyType: "evm",
  });
  console.log("Imported end user:", endUser);
  console.log("EVM accounts:", endUser.evmAccountObjects);
} catch (error) {
  console.error("Error importing end user:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.email_authentication import EmailAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            # Import an end user with an existing private key.
            # For Solana: use key_type="solana" with a base58-encoded private key.
            end_user = await cdp.end_user.import_end_user(
                authentication_methods=[
                    AuthenticationMethod(EmailAuthentication(type="email", email="user@example.com"))
                ],
                private_key="0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                key_type="evm",
            )
            print("Imported end user:", end_user)
            print("EVM accounts:", end_user.evm_account_objects)
        except Exception as e:
            print(f"Error importing end user: {e}")
            raise e
asyncio.run(main())

```

#### SMS authentication

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  // Import an end user with an existing private key.
  // For Solana: use keyType: "solana" with a base58-encoded private key.
  const endUser = await cdp.endUser.importEndUser({
    authenticationMethods: [
      { type: "sms", phoneNumber: "+12055555555" }
    ],
    privateKey: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    keyType: "evm",
  });
  console.log("Imported end user:", endUser);
  console.log("EVM accounts:", endUser.evmAccountObjects);
} catch (error) {
  console.error("Error importing end user:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.sms_authentication import SmsAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            # Import an end user with an existing private key.
            # For Solana: use key_type="solana" with a base58-encoded private key.
            end_user = await cdp.end_user.import_end_user(
                authentication_methods=[
                    AuthenticationMethod(SmsAuthentication(type="sms", phone_number="+12055555555"))
                ],
                private_key="0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                key_type="evm",
            )
            print("Imported end user:", end_user)
            print("EVM accounts:", end_user.evm_account_objects)
        except Exception as e:
            print(f"Error importing end user: {e}")
            raise e
asyncio.run(main())

```

#### Custom (JWT) authentication

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  // Import an end user with an existing private key.
  // For Solana: use keyType: "solana" with a base58-encoded private key.
  const endUser = await cdp.endUser.importEndUser({
    authenticationMethods: [
      { type: "jwt", sub: "auth0|69387f18541e0e673845c6b6", kid: "1234567890" }
    ],
    privateKey: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    keyType: "evm",
  });
  console.log("Imported end user:", endUser);
  console.log("EVM accounts:", endUser.evmAccountObjects);
} catch (error) {
  console.error("Error importing end user:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.developer_jwt_authentication import DeveloperJWTAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            # Import an end user with an existing private key.
            # For Solana: use key_type="solana" with a base58-encoded private key.
            end_user = await cdp.end_user.import_end_user(
                authentication_methods=[
                    AuthenticationMethod(DeveloperJWTAuthentication(type="jwt", sub="auth0|69387f18541e0e673845c6b6", kid="1234567890"))
                ],
                private_key="0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                key_type="evm",
            )
            print("Imported end user:", end_user)
            print("EVM accounts:", end_user.evm_account_objects)
        except Exception as e:
            print(f"Error importing end user: {e}")
            raise e
asyncio.run(main())

```

## Account Configuration

By default, pre-generated wallets are created as EOA (Externally Owned Accounts). You can configure the account type and features using the `evmAccount` and `solanaAccount` parameters.

### Smart Accounts

Create an EVM smart account instead of an EOA by setting `createSmartAccount: true`.

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  const endUser = await cdp.endUser.createEndUser({
    authenticationMethods: [
      { type: "email", email: "user@example.com" }
    ],
    evmAccount: { createSmartAccount: true },
    solanaAccount: { createSmartAccount: false }
  });
  console.log("Created end user with smart account:", endUser);
  
  // Access the smart account object
  const smartAccount = endUser.evmSmartAccountObjects?.[0];
  console.log("Smart account address:", smartAccount?.address);
  console.log("Owner addresses:", smartAccount?.ownerAddresses);
  console.log("Solana address:", endUser.solanaAccounts?.[0]);
} catch (error) {
  console.error("Error creating end user:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.create_end_user_request_evm_account import (
    CreateEndUserRequestEvmAccount,
)
from cdp.openapi_client.models.create_end_user_request_solana_account import (
    CreateEndUserRequestSolanaAccount,
)
from cdp.openapi_client.models.email_authentication import EmailAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            end_user = await cdp.end_user.create_end_user(
                authentication_methods=[
                    AuthenticationMethod(EmailAuthentication(type="email", email="user@example.com"))
                ],
                evm_account=CreateEndUserRequestEvmAccount(create_smart_account=True),
                solana_account=CreateEndUserRequestSolanaAccount(create_smart_account=False),
            )
            print("Created end user with smart account:", end_user)
            
            smart_account = end_user.evm_smart_account_objects[0]
            print(f"Smart account address: {smart_account.address}")
            print(f"Owner addresses: {smart_account.owner_addresses}")
            print(f"Solana address: {end_user.solana_accounts[0]}")
        except Exception as e:
            print(f"Error creating end user: {e}")
            raise e
asyncio.run(main())

```

### Spend Permissions

Enable spend permissions on an EVM smart account by setting `enableSpendPermissions: true`. This requires `createSmartAccount: true`.

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  const endUser = await cdp.endUser.createEndUser({
    authenticationMethods: [
      { type: "email", email: "user@example.com" }
    ],
    evmAccount: { 
      createSmartAccount: true,
      enableSpendPermissions: true
    },
    solanaAccount: { createSmartAccount: false }
  });
  console.log("Created end user with spend permissions:", endUser);
  
  const smartAccount = endUser.evmSmartAccountObjects?.[0];
  console.log("Smart account address:", smartAccount?.address);
  
  // When spend permissions are enabled, there are 2 owner addresses:
  // 1. User's owner address
  // 2. Spend Permission Manager address
  console.log("Owner addresses:", smartAccount?.ownerAddresses);
  console.log("Solana address:", endUser.solanaAccounts?.[0]);
} catch (error) {
  console.error("Error creating end user:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.create_end_user_request_evm_account import (
    CreateEndUserRequestEvmAccount,
)
from cdp.openapi_client.models.create_end_user_request_solana_account import (
    CreateEndUserRequestSolanaAccount,
)
from cdp.openapi_client.models.email_authentication import EmailAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            end_user = await cdp.end_user.create_end_user(
                authentication_methods=[
                    AuthenticationMethod(EmailAuthentication(type="email", email="user@example.com"))
                ],
                evm_account=CreateEndUserRequestEvmAccount(
                    create_smart_account=True, 
                    enable_spend_permissions=True
                ),
                solana_account=CreateEndUserRequestSolanaAccount(create_smart_account=False),
            )
            print("Created end user with spend permissions:", end_user)
            
            smart_account = end_user.evm_smart_account_objects[0]
            print(f"Smart account address: {smart_account.address}")
            
            # When spend permissions are enabled, there are 2 owner addresses:
            # 1. User's owner address
            # 2. Spend Permission Manager address
            print(f"Owner addresses: {smart_account.owner_addresses}")
            print(f"Solana address: {end_user.solana_accounts[0]}")
        except Exception as e:
            print(f"Error creating end user: {e}")
            raise e
asyncio.run(main())

```

## Adding Accounts to Existing End Users

After creating an end user, you can add additional accounts directly on the `EndUser` object. Each end user can have up to 10 EVM EOA accounts, 10 EVM smart accounts, and 10 Solana accounts.

-   TypeScript
    
-   Python
    

```
import { CdpClient } from "@coinbase/cdp-sdk";
import "dotenv/config";
const cdp = new CdpClient();
try {
  // Create an end user
  const endUser = await cdp.endUser.createEndUser({
    authenticationMethods: [
      { type: "email", email: "user@example.com" }
    ]
  });
  // Add an EVM EOA account
  const evmResult = await endUser.addEvmAccount();
  console.log("EVM account address:", evmResult.evmAccount.address);
  // Add an EVM smart account
  const smartResult = await endUser.addEvmSmartAccount({
    enableSpendPermissions: false
  });
  console.log("Smart account address:", smartResult.evmSmartAccount.address);
  // Add a Solana account
  const solanaResult = await endUser.addSolanaAccount();
  console.log("Solana account address:", solanaResult.solanaAccount.address);
} catch (error) {
  console.error("Error adding accounts:", error);
}

```

```
import asyncio
from cdp import CdpClient
from cdp.openapi_client.models.authentication_method import AuthenticationMethod
from cdp.openapi_client.models.email_authentication import EmailAuthentication
from dotenv import load_dotenv
load_dotenv()
async def main():
    async with CdpClient() as cdp:
        try:
            # Create an end user
            end_user = await cdp.end_user.create_end_user(
                authentication_methods=[
                    AuthenticationMethod(EmailAuthentication(type="email", email="user@example.com"))
                ]
            )
            # Add an EVM EOA account
            evm_result = await end_user.add_evm_account()
            print(f"EVM account address: {evm_result.evm_account.address}")
            # Add an EVM smart account
            smart_result = await end_user.add_evm_smart_account(
                enable_spend_permissions=False
            )
            print(f"Smart account address: {smart_result.evm_smart_account.address}")
            # Add a Solana account
            solana_result = await end_user.add_solana_account()
            print(f"Solana account address: {solana_result.solana_account.address}")
        except Exception as e:
            print(f"Error adding accounts: {e}")
            raise e
asyncio.run(main())

```

### Using client methods

You can also add accounts using the client methods directly by providing the end user’s ID:

-   TypeScript
    
-   Python
    

```
// Add accounts via client methods
const evmResult = await cdp.endUser.addEndUserEvmAccount({
  userId: endUser.id
});
const smartResult = await cdp.endUser.addEndUserEvmSmartAccount({
  userId: endUser.id,
  enableSpendPermissions: false
});
const solanaResult = await cdp.endUser.addEndUserSolanaAccount({
  userId: endUser.id
});

```

```
# Add accounts via client methods
evm_result = await cdp.end_user.add_end_user_evm_account(
    user_id=end_user.id
)
smart_result = await cdp.end_user.add_end_user_evm_smart_account(
    user_id=end_user.id,
    enable_spend_permissions=False
)
solana_result = await cdp.end_user.add_end_user_solana_account(
    user_id=end_user.id
)

```

## What to read next