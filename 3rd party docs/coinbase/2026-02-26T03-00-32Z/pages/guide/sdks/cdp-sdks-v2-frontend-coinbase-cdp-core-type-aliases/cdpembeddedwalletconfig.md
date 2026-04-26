# cdpembeddedwalletconfig

```
type CDPEmbeddedWalletConfig<chains> = {
  chains: chains;
  transports: Record<chains[number]["id"], Transport>;
  announceProvider?: boolean;
};

```

Config parameters for the CDP embedded wallet 1193 provider. Note: The transports are currently only used for non-Base transactions. For non-Base transactions, the provider internally signs the transaction via the CDP APIs and broadcasts it via the provided transports, whereas for Base transactions the CDP API both signs and broadcasts the transaction.

## Type Parameters

Type Parameter

Default type

Description

`chains` *extends* readonly \[`Chain`, `...Chain[]`\]

readonly \[`Chain`, `...Chain[]`\]

The chains to support

## Properties

Property

Type

Description

`chains`

`chains`

\-

`transports`

`Record`<`chains`\[`number`\]\[`"id"`\], `Transport`\>

The transports to use for each chain

`announceProvider?`

`boolean`

Whether to announce the provider to the wallet