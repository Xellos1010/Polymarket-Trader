# usecreatespendpermission

```
function useCreateSpendPermission(): UseCreateSpendPermissionReturnType;

```

Hook that provides a wrapped function to create spend permissions for EVM Smart Accounts with enhanced developer experience. This hook automatically resolves token addresses, handles periodInDays, and auto-detects the owner Smart Account from the current user.

## Returns

[`UseCreateSpendPermissionReturnType`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/UseCreateSpendPermissionReturnType)

## Example

```
function CreateSpendPermission() {
  const { createSpendPermission } = useCreateSpendPermission();
  const handleCreateSpendPermission = async () => {
    try {
      const result = await createSpendPermission({
        network: "base-sepolia",
        spender: "0x742D35Cc6634C0532925a3b8D6Ec6F1C2b9c1E46",
        token: "usdc", // Automatically resolves to USDC contract address
        allowance: parseUnits("10", 6), // 10 USDC
        periodInDays: 7, // 7 days (automatically converted to seconds)
        useCdpPaymaster: true
      });
      console.log("User Operation Hash:", result.userOperationHash);
    } catch (error) {
      console.error("Failed to create spend permission:", error);
    }
  };
  return (
    <button onClick={handleCreateSpendPermission}>
      Create Spend Permission
    </button>
  );
}

```