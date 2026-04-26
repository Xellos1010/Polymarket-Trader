# hash

## 

[​](#functions)

Functions

### 

[​](#authhash)

authHash()

```
function authHash(data: Buffer): Promise<string>;

```

Defined in: [utils/hash.ts:14](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/auth/utils/hash.ts#L14) Auth-specific hash function using uncrypto for Edge runtime compatibility. Computes SHA-256 hash of the given data.

#### 

[​](#parameters)

Parameters

##### data

`Buffer` The data to hash

#### 

[​](#returns)

Returns

`Promise`<`string`\> Promise that resolves to the hex-encoded hash