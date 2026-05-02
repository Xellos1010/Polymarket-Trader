# createkeyexportiframeresult

[Coinbase Developer Documentation home page![light logo](https://mintcdn.com/coinbase-prod/SAtIMuOJhKzTXIB7/logos/wordmark-light.svg?fit=max&auto=format&n=SAtIMuOJhKzTXIB7&q=85&s=49e34327ccb39158f220c9776bc3d3dd)![dark logo](https://mintcdn.com/coinbase-prod/SAtIMuOJhKzTXIB7/logos/wordmark-dark.svg?fit=max&auto=format&n=SAtIMuOJhKzTXIB7&q=85&s=747c97aefc3b41668a0538b6c5809148)](https://developer.chrome.com/)

-   [Get help](https://discord.com/invite/cdp)
-   [Dev portal](https://portal.cdp.coinbase.com/)
-   [
    
    Dev portal
    
    
    
    ](https://portal.cdp.coinbase.com/)

[Docs](https://developer.chrome.com/)[API Reference](https://developer.chrome.com/api-reference/v2/introduction)[SDKs](https://developer.chrome.com/sdks)[Demo Apps](https://developer.chrome.com/get-started/demo-apps/learn)[Changelogs](https://developer.chrome.com/get-started/changelog)

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

Result of setting up a key export iframe.

## 

[​](#properties)

Properties

Property

Type

Description

`iframe`

`HTMLIFrameElement`

The iframe element that was set up.

`cleanup`

() => `void`

Function to clean up event listeners and remove the iframe.

`updateTheme`

(`theme`: `Partial`<[`SecureIframeTheme`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Type-Aliases/SecureIframeTheme)\>) => `void`

Function to update the theme of the iframe.

Was this page helpful?

[

CreateKeyExportIframeOptions

Previous

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeOptions)[

EIP712Domain

Next

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EIP712Domain)

⌘I