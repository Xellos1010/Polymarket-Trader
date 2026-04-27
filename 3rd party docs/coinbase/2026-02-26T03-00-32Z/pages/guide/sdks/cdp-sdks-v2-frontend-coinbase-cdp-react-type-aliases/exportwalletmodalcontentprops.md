# exportwalletmodalcontentprops

```
type ExportWalletModalContentProps = Omit<ModalContentProps, "children"> & Pick<ExportWalletProps, "children"> & {
  title?: ReactNode;
};

```

Props for the export wallet modal content.

## 

[​](#type-declaration)

Type declaration

### 

[​](#title)

title?

```
optional title: ReactNode;

```

A title for the dialog element

## 

[​](#see)

See

[ExportWalletModalContent](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/ExportWalletModalContent)