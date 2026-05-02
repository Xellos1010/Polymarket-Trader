# rate limits

The CDP API enforces rate limits to ensure fair usage and system stability. Rate limits are calculated using rolling 10-second windows and are applied separately for read and write operations.

## Rate limit structure

Operation Type

Rate Limit

Time Window

Write APIs

500 requests

10 seconds

Read APIs

600 requests

10 seconds

### Write operations

Write operations include any API endpoints that modify resources (`POST`, `PUT`, `DELETE` methods). These are limited to **500 requests per 10 seconds**.

### Read operations

Read operations include any API endpoints that retrieve data (`GET` methods). These are limited to **600 requests per 10 seconds**.

## Rate limit response

When you exceed the rate limit, the API will respond with a `429` (“Too Many Requests”) status code. The response will include an error object with details about the rate limit violation:

```
{
  "errorType": "rate_limit_exceeded",
  "errorMessage": "Rate limit exceeded. Please try again later."
}

```

## Best practices

To effectively work with these rate limits, consider implementing the following best practices:

### Exponential backoff

You should implement an exponential backoff strategy to handle rate limit errors.

More on exponential backoff

**Exponential backoff** is a strategy used to retry requests after a failure, increasing the delay between retries each time. This is especially useful for rate limits and temporary server errors.

```
async function withRetry(fn: () => Promise<any>) {
   let delay = 1000;
   while (true) {
      try {
      return await fn();
      } catch (e) {
      if (e.errorType === "rate_limit_exceeded") {
        await sleep(delay); // wait before retrying
        delay *= 2; // increase delay each time
        continue;
       }
      throw e;
      }
   }
}

```

### Cache responses

Caching improves performance and reduces unnecessary load on your server or APIs. You should:

-   Cache frequently-accessed data locally when possible
-   Implement response caching for read operations that don’t require real-time data

### Batch Operations

Instead of making many individual requests, combine them into a single call and use bulk endpoints when possible.

### Monitor usage

Given our rate limit structure, you should track your API usage to stay within limits, and set up alerts before reaching rate limit thresholds.

### Rate Limits

Consider the following when designing your application around rate limits:

#### Optimize API usage

-   Only request the data you need
-   Use pagination for large data sets
-   Implement efficient polling strategies

#### Handle rate limits gracefully

-   Implement proper error handling for [429 responses](#rate-limit-response)
-   Add retry logic with [exponential backoff](#exponential-backoff)
-   Consider using a queue system for high-volume operations

### Plan for scale

-   Design your application architecture to work within these limits
-   Consider [caching strategies](#cache-responses) for frequently-accessed data
-   Distribute requests evenly across [time windows](#rate-limit-structure) when possible

## Support

If you consistently hit rate limits and need higher limits for your use case, please reach out to us in the **#wallet-api** channel of the [CDP Discord](https://discord.com/invite/cdp) to discuss your requirements.