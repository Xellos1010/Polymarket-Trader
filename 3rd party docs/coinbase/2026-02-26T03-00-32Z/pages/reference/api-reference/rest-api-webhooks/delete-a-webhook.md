# delete a webhook

```
curl --request DELETE \
  --url https://api.cdp.coinbase.com/platform/v1/webhooks/{webhook_id} \
  --header 'Authorization: Bearer <token>'
```

```
{
  "code": "<string>",
  "message": "<string>",
  "correlation_id": "<string>"
}
```

Delete a webhook

DELETE

/

v1

/

webhooks

/

{webhook\_id}

```
curl --request DELETE \
  --url https://api.cdp.coinbase.com/platform/v1/webhooks/{webhook_id} \
  --header 'Authorization: Bearer <token>'
```

```
{
  "code": "<string>",
  "message": "<string>",
  "correlation_id": "<string>"
}
```

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The Webhook uuid that needs to be deleted

Example:

`"582307c2f9e1fac308a5f575"`

#### Response