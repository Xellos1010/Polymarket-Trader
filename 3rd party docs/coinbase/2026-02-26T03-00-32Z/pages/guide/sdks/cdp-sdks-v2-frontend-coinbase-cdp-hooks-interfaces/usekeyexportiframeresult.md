# usekeyexportiframeresult

##### SDKs

-   [](https://developer.chrome.com/sdks)

##### BUILD ONCHAIN

-   -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend)
        -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/create-cdp-app)
        
        -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks)
            
            -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/CDPContextValue)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/CDPHooksProviderProps)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/EIP712TypedData)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/OAuthFlowState)
                -   [
                    
                    UseKeyExportIframeOptions
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/UseKeyExportIframeOptions)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/UseKeyExportIframeResult)
                -   [
                    
                    UseRegisterMfaListenerOptions
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/UseRegisterMfaListenerOptions)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Interfaces/UserOperationReceipt)
            
            -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/reference)
        

-   [](https://docs.base.org/builderkits/minikit/overview)

##### CONSUMER APIs

##### INSTITUTIONAL APIs

-   [Properties](#properties)

Return type for the key export iframe hooks.

## Properties

Property

Type

Description

`status`

`null` | `SecureIframeStatus`

The current status of the iframe.

`message`

`undefined` | `string`

Optional message associated with the status.

`updateTheme`

(`theme`: `Partial`<`SecureIframeTheme`\>) => `void`

Function to update the theme of the iframe.

`cleanup`

() => `void`

Function to manually clean up the iframe. Note: Cleanup is automatic when the component unmounts or when the iframe expires.