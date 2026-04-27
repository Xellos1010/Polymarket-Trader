# usesendusdc

```
function useSendUsdc(): UseSendUsdcReturnType;

```

Hook to send USDC on EVM or Solana networks with automatic address type detection. For EVM EOA accounts, this hook waits for transaction receipt. For EVM Smart Accounts, this hook waits for user operation completion. For Solana accounts, returns the transaction signature immediately. If `from` is not provided, automatically selects the address only when there is exactly one account to avoid ambiguity. If multiple accounts exist, explicit selection is required.

## Returns

[`UseSendUsdcReturnType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/UseSendUsdcReturnType) UseSendUsdcReturnType

## Example

```
import { useSendUsdc } from "@coinbase/cdp-hooks";
function SendUsdcComponent() {
  const { sendUsdc, data, error, status } = useSendUsdc();
  // Simplest usage - auto-selects sender address
  const handleSendSimple = async () => {
    try {
      const result = await sendUsdc({
        to: "0x1234567890123456789012345678901234567890",
        amount: "10.00",
        network: "base-sepolia",
      });
      console.log("Sent!", result);
    } catch (error) {
      console.error("Failed to send USDC:", error);
    }
  };
  // Or explicitly specify sender
  const handleSendExplicit = async () => {
    try {
      const result = await sendUsdc({
        from: "0x1234567890123456789012345678901234567890",
        to: "0x9876543210987654321098765432109876543210",
        amount: "10.00",
        network: "base-sepolia",
      });
      if (result.type === "evm-eoa") {
        console.log("EOA TX:", result.transactionHash);
      } else if (result.type === "evm-smart") {
        console.log("Smart Account UserOp:", result.userOpHash);
      } else {
        console.log("Solana TX:", result.transactionSignature);
      }
    } catch (error) {
      console.error("Failed to send USDC:", error);
    }
  };
  return (
    <div>
      <button onClick={handleSendSimple} disabled={status === "pending"}>
        {status === "pending" ? "Sending..." : "Send USDC (Auto)"}
      </button>
      <button onClick={handleSendExplicit} disabled={status === "pending"}>
        {status === "pending" ? "Sending..." : "Send USDC (Explicit)"}
      </button>
      {status === "success" && data && <p>USDC sent successfully!</p>}
      {error && <p>Error: {error.message}</p>}
    </div>
  );
}

```