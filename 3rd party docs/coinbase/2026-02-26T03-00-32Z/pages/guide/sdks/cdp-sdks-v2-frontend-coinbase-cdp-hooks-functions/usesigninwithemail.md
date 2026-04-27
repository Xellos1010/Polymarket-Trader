# usesigninwithemail

```
function useSignInWithEmail(): {
  signInWithEmail: (options: SignInWithEmailOptions) => Promise<SignInWithEmailResult>;
};

```

Hook that provides access to the email-based sign-in functionality. This is the first step in the email authentication flow.

## Returns

```
{
  signInWithEmail: (options: SignInWithEmailOptions) => Promise<SignInWithEmailResult>;
}

```

### signInWithEmail()

```
signInWithEmail: (options: SignInWithEmailOptions) => Promise<SignInWithEmailResult>;

```

#### Parameters

#### Returns

`Promise`<[`SignInWithEmailResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/SignInWithEmailResult)\>

## Example

```
function SignInForm() {
  const [email, setEmail] = useState("");
  const [flowId, setFlowId] = useState("");
  const { signInWithEmail } = useSignInWithEmail();
  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const result = await signInWithEmail({ email });
      setFlowId(result.flowId);
    } catch (error) {
      console.error("Failed to sign in:", error);
    }
  };
  return (
    <form onSubmit={handleSubmit}>
      <input
        type="email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        placeholder="Enter your email"
        aria-label="Enter your email"
      />
      <button type="submit">Sign In</button>
    </form>
  );
}

```