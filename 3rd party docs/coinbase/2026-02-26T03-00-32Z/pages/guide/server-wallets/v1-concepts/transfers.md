# transfers

A transfer is the act of sending an [asset](https://developer.chrome.com/server-wallets/v1/concepts/assets) from one [wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets) or [address](https://developer.chrome.com/server-wallets/v1/concepts/addresses) to another. To transfer an asset, ensure that the source contains some ETH (by using a [faucet](https://developer.chrome.com/faucets/introduction/welcome) if on testnet, for example). This is required because the network uses ETH to pay for transaction fees. Crypto transactions take varying amounts of time—anywhere from hundreds of milliseconds, to tens of minutes, depending on the blockchain network and wallet set-up. For example, transactions on Bitcoin can take upwards of 30 minutes, while transactions on Base take a second or two. Once your source has ETH in it, call the `transfer` function as follows:

-   Typescript
    
-   Python
    

**SDK Documentation**You can refer to the [Transfer class SDK docs](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_transfer.Transfer.html) for a full list of supported methods.

```
// Transfer 0.00001 Ether to the destination address.
let transfer = await wallet.createTransfer({
  amount: 0.00001,
  assetId: Coinbase.assets.Usdc,
  destination: anotherWallet
});
// Wait for the transfer to settle.
await transfer.wait()

```

**SDK Documentation**You can refer to the [Transfer class SDK docs](https://coinbase.github.io/cdp-sdk-python/cdp.html#cdp.transfer.Transfer) for a full list of supported methods.

```
# Transfer 0.00001 Ether to the destination address.
transfer = wallet.transfer(0.00001, "eth", another_wallet)
# Wait for the transfer to settle.
transfer.wait()

```

## Transfers of arbitrary ERC20 assets

You can transfer ERC20 assets that are not [assets supported by symbol](https://developer.chrome.com/server-wallets/v1/concepts/assets#assets-supported-by-symbol) by using the contract address as the asset ID.

## Gasless Transfers

Coinbase will pay for the gas for transfers of USDC, EURC and cbBTC on Base Mainnet and Base Sepolia! To initiate a USDC transfer on Base Mainnet with gas fees covered, set the `gasless` flag to true.

### Batching

By default, gasless transfers are batched together, which optimizes for high transaction throughput. While batched transfers typically take longer than non-batched ones to process, this approach allows your application to handle many concurrent transactions efficiently. You can significantly reduce processing time by disabling batching with `skipBatching: true`. However, this comes with an important tradeoff:

-   Disabling batching significantly reduces your application’s ability to handle concurrent transactions.

We recommend keeping batching enabled to ensure reliable performance as your transaction volume grows.

## Processing multiple transfers for same address

When creating multiple transfers for the same source address, it is important to create them sequentially instead of all at once. Wait for the previous transfer to have a final state (`COMPLETE` / `FAILED`) before creating a new one. Creating multiple transactions simultaneously can lead to failures due to how nonces are managed by the CDP APIs. An example of how to process transactions sequentially:

## Transfer to ENS or Basenames

[ENS](https://app.ens.domains/) names and [Basenames](https://www.base.org/names) are core building blocks that enable anyone to establish their onchain identity by registering human-readable names for their wallet addresses. CDP SDK supports ENS or Basename as the destination address in your transfers.