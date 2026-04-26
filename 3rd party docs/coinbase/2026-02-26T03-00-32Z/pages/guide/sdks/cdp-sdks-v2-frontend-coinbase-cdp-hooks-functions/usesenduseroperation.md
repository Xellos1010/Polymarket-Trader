# usesenduseroperation

```
function useSendUserOperation(): UseSendUserOperationReturnType;

```

Hook that provides a wrapped function to send user operations from Smart Accounts with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to send user operations. The hook internally waits for user operations to succeed and returns the related success/error via the `data` discriminated union object. Note: The `data` returned from the hook only represents the *last* sent user operation. If you wish to call one instance of the hook multiple times in quick succession it is recommended to save the userOperationHash returned from `sendUserOperation` yourself and handle waiting for the result.

## Returns

[`UseSendUserOperationReturnType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/UseSendUserOperationReturnType)

## Example

```
function SendUserOperation() {
  const { sendUserOperation, data, isError, error } = useSendUserOperation();
  const { currentUser } = useCurrentUser();
  const handleSendUserOperation = async () => {
    const smartAccount = currentUser?.evmSmartAccounts?.[0];
    if (!smartAccount) return;
    try {
      const result = await sendUserOperation({
        evmAccount: smartAccount,
        network: "base-sepolia",
        calls: [{
          to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
          value: "0x0",
          data: "0x",
        }],
        dataSuffix: "0x62617365617070070080218021802180218021802180218021", // Optional: ERC-8021 attribution
      });
      console.log("User Operation Hash:", result.userOperationHash);
    } catch (error) {
      console.error("Failed to send user operation:", error);
    }
  };
  return (
    <div>
      {isError && <p>Error: {error?.message}</p>}
      {data && (
        <div>
          <p>User operation successful!</p>
          <p>Transaction Hash: {data.transactionHash}</p>
        </div>
      )}
      <button onClick={handleSendUserOperation}>Send User Operation</button>
    </div>
  );
}

```