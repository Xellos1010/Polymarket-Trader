# rate limits

### FIX 5.0

-   2 logons per second per API key
-   100 requests per second

### FIX Maximums

-   Maximum API keys per session/connection: 1
-   Maximum connections per profile: 75. See [FIX Best Practices](https://developer.chrome.com/exchange/fix-api/best-practices).
-   Maximum connections per user across all profiles: 175
-   Maximum profiles per user: 100
-   Maximum orders per batch message (new and cancelled): 15

## How Rate Limits Work

Rate-limiting for both the Exchange REST API and the FIX API use a **lazy-fill token bucket** implementation. A TokenBucket stores a maximum amount of tokens, which is the **burst size**, and fills at a given rate called the **refresh rate**. The bucket starts full, and as requests are received, a token is removed for each request. Tokens are continuously added to the bucket at the refresh rate until full. When a user sends a request, the TokenBucket calculates whether to rate limit the user as follows:

1.  Fill the user’s TokenBucket to a token size based on the following formula: `token_amount = min(burst, previous_token_amount + (current_time - previous_request_time) * refresh_rate)`
2.  Remove 1 token if possible, otherwise rate limit the request.
3.  Repeat Steps 1 and 2 for each subsequent request.

### TokenBucket Example

Let’s say you have a TokenBucket with burst = 3 and refresh\_rate = 1. The table below represents the state of your token bucket after a series of requests:

Action

Time

Tokens

Notes

Initial State

0.0

3.0

New TokenBucket is initialized to max capacity (burst)

Request 1

0.5

2.0

Fill TokenBucket, then remove a token, because we are at max capacity, and subtract 1 token from 3

Request 2

0.8

1.3

Fill TokenBucket to 2.3 (`min(3, (2 + (.8 - .5) * 1.0)) = min(3, 2.3) = 2.3`), then subtract 1

Request 3

0.9

0.4

Fill TokenBucket to 1.4 (`min(3, (1.3 + (.9 - .8) * 1.0)) = min(3, 1.4) = 1.4`), then subtract 1

Request 4

1.0

0.5

Fill TokenBucket to 0.5 (`min(3, (.4 + (1.0 - .9) * 1.0)) = min(3, 0.5) = 0.5`). **Ratelimit** because we don’t have enough tokens available

Request 5

1.4

0.9

Fill TokenBucket to 0.9 (`min(3, (0.5 + (1.4 - 1.0) * 1.0)) = min(3, 0.9) = 0.9`). **Ratelimit** because we don’t have enough tokens available

Request 6

1.8

0.3

Fill TokenBucket to 1.3 (`min(3, (0.9 + (1.8 - 1.4) * 1.0)) = min(3, 1.3) = 1.3`), then remove 1

Request 7

5.0

2.0

Fill TokenBucket to 3.0 (`min(3, (0.3 + (5.0 - 1.8) * 1.0)) = min(3, 3.5) = 3`), since we would “overflow” with our calculations, then subtract 1