# usesendevmsmartaccountusdc

```
function useSendEvmSmartAccountUsdc(): UseSendEvmSmartAccountUsdcReturnType;

```

Hook to send USDC on an EVM network from a Smart Account. This hook waits for the user operation to complete and provides real-time status updates. Returns user operation data rather than transaction receipts, since Smart Account transactions use ERC-4337 user operations.

## Returns

[`UseSendEvmSmartAccountUsdcReturnType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/UseSendEvmSmartAccountUsdcReturnType)

## Example

```
import { useSendEvmSmartAccountUsdc, useCurrentUser } from "@coinbase/cdp-hooks";
function MyComponent() {
  const { sendEvmSmartAccountUsdc, status, data, error } = useSendEvmSmartAccountUsdc();
  const { currentUser } = useCurrentUser();
  const evmSmartAccount = currentUser?.evmSmartAccountObjects?.[0]?.address;
  const handleSendUsdc = async () => {
    if (!evmSmartAccount) return;
    try {
      const result = await sendEvmSmartAccountUsdc({
        evmSmartAccount,
        to: "0x1234567890123456789012345678901234567890",
        amount: "10.00",
        network: "base-sepolia",
        useCdpPaymaster: true, // Optional: sponsor gas fees
      });
      console.log("User Operation Hash:", result.userOpHash);
    } catch (error) {
      console.error("Failed to send USDC:", error);
    }
  };
  return (
    <div>
      {status === "pending" && <div>Sending USDC...</div>}
      {status === "success" && data && (
        <div>
          <div>USDC sent! TX: {data.transactionHash}</div>
          <div>Status: {data.status}</div>
        </div>
      )}
      {status === "error" && <div>Error: {error?.message}</div>}
      <button onClick={handleSendUsdc} disabled={status === "pending"}>
        Send USDC
      </button>
    </div>
  );
}

```