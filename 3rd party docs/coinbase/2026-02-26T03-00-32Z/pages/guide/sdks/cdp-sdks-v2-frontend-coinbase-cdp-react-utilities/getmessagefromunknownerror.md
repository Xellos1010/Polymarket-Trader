# getmessagefromunknownerror

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

-   [Parameters](#parameters)
-   [Returns](#returns)

```
function getMessageFromUnknownError(error: unknown, defaultMesasge?: string): string;

```

Get a message from an unknown error with a fallback in case one is not found.

## Parameters

Parameter

Type

Default value

Description

`error`

`unknown`

`undefined`

The error to get a message from.

`defaultMesasge?`

`string`

`"Something went wrong"`

The default message to return if no message is found.

## Returns

`string` The message from the error.