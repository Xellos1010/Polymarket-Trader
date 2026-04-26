# zindex

```
const zIndex: {
  select: {
     list: {
        value: "{zIndex.popup}";
     };
  };
  modal: {
     overlay: {
        value: "{zIndex.scrim}";
     };
     dialog: {
        value: "{zIndex.floating}";
     };
  };
  base: {
     value: 0;
     unit: "none";
  };
  raised: {
     value: 1;
     unit: "none";
  };
  popup: {
     value: 200;
     unit: "none";
  };
  scrim: {
     value: 400;
     unit: "none";
  };
  floating: {
     value: 500;
     unit: "none";
  };
};

```

All the z-index tokens.

## Type declaration

### select

```
readonly select: {
  list: {
     value: "{zIndex.popup}";
  };
};

```

#### Type declaration

#### select.list

```
readonly list: {
  value: "{zIndex.popup}";
};

```

##### Type declaration

#### select.list.value

```
readonly value: "{zIndex.popup}" = "{zIndex.popup}";

```

### modal

```
readonly modal: {
  overlay: {
     value: "{zIndex.scrim}";
  };
  dialog: {
     value: "{zIndex.floating}";
  };
};

```

#### Type declaration

#### modal.overlay

```
readonly overlay: {
  value: "{zIndex.scrim}";
};

```

##### Type declaration

#### modal.overlay.value

```
readonly value: "{zIndex.scrim}" = "{zIndex.scrim}";

```

#### modal.dialog

```
readonly dialog: {
  value: "{zIndex.floating}";
};

```

##### Type declaration

#### modal.dialog.value

```
readonly value: "{zIndex.floating}" = "{zIndex.floating}";

```

### base

```
readonly base: {
  value: 0;
  unit: "none";
};

```

#### Type declaration

#### base.value

#### base.unit

```
readonly unit: "none" = "none";

```

### raised

```
readonly raised: {
  value: 1;
  unit: "none";
};

```

#### Type declaration

#### raised.value

#### raised.unit

```
readonly unit: "none" = "none";

```

```
readonly popup: {
  value: 200;
  unit: "none";
};

```

#### Type declaration

```
readonly value: 200 = 200;

```

```
readonly unit: "none" = "none";

```

### scrim

```
readonly scrim: {
  value: 400;
  unit: "none";
};

```

#### Type declaration

#### scrim.value

```
readonly value: 400 = 400;

```

#### scrim.unit

```
readonly unit: "none" = "none";

```

### floating

```
readonly floating: {
  value: 500;
  unit: "none";
};

```

#### Type declaration

#### floating.value

```
readonly value: 500 = 500;

```

#### floating.unit

```
readonly unit: "none" = "none";

```