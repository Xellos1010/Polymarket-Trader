# troubleshooting

## Common Issues

401 Unauthorized Errors

**Problem**: Getting authentication errors when calling the API**Solutions**:

-   Verify you’re using the correct API keys for your environment (sandbox vs production)
-   Check that the API key has the required permissions
-   Ensure the `Authorization` header is properly formatted
-   Confirm the API key hasn’t expired

Rate Limiting

**Problem**: Hitting rate limits during testing or production use**Rate Limit**: 100 requests per minute (RPM)**Solutions**:

-   Implement exponential backoff in your code
-   Add delays between rapid consecutive requests
-   Cache responses when appropriate
-   Contact support if you need higher limits for testing

Transfers Stuck in Pending

**Problem**: Transfers not completing**Solutions**:

-   Check the transfer amount (some amounts trigger delays)
-   Verify account IDs are valid accounts
-   Ensure you’ve called the execute endpoint
-   Review transfer status for error messages

Environment Mismatch

**Problem**: Code works in sandbox but fails in production**Solutions**:

-   Verify all configuration uses environment variables
-   Check for hardcoded sandbox-specific values
-   Ensure production API keys have correct permissions
-   Review any differences in account setup

Invalid Request Errors

**Problem**: Receiving 400 Bad Request responses**Solutions**:

-   Review the [API Conventions](https://developer.chrome.com/api-reference/payment-apis/conventions) for correct request formatting
-   Check the [Errors](https://developer.chrome.com/api-reference/payment-apis/errors) page for specific error codes
-   Validate your request payload matches the expected schema
-   Ensure all required fields are provided

## Getting Help

If you encounter issues with the CDP Payments API:

## Additional Resources