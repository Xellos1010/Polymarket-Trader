# secure iframe event type

##### SDKs

-   [](https://developer.chrome.com/sdks)

##### BUILD ONCHAIN

-   -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend)
        -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/create-cdp-app)
        -   -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core)
            
            -   -   [
                    
                    DEFAULT\_SECURE\_IFRAME\_BASE\_PATH
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/DEFAULT_SECURE_IFRAME_BASE_PATH)
                -   [
                    
                    DEFAULT\_SECURE\_IFRAME\_THEME
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/DEFAULT_SECURE_IFRAME_THEME)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/ErrorType)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/HttpErrorType)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/MFA_PROTECTED_ACTIONS)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/MFA_PROTECTED_FUNCTIONS)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/OAuth2ProviderType)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/SECURE_IFRAME_EVENT_TYPE)
                -   [
                    
                    SECURE\_IFRAME\_EVENT\_TYPE\_PREFIX
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/SECURE_IFRAME_EVENT_TYPE_PREFIX)
                -   [
                    
                    SECURE\_IFRAME\_KEY\_EXPORT\_EVENT\_TYPE
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/SECURE_IFRAME_KEY_EXPORT_EVENT_TYPE)
                -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/STANDARD_ERROR_CODES)
                -   [
                    
                    SendEvmTransactionWithEndUserAccountBodyNetwork
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/SendEvmTransactionWithEndUserAccountBodyNetwork)
                -   [
                    
                    SendSolanaTransactionWithEndUserAccountBodyNetwork
                    
                    
                    
                    ](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/Variables/SendSolanaTransactionWithEndUserAccountBodyNetwork)
            -   [](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-core/reference)
        

-   [](https://docs.base.org/builderkits/minikit/overview)

##### CONSUMER APIs

##### INSTITUTIONAL APIs

-   [Type declaration](#type-declaration)
-   [INIT](#init)
-   [LISTENING](#listening)
-   [STATUS](#status)
-   [THEME](#theme)

```
const SECURE_IFRAME_EVENT_TYPE: {
  INIT: "CDP_WEB_SECURE_IFRAME_INIT";
  LISTENING: "CDP_WEB_SECURE_IFRAME_LISTENING";
  STATUS: "CDP_WEB_SECURE_IFRAME_STATUS";
  THEME: "CDP_WEB_SECURE_IFRAME_THEME";
};

```

The base type of event for the secure iframe.

## Type declaration

### INIT

```
readonly INIT: "CDP_WEB_SECURE_IFRAME_INIT";

```

### LISTENING

```
readonly LISTENING: "CDP_WEB_SECURE_IFRAME_LISTENING";

```

### STATUS

```
readonly STATUS: "CDP_WEB_SECURE_IFRAME_STATUS";

```

### THEME

```
readonly THEME: "CDP_WEB_SECURE_IFRAME_THEME";

```