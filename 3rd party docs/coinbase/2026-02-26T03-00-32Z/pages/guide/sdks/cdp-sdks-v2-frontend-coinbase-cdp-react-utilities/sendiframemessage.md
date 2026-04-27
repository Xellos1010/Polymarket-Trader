# sendiframemessage

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
                
                
                
                ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react)
            
            -   -   [
                    
                    clamp
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/clamp)
                -   [
                    
                    flattenTokensObject
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/flattenTokensObject)
                -   [
                    
                    getMessageFromUnknownError
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/getMessageFromUnknownError)
                -   [
                    
                    isApiError
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/isApiError)
                -   [
                    
                    isEmailInvalid
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/isEmailInvalid)
                -   [
                    
                    parseValuesFromPhoneNumber
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/parseValuesFromPhoneNumber)
                -   [
                    
                    sendIframeMessage
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/sendIframeMessage)
                -   [
                    
                    themeToCssVariables
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/themeToCssVariables)
            
            -   [
                
                Reference
                
                
                
                ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/reference)
        

-   [
    
    MiniKit
    
    
    
    ](https://docs.base.org/builderkits/minikit/overview)

##### CONSUMER APIs

##### INSTITUTIONAL APIs

-   [Type Parameters](#type-parameters)
-   [Parameters](#parameters)
-   [Returns](#returns)

Utilities

```
function sendIframeMessage<T>(el: null | HTMLIFrameElement, message: T): void;

```

Sends a message to an iframe.

## 

[​](#type-parameters)

Type Parameters

Type Parameter

`T` *extends* `Record`<`string` | `number` | `symbol`, `unknown`\>

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`el`

`null` | `HTMLIFrameElement`

The iframe element to send the message to.

`message`

`T`

The message to send to the iframe.

## 

[​](#returns)

Returns

`void`

Was this page helpful?

[

parseValuesFromPhoneNumber

Previous

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/parseValuesFromPhoneNumber)[

themeToCssVariables

Next

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/themeToCssVariables)

⌘I