# evm token balances

The EVM Token Balances APIs enable you to retrieve the balances of EVM addresses. This includes tokens (i.e. ERC-20s) and the native gas token of the network.

## Denomination

-   ‘amount’ is denominated in the smallest indivisible unit of the token. For ETH, the smallest indivisible unit is Wei (10^-18 ETH). For ERC-20s, the smallest unit is the unit returned from `function totalSupply() public view returns (uint256)`.
-   ‘decimals’ is the exponential value N that satisfies the equation `amount * 10^-N = standard_denomination`. The standard denomination is the most commonly used denomination for the token.
    -   In the case of the native gas token, `decimals` is defined via convention. As an example, for ETH of Ethereum mainnet, the standard denomination is 10^-18 the smallest denomination (Wei). As such, for ETH on Ethereum mainnet, `decimals` is 18.
    -   In the case of ERC-20 tokens, `decimals` is defined via configuration. `decimals` will be the number returned by `function decimals() public view returns (uint8)` on the underlying token contract.