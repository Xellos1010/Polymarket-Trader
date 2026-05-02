# config

```
type Config = CoreConfig & {
  transports?: Record<typeof base.id | typeof baseSepolia.id, Transport>;
};

```

The config for the CDP hooks.

## 

[​](#type-declaration)

Type declaration

### 

[​](#transports)

transports?

```
optional transports: Record<typeof base.id | typeof baseSepolia.id, Transport>;

```

## 

[​](#param)

Param

The optional transports to use for the public clients. If not provided, the default `http()` transport is used.

## 

[​](#returns)

Returns

The config for the CDP hooks.