# fundmodal

```
function FundModal(props: FundModalProps): Element;

```

A fund modal component that wraps the [Fund](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/Fund) component.

## Parameters

Parameter

Type

Description

`props`

[`FundModalProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/FundModalProps)

The props for the FundModal component.

## Returns

`Element` The FundModal component.

## See

-   [FundModalTrigger](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/FundModalTrigger) for the trigger button.
-   [FundModalContent](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/FundModalContent) for the modal content.

## Examples

```
// Render the FundModal component
function App() {
  const fetchBuyQuote: FundProps["fetchBuyQuote"] = async (params) => {
    // call the buy quote API
  }
  const fetchBuyOptions: FundProps["fetchBuyOptions"] = async params => {
    // call the buy options API
  }
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <FundModal
        country="US"
        subdivision="NY"
        cryptoCurrency="eth"
        fiatCurrency="usd"
        fetchBuyQuote={fetchBuyQuote}
        fetchBuyOptions={fetchBuyOptions}
        network="base"
        presetAmountInputs={[10, 25, 50]}
      />
    </CDPReactProvider>
  );
}

```

```
// Render the FundModal component with a custom trigger button
function App() {
  const fetchBuyQuote: FundProps["fetchBuyQuote"] = async (params) => {
    // call the buy quote API
  }
  const fetchBuyOptions: FundProps["fetchBuyOptions"] = async params => {
    // call the buy options API
  }
  return (
    <CDPReactProvider config={config} theme={themeOverrides}>
      <FundModal
        country="US"
        subdivision="NY"
        cryptoCurrency="eth"
        fiatCurrency="usd"
        fetchBuyQuote={fetchBuyQuote}
        fetchBuyOptions={fetchBuyOptions}
        network="base"
        presetAmountInputs={[10, 25, 50]}
      >
        <button className="fund-button">
          Get ETH
        </button>
      </FundModal>
    </CDPReactProvider>
  );
}

```