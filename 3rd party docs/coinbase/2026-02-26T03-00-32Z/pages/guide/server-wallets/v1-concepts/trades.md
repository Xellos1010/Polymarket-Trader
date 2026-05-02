# trades

A trade is the act of converting one [asset](https://developer.chrome.com/server-wallets/v1/concepts/assets) into a different [asset](https://developer.chrome.com/server-wallets/v1/concepts/assets). A trade is initiated from a [wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets) or [address](https://developer.chrome.com/server-wallets/v1/concepts/addresses) and the resulting asset is sent back to the same source. To trade from one asset to another, ensure that the address contains sufficient ETH to pay for transaction fees, and sufficient funds of the asset you are trading from. Crypto transactions take varying amounts of time—anywhere from hundreds of milliseconds, to tens of minutes, depending on the blockchain network and wallet set-up. For example, transactions on Bitcoin can take upwards of 30 minutes, while transactions on Base take a second or two. Once your address has ETH in it, we can call the `trade` function to trade `ETH` for `USDC`:

-   Typescript
    
-   Python
    

**SDK Documentation**You can refer to the [Trade class SDK docs](https://coinbase.github.io/coinbase-sdk-nodejs/classes/coinbase_trade.Trade.html) for a full list of supported methods.

```
// Create a wallet on `base-mainnet`
let wallet = Wallet.create({ networkId: Coinbase.networks.BaseMainnet });
// Fund the wallet with ETH on `base-mainnet` from an external source.
// Trade 0.00001 Ether for USDC.
let trade = await wallet.createTrade({
  amount: 0.00001,
  fromAssetId: Coinbase.assets.Eth,
  toAssetId: Coinbase.assets.Usdc
});
// Wait for the trade to settle.
await trade.wait();

```

**SDK Documentation**You can refer to the [Trade class SDK docs](https://coinbase.github.io/cdp-sdk-python/cdp.html#cdp.trade.Trade) for a full list of supported methods.

```
# Create a wallet on `base-mainnet`
wallet = Wallet.create(network_id="base-mainnet")
# Fund the wallet with ETH on `base-mainnet` from an external source.
# Trade 0.00001 Ether for USDC.
trade = wallet.trade(0.00001, "eth", "usdc")
# Wait for the trade to settle.
trade.wait()

```

## Querying Trade Status

You can use `status` to query a crypto transaction’s `status`.

-   Typescript
    
-   Python
    

A common case is to check whether the transaction succeeded or failed.

```
    let status = TransactionStatus.PENDING;
    while (status === TransactionStatus.PENDING) {
      // Do something here.
      // ....
      // Check the status of the trade.
      status = await trade.getStatus();
    }

```

A common case is to check whether the transaction succeeded or failed after calling `wait`.

```
while trade.status is Transaction.Status.PENDING:
  # Do something here.
  # ....
  time.sleep(1)

```