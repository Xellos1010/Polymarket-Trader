# cdpwalletaccount

CDP Wallet Account implementation for Solana accounts. This class represents a Solana account within the CDP wallet ecosystem.

## Implements

-   `WalletAccount`

## Accessors

### address

#### Get Signature

Get the base58 encoded address of this account.

##### Returns

`string` The base58 encoded Solana address

#### Implementation of

* * *

### publicKey

#### Get Signature

```
get publicKey(): Uint8Array<ArrayBuffer>;

```

Get the public key bytes of this account.

##### Returns

`Uint8Array`<`ArrayBuffer`\> A copy of the public key bytes

#### Implementation of

* * *

### chains

#### Get Signature

```
get chains(): readonly ["solana:mainnet", "solana:devnet"];

```

Get the supported chains for this account.

##### Returns

readonly \[`"solana:mainnet"`, `"solana:devnet"`\] Array of supported Solana chains

#### Implementation of

* * *

### features

#### Get Signature

```
get features(): readonly ["solana:signAndSendTransaction", "solana:signTransaction", "solana:signMessage"];

```

Get the supported features for this account.

##### Returns

readonly \[`"solana:signAndSendTransaction"`, `"solana:signTransaction"`, `"solana:signMessage"`\] Array of supported wallet standard features

#### Implementation of

## Constructors

### Constructor

```
new CdpWalletAccount(publicKey: Uint8Array): CdpWalletAccount;

```

Create a new CDP Wallet Account.

#### Parameters

Parameter

Type

Description

`publicKey`

`Uint8Array`

The public key bytes for this account (must be exactly 32 bytes for Solana)

#### Returns

`CdpWalletAccount`

#### Throws

If publicKey is null, undefined, or not exactly 32 bytes