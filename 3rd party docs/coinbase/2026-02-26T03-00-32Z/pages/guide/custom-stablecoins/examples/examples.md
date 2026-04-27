# examples

Code examples for common swap scenarios using the Stableswapper program on Solana devnet.

## Setup

Use these devnet addresses in your examples. For mainnet, replace with your production addresses from [Key Addresses](https://developer.chrome.com/custom-stablecoins/key-addresses).

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
// Devnet addresses
const PROGRAM_ID        = new PublicKey("9vDwZVJXw5nxymWmUcgmNpemDH5EBcJwLNhtsznrgJDH");
const USDC_MINT         = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
const CUSTOM_TOKEN_MINT = new PublicKey("AreQsswF44khJHLjsuWzgboJcG31JvALcWMkMcsVs2uC"); // SFTUSD25
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
// @ts-ignore - IDL type compatibility
const program = new Program(idl, provider);

```

* * *

## Swap USDC for custom token

```
async function swapUsdcForCustomToken() {
  const swapAmount   = new anchor.BN(0.1 * 10 ** 6);   // 0.1 USDC (6 decimals)
  const minAmountOut = new anchor.BN(0.09 * 10 ** 6);  // Allow up to 10% for fees/slippage
  // Derive PDAs
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
  // User token accounts
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
  // Create destination ATA if it doesn't exist
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
  // Optionally prepend ATA creation
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
  const tx = await provider.sendAndConfirm(transaction);
  console.log("Swap successful:", tx);
}

```

* * *

## Swap custom token for USDC

```
async function swapCustomTokenForUsdc() {
  const swapAmount   = new anchor.BN(0.1 * 10 ** 6);   // 0.1 custom tokens (6 decimals)
  const minAmountOut = new anchor.BN(0.09 * 10 ** 6);  // Allow up to 10% for fees/slippage
  // Derive PDAs
  const [pool] = PublicKey.findProgramAddressSync(
    [Buffer.from("liquidity_pool")],
    PROGRAM_ID
  );
  const [customTokenVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("token_vault"), pool.toBuffer(), CUSTOM_TOKEN_MINT.toBuffer()],
    PROGRAM_ID
  );
  const [usdcVault] = PublicKey.findProgramAddressSync(
    [Buffer.from("token_vault"), pool.toBuffer(), USDC_MINT.toBuffer()],
    PROGRAM_ID
  );
  const [customTokenVaultTokenAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault_token_account"), customTokenVault.toBuffer()],
    PROGRAM_ID
  );
  const [usdcVaultTokenAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault_token_account"), usdcVault.toBuffer()],
    PROGRAM_ID
  );
  const [whitelist] = PublicKey.findProgramAddressSync(
    [Buffer.from("address_whitelist")],
    PROGRAM_ID
  );
  // User token accounts
  const userCustomTokenAccount = await getAssociatedTokenAddress(
    CUSTOM_TOKEN_MINT,
    provider.wallet.publicKey
  );
  const userUsdcAccount = await getAssociatedTokenAddress(
    USDC_MINT,
    provider.wallet.publicKey
  );
  // Fetch pool to get fee recipient
  const poolAccount = await (program.account as any).liquidityPool.fetch(pool);
  const feeRecipientCustomTokenAccount = await getAssociatedTokenAddress(
    CUSTOM_TOKEN_MINT,
    poolAccount.feeRecipient
  );
  // Build and send swap
  const swapIx = await program.methods
    .swap(swapAmount, minAmountOut)
    .accounts({
      pool,
      inVault: customTokenVault,
      outVault: usdcVault,
      inVaultTokenAccount: customTokenVaultTokenAccount,
      outVaultTokenAccount: usdcVaultTokenAccount,
      userFromTokenAccount: userCustomTokenAccount,
      toTokenAccount: userUsdcAccount,
      feeRecipientTokenAccount: feeRecipientCustomTokenAccount,
      feeRecipient: poolAccount.feeRecipient,
      fromMint: CUSTOM_TOKEN_MINT,
      toMint: USDC_MINT,
      user: provider.wallet.publicKey,
      whitelist,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    } as any)
    .instruction();
  const transaction = new anchor.web3.Transaction();
  transaction.add(swapIx);
  const tx = await provider.sendAndConfirm(transaction);
  console.log("Swap successful:", tx);
}

```

* * *

## Swap between two custom stablecoins

* * *

## What to read next