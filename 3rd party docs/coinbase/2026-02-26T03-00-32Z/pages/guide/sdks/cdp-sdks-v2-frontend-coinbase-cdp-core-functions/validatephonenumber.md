# validatephonenumber

```
function validatePhoneNumber(phoneNumber: string): void;

```

Validates a phone number is in E.164 format. E.164 format requirements:

-   Starts with +
-   Followed by country code (1-3 digits)
-   Followed by subscriber number
-   Total length: 1-15 digits (excluding the + prefix)

## Parameters

Parameter

Type

Description

`phoneNumber`

`string`

The phone number to validate.

## Returns

`void`

## Throws

Error if the phone number is not in valid E.164 format.

## Example

```
// Valid formats
validatePhoneNumber("+14155552671");  // US
validatePhoneNumber("+442071838750"); // UK
validatePhoneNumber("+81312345678");  // Japan
// Invalid formats (will throw)
validatePhoneNumber("4155552671");     // Missing +
validatePhoneNumber("+1-415-555-2671"); // Contains hyphens
validatePhoneNumber("+0412345678");    // Starts with 0

```