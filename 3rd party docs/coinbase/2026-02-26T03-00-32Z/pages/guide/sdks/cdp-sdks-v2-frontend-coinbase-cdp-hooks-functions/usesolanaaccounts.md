# usesolanaaccounts

```
function useSolanaAccounts(): {
  solanaAccounts:   | null
     | EndUserSolanaAccount[];
};

```

Hook to access all Solana account objects for the current user. Returns an array of account objects with address and creation timestamp. Users can have up to 10 Solana accounts.

## Returns

```
{
  solanaAccounts:   | null
     | EndUserSolanaAccount[];
}

```

### solanaAccounts

```
solanaAccounts: 
  | null
  | EndUserSolanaAccount[];

```

## Example

```
function SolanaAccountsList() {
  const { solanaAccounts } = useSolanaAccounts();
  if (!solanaAccounts || solanaAccounts.length === 0) {
    return <p>No Solana accounts found</p>;
  }
  return (
    <div>
      <h3>Your Solana Accounts</h3>
      <ul>
        {solanaAccounts.map((account, index) => (
          <li key={account.address}>
            Account {index + 1}: {account.address}
            <br />
            Created: {new Date(account.createdAt).toLocaleDateString()}
          </li>
        ))}
      </ul>
    </div>
  );
}

```