# cosmos faq

## Why is Cosmos’ expected reward \* rate a range of numbers?

Rewards are dependent upon the number of ATOMs currently staked in the protocol. If less than two-thirds of the total atom supply is staked, the annualized inflation increases anywhere from 7-20% until two-thirds of all ATOMs are staked again. To determine the current reward rate, divide the inflation rate by the staking ratio. You can find those numbers on [Mintscan](https://www.mintscan.io/), a Cosmos block explorer. `*` Reward rates published by Coinbase are estimates based on publicly available information from third-party sources. Coinbase has not verified and does not guarantee the accuracy of this information. Reward rates on some protocols may vary based on the amount staked and/or other variables, including validator performance, so you should not rely on the accuracy of any reward rate ranges we publish, which are intended to provide an estimate. The actual rate of rewards earned may vary substantially and may change over time and Coinbase does not guarantee that you will receive any staking rewards. Staked assets may be subject to slashing penalties and risk of loss is possible, including up to the full loss of principal.

## What are the risks associated with delegating?

If a validator doesn’t follow protocol rules, Cosmos slashes their bonded tokens as well as the ATOMs delegated to them. ATOMs remain at risk for slashing even during the 21-day unbonding period. Slashing primarily happens for one of the two reasons outlined below.

-   **Double signing:** If someone reports on chain A that a validator signed two blocks at the same height on chain A and chain B, the validator will get slashed on chain A.
-   **Unavailability:** If a validator’s signature has not been included in the last X blocks, the validator will get slashed by a marginal amount proportional to X. If X is above a certain limit Y, then the validator will get unbonded.

Coinbase takes these risks extremely seriously and have built our infrastructure to protect our customers and minimize slashing risk.

## How does Coinbase’s service fee work?

Coinbase charges a percentage service fee on all rewards earned by tokens delegated to our Cosmos validator. The service fee is paid to Coinbase automatically by the protocol. You can withdraw your rewards from the protocol directly.