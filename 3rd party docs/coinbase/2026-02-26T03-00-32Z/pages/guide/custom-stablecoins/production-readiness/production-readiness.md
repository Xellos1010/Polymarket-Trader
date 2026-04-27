# production readiness

Everything you need for production swaps.

## Best practices

### Token decimals

The program supports tokens with **6 to 9 decimals**. Amounts are automatically normalized during swaps so equivalent values are preserved across different decimal precisions.

```
// Example: expressing 100 tokens for a 6-decimal token
const amount = 100 * Math.pow(10, 6); // = 100_000_000
// Using anchor.BN for large numbers
const amountBN = new anchor.BN(100 * 10 ** 6);

```

### Slippage protection

Always set `min_amount_out` to protect against fee rate changes between when you construct the transaction and when it executes.

```
// Calculate slippage tolerance
const expectedOutput = new anchor.BN(100 * 10 ** 6);
const slippageTolerance = 0.02; // 2%
const minAmountOut = expectedOutput.muln(1 - slippageTolerance);
// Or use the helper function
const { amountOut, fee } = await calculateSwapOutput(amountIn);
const minAmountOut = amountOut.muln(0.98); // Allow 2% variance

```

### Address whitelist

**For smart contract integrations via CPI (Cross-Program Invocation):** The **PDA address** (not its token accounts) must be whitelisted. Ensure the PDA has sufficient SOL to cover potential account creation fees when calling via `invoke_signed`.

* * *

## Security considerations

Always verify addresses

1.  **Verify the Program ID** before sending transactions — use the address from the [Key Addresses](https://developer.chrome.com/custom-stablecoins/key-addresses) page
2.  **Double-check token mint addresses** — confirm both `fromMint` and `toMint` against the Key Addresses page
3.  **Validate user inputs** — sanitize and bounds-check all amounts and addresses

Test thoroughly

1.  **Test on devnet first** with small amounts
2.  **Run integration tests** covering success and failure cases
3.  **Test edge cases** like zero amounts, maximum amounts, and missing accounts

Handle failures gracefully

1.  **Never assume success** — always check transaction results
2.  **Provide clear error messages** to users
3.  **Implement retry logic** for network failures (but not for validation errors)

Monitor transactions

1.  **Log transaction signatures** for debugging and support
2.  **Track failed transactions** to identify patterns
3.  **Monitor pool liquidity** to warn users before failures

* * *

## Pre-flight checklist

Before sending a swap on mainnet, verify:

-   User has sufficient input token balance
-   User has sufficient SOL for transaction fees (0.000005 SOL typical, plus ~0.002 SOL if creating a new token account)
-   Destination token account exists, or ATA creation is prepended to the transaction
-   Pool has sufficient liquidity for the swap amount
-   User wallet is whitelisted (if whitelist is enabled)
-   Slippage tolerance accounts for current fee rate
-   Program ID and mint addresses match the Key Addresses page

* * *

## What to read next