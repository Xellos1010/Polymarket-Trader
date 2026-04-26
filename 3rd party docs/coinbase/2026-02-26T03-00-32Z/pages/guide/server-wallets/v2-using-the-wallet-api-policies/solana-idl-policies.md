# solana idl policies

## Overview

The `solData` criterion provides advanced validation of Solana transaction instruction data using Interface Definition Language (IDL) specifications. This criterion allows you to decode and validate instruction parameters against specific rules, ensuring that transactions meet precise requirements before signing. A `solData` criterion uses IDL specifications to identify which Solana programs to validate against and defines instruction-specific validation rules including instruction names and parameter constraints. The `idls` field specifies which programs to validate, while the `conditions` field defines the validation rules. The `idls` field is a list that can include either:

-   [known program names](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/solana-idl-policies#known-program-shortcuts) like “SystemProgram” and “TokenProgram”
-   [custom IDL objects](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/solana-idl-policies#custom-idl-objects)

For general policy concepts and setup instructions, see the [Policies Overview](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview).

## IDL Specifications

### Anchor IDL Format

IDL specifications must follow Anchor’s IDL format v0.30+. This standardized format ensures compatibility and proper instruction decoding.

### Supported Argument Types

The following primitive argument types are currently supported:

-   **Boolean**: `bool`
-   **String**: `string`
-   **Public Key**: `pubkey`
-   **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `u256`
-   **Signed Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `i256`
-   **Floating Point**: `f32`, `f64`

### Current Limitations

Complex IDL argument types are not currently supported, including:

-   User-defined types
-   Arrays
-   Vectors
-   Optional types

If you need support for any of these features, please reach out to us on our [CDP Discord](https://discord.com/invite/cdp).

### IDL Compatibility

To convert older IDL formats to the required v0.30+ format, use the Anchor CLI:

```
anchor idl convert internal/policy/fixtures/solana_system_program_idl.json -o internal/policy/fixtures/solana_system_program_idl.json

```

## Instruction Discriminators

Instruction discriminators are unique byte sequences that identify specific instructions within a program. Different program types use different discriminator formats and sizes.

### Discriminator Formats by Program Type

Program Type

Discriminator Format

Size

Example

**SystemProgram**

4-byte little-endian u32

4 bytes

Transfer = `[2,0,0,0]`

**SPL Token**

1-byte enum index

1 byte

Transfer = `3`

**Associated Token**

Borsh-encoded enum

1 byte

Create = `0`

**Anchor Programs**

SHA256 hash of “global:instruction\_name”

8 bytes

transfer = `[163,52,200,...]`

### Anchor Discriminator Generation

For Anchor programs, the 8-byte discriminator is generated using:

```
SHA256("global:instruction_name")

```

This ensures consistent and unique identification of instructions across Anchor-based programs.

## IDL Configuration

### Known Program Shortcuts

For common Solana programs, you can use predefined program names instead of providing IDL specifications:

-   `"SystemProgram"` - Native Solana system program
-   `"TokenProgram"` - SPL Token program
-   `"AssociatedTokenProgram"` - Associated Token Account program

### Custom IDL Objects

For custom programs or when you need specific instruction definitions, provide IDL objects with:

-   **address**: The program’s public key address
-   **instructions**: Array of instruction definitions containing:
    -   **name**: Instruction name
    -   **discriminator**: Byte array identifying the instruction
    -   **args**: Array of argument definitions with name and type

## Conditions

Conditions in your `solData` criteria allow you to validate specific instruction parameters against defined constraints.

### Condition Evaluation

-   If there are **multiple conditions** defined in a solData criterion, they are evaluated with **OR** logic - if any condition matches, the validation passes
-   **Parameters within a condition** are evaluated with **AND** logic - all parameters must match for the condition to pass

### Condition Structure

Each condition includes:

-   **instruction**: Name matching an instruction in one of the provided IDLs
-   **params** (optional): Array of parameter validations, each with:
    -   **name**: Parameter name matching the instruction argument
    -   **operator**: Comparison operator (`==`, `<=`, `>=`, `<`, `>`, `!=`) for single value comparison, or (`in`, `not in`) for list comparisons
    -   **value** or **values**: Expected value or list of values for comparison

## Examples

### Using Known IDLs

This example uses predefined program names for common Solana programs:

### Using Custom IDLs

For custom programs, provide IDL specifications:

It is critical that instructions are properly formatted with the correct discriminator and parameter encoding for policy validation to work correctly. The discriminator must match exactly what’s defined in the IDL, and parameters must be encoded according to their specified types.

Anchor-formatted instructions for the above policy examples:

## Use Cases

The `solData` criterion enables sophisticated policy controls for any Solana program, from native system operations to complex DeFi protocols. By providing custom IDL specifications, you can validate instruction parameters for both your own programs and popular ecosystem protocols.

### Example Ecosystem Program Integration

**DeFi and Trading Protocols:**

-   **Jupiter** - Control swap parameters, slippage limits, and route restrictions for decentralized exchanges
-   **Raydium** - Set constraints on liquidity provision and farming operations

**Staking and Validation:**

-   **Jito** - Govern staking operations, validator selection, and MEV reward distributions
-   **Marinade** - Control liquid staking deposits, withdrawals, and stake account management

**NFT and Digital Assets:**

-   **Metaplex** - Restrict NFT minting parameters, royalty settings, and marketplace interactions
-   **Magic Eden** - Validate NFT marketplace transactions and bid placements

For your own custom programs, `solData` policies provide granular control over inputs to your instruction parameters.

## Key Considerations

### IDL Format Requirements

-   Always use Anchor IDL format v0.30+ for compatibility
-   Verify IDL structure matches the expected format
-   Convert older IDL formats using the Anchor CLI conversion tool
-   Ensure discriminator values exactly match the program’s instruction identifiers

## What to read next

-   [**Policies Overview**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/overview): Learn about general policy concepts and setup
-   [**Solana Policies**](https://developer.chrome.com/server-wallets/v2/using-the-wallet-api/policies/solana-policies): Learn about Solana-specific policy examples
-   [**v2 Server Wallet Security**](https://developer.chrome.com/server-wallets/v2/introduction/security): Learn more about the security features of the CDP v2 Server Wallet
-   [**v2 API Reference**](https://developer.chrome.com/api-reference/v2/rest-api/policy-engine/list-policies): Explore the API reference for CDP Policies