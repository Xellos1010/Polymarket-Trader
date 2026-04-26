# walletsecret

##### SDKs

-   [
    
    Overview
    
    
    
    ](https://developer.chrome.com/sdks)

##### BUILD ONCHAIN

-   -   -   [
            
            Overview
            
            
            
            ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend)
        -   [
            
            @coinbase/create-cdp-app
            
            
            
            ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/create-cdp-app)
        -   -   [
                
                Overview
                
                
                
                ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core)
            
            -   -   [
                    
                    AuthState
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/AuthState)
                -   [
                    
                    BuildKeyExportIframeUrlOptions
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/BuildKeyExportIframeUrlOptions)
                -   [
                    
                    CreateKeyExportIframeOptions
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeOptions)
                -   [
                    
                    CreateKeyExportIframeResult
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeResult)
                -   [
                    
                    EIP712Domain
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EIP712Domain)
                -   [
                    
                    EIP712TypedData
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EIP712TypedData)
                -   [
                    
                    EIP712Types
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EIP712Types)
                -   [
                    
                    EndUserEvmAccount
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserEvmAccount)
                -   [
                    
                    EndUserEvmSmartAccount
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserEvmSmartAccount)
                -   [
                    
                    EndUserSolanaAccount
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserSolanaAccount)
                -   [
                    
                    IAuthManager
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager)
                -   [
                    
                    IOAuthManager
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IOAuthManager)
                -   [
                    
                    MFAMethods
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/MFAMethods)
                -   [
                    
                    OAuthFlowState
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/OAuthFlowState)
                -   [
                    
                    UserOperationReceipt
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/UserOperationReceipt)
                -   [
                    
                    WalletSecret
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/WalletSecret)
            
            -   [
                
                Reference
                
                
                
                ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/reference)
        

-   [
    
    MiniKit
    
    
    
    ](https://docs.base.org/builderkits/minikit/overview)

##### CONSUMER APIs

##### INSTITUTIONAL APIs

-   [Properties](#properties)

Interfaces

A temporary wallet secret.

## 

[​](#properties)

Properties

Property

Type

Description

`walletSecretId`

`string`

The wallet secret ID.

`keyPair`

[`KeyPair`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/KeyPair)

The underlying key pair.

`expiresAt`

`number`

The Unix timestamp, in milliseconds, of the expiration date of the wallet secret.

Was this page helpful?

[

UserOperationReceipt

Previous

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/UserOperationReceipt)[

APIErrorType

Next

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/APIErrorType)

⌘I