# signinaction

```
type SignInAction = 
  | {
  type: "SET_AUTH_METHOD";
  payload: {
     authMethod: AuthMethod;
  };
}
  | {
  type: "SET_AUTH_METHODS";
  payload: {
     authMethods: AuthMethod[];
  };
}
  | {
  type: "SET_EMAIL";
  payload: {
     email: string;
  };
}
  | {
  type: "SUBMIT_EMAIL";
  payload: {
     email: string;
  };
}
  | {
  type: "SUBMIT_EMAIL_SUCCESS";
  payload: {
     flowId: string;
  };
}
  | {
  type: "SUBMIT_EMAIL_FAILURE";
  payload: {
     error:   | string
        | APIError;
  };
}
  | {
  type: "SET_PHONE_NUMBER";
  payload: {
     phoneNumber: string;
  };
}
  | {
  type: "SUBMIT_PHONE_NUMBER";
  payload: {
     phoneNumber: string;
  };
}
  | {
  type: "SUBMIT_PHONE_NUMBER_SUCCESS";
  payload: {
     flowId: string;
  };
}
  | {
  type: "SUBMIT_PHONE_NUMBER_FAILURE";
  payload: {
     error:   | string
        | APIError;
  };
}
  | {
  type: "SET_OTP";
  payload: {
     otp: string;
  };
}
  | {
  type: "SUBMIT_OTP";
  payload: {
     otp: string;
  };
}
  | {
  type: "SUBMIT_OTP_SUCCESS";
  payload: {
     otp: string;
  };
}
  | {
  type: "SUBMIT_OTP_FAILURE";
  payload: {
     error:   | string
        | APIError;
  };
}
  | {
  type: "ALLOW_RESET_OTP";
}
  | {
  type: "RESET_OTP";
}
  | {
  type: "CLEAR_ERROR";
}
  | {
  type: "RESET_STATE";
};

```

The actions that can be performed on the SignIn state.

## See

-   [SignIn](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/SignIn)
-   [useSignInReducer](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Hooks/useSignInReducer)