# managing accounts

## Creating Accounts

You can assign a name to an account to make it easier to access. Account names can consist of alphanumeric characters and hyphens, and must be between 2 and 36 characters long. Account names must be unique within a single CDP project for each account type (e.g., all Solana accounts). You can assign an account name at the time of account creation, and retrieve it later using the name. The `getOrCreateAccount` method will create an account if it doesn’t exist, and return the existing account if it does.

### EVM Accounts

You can create EVM accounts with or without names. Here are examples of both approaches:

### Smart Accounts

You can also create and manage smart accounts that are owned by an EVM account. Each owner account can only have one smart account associated with it. The `getOrCreateSmartAccount` method will create a smart account if it doesn’t exist for the owner, and return the existing smart account if it does.

### Solana Accounts

You can create Solana accounts with or without names. Here are examples of both approaches:

## Managing Existing Accounts

Once you’ve created accounts, you can retrieve, list, and update them as needed.

### Getting Accounts by Address or Name

You can retrieve a specific account by its address or name using the `getAccount` method.

### Listing All Accounts

You can list all accounts of a specific type in a single CDP project by calling the `listAccounts` method:

### Updating Accounts

After creating an account, you can modify various properties including the account name and attach policies to govern account behavior.

#### Changing Account Names

You can change the name of an existing account using the `updateAccount` method:

#### Attaching Policies

You can attach [policies](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview) to accounts to govern their behavior, such as restricting transactions to specific addresses or limiting transaction values. Policies can be attached during account creation or added later using the `updateAccount` method:

### Account Manager UI

You can also manage account in the [CDP Portal Account Manager UI](https://portal.cdp.coinbase.com/products/server-wallets/accounts). From here you can view your accounts by chain and type, and you can click into an account to view more information like balances and policies.