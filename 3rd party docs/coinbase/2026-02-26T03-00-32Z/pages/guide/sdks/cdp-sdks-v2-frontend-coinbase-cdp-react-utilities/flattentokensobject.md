# flattentokensobject

##### SDKs

-   [](https://developer.chrome.com/sdks)

##### BUILD ONCHAIN

-   -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend)
        -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/create-cdp-app)
        
        -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react)
            
            -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/clamp)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/flattenTokensObject)
                -   [
                    
                    getMessageFromUnknownError
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/getMessageFromUnknownError)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/isApiError)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/isEmailInvalid)
                -   [
                    
                    parseValuesFromPhoneNumber
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/parseValuesFromPhoneNumber)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/sendIframeMessage)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/themeToCssVariables)
            
            -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/reference)
        

-   [](https://docs.base.org/builderkits/minikit/overview)

##### CONSUMER APIs

##### INSTITUTIONAL APIs

-   [Type Parameters](#type-parameters)
-   [Parameters](#parameters)
-   [Returns](#returns)

```
function flattenTokensObject<T>(tokensObject: T, cssVarPrefix?: string): Flattened<T>;

```

Flattens a nested theme object into a single-level object with CSS variable representations.

## Type Parameters

## Parameters

Parameter

Type

Default value

Description

`tokensObject`

`T`

`undefined`

The nested tokens object to flatten.

`cssVarPrefix?`

`string`

`"cdp-web"`

An optional prefix for the generated CSS variables.

## Returns

[`Flattened`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Type-Aliases/Flattened)<`T`\> A flattened theme object.