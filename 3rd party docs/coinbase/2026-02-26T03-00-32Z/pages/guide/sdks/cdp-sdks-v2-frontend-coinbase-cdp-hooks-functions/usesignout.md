# usesignout

```
function useSignOut(): {
  signOut: () => Promise<void>;
};

```

Hook that provides a wrapped sign-out function with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to sign out.

## Returns

```
{
  signOut: () => Promise<void>;
}

```

### signOut()

```
signOut: () => Promise<void>;

```

#### Returns

`Promise`<`void`\>

## Example

```
function SignOutButton() {
  const { signOut } = useSignOut();
  const navigate = useNavigate();
  const handleSignOut = async () => {
    try {
      await signOut();
      navigate("/sign-in");
    } catch (error) {
      console.error("Failed to sign out:", error);
    }
  };
  return (
    <button onClick={handleSignOut}>
      Sign Out
    </button>
  );
}

```