# usecreatesolanaaccount

```
function useCreateSolanaAccount(): {
  createSolanaAccount: () => Promise<string>;
};

```

A hook for creating a Solana account for the current user. This function will throw an error if the user already has a Solana account.

## Returns

```
{
  createSolanaAccount: () => Promise<string>;
}

```

An object containing the createSolanaAccount function.

### createSolanaAccount()

```
createSolanaAccount: () => Promise<string>;

```

#### Returns

`Promise`<`string`\>

## Example

```
import { useCreateSolanaAccount } from '@coinbase/cdp-hooks';
function MyComponent() {
  const { createSolanaAccount } = useCreateSolanaAccount();
  const handleCreateAccount = async () => {
    try {
      const account = await createSolanaAccount();
      console.log('Solana account created:', account);
    } catch (error) {
      console.error('Failed to create Solana account:', error);
    }
  };
  return <button onClick={handleCreateAccount}>Create Solana Account</button>;
}

```