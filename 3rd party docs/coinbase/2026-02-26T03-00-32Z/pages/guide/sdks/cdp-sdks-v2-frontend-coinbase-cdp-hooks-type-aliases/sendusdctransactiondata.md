# sendusdctransactiondata

```
type SendUsdcTransactionData = 
  | {
  type: "evm-eoa";
  transactionHash: Hex;
  receipt?: TransactionReceipt;
}
  | {
  type: "evm-smart";
  userOpHash: Hex;
  transactionHash?: Hex;
  receipts?: UserOperationReceipt[];
  userOperation?: GetUserOperationResult;
}
  | {
  type: "solana";
  transactionSignature: string;
};

```

Represents the data from a USDC transaction across all account types. Uses a discriminated union to properly represent different transaction types:

-   EOA: transactionHash + optional TransactionReceipt (populated when confirmed)
-   Smart Account: userOpHash + optional transactionHash + UserOperationReceipt\[\]
-   Solana: transactionSignature