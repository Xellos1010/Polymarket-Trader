# get a price estimate for a swap

Get a price estimate for a swap

```
{  
  "blockNumber": "17038723",  
  "toAmount": "1000000000000000000",  
  "toToken": "0x7F5c764cBc14f9669B88837ca1490cCa17c31607",  
  "fees": {  
    "gasFee": {  
      "amount": "1000000000000000000",  
      "token": "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"  
    },  
    "protocolFee": {  
      "amount": "1000000000000000000",  
      "token": "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"  
    }  
  },  
  "issues": {  
    "allowance": {  
      "currentAllowance": "1000000000",  
      "spender": "0x000000000022D473030F116dDEE9F6B43aC78BA3"  
    },  
    "balance": {  
      "token": "0x6B175474E89094C44Da98b954EedeAC495271d0F",  
      "currentBalance": "1000000000000000000",  
      "requiredBalance": "1000000000000000000"  
    },  
    "simulationIncomplete": false  
  },  
  "liquidityAvailable": true,  
  "minToAmount": "900000000000000000",  
  "fromAmount": "1000000000000000000",  
  "fromToken": "0x6B175474E89094C44Da98b954EedeAC495271d0F",  
  "gas": "100000",  
  "gasPrice": "1000000000"  
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Query Parameters

The network on which to perform the swap.

Available options

:

`base`,

`ethereum`,

`arbitrum`,

`optimism`

The 0x-prefixed contract address of the token to receive.

Example:

`"0x7F5c764cBc14f9669B88837ca1490cCa17c31607"`

The 0x-prefixed contract address of the token to send.

Example:

`"0x6B175474E89094C44Da98b954EedeAC495271d0F"`

The amount of the `fromToken` to send in atomic units of the token. For example, `1000000000000000000` when sending ETH equates to 1 ETH, `1000000` when sending USDC equates to 1 USDC, etc.

Example:

`"1000000000000000000"`

The 0x-prefixed address that holds the `fromToken` balance and has the `Permit2` allowance set for the swap.

Example:

`"0xAc0974bec39a17e36ba4a6b4d238ff944bacb478"`

The 0x-prefixed Externally Owned Account (EOA) address that will sign the `Permit2` EIP-712 permit message. This is only needed if `taker` is a smart contract.

Example:

`"0x922f49447d8a07e3bd95bd0d56f35241523fbab8"`

The target gas price for the swap transaction, in Wei. For EIP-1559 transactions, this value should be seen as the `maxFeePerGas` value. If not provided, the API will use an estimate based on the current network conditions.

The maximum acceptable slippage of the `toToken` in basis points. If this parameter is set to 0, no slippage will be tolerated. If not provided, the default slippage tolerance is 100 bps (i.e., 1%).

Required range

: `0 <= x <= 10000`

#### Response

A price estimate for the swap.

-   GetSwapPriceResponse
    
-   SwapUnavailableResponse
    

A wrapper for the response of a swap price operation.

The block number at which the liquidity conditions were examined.

The amount of the `toToken` that will be received in atomic units of the `toToken`. For example, `1000000000000000000` when receiving ETH equates to 1 ETH, `1000000` when receiving USDC equates to 1 USDC, etc.

Example:

`"1000000000000000000"`

The 0x-prefixed contract address of the token that will be received.

Example:

`"0x7F5c764cBc14f9669B88837ca1490cCa17c31607"`

The estimated fees for the swap.

Example:

```
{  
  "gasFee": {  
    "amount": "1000000000000000000",  
    "token": "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"  
  },  
  "protocolFee": {  
    "amount": "1000000000000000000",  
    "token": "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"  
  }  
}
```

An object containing potential issues discovered during validation that could prevent the swap from being executed successfully.

Example:

```
{  
  "allowance": {  
    "currentAllowance": "1000000000",  
    "spender": "0x000000000022D473030F116dDEE9F6B43aC78BA3"  
  },  
  "balance": {  
    "token": "0x6B175474E89094C44Da98b954EedeAC495271d0F",  
    "currentBalance": "900000000000000000",  
    "requiredBalance": "1000000000000000000"  
  },  
  "simulationIncomplete": false  
}
```

Whether sufficient liquidity is available to settle the swap. All other fields in the response will be empty if this is false.

Available options

:

`true`,

`false`

The minimum amount of the `toToken` that must be received for the swap to succeed, in atomic units of the `toToken`. For example, `1000000000000000000` when receiving ETH equates to 1 ETH, `1000000` when receiving USDC equates to 1 USDC, etc. This value is influenced by the `slippageBps` parameter.

Example:

`"900000000000000000"`

The amount of the `fromToken` that will be sent in this swap, in atomic units of the `fromToken`. For example, `1000000000000000000` when sending ETH equates to 1 ETH, `1000000` when sending USDC equates to 1 USDC, etc.

Example:

`"1000000000000000000"`

The 0x-prefixed contract address of the token that will be sent.

Example:

`"0x6B175474E89094C44Da98b954EedeAC495271d0F"`

The estimated gas limit that should be used to send the transaction to guarantee settlement.

The gas price, in Wei, that should be used to send the transaction. For EIP-1559 transactions, this value should be seen as the `maxFeePerGas` value. The transaction should be sent with this gas price to guarantee settlement.