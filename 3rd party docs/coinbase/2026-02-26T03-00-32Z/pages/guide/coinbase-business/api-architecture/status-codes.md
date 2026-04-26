# status codes

Coinbase Business supports the following status codes: [Success Messages](https://www.rfc-editor.org/rfc/rfc9110.html#name-successful-2xx)

-   `200 OK` Successful request
-   `201 Created` New object saved
-   `204 No content` Object deleted

[Client Errors](https://www.rfc-editor.org/rfc/rfc9110.html#name-client-error-4xx)

-   `400 Bad Request` Returns JSON with the error message
-   `401 Unauthorized` Couldn’t authenticate your request
-   `402 2FA Token required` Re-try request with user’s 2FA token as `CB-2FA-Token` header
-   `403 Invalid scope` User hasn’t authorized necessary scope
-   `404 Not Found` No such object
-   `429 Too Many Requests` Your connection is being rate limited

[Server Errors](https://www.rfc-editor.org/rfc/rfc9110.html#name-server-error-5xx)

-   `500 Internal Server Error` Something went wrong
-   `503 Service Unavailable` Your connection is being throttled or the service is down for maintenance