# usesolanaaddress

```
function useSolanaAddress(): {
  solanaAddress: null | string;
};

```

Hook to access the user’s primary Solana address. Returns the first Solana address associated with the user’s account. If the user has no Solana accounts, this will return null. **Note:** This hook always returns the first account (index 0) as the “primary” address. If your users may have multiple Solana accounts and need to select between them, use `useSolanaAccounts()` instead to get all accounts with metadata.

## Returns

```
{
  solanaAddress: null | string;
}

```

### solanaAddress

```
solanaAddress: null | string;

```

## Example

```
function SolanaWalletInfo() {
  const { solanaAddress } = useSolanaAddress();
  if (!solanaAddress) {
    return <p>No Solana wallet connected</p>;
  }
  return (
    <div>
      <h3>Your Solana Wallet</h3>
      <p>Address: {solanaAddress}</p>
    </div>
  );
}

```

## See

useSolanaAccounts for multi-account support