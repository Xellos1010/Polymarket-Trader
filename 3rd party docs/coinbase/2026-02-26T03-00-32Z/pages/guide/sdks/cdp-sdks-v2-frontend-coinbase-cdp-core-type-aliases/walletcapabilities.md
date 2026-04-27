# walletcapabilities

```
type WalletCapabilities = {
[chainId: `0x${string}`]: {
  atomic: {
     status: "supported" | "unsupported";
  };
  paymasterService: {
     supported: boolean;
  };
  dataSuffix: {
     supported: boolean;
  };
};
};

```

The capabilities of the wallet by chainId. If the wallet does not support Smart Account operations on a chain, it will not be included in the object.

## Index Signature

```
[chainId: `0x${string}`]: {
  atomic: {
     status: "supported" | "unsupported";
  };
  paymasterService: {
     supported: boolean;
  };
  dataSuffix: {
     supported: boolean;
  };
}

```

## See

[https://eips.ethereum.org/EIPS/eip-5792#wallet\_getcapabilities](https://eips.ethereum.org/EIPS/eip-5792#wallet_getcapabilities)