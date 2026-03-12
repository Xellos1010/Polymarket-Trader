# delete webhook subscription

Delete webhook subscription

Permanently delete a transfer webhook subscription and stop all event deliveries. This action cannot be undone.

### Important Notes

-   All transfer webhook deliveries will cease immediately
-   Subscription cannot be recovered after deletion
-   Consider disabling instead of deleting for temporary pauses

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

Unique identifier for the webhook subscription.

#### Response

Webhook subscription deleted successfully.