# fund

```
function Fund(props: FundProps): Element;

```

The Fund component.

## Parameters

Parameter

Type

Description

`props`

[`FundProps`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Interfaces/FundProps)

The props of the Fund component.

## Returns

`Element` The Fund component.

## Examples

```
// Basic usage
const App = () => {
  const fetchBuyQuote: FundProps["fetchBuyQuote"] = async (params) => {
    // call the buy quote API
  }
  const fetchBuyOptions: FundProps["fetchBuyOptions"] = async params => {
    // call the buy options API
  }
  return (
    <Fund
      country="US"
      subdivision="NY"
      cryptoCurrency="eth"
      fiatCurrency="usd"
      fetchBuyQuote={fetchBuyQuote}
      fetchBuyOptions={fetchBuyOptions}
      network="base"
      presetAmountInputs={[10, 25, 50]}
    />
  )
}

```

```
// Example customizing the children to render the title as a page title
// and add a custom error message
const App = () => {
  const fetchBuyQuote: FundProps["fetchBuyQuote"] = async (params) => {
    // call the buy quote API
  }
  const fetchBuyOptions: FundProps["fetchBuyOptions"] = async params => {
    // call the buy options API
  }
  const title = "Buy ETH";
  const titleId = useId();
  const submitLabel = "Purchase now";
  return (
    <Fund
      country="US"
      subdivision="NY"
      cryptoCurrency="eth"
      fiatCurrency="usd"
      fetchBuyQuote={fetchBuyQuote}
      fetchBuyOptions={fetchBuyOptions}
      network="base"
      presetAmountInputs={[10, 25, 50]}
      submitLabel={submitLabel}
      title={title}
    >
      <FundTitle as="h1" id={titleId}>{title}</FundTitle>
      <FundForm aria-labelledby={titleId} submitLabel={submitLabel}>
        {({ view, Content }) => (
          <>
            {Content}
            {view === "error" && <p>Contact support at support@example.com</p>}
          </>
        )}
      </FundForm>
      <FundFooter />
    </Fund>
  )
}

```

## Further reading

-   [Fund Overview](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-react/Components/Fund.README)