# rate limits

The REST API throttles endpoints by IP address **AND** portfolio ID:

-   IP address at 100 requests per second (rps). If you exceed 100 rps, a 429 error is thrown and you have to wait 30 seconds before resuming.
-   Portfolio ID at 25 rps with a burst of 50 rps.