# usesolanastandardwallets

```
function useSolanaStandardWallets(): {
  wallets: readonly Wallet[];
};

```

Hook to get all Standard Solana Wallets registered in the wallet standard. This hook:

-   Gets all wallets using getWallets().get()
-   Listens for wallet registration/unregistration events
-   Returns the complete list including the CDP wallet once registered

## Returns

```
{
  wallets: readonly Wallet[];
}

```

Object containing array of all registered wallets

### wallets

```
wallets: readonly Wallet[];

```

## Example

```
function WalletList() {
  const { wallets } = useSolanaStandardWallets();
  return (
    <div>
      <h3>Available Wallets:</h3>
      {wallets.map((wallet) => (
        <div key={wallet.name}>
          {wallet.name} {wallet.features['cdp:'] ? '(CDP)' : ''}
        </div>
      ))}
    </div>
  );
}

```