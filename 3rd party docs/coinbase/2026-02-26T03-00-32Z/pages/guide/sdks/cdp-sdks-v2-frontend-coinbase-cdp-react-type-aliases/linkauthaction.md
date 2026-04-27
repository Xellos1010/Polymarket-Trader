# linkauthaction

```
type LinkAuthAction = 
  | {
  type: "LINK_AUTH_METHOD";
  payload: {
     method: LinkAuthState["methodToLink"];
  };
}
  | {
  type: "LINK_AUTH_METHOD_ERROR";
  payload: {
     error: LinkAuthState["error"];
  };
}
  | {
  type: "SET_AUTH_METHODS";
  payload: {
     methods: LinkAuthState["authMethods"];
  };
}
  | {
  type: "RESET_STATE";
};

```

The actions that can be performed on the LinkAuth state.

## 

[​](#see)

See

-   [LinkAuthState](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/LinkAuthState)
-   [LinkAuth](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/LinkAuth)