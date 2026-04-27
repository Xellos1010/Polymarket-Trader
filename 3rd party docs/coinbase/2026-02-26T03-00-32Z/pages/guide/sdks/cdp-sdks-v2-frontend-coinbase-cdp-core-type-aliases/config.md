# config

```
type Config = BaseConfig & {
  ethereum?: NetworkConfig["ethereum"];
  solana?: NetworkConfig["solana"];
};

```

Configuration for the core package with at least one network specified.

## 

[​](#type-declaration)

Type declaration

### 

[​](#ethereum)

ethereum?

```
optional ethereum: NetworkConfig["ethereum"];

```

Ethereum/EVM account configuration.

### 

[​](#solana)

solana?

```
optional solana: NetworkConfig["solana"];

```

Solana account configuration.