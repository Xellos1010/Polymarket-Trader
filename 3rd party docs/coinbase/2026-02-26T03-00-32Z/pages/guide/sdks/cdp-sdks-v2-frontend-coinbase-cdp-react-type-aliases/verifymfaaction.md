# verifymfaaction

```
type VerifyMfaAction = 
  | {
  type: "SET_STEP";
  payload: {
     step: VerifyMfaStep;
     flowDirection?: "left" | "right";
     method?: MfaMethod;
  };
}
  | {
  type: "SET_METHOD";
  payload: {
     method: MfaMethod;
  };
}
  | {
  type: "SET_MFA_CODE";
  payload: {
     mfaCode: string;
  };
}
  | {
  type: "SUBMIT_MFA_CODE";
  payload: {
     mfaCode: string;
  };
}
  | {
  type: "SUBMIT_MFA_CODE_SUCCESS";
}
  | {
  type: "SUBMIT_MFA_CODE_FAILURE";
  payload: {
     error:   | string
        | APIError;
  };
}
  | {
  type: "INITIATE_MFA_VERIFICATION_FAILURE";
  payload: {
     error:   | string
        | APIError;
  };
}
  | {
  type: "SET_METHODS";
  payload: {
     methods: MfaMethod[];
  };
}
  | {
  type: "CLEAR_ERROR";
}
  | {
  type: "RESET_STATE";
};

```

The actions that can be performed on the VerifyMfa state.