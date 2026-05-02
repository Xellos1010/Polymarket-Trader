# schema

The SQL API schema is a set of opinionated tables and columns used to organize onchain data for efficient retrieval.

## Supported Tables

Table

Description

[base.blocks](#base-blocks)

Block metadata including timestamps and difficulty

[base.events](#base-events)

Decoded event logs with contract interactions on Base

[base.transactions](#base-transactions)

Transaction data including hash, block number, gas usage

[base.encoded\_logs](#base-encoded-logs)

Encoded log data of event logs that aren’t able to be decoded by our event decoder (ex: log0 opcode)

[base.transfers](#base-transfers)

Token transfer events including block details, addresses, and amounts

## base.blocks

Block metadata including timestamps and difficulty.

Field

Type

Description

block\_number

uint64

The number of the block

block\_hash

String

The unique hash identifying this block

parent\_hash

String

The hash of the parent block

timestamp

DateTime

The timestamp when this block was created

miner

String

The address of the miner/validator who created this block

nonce

uint64

The proof-of-work nonce value

sha3\_uncles

String

The hash of the uncles list for this block

transactions\_root

String

The root hash of the transactions trie

state\_root

String

The root hash of the state trie

receipts\_root

String

The root hash of the receipts trie

logs\_bloom

String

The bloom filter for the logs of the block

gas\_limit

uint64

The maximum gas allowed in this block

gas\_used

uint64

The total gas used by all transactions in this block

base\_fee\_per\_gas

uint64

The base fee per gas in this block (EIP-1559)

total\_difficulty

String

The total difficulty of the chain up to this block

size

uint64

The size of this block in bytes

extra\_data

String

Extra data field for this block

mix\_hash

String

The mix hash for this block

withdrawals\_root

String

The root hash of withdrawals (post-merge)

parent\_beacon\_block\_root

String

The parent beacon block root (post-merge)

blob\_gas\_used

uint64

The amount of blob gas used in this block

excess\_blob\_gas

uint64

The excess blob gas in this block

transaction\_count

uint64

The number of transactions in this block

action

Int8

Indicates if block was added (1) or removed (-1) due to chain reorganization

## base.events

Decoded event logs with contract interactions on Base.

Field

Type

Description

block\_number

uint64

The block number

block\_hash

String

A keccak-256 (SHA-3) hash of the block’s header data. Unique to the block’s contents. Used to verify the integrity of the block

timestamp

DateTime64

Time at which the block was created

transaction\_hash

String

A keccak-256 hash of the signed transaction data. Unique identifier, on the blockchain, for this specific transaction

transaction\_to

String

The address the transaction is acting against. Could be either an EOA (ex: ETH transfer) or a contract (ex: smart contract call)

transaction\_from

String

The address that originated the transaction. Will be an EOA

transaction\_index

uint64

The order in which the transaction was included in the block. Commonly used to match transactions to their logs

log\_index

uint64

The index of the log within the transaction. First log is in the transaction at index 0, second is index 1, etc

address

String

The address of the contract that the log was created from

topics

Array(String)

The topics of the log. Topics are the indexed parameters of the event and the keccak256 hash of the event signature

event\_name

String

Human-readable name of the event

event\_signature

String

Full canonical declaration of the event, including its name and parameter types. Used to generate the hash

parameters

Map(String, Variant(Bool, Int256, String, uint256))

Map of parameter name to its value

parameter\_types

Map(String, String)

Map of parameter name to its ABI type

action

Int8

If the log is created, it is 1. If the log is re-orged out it is -1. If the sum of all actions for a given log is greater than 0, the log is “active”, meaning it is still in the chain (has not been re-orged out)

## base.transactions

Transaction data including hash, block number, gas usage.

Field

Type

Description

block\_number

uint64

The number of the block that contains this transaction

block\_hash

String

The hash of the block that contains this transaction

transaction\_hash

String

The unique hash identifying this transaction

transaction\_index

uint64

The index position of this transaction within its block

from\_address

String

The address that originated this transaction

to\_address

String

The destination address for this transaction

value

String

The value being transferred in this transaction

gas

uint64

The amount of gas allocated for this transaction

gas\_price

uint64

The price of gas (in wei) for this transaction

input

String

The data payload sent with this transaction

nonce

uint64

The number of transactions sent from this address before this one

type

uint64

The transaction type

max\_fee\_per\_gas

uint64

The maximum fee per gas the sender is willing to pay

max\_priority\_fee\_per\_gas

uint64

The maximum priority fee per gas the sender is willing to pay

chain\_id

uint64

The chain ID this transaction is valid for

v

String

The v component of the transaction signature

r

String

The r component of the transaction signature

s

String

The s component of the transaction signature

is\_system\_tx

Bool

Whether this is a system transaction

max\_fee\_per\_blob\_gas

String

The maximum fee per blob gas the sender is willing to pay

blob\_versioned\_hashes

Array(String)

Array of versioned hashes for any blobs associated with this transaction

timestamp

DateTime64

The timestamp when this transaction was included in a block

action

Int8

Indicates if transaction was added (1) or removed (-1) due to chain reorganization

## base.encoded\_logs

Encoded log data of event logs that aren’t able to be decoded by our event decoder (ex: log0 opcode).

Field

Type

Description

block\_number

uint64

The number of the block that the log is in

block\_hash

String

The hash of the block that the log is in

block\_timestamp

DateTime64

The timestamp of the block that the log is in

transaction\_hash

String

The hash of the transaction that the log is in

transaction\_to

String

The address the transaction is acting against. Could be either an EOA (ex: ETH transfer) or a contract (ex: smart contract call)

transaction\_from

String

The address that originated the transaction. Will be an EOA

log\_index

uint32

The index of the log within the transaction. First log is in the transaction at index 0, second is index 1, etc

address

String

The address of the contract that the log was created from

topics

Array(String)

The topics of the log. Topics are the indexed parameters of the event and the keccak256 hash of the event signature

action

Enum8(‘removed’ = -1, ‘added’ = 1)

If the log is created, it is 1. If the log is re-orged out it is -1. If the sum of all actions for a given log is greater than 0, the log is “active”, meaning it is still in the chain (has not been re-orged out)

## base.transfers

Token transfer events including block details, addresses, and amounts

Field

Type

Description

block\_number

uint64

Block number containing the transfer

block\_timestamp

DateTime64

Block timestamp

transaction\_to

String

Transaction recipient address

transaction\_from

String

Transaction sender address

log\_index

uint32

Log index within the transaction

token\_address

String

Address of the token contract

from\_address

String

Address tokens are transferred from

to\_address

String

Address tokens are transferred to

value

uint256

Amount of tokens transferred

action

Enum8

Action: 1 for add, -1 for re-org removal