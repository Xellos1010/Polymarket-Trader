# buildkeyexportiframeurloptions

-   [Get help](https://discord.com/invite/cdp)
-   [Dev portal](https://portal.cdp.coinbase.com/)
-   [](https://portal.cdp.coinbase.com/)

##### SDKs

-   [](https://developer.chrome.com/sdks)

##### BUILD ONCHAIN

-   -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend)
        -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/create-cdp-app)
        -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core)
            
            -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/AuthState)
                -   [
                    
                    BuildKeyExportIframeUrlOptions
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/BuildKeyExportIframeUrlOptions)
                -   [
                    
                    CreateKeyExportIframeOptions
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeOptions)
                -   [
                    
                    CreateKeyExportIframeResult
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/CreateKeyExportIframeResult)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EIP712Domain)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EIP712TypedData)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EIP712Types)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserEvmAccount)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserEvmSmartAccount)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/EndUserSolanaAccount)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IAuthManager)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/IOAuthManager)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/MFAMethods)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/OAuthFlowState)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/UserOperationReceipt)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Interfaces/WalletSecret)
            
            -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/reference)
        

-   [](https://docs.base.org/builderkits/minikit/overview)

##### CONSUMER APIs

##### INSTITUTIONAL APIs

-   [Properties](#properties)

Options for building a key export iframe URL.

## Properties

Property

Type

Description

`projectId`

`string`

The project ID for authentication.

`basePath?`

`string`

The base path of the secure iframe. **Default** `"https://secure-wallet.cdp.coinbase.com"`

`label?`

`string`

The label for the button displayed in the iframe.

`copiedLabel?`

`string`

The label to display when the key is copied successfully.

`icon?`

`boolean`

Whether to show an icon in the button. **Default** `true`