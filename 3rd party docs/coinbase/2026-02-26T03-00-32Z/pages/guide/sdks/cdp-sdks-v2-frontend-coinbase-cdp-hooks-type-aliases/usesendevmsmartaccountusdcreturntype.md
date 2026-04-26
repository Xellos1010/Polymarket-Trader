# usesendevmsmartaccountusdcreturntype

```
type UseSendEvmSmartAccountUsdcReturnType = {
  sendEvmSmartAccountUsdc: (options: SendEvmSmartAccountUsdcOptions) => Promise<SendEvmSmartAccountUsdcResult>;
} & UseWaitForUserOperationReturnType;

```

Represents the return type of the `useSendEvmSmartAccountUsdc` hook. Extends UseWaitForUserOperationReturnType with a send function.

## Type declaration

### sendEvmSmartAccountUsdc()

```
sendEvmSmartAccountUsdc: (options: SendEvmSmartAccountUsdcOptions) => Promise<SendEvmSmartAccountUsdcResult>;

```

#### Parameters

Parameter

Type

`options`

`SendEvmSmartAccountUsdcOptions`

#### Returns

`Promise`<`SendEvmSmartAccountUsdcResult`\>