# usecreateevmeoaaccount

```
function useCreateEvmEoaAccount(): {
  createEvmEoaAccount: () => Promise<`0x${string}`>;
};

```

A hook for creating an EVM EOA (Externally Owned Account) for the current user. This function will throw an error if the user already has an EVM EOA account.

## Returns

```
{
  createEvmEoaAccount: () => Promise<`0x${string}`>;
}

```

An object containing the createEvmEoaAccount function.

### createEvmEoaAccount()

```
createEvmEoaAccount: () => Promise<`0x${string}`>;

```

#### Returns

`Promise`<`` `0x${string}` ``\>

## Example

```
import { useCreateEvmEoaAccount } from '@coinbase/cdp-hooks';
function MyComponent() {
  const { createEvmEoaAccount } = useCreateEvmEoaAccount();
  const handleCreateAccount = async () => {
    try {
      const account = await createEvmEoaAccount();
      console.log('EVM EOA account created:', account);
    } catch (error) {
      console.error('Failed to create EVM EOA account:', error);
    }
  };
  return <button onClick={handleCreateAccount}>Create EVM EOA Account</button>;
}

```