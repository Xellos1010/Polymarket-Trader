# troubleshooting

This guide covers common issues when sending sponsored userOperations with CDP Paymaster and how to debug them.

## Transaction Rejected by Policy

**Symptom:** Your userOperation is rejected before reaching the chain. **Common causes:**

-   Contract address not in your allowlist
-   Function not allowlisted for the contract
-   Per-user or global spend limits exceeded
-   Sender address doesn’t have required attestation

**Solution:** Check your [Paymaster Configuration](https://portal.cdp.coinbase.com/products/bundler-and-paymaster) in CDP Portal:

1.  Verify the contract address is in your allowlist
2.  Confirm the specific function you’re calling is allowed
3.  Review your spend limits in the Configuration tab

## Execution Reverted

**Symptom:** The userOperation made it onchain but the transaction reverted. This means the smart contract execution failed. Common causes:

Cause

Solution

Insufficient gas

Increase `callGasLimit` or `preVerificationGas`

Invalid callData

Verify your encoded function call is correct

Contract logic error

Debug your contract with Tenderly

Insufficient token balance

Ensure the account has required tokens

### Debugging with Tenderly

[Tenderly](https://dashboard.tenderly.co/) is useful for simulating and debugging userOperations. **To debug gas issues:**

1.  Go to Tenderly and select the Entrypoint contract: `0x5ff137d4b0fdcd49dca30c7cf57e578a026d2789`
2.  Use the `simulateHandleOp` function
3.  Paste your userOperation in the `op` field (wrap it in array brackets `[{...}]`)

**To debug contract logic:**

1.  Enter your smart contract’s address
2.  Paste the `callData` from your userOperation
3.  Run the simulation to see where it fails

### Decoding Revert Data

If you see an error like:

```
{
  "code": -32004,
  "message": "execution reverted with data",
  "data": "0xed6c3dec00000000000000000000000036e53f56454e1206f775dafe2b33c1b737c43632"
}

```

Use an [ABI decoder](https://bia.is/tools/abi-decoder/) to decode the error:

1.  Upload your contract’s ABI
2.  Paste the `data` field
3.  The decoder will show the specific error and parameters

## Invalid Chain ID

**Symptom:** Request rejected with chain ID mismatch error. **Cause:** You’re using a Paymaster URL for a different network than your transaction targets. **Solution:** In [CDP Portal](https://portal.cdp.coinbase.com/products/bundler-and-paymaster), check the network dropdown in the top right corner and ensure it matches your transaction’s chain:

-   **Base Mainnet** — Chain ID `8453`
-   **Base Sepolia** — Chain ID `84532`

## Invalid Signature

**Symptom:** Error message about account signature or paymaster signature check failing.

```
UserOperation rejected because account signature check failed 
(or paymaster signature, if the paymaster uses its data as signature).

```

**Cause:** The userOperation was modified after the Paymaster signed it. The Paymaster signs the complete userOperation. If you change any field after calling `getPaymasterData()`, the signature becomes invalid. **Solution:**

1.  Finalize all userOperation fields (`callData`, gas limits, etc.) **before** requesting the Paymaster signature
2.  Don’t modify the userOperation after receiving `paymasterAndData`
3.  If you need to change something, request a new Paymaster signature

**Symptom:** Transaction goes through but user paid gas instead of sponsorship. **Common causes:**

-   Wallet doesn’t support paymaster capabilities
-   `paymasterService` capability not passed correctly
-   Using an EOA instead of a smart account

**Solution:**

1.  Verify you’re using a smart account (check with `eth_getCode` — should return non-empty)
2.  Confirm the wallet supports `paymasterService` capability
3.  Check that you’re passing the paymaster URL in the correct format for your SDK

## Still Stuck?

If you’ve tried the above and are still having issues:

1.  **Check the Logs tab** in [CDP Portal](https://portal.cdp.coinbase.com/products/bundler-and-paymaster) for detailed request/response data
2.  **Export your userOp logs** for offline analysis
3.  **Reach out in [CDP Discord](https://discord.com/invite/cdp)** in the `#paymaster` channel — include your error message, userOperation details, and what you’ve tried