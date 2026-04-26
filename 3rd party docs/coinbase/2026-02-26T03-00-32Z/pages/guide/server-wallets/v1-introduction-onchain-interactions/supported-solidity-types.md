# supported solidity types

The following table explains how the CDP SDK handles different Solidity types. For support with other Solidity types, contact us in the **#wallet-api** channel of the [CDP Discord](https://discord.com/invite/cdp).

Solidity Type

Parameter Argument

Example

int8, int16, int32, int64, int128, int256

string

`"-42"`

uint8, uint16, uint32, uint64, uint128, uint256

string

`"123456"`

function

hex-encoded string (4-byte array)

`"0xa9059cbb"`

bool

boolean

`true`

bytes

hex-encoded string

`"0x1234abcd"`

fixed bytes (e.g., bytes32)

hex-encoded string (fixed length)

`"0x000000000000000000000000a0b82847ab218b36c1d19d4a2e9eb0ce3606eb48"`

string

string

`"Hello, World!"`

address

hex-encoded string

`"0xBe9895146f7AF43049ca1c1AE358B0541Ea49704"`

arrays

array with elements following type rules

`["1", "2", "3"]`

tuple

array of tuple fields

`["John Doe", "25", "0xBe9895146f7AF43049ca1c1AE358B0541Ea49704"]`