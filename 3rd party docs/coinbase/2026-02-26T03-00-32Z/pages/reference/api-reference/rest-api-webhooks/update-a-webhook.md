# update a webhook

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

The Webhook id that needs to be updated

Example:

`"582307c2f9e1fac308a5f575"`

#### Body

Filter for wallet activity events. This filter allows the client to specify one or more wallet addresses to monitor for activities such as transactions, transfers, or other types of events that are associated with the specified addresses.

-   Option 1
    
-   Option 2
    

Webhook will monitor all events that matches any one of the event filters.

The Webhook uri that updates to

Example:

`"https://webhook.site/582307c2f9e1fac308a5f575"`

The status of the webhook.

Available options

:

`active`,

`inactive`

#### Response

Webhook that is used for getting notifications when monitored events occur.

The status of the webhook.

Available options

:

`active`,

`inactive`

Identifier of the webhook.

Example:

`"582307c2f9e1fac308a5f575"`

Blockchain network identifier.

Available options

:

`unspecified`,

`erc20_transfer`,

`erc721_transfer`,

`wallet_activity`,

`smart_contract_event_activity`

Filter for wallet activity events. This filter allows the client to specify one or more wallet addresses to monitor for activities such as transactions, transfers, or other types of events that are associated with the specified addresses.

-   Option 1
    
-   Option 2
    

Webhook will monitor all events that matches any one of the event filters.

The URL to which the notifications will be sent.

Example:

`"https://webhook.site/582307c2f9e1fac308a5f575"`

The date and time the webhook was created.

Example:

`"2024-07-21T17:32:28Z"`

The date and time the webhook was last updated.

Example:

`"2024-07-21T17:32:28Z"`

The header that will contain the signature of the webhook payload.

Example:

`"your-signature-header"`