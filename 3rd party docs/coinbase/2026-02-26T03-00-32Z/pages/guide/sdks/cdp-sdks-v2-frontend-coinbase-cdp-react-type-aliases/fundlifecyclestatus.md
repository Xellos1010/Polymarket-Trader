# fundlifecyclestatus

```
type FundLifecycleStatus = 
  | {
  statusName: "init";
  statusData: null;
}
  | {
  statusName: "exit";
  statusData: null;
}
  | {
  statusName: "error";
  statusData: OnrampError;
}
  | {
  statusName: "transactionSubmitted";
  statusData: null;
}
  | {
  statusName: "transactionSuccess";
  statusData:   | OnrampSuccessEventData
     | null;
}
  | {
  statusName: "transactionPending";
  statusData: null;
};

```

The lifecycle statuses of the Fund component.