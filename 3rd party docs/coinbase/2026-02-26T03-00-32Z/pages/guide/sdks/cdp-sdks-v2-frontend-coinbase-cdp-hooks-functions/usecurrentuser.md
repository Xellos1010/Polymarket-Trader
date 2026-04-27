# usecurrentuser

```
function useCurrentUser(): {
  currentUser:   | null
     | User;
};

```

Hook to access the currently authenticated user’s information.

## 

[​](#returns)

Returns

```
{
  currentUser:   | null
     | User;
}

```

### 

[​](#currentuser)

currentUser

```
currentUser: 
  | null
  | User;

```

## 

[​](#example)

Example

```
function UserProfile() {
  const { currentUser } = useCurrentUser();
  if (!currentUser) {
    return null;
  }
  return (
    <div>
      <h2>User Profile</h2>
      <p>User ID: {currentUser.userId}</p>
      <p>EVM Accounts: {currentUser.evmAccountObjects.map(acc => acc.address).join(", ")}</p>
    </div>
  );
}

```