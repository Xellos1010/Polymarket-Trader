# quickstart

This quickstart shows how to sponsor gas for transactions using **CDP Embedded Wallets** with smart accounts on Base. By the end, you’ll have a working example of gasless transactions.

## Prerequisites

-   A [CDP account](https://portal.cdp.coinbase.com/) with an active project
-   Node.js 22+ and npm/pnpm/yarn
-   Basic familiarity with React and TypeScript
-   Your app domain [allowlisted](https://developer.chrome.com/embedded-wallets/domains) in CDP Portal

## Step 1: Configure Paymaster in CDP Portal

1.  [Sign in](https://portal.cdp.coinbase.com/) to your CDP account
2.  Navigate to [Paymaster](https://portal.cdp.coinbase.com/products/bundler-and-paymaster) under **Onchain Tools** in the left-nav
3.  Select **Base Sepolia** in the network dropdown (top right of the configuration panel)
4.  Under **Configuration**, add any contracts you want to sponsor to the allowlist

## Step 2: Set Up CDP Embedded Wallet with Smart Accounts

Install the CDP SDK packages:

```
npm install @coinbase/cdp-core @coinbase/cdp-hooks

```

Configure the provider to create smart accounts for new users:

```
import { CDPHooksProvider } from "@coinbase/cdp-hooks";
function App() {
  return (
    <CDPHooksProvider
      config={{
        projectId: "your-project-id",
        ethereum: {
          createOnLogin: "smart", // Creates smart accounts for AA features
        }
      }}
    >
      <YourApp />
    </CDPHooksProvider>
  );
}

```

Use `useSendUserOperation` with `useCdpPaymaster: true`. This is the recommended approach — it handles Paymaster integration securely without exposing your endpoint URL:

```
import { useSendUserOperation, useCurrentUser } from "@coinbase/cdp-hooks";
import { encodeFunctionData } from "viem";
// Example: ERC-20 transfer ABI
const erc20Abi = [
  {
    name: "transfer",
    type: "function",
    stateMutability: "nonpayable",
    inputs: [
      { name: "to", type: "address" },
      { name: "value", type: "uint256" },
    ],
    outputs: [{ name: "", type: "bool" }],
  },
] as const;
function SponsoredTransaction() {
  const { sendUserOperation, status, data, error } = useSendUserOperation();
  const { currentUser } = useCurrentUser();
  const handleSend = async () => {
    const smartAccount = currentUser?.evmSmartAccounts?.[0];
    if (!smartAccount) return;
    const result = await sendUserOperation({
      evmSmartAccount: smartAccount,
      network: "base-sepolia",
      calls: [{
        to: "0x036CbD53842c5426634e7929541eC2318f3dCF7e", // USDC on Base Sepolia
        value: 0n,
        data: encodeFunctionData({
          abi: erc20Abi,
          functionName: "transfer",
          args: ["0xRecipientAddress", 1_000_000n], // 1 USDC (6 decimals)
        }),
      }],
      // Use CDP's built-in Paymaster (Base only)
      useCdpPaymaster: true,
    });
    console.log("Transaction hash:", result.transactionHash);
  };
  return (
    <div>
      <button onClick={handleSend} disabled={status === "pending"}>
        {status === "pending" ? "Sending..." : "Send Sponsored Transaction"}
      </button>
      
      {status === "success" && data && (
        <p>Success! Tx: {data.transactionHash}</p>
      )}
      
      {status === "error" && (
        <p>Error: {error?.message}</p>
      )}
    </div>
  );
}

```

## Step 4: Verify It Works

1.  Start your development server
2.  Sign in to create a smart account
3.  Click the button to send a sponsored transaction
4.  Check the [Paymaster Logs](https://portal.cdp.coinbase.com/products/bundler-and-paymaster) in CDP Portal to see your sponsored transaction

## Batch Multiple Calls

Smart accounts can execute multiple calls atomically in a single user operation:

```
const result = await sendUserOperation({
  evmSmartAccount: smartAccount,
  network: "base-sepolia",
  calls: [
    { to: contractA, value: 0n, data: callDataA },
    { to: contractB, value: 0n, data: callDataB },
    { to: contractC, value: 0n, data: callDataC },
  ],
  useCdpPaymaster: true,
});

```

All calls execute in order. If any call fails, the entire operation reverts.

## Next Steps

-   **[Security](https://developer.chrome.com/paymaster/reference-troubleshooting/security)** — Learn about protecting your Paymaster endpoint
-   **[FAQs](https://developer.chrome.com/paymaster/faqs)** — Common questions about billing, Account Abstraction, and EIP-7702
-   **[Paymaster Proxy Guide](https://developer.chrome.com/paymaster/guides/paymaster-proxy)** — Detailed guide on implementing a production proxy
-   **[Examples Repository](https://github.com/coinbase/paymaster-bundler-examples)** — More integration examples with various SDKs