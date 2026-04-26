# layer 2 networks

A Layer 2 (L2) is a blockchain that extends the Ethereum blockchain and inherits the security guarantees of Ethereum. See [What is layer 2](https://ethereum.org/en/layer-2/) in the Ethereum documentation. To date, [coinbase.com](https://www.coinbase.com/) has sent assets on each asset’s default network; for example, AVAX is sent on Avalanche C Chain, ETH is sent on Ethereum. With L2s, the user can control the network on which to send an asset, and may benefit from cheaper gas fees on some networks.

## Available Assets

The following assets are available for L2 Sends:

Asset

Networks

DAI

Ethereum / Avalanche C-Chain / Optimism / Arbitrum

ETH

Ethereum / Base / Polygon / Avalanche C-Chain / Optimism / Arbitrum

MATIC

Ethereum / Polygon

USDC

Ethereum / Base / Polygon / Solana / Optimism / Avalanche C-Chain

wBTC

Ethereum / Avalanche C-Chain / Optimism / Arbitrum

## Sample Implementations

This section includes multiple examples of L2 assets that are currently available for testing.

### (1) One asset, one network

##### Sample implementation #1: Enable an asset on a single network

This implementation lets your users buy and send `ETH` on Ethereum network only (even though Coinbase Onramp supports additional networks for `ETH`).

```
// Enable an asset on a single network only
addresses: { 
  “0x1”: ["ethereum"]
},
assets: [“ETH”],

```

### (2) One asset, many networks

##### Sample implementation #2: Enable an asset on multiple networks

This implementation lets your users buy and send `USDC` on multiple supported networks — Ethereum, Polygon and Solana. To enable `USDC` on the Solana network, you must pass in a Solana formatted destination address.

```
addresses: {
  "0x1": [“ethereum”, “polygon”],
  "1a2": ["solana"],
},
assets: ["USDC"],

```

### (3) One asset, many networks (with default)

##### Sample implementation #3: Enable an asset on multiple networks, with default network

This implementation is like #2 except that Coinbase Onramp selects Solana as the default destination address. Users have the option to update the destination network to Ethereum or Polygon.

```
addresses: {
  "0x1": [“ethereum”, “polygon”],
  "1a2": ["solana"],
},
assets: ["USDC"],
defaultNetwork: “solana”,

```

### (4) Many assets, one network

##### Sample implementation #4: Enable multiple assets on a single network

This implementation let users send funds on the Ethereum network only. You can select all `ETH` and `ERC20` tokens that Coinbase Onramp supports for Ethereum.

```
addresses: { 
  "0x1": [“ethereum”]
},
assets: [“ETH”,“USDC”,“MATIC”],

```

### (5) Many assets, many networks

##### Sample implementation #5: Enable multiple assets on multiple networks

Each asset in the assets parameter will be available on any network included in the addresses parameter that supports that asset.

```
addresses: {
  "0x1": [“ethereum”, “polygon”],
  "1a2": ["solana"],
},
assets: ["USDC","SOL","AVAX","ETH","MATIC"],

```

### (6) All assets, one network

##### Sample implementation #6: Enable all assets on a single network

This implementation lets you offer all supported assets on Ethereum to your users. Because Coinbase is constantly adding support for more assets (including `ERC20` tokens), this implementation lets you make newly supported assets immediately available to your users.

```
addresses: {
  "0x1": [“ethereum”],
},

```

### (7) All assets, many networks

##### Sample implementation #7: Enable all assets on multiple networks

```
addresses: {
  "0x1": [“ethereum”,"polygon","avachain"],
},

```