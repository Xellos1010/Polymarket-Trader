# create a swap quote

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
  "permit2": {  
    "hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",  
    "eip712": {  
      "domain": {  
        "name": "Permit2",  
        "chainId": 1,  
        "verifyingContract": "0x000000000022D473030F116dDEE9F6B43aC78BA3"  
      },  
      "types": {  
        "EIP712Domain": [  
          {  
            "name": "name",  
            "type": "string"  
          },  
          {  
            "name": "chainId",  
            "type": "uint256"  
          },  
          {  
            "name": "verifyingContract",  
            "type": "address"  
          }  
        ],  
        "PermitTransferFrom": [  
          {  
            "name": "permitted",  
            "type": "TokenPermissions"  
          },  
          {  
            "name": "spender",  
            "type": "address"  
          },  
          {  
            "name": "nonce",  
            "type": "uint256"  
          },  
          {  
            "name": "deadline",  
            "type": "uint256"  
          }  
        ],  
        "TokenPermissions": [  
          {  
            "name": "token",  
            "type": "address"  
          },  
          {  
            "name": "amount",  
            "type": "uint256"  
          }  
        ]  
      },  
      "primaryType": "PermitTransferFrom",  
      "message": {  
        "permitted": {  
          "token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",  
          "amount": "1000000"  
        },  
        "spender": "0xFfFfFfFFfFFfFFfFFfFFFFFffFFFffffFfFFFfFf",  
        "nonce": "123456",  
        "deadline": "1717123200"  
      }  
    }  
  },  
  "transaction": {  
    "to": "0x000000000022D473030F116dDEE9F6B43aC78BA3",  
    "data": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",  
    "gas": "100000",  
    "gasPrice": "1000000000",  
    "value": "1000000000000000000"  
  }  
}
```