# isapierror

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

-   [Parameters](#parameters)
-   [Returns](#returns)
-   [Example](#example)

Utilities

```
function isApiError(error: unknown): error is APIError;

```

Type guard to check if the error is an API error.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`error`

`unknown`

The error to check.

## 

[​](#returns)

Returns

`error is APIError`

-   True if the error is an API error, false otherwise.

## 

[​](#example)

Example

```
try {
  ...
}
catch (error) {
  if (isApiError(error)) {
    // Handle API error
    console.log(error.errorMessage);
  }
}

```

Was this page helpful?

[

getMessageFromUnknownError

Previous

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/getMessageFromUnknownError)[

isEmailInvalid

Next

](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Utilities/isEmailInvalid)

⌘I