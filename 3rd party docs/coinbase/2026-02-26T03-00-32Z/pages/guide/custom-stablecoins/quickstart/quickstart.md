# quickstart

Get up and running with Stableswapper, the onchain liquidity program that powers instant swaps between USDC and custom stablecoins. This guide walks through swapping USDC for a custom stablecoin on Solana devnet. Learn more about [Custom Stablecoins](https://developer.chrome.com/custom-stablecoins/overview).

## Prerequisites

-   Node.js 18+ installed
-   A Solana wallet keypair file
-   Devnet SOL - at least 0.05 SOL recommended for transaction fees and rent (get from [CDP Faucet](https://developer.chrome.com/faucets/introduction/quickstart))
-   Devnet USDC tokens (get from [CDP Faucet](https://developer.chrome.com/faucets/introduction/quickstart))
-   A custom stablecoin to swap with (this guide uses SFTUSD25, a test token on devnet)
-   Basic familiarity with TypeScript

Don't have a Solana wallet?

```
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
# Create wallet keypair
solana-keygen new --outfile ~/.config/solana/id.json

```

For production, use [CDP Server Wallet v2](https://developer.chrome.com/server-wallets/v2/introduction/quickstart) for secure key management.

## 1\. Create tsconfig.json

```
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "CommonJS",
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "strict": false,
    "noImplicitAny": false
  }
}

```

## 2\. Install dependencies

```
npm install @coral-xyz/anchor @solana/web3.js @solana/spl-token
npm install --save-dev ts-node typescript @types/node

```

## 3\. Set environment variables

```
export ANCHOR_WALLET=~/.config/solana/id.json
export ANCHOR_PROVIDER_URL=https://api.devnet.solana.com

```

What do these environment variables mean?

-   **ANCHOR\_WALLET**: Path to your Solana wallet keypair file
-   **ANCHOR\_PROVIDER\_URL**: Solana RPC endpoint URL

**Available RPC endpoints:**

-   Devnet (testing): `https://api.devnet.solana.com`
-   Mainnet (production): `https://api.mainnet-beta.solana.com`

## 4\. Get the IDL

The **Stableswapper program** is already deployed on Solana at program ID `9vDwZVJXw5nxymWmUcgmNpemDH5EBcJwLNhtsznrgJDH` (see [Key Addresses](https://developer.chrome.com/custom-stablecoins/key-addresses)). To call this program from your code, you can access its instructions and required parameters from the IDL (Interface Definition Language). This guide uses the [Anchor framework](https://www.anchor-lang.com/) to simplify interacting with the program. You can also download the IDL from [Solana Explorer](https://explorer.solana.com/address/9vDwZVJXw5nxymWmUcgmNpemDH5EBcJwLNhtsznrgJDH/idl?cluster=devnet):

1.  Visit the IDL page link above
2.  Click the download button or copy the JSON
3.  Save as `custom_stablecoins.json` in your project directory

## 5\. Create your swap script

Now we’ll create a script that swaps **0.1 USDC** for the custom stablecoin (SFTUSD25). The script will:

-   Connect to the Stableswapper program using the IDL
-   Derive the required program addresses (pool, vaults, token accounts)
-   Build a swap instruction with your parameters
-   Submit the transaction to Solana devnet

```
import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  getAccount,
  createAssociatedTokenAccountInstruction,
} from "@solana/spl-token";
import idl from "./custom_stablecoins.json";
// Devnet addresses - see Key Addresses page for details
const PROGRAM_ID        = new PublicKey("9vDwZVJXw5nxymWmUcgmNpemDH5EBcJwLNhtsznrgJDH");
const USDC_MINT         = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
const CUSTOM_TOKEN_MINT = new PublicKey("AreQsswF44khJHLjsuWzgboJcG31JvALcWMkMcsVs2uC"); // SFTUSD25 testnet token
// Initialize provider
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
// @ts-ignore - IDL type compatibility
const program = new Program(idl, provider);
async function swapUsdcForCustomToken() {
  // Swap 0.1 USDC for custom token
  const swapAmount   = new anchor.BN(0.1 * 10 ** 6);
  const minAmountOut = new anchor.BN(0.09 * 10 ** 6);
  // Derive program addresses
  const [pool] = PublicKey.findProgramAddressSync(
    [Buffer.from("liquidity_pool")],
    PROGRAM_ID
  );
  const [usdcVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("token_vault"), pool.toBuffer(), USDC_MINT.toBuffer()],
    PROGRAM_ID
  );
  const [customTokenVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("token_vault"), pool.toBuffer(), CUSTOM_TOKEN_MINT.toBuffer()],
    PROGRAM_ID
  );
  const [usdcVaultTokenAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault_token_account"), usdcVault.toBuffer()],
    PROGRAM_ID
  );
  const [customTokenVaultTokenAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault_token_account"), customTokenVault.toBuffer()],
    PROGRAM_ID
  );
  const [whitelist] = PublicKey.findProgramAddressSync(
    [Buffer.from("address_whitelist")],
    PROGRAM_ID
  );
  // Get user token accounts
  const userUsdcAccount = await getAssociatedTokenAddress(
    USDC_MINT,
    provider.wallet.publicKey
  );
  const userCustomTokenAccount = await getAssociatedTokenAddress(
    CUSTOM_TOKEN_MINT,
    provider.wallet.publicKey
  );
  // Fetch pool to get fee recipient
  const poolAccount = await (program.account as any).liquidityPool.fetch(pool);
  const feeRecipientUsdcAccount = await getAssociatedTokenAddress(
    USDC_MINT,
    poolAccount.feeRecipient
  );
  // Check if destination account needs to be created
  let needsAccountCreation = false;
  try {
    await getAccount(provider.connection, userCustomTokenAccount);
  } catch {
    needsAccountCreation = true;
  }
  // Build swap instruction
  const swapIx = await program.methods
    .swap(swapAmount, minAmountOut)
    .accounts({
      pool,
      inVault: usdcVault,
      outVault: customTokenVault,
      inVaultTokenAccount: usdcVaultTokenAccount,
      outVaultTokenAccount: customTokenVaultTokenAccount,
      userFromTokenAccount: userUsdcAccount,
      toTokenAccount: userCustomTokenAccount,
      feeRecipientTokenAccount: feeRecipientUsdcAccount,
      feeRecipient: poolAccount.feeRecipient,
      fromMint: USDC_MINT,
      toMint: CUSTOM_TOKEN_MINT,
      user: provider.wallet.publicKey,
      whitelist,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    } as any)
    .instruction();
  // Build transaction
  const transaction = new anchor.web3.Transaction();
  if (needsAccountCreation) {
    transaction.add(
      createAssociatedTokenAccountInstruction(
        provider.wallet.publicKey,
        userCustomTokenAccount,
        provider.wallet.publicKey,
        CUSTOM_TOKEN_MINT
      )
    );
  }
  transaction.add(swapIx);
  // Send transaction
  try {
    const tx = await provider.sendAndConfirm(transaction);
    console.log("✅ Swap successful!");
    console.log("Transaction signature:", tx);
  } catch (error: any) {
    console.error("❌ Swap failed:", error.message);
    throw error;
  }
}
swapUsdcForCustomToken()
  .then(() => process.exit(0))
  .catch((err) => { console.error(err); process.exit(1); });

```

## 6\. Run your swap

## What’s next?