# enrollmfaaction

```
type EnrollMfaAction = 
  | {
  type: "SET_STEP";
  payload: {
     step: EnrollMfaStep;
     flowDirection?: "left" | "right";
     method?: MfaMethod;
  };
}
  | {
  type: "SET_MFA_CODE";
  payload: {
     mfaCode: string;
  };
}
  | {
  type: "SET_PHONE_NUMBER";
  payload: {
     phoneNumber: string;
  };
}
  | {
  type: "INITIATE_ENROLLMENT";
  payload: {
     method: MfaMethod;
  };
}
  | {
  type: "INITIATE_ENROLLMENT_SUCCESS";
  payload: InitiateMfaEnrollmentResult;
}
  | {
  type: "INITIATE_ENROLLMENT_FAILURE";
  payload: {
     error:   | string
        | APIError;
  };
}
  | {
  type: "ENROLLMENT_SESSION_EXPIRED";
}
  | {
  type: "SUBMIT_ENROLLMENT";
}
  | {
  type: "SUBMIT_ENROLLMENT_SUCCESS";
}
  | {
  type: "SUBMIT_ENROLLMENT_FAILURE";
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
  type: "GO_TO_PREVIOUS_STEP";
}
  | {
  type: "RESET_STATE";
};

```

The actions that can be performed on the EnrollMfa state.