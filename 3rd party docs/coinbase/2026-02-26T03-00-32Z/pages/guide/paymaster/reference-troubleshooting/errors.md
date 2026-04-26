# errors

This page lists error codes returned by CDP Paymaster and Bundler. Use this as a quick reference when debugging failed userOperations.

## Paymaster Errors

These errors are returned by the CDP Paymaster service.

Error

Code

Description

How to Fix

`INTERNAL_ERROR`

\-32000

Internal service error

Retry the request. If it persists, [contact support](#need-help).

`UNAUTHORIZED_ERROR`

\-32001

Invalid API key or endpoint

Verify your Paymaster URL is correct and the API key is active.

`DENIED_ERROR`

\-32001

Request denied by gas policy

Check your [allowlist and spend limits](https://portal.cdp.coinbase.com/products/bundler-and-paymaster) in CDP Portal.

`UNAVAILABLE_ERROR`

\-32003

Service temporarily unavailable

Retry after a short delay. If it persists, [contact support](#need-help).

`GAS_ESTIMATION_ERROR`

\-32004

Gas estimation failed

Usually insufficient gas or invalid paymaster signature. See [Troubleshooting](https://developer.chrome.com/paymaster/reference-troubleshooting/troubleshooting#execution-reverted).

`METHOD_NOT_FOUND`

\-32601

Unknown JSON-RPC method

Verify Paymaster is enabled and you’re calling a supported method.

`INVALID_ARGUMENT`

\-32602

Invalid userOperation parameters

Check that all required fields are present and correctly formatted.

`PARSE_ERROR`

\-32700

Malformed request body

Verify your JSON is valid and the request body is properly structured.

## Policy Rejection Messages

These messages indicate your userOperation was rejected by your configured gas policy.

Message

Cause

Solution

`rejected due to max per user op spend limit exceeded`

Single operation costs too much

Increase Per UserOperation limit in Configuration

`rejected due to max monthly org spend limit`

Monthly budget exhausted

[Apply for more credits](https://docs.google.com/forms/d/1yPnBFW0bVUNLUN_w3ctCqYM9sjdIQO3Typ53KXlsS5g/viewform) or wait for reset

`rejected due to max global usd spend limit reached`

Total spend limit hit

Adjust your global limit in Configuration

`rejected due to maximum per address transaction count reached`

Too many transactions from this sender

Increase per-address transaction limit

`rejected due to maximum per address sponsorship reached`

Sender hit USD sponsorship cap

Increase per-address sponsorship limit

`attestation not found for address`

Sender lacks required attestation

Ensure sender has the required onchain attestation

`target address not in allowed contracts`

Contract not allowlisted

Add the contract address to your allowlist

`method not in allowed methods`

Function not allowlisted

Add the specific function to your contract’s allowlist

## Bundler Errors

These errors are returned by the Bundler when processing userOperations.

Error

Code

Description

`REJECTED_BY_EP_OR_ACCOUNT`

\-32500

Rejected by EntryPoint or smart account

`REJECTED_BY_PAYMASTER`

\-32501

Paymaster refused to sponsor

`BANNED_OPCODE`

\-32502

UserOperation contains a banned opcode

`SHORT_DEADLINE`

\-32503

Transaction deadline too short

`BANNED_OR_THROTTLED_ENTITY`

\-32504

Sender or paymaster is throttled

`INVALID_ENTITY_STAKE`

\-32505

Invalid staking for entity

`INVALID_AGGREGATOR`

\-32506

Invalid signature aggregator

`INVALID_SIGNATURE`

\-32507

Signature verification failed

`EXECUTION_REVERTED`

\-32521

Onchain execution reverted

`INVALID_FIELDS`

\-32602

UserOperation has invalid fields

## EntryPoint Errors

These `AA` prefixed errors come from the ERC-4337 EntryPoint contract.

Error

Description

Solution

`AA10 sender already constructed`

Account exists but initCode provided

Remove `initCode` from your userOperation

`AA13 initCode failed or OOG`

Account creation failed or ran out of gas

Increase `verificationGasLimit`

`AA14 initCode must return sender`

Factory didn’t return sender address

Check your factory contract implementation

`AA15 initCode must create sender`

Factory didn’t deploy the account

Verify factory contract and initCode

`AA20 Account not deployed`

Account doesn’t exist and no initCode

Include `initCode` for first transaction

`AA21 didn't pay prefund`

Account can’t cover gas prefund

Ensure Paymaster is configured or account has ETH

`AA23 reverted (or OOG)`

Validation reverted or out of gas

Check signature validity and gas limits

`AA24 Signature error`

Invalid userOperation signature

Verify hash, entrypoint address, and chain ID

`AA25 Invalid account nonce`

Nonce mismatch

Use current nonce; don’t reuse old ones

`AA40 Over verification gas limit`

Verification exceeded gas limit

Increase `verificationGasLimit`

`AA41 Too little verification gas`

Verification ran out of gas

Increase `verificationGasLimit`

`AA50 PostOp reverted`

Post-operation hook failed

Debug paymaster `postOp` logic

`AA51 prefund below actualGasCost`

Approved gas less than actual cost

Increase gas limits or prefund amount

## Request Logs

Download detailed logs for debugging from the **Logs** tab in [CDP Portal](https://portal.cdp.coinbase.com/products/bundler-and-paymaster). Click **Download CSV** to export.

Column

Description

`OrganizationId`

Your CDP Organization ID

`ProjectId`

Your CDP Project ID

`Network`

`base` or `base-sepolia`

`Status`

`completed`, `in progress`, or `failed`

`UserOpHash`

Hash of the userOperation

`Sender`

Smart account address

`Paymaster`

Paymaster contract that sponsored the operation

`TransactionHash`

Onchain transaction hash (if included)

`GasCost`

Gas price in Gwei

`GasUsed`

Total gas consumed

`Method`

JSON-RPC method called

`ErrorCode`

Error code (for failed requests)

`ErrorMessage`

Error details (for failed requests)

## Need Help?

If you’re still stuck after checking these errors:

1.  **Export your logs** from CDP Portal for detailed request data
2.  **Check [Troubleshooting](https://developer.chrome.com/paymaster/reference-troubleshooting/troubleshooting)** for debugging workflows
3.  **Ask in Discord** — Post in the `#paymaster` channel in [CDP Discord](https://discord.com/invite/cdp) with your error details