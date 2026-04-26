# usesigninwithsms

```
function useSignInWithSms(): {
  signInWithSms: (options: SignInWithSmsOptions) => Promise<SignInWithSmsResult>;
};

```

Hook that provides access to the SMS-based sign-in functionality. This is the first step in the SMS authentication flow.

## Returns

```
{
  signInWithSms: (options: SignInWithSmsOptions) => Promise<SignInWithSmsResult>;
}

```

### signInWithSms()

```
signInWithSms: (options: SignInWithSmsOptions) => Promise<SignInWithSmsResult>;

```

#### Parameters

Parameter

Type

`options`

`SignInWithSmsOptions`

#### Returns

`Promise`<`SignInWithSmsResult`\>

## Example

```
function SignInForm() {
  const [phoneNumber, setPhoneNumber] = useState("");
  const [flowId, setFlowId] = useState("");
  const { signInWithSms } = useSignInWithSms();
  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const result = await signInWithSms({ phoneNumber });
      setFlowId(result.flowId);
    } catch (error) {
      console.error("Failed to sign in:", error);
    }
  };
  return (
    <form onSubmit={handleSubmit}>
      <input
        type="tel"
        value={phoneNumber}
        onChange={(e) => setPhoneNumber(e.target.value)}
        placeholder="Enter your phone number"
        aria-label="Enter your phone number"
      />
      <button type="submit">Sign In</button>
    </form>
  );
}

```