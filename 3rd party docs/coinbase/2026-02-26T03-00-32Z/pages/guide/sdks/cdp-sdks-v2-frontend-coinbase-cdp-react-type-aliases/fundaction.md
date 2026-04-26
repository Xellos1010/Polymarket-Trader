# fundaction

```
type FundAction = 
  | {
  type: "SET_FIELD";
  payload: { [K in keyof FundState]: { field: K; value: FundState[K] } }[keyof FundState];
}
  | {
  type: "SET_AMOUNTS";
  payload: {
     cryptoAmount: number;
     fiatAmount: number;
  };
}
  | {
  type: "FETCH_EXCHANGE_RATE";
}
  | {
  type: "SET_EXCHANGE_RATE_SUCCESS";
  payload: {
     exchangeRate: number | undefined;
  };
}
  | {
  type: "SET_EXCHANGE_RATE_ERROR";
  payload: {
     error: Partial<NonNullable<FundState["exchangeRateError"]>>;
  };
}
  | {
  type: "FETCH_PAYMENT_METHODS";
}
  | {
  type: "SET_PAYMENT_METHODS_SUCCESS";
  payload: {
     paymentMethods: FundPaymentMethod[];
  };
}
  | {
  type: "SET_PAYMENT_METHODS_ERROR";
  payload: {
     error: Partial<NonNullable<FundState["paymentMethodsError"]>>;
  };
}
  | {
  type: "SET_TRANSACTION_STATUS";
  payload: {
     transactionStatus: FundLifecycleStatus;
  };
}
  | {
  type: "SYNC_WITH_PROPS";
  payload: FundStateProps;
};

```

The actions that can be dispatched to the Fund component.