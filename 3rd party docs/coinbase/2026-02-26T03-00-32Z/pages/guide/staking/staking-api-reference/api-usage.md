# api usage

## Authentication

Our REST APIs use JWT tokens for authentication. Find more information on generating a JWT token in the language of your choice [here](https://developer.chrome.com/get-started/authentication/jwt-authentication#code-samples).

## Making an API request

1.  Click through any API in the reference [here](https://developer.chrome.com/api-reference/rest-api/staking/build-a-new-staking-operation) to learn more about its parameters and usage.
2.  Fill in the parameters. For APIs like [BuildStakingOperation](https://developer.chrome.com/api-reference/rest-api/staking/build-a-new-staking-operation) and [GetStakingContext](https://developer.chrome.com/api-reference/rest-api/staking/get-staking-context) that require custom options, refer to the [staking options](#staking-options) section below for available options.
3.  Select your preferred language on the right to generate a sample request code. Use this as a reference to make a staking API request. For example, for the `BuildStakingOperation` API, the generated code for a Partial ETH stake of `0.1 ETH` will look like this in some of the common languages:
4.  Finally, add the JWT token, obtained from the authentication section [above](#authentication), as a Bearer header in your request. The code will look like this:

* * *

## Staking Options

Some staking APIs such as [BuildStakingOperation](https://developer.chrome.com/api-reference/rest-api/staking/build-a-new-staking-operation) and [GetStakingContext](https://developer.chrome.com/api-reference/rest-api/staking/get-staking-context) require additional options to be passed in the request body specific to the staking network. See below to find the options available for each staking network. Once you have identified which options you need, you can add them to the request body under the `options` field like so:

```
{
  ...
  "options": {
    "mode": "partial",
    "amount": "100000000000000000",
    "integrator_contract_address": "custom-integrator-address"
  }
}

```

### Shared ETH Staking

Shared ETH staking supports the following `BuildStakingOperation` actions: `stake`, `unstake` and `claim_stake`. See the tabs below for details on the options that can be used with each one.

-   Stake
    
-   Unstake
    
-   Claim Stake
    
-   Get Staking Context
    

Field Name

Description

mode  
*required*

The mode of staking.  
For Shared ETH Staking this should be `partial`.

amount  
*required*

The amount to stake in `wei`.

integrator\_contract\_address  
*optional*

The contract address for the staking operation.  
Defaults to the integrator contract address associated with the CDP account or a [shared integrator contract address](https://developer.chrome.com/staking/staking-api/protocols/shared-eth/overview) for that network.

Field Name

Description

mode  
*required*

The mode of staking.  
For Shared ETH Staking this should be `partial`.

amount  
*required*

The amount to unstake in `wei`.

integrator\_contract\_address  
*optional*

The contract address for the staking operation.  
Defaults to the integrator contract address associated with the CDP account or a [shared integrator contract address](https://developer.chrome.com/staking/staking-api/protocols/shared-eth/overview) for that network.

Field Name

Description

mode  
*required*

The mode of staking.  
For Shared ETH Staking this should be `partial`.

integrator\_contract\_address  
*optional*

The contract address for the staking operation.  
Defaults to the integrator contract address associated with the CDP account or a [shared integrator contract address](https://developer.chrome.com/staking/staking-api/protocols/shared-eth/overview) for that network.

Field Name

Description

mode  
*required*

The mode of staking.  
For Shared ETH Staking this should be `partial`.

integrator\_contract\_address  
*optional*

The contract address for the staking operation.  
Defaults to the integrator contract address associated with the CDP account or a [shared integrator contract address](https://developer.chrome.com/staking/staking-api/protocols/shared-eth/overview) for that network.

### Dedicated ETH Staking

Dedicated ETH staking supports the following `BuildStakingOperation` actions: `stake` and `unstake`. See the tabs below for details on the options that can be used with each one.

-   Stake
    
-   Unstake
    
-   Consolidate
    
-   Get Staking Context
    

Field Name

Description

mode  
*required*

The mode of staking.  
For Dedicated ETH Staking this should be `native`.

amount  
*required*

The amount to stake in `wei` and in `multiples of 32 ETH`.

funding\_address  
*optional*

Funding address for the stake operation.  
Defaults to the address initiating the stake operation.

withdrawal\_address  
*optional*

Rewards and withdrawal address.  
Defaults to the address initiating the stake operation.

fee\_recipient\_address  
*optional*

Tx fee recipient address.  
Defaults to the address initiating the stake operation.

withdrawal\_credential\_type  
*optional*

Prefix indicating the type of withdrawal credentials for the validator.  
Set to `0x02` for provisioning post Pectra validators.  
Possible values: `0x01`, `0x02`  
Defaults to pre Pectra validator prefix of `0x01`.

top\_up\_validator\_pubkey  
*optional*

The validator public key to top up.  
If provided, instead of creating a new validator, the existing validator will be topped up with the specified amount.

Field Name

Description

mode  
*required*

The mode of staking.  
For Dedicated ETH Staking this should be `native`.

amount  
*required*

The amount to unstake in `wei` and in `multiples of 32 ETH`.

unstake\_type  
*optional*

The type of unstaking operation to perform.  
Possible values: `consensus`, `execution`  
Defaults to `consensus`.

immediate  
*optional*

Set to `true` for immediate unstake using `Coinbase managed unstake` process.  
Defaults to `false` for `User managed unstake` process.

validator\_pub\_keys  
*optional*

Comma-separated list of validator public keys to unstake.  
Defaults to validators selected based on the unstake amount.

Field Name

Description

source\_validator\_pubkey  
*required*

The source validator public key to consolidate. This can be either a 0x01 or 0x02 validator.

target\_validator\_pubkey  
*required*

The target validator public key to which the source validator will be consolidated into. This can be either a 0x01 or 0x02 validator.

Field Name

Description

mode  
*required*

The mode of staking.  
For Dedicated ETH Staking this should be `native`.

validator\_pub\_keys  
*optional*

List of comma separated validator public keys to retrieve unstakeable balance for.  
Defaults to all validators.

withdrawal\_credential\_type  
*optional*

Prefix indicating the type of validator for which we want to get the context.  
Set to `0x02` for post Pectra validators.  
Possible values: `0x01`, `0x02`  
Defaults to pre Pectra validator prefix of `0x01`.

### SOL Staking

SOL staking supports the following `BuildStakingOperation` actions: `stake`, `unstake` and `claim_stake`. See the tabs below for details on the options that can be used with each one.

-   Stake
    
-   Unstake
    

Field Name

Description

amount  
*required*

The amount to stake in `lamports`.

validator\_address  
*optional*

The validator address to which you want to stake.  
Defaults to the Coinbase Solana validator. See [here](https://developer.chrome.com/staking/staking-api/protocols/sol/overview#validator-details) for validator details.

Field Name

Description

amount  
*required*

The amount to unstake in `lamports`.