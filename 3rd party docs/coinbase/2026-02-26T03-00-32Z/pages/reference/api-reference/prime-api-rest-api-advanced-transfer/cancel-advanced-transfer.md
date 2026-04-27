# cancel advanced transfer

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/advanced_transfers/{advanced_transfer_id}/cancel \
  --header 'Content-Type: application/json' \
  --data '{}'
```

```
{
  "advanced_transfer_id": "<string>"
}
```

Cancel advanced transfer for a given portfolio. This API is currently not available to all clients. Please reach out to Prime Operations with any questions.

POST

/

v1

/

portfolios

/

{portfolio\_id}

/

advanced\_transfers

/

{advanced\_transfer\_id}

/

cancel

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/advanced_transfers/{advanced_transfer_id}/cancel \
  --header 'Content-Type: application/json' \
  --data '{}'
```

```
{
  "advanced_transfer_id": "<string>"
}
```

#### Path Parameters

The ID of the canceled Advanced Transfer

#### Body

CancelAdvancedTransferRequest is the request to cancel an advanced transfer.

#### Response

CancelAdvancedTransferResponse is the response after canceling an advanced transfer.

The ID of the canceled Advanced Transfer