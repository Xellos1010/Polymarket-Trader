# usewaitforuseroperation

```
function useWaitForUserOperation(parameters: UseWaitForUserOperationParameters): UseWaitForUserOperationReturnType;

```

Hook that waits for a user operation to be confirmed and provides its current state. This hook polls the user operation status until it’s confirmed or fails. Note: The `data` returned from the hook only represents the *last* sent user operation. If you wish to call one instance of the hook multiple times in quick succession it is recommended to save the userOpHash returned from `waitForUserOperation` yourself and handle waiting for the result.

## Parameters

Parameter

Type

Description

`parameters`

[`UseWaitForUserOperationParameters`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/UseWaitForUserOperationParameters)

Configuration object for waiting for user operation

## Returns

[`UseWaitForUserOperationReturnType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/UseWaitForUserOperationReturnType) Query result with data, loading state, and error information

## Example

```
function WaitForUserOperation() {
  const { status, data, error } = useWaitForUserOperation({
    userOperationHash: "0x1234...",
    evmSmartAccount: "0x1234567890123456789012345678901234567890",
    network: "base-sepolia"
  });
  return (
    <div>
      {status === "error" && <p>Error: {error?.message}</p>}
      {status === "success" && (
        <div>
          <p>User operation successful!</p>
          <p>Transaction Hash: {data.transactionHash}</p>
        </div>
      )}
    </div>
  );
}

```