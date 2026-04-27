# errors

An error message displays when the client is actively disconnected for any of these reasons:

-   The client has too many backed up messages (`ErrSlowConsume`).
-   The client is sending too many messages (`ErrSlowRead`).
-   The message size is too large (`Message too big`)
-   There are intermittent network issues.

Most failure cases trigger an `error` message—specifically, a message with the `type` `"error"`. This can be helpful when implementing a client or debugging issues.

```
{
  "type": "error",
  "message": "error message"
  /* ... */
}

```