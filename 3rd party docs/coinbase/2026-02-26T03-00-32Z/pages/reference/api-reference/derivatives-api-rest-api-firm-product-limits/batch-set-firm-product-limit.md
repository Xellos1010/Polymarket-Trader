# batch set firm product limit

Batch Set Firm Product Limit

#### Body

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

firm\_product\_limits\_requests

Example:

```
[  
  {  
    "shortDailyPositionLimit": 15000,  
    "longRealPositionLimit": 15000,  
    "tradingDisabled": false,  
    "optionsFillProtectionThreshold": 100,  
    "longDailyPositionLimit": 10000,  
    "shortRealPositionLimit": 20000,  
    "productCode": "BIPZ30"  
  },  
  {  
    "shortDailyPositionLimit": 25000,  
    "longRealPositionLimit": 25000,  
    "tradingDisabled": false,  
    "optionsFillProtectionThreshold": 100,  
    "longDailyPositionLimit": 20000,  
    "shortRealPositionLimit": 30000,  
    "productCode": "BITZ25"  
  }  
]
```

#### Response

Successfully created firm product limit

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

Example:

`"Bitcoin Perpetual Index Futures Dec 2030"`

long\_daily\_position\_limit

short\_daily\_position\_limit

short\_real\_position\_limit