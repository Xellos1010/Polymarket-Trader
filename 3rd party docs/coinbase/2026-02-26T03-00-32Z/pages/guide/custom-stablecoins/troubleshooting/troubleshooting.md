# troubleshooting

Common issues and solutions when integrating with Custom Stablecoins swaps.

## Common Errors

### AccountNotInitialized

**Error:** `user_from_token_account` - Account not initialized **Solution:** You need USDC tokens in your wallet. Get devnet USDC from [CDP Faucet](https://developer.chrome.com/faucets/introduction/quickstart).

### Insufficient lamports

**Error:** `insufficient lamports` or `insufficient funds for rent` **Solution:** You need more SOL for transaction fees and rent. Get at least 0.05 SOL from [CDP Faucet](https://developer.chrome.com/faucets/introduction/quickstart).

### InsufficientLiquidity

**Error:** Not enough reserves in the output vault **Solution:** Reduce swap amount or wait for liquidity to be added.

### SlippageExceeded

**Error:** Output is less than `min_amount_out` **Solution:** Increase slippage tolerance or re-fetch current fee rate.

### NotWhitelisted

**Error:** Transaction signer is not on the whitelist **Solution:** Contact support to be added to the whitelist.

* * *

## What to read next