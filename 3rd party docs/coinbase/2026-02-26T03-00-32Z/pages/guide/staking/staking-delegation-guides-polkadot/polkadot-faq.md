# polkadot faq

## What are Polkadot’s inflation and reward rates?

Polkadot’s inflation rate is dependent on the staking rate of the network. It increases to 10% as the staking ratio approaches 50% (for a reward rate of 20%) and then declines sharply thereafter.

## What is the difference between joining a pool and direct nomination?

Direct nomination is limited to only the top 256 nominators per validator ranked by amount staked. This means that the minimum DOT to earn rewards using direct nomination is dynamic, and much higher than the 1 DOT minimum required to join a pool. Using direct nomination allows you to select validators individually, and nomination pools that selection is managed by the pool operator.

## What is the minimum DOT to stake?

The minimum amount of DOT required to join a nomination pool is currently 1 DOT. Delegators should also always leave 0.1 - 1.0 DOT in their account un-nominated to cover the cost of transaction fees.

## What are the risks associated with bonding?

Validators can be slashed for misbehavior (e.g. being offline, equivocation). The slashed amount is a fixed percentage. A validator with more stake gets slashed more total DOTs. Since rewards are evenly distributed among validators elected to the consensus group, there is no economic advantage in staking more DOTs than required to be in the active set. In fact, because slashing is proportional, staking excess DOTs increases the loss in the event of slashing.

Threat Level

Slashable Behavior

Max % Slashed

Level 1

Low security threats such as isolated cases of unresponsiveness

0.1% (or kicking the validator out of the active set)

Level 2

Misconduct that occurs in good faith but is due to bad practices

1%

Level 3

Misconduct unlikely to happen in good faith or by accident, but does not lead to serious security risks or resource use

10%

Level 4

Misconduct that a) poses a serious security risk to the system, b) enables large levels of collusion among validators, and/or c) forces the system to spend a large amount of resources to deal with them

100%

## How does Coinbase’s service fee work?

Coinbase charges a percentage service fee on all rewards earned by tokens delegated to our Polkadot validator. The service fee is paid to Coinbase automatically by the protocol. You can withdraw your rewards from the protocol directly.

## Why does the Nomination Pool select all Coinbase validator addresses to nominate when staking?

Polkadot allows nominating up to 16 validators, each era the protocol will automatically allocate your stake to the validator(s) you have nominated which have also been selected to participate in the active set — and to earn rewards in exchange for doing so. Because of this ability within Polkadot, your performance will be optimized by nominating all of the validators in the list, as the more validators you nominate, the greater the likelihood that at least one of those validators will be selected to earn rewards each era. You can learn more about Active and Inactive Nominations, and how this allocation works, in Polkadot’s documentation.

## What is a Polkadot Controller?

A Controller is a 1-to-1 proxy for your stash (where your tokens live). A Controller can act on behalf of the tokens in your stash to nominate validators and participate in consensus, without being able to send transactions. This helps protect your funds because your stash account will not need to be accessed often.