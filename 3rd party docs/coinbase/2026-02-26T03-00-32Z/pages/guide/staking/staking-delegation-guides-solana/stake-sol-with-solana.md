# stake sol with solana

## 1\. Stake SOL to our public validator

1.  Open your stake account on a supported wallet.
2.  Use Coinbase’s vote account address `beefK6BWeSPhZYBHZxWp5So7wdQX6mu4ZHcSH3uTar` in Solana delegate-stake.

```
solana delegate-stake --stake-authority <KEYPAIR> <STAKE_ACCOUNT_ADDRESS> <VOTE_ACCOUNT_ADDRESS> \
--fee-payer <KEYPAIR>

```

**COINBASE VALIDATOR INFORMATION****Validator name:** [Coinbase](https://stakewiz.com/validator/beefKGBWeSpHzYBHZXwp5So7wdQGX6mu4ZHCsH3uTar)**Validator ID:** `XkCriyrNwS3G4rzAxtG5B1nnvb5KaJtUcku93VqekAr`**Validator vote account address:** `beefK6BWeSPhZYBHZxWp5So7wdQX6mu4ZHcSH3uTar`**Validator name:** [Coinbase 02](https://stakewiz.com/validator/6D2jqw9hyVCpppZexquxa74Fn33rJzzBx38T58VucHx9)**Validator ID:** `CW9C7HBwAMgNdXkNgFg9Ujr3edR24b9ymEuQnVacidA`**Validator vote account address:** `6D2jqw9hyYCcppZeyxuax74Fn33rJzzBx38T58VuchX9`

## 2\. View your delegation

Use `solana stake-account` to see the changes in your stake account.

```
solana stake-account <STAKE_ACCOUNT_ADDRESS>

```

**Congratulations! You are now staking with Coinbase.**