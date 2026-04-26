# list onchain address groups

List Onchain Address Groups

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/onchain_address_groups
```

```
{
  "address_groups": [
    {
      "id": "<string>",
      "name": "<string>",
      "network_type": "NETWORK_TYPE_UNSPECIFIED",
      "addresses": [
        {
          "name": "<string>",
          "address": "<string>",
          "chain_ids": [
            "<string>"
          ]
        }
      ],
      "added_at": "2023-11-07T05:31:56Z"
    }
  ]
}
```

Lists all onchain address groups for a given portfolio ID

GET

/

v1

/

portfolios

/

{portfolio\_id}

/

onchain\_address\_groups

List Onchain Address Groups

```
curl --request GET \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/onchain_address_groups
```

```
{
  "address_groups": [
    {
      "id": "<string>",
      "name": "<string>",
      "network_type": "NETWORK_TYPE_UNSPECIFIED",
      "addresses": [
        {
          "name": "<string>",
          "address": "<string>",
          "chain_ids": [
            "<string>"
          ]
        }
      ],
      "added_at": "2023-11-07T05:31:56Z"
    }
  ]
}
```

#### Path Parameters

#### Response