# idempotency

Idempotency ensures that an API request produces the same result regardless of how many times it is sent. This is particularly important for operations that modify state, like creating accounts or signing transactions, where duplicate requests could cause unintended side effects.

## How it works

The CDP APIs support idempotency through the `X-Idempotency-Key` header. When you include an idempotency key with a request, the API responds as follows:

-   Processes the request as normal if it’s the first use of the key within the last 24 hours
-   Returns the exact same response as the first request if the same request is retried with the same key
-   Returns an error if the same key is used with different request parameters

This mechanism ensures that temporary issues like network failures don’t result in duplicate operations.

## Using idempotency keys

The `X-Idempotency-Key` header must be a valid UUID v4 string. For example:

```
X-Idempotency-Key: 8e03978e-40d5-43e8-bc93-6894a57f9324

```

### Generating keys

Each unique request must use a new UUID v4. The API strictly enforces the UUID v4 format requirement and will reject any other format. Here’s how to generate a valid UUID v4 in different languages:

## Best practices

1.  **Generate new keys**: Always generate a new UUID v4 for each unique request.
2.  **Store keys**: Keep track of idempotency keys and their responses for retry scenarios.
3.  **Key lifetime**: Keys should be unique within a rolling 24-hour timeframe.
4.  **Retry logic**: Implement exponential backoff when retrying failed requests:

## Supported endpoints

You can identify endpoints that support idempotency by looking for the `X-Idempotency-Key` parameter in the documentation for the related endpoint. When present, this optional header parameter indicates that the endpoint supports idempotent requests. For example, in the [Create an EVM account](https://developer.chrome.com/api-reference/v2/rest-api/evm-accounts/create-an-evm-account) endpoint documentation, you’ll see a field that allows you to pass the `X-Idempotency-Key` header.

## Error handling

### Invalid idempotency key format

If you provide an idempotency key that is not a valid UUID v4, the API will reject the request:

```
{
  "errorType": "invalid_request",
  "errorMessage": "parameter \"X-Idempotency-Key\" in header has an error: must be a valid UUID v4 format (e.g., 8e03978e-40d5-43e8-bc93-6894a57f9324)"
}

```

### Idempotency key conflict

This occurs when you use the same idempotency key with different request parameters:

```
{
  "errorType": "idempotency_error",
  "errorMessage": "Idempotency key '8e03978e-40d5-43e8-bc93-6894a57f9324' was already used with a different request payload. Please try again with a new idempotency key."
}

```

### Already processing

This error occurs in certain highly-concurrent scenarios whereby you send multiple requests within a short period with the same idempotency key. Note that under normal circumstances, sending the same request with the same idempotency key within a 24-hour period will return the same response as the first request.

```
{
  "errorType": "already_exists",
  "errorMessage": "Another request with the same idempotency key is currently processing."
}

```

## Usage constraints

Idempotency keys are subject to the following limitations:

-   Keys must be valid UUID v4 strings
-   Keys should be unique within a 24-hour window
-   Duplicate requests with the same key count towards your API rate limits

## Security considerations

While idempotency keys aren’t sensitive like API keys, we recommend following these security practices:

-   Use cryptographically secure UUID v4 generators
-   Don’t use sequential or predictable keys (only UUID v4 format is accepted)
-   Don’t reuse keys across different operations
-   Store keys securely if you need to reference them later