# remove firm product group limit 1

Remove Firm Product Group Limit

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/v2/firm-product-group-limit/remove \
  --header 'Content-Type: application/json' \
  --data '
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_group": "CRYPTO"
}
'
```

```
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_group": "CRYPTO",
  "trading_disabled": true
}
```

Firm Product Group Limits

Removes any product group level limit for a specific firm/product group combination

POST

/

rest

/

v2

/

firm-product-group-limit

/

remove

Remove Firm Product Group Limit

```
curl --request POST \
  --url https://api.exchange.fairx.net/rest/v2/firm-product-group-limit/remove \
  --header 'Content-Type: application/json' \
  --data '
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_group": "CRYPTO"
}
'
```

```
{
  "firm_uuid": "f81d4fae-7dec-11d0-a765-00a0c91e6bf6",
  "product_group": "CRYPTO",
  "trading_disabled": true
}
```

#### Body

Product group limit removal parameters

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

Available options

:

`CURRENCY`,

`EQUITY`,

`ENERGY`,

`METALS`,

`INTEREST_RATE`,

`AGRICULTURE`,

`CRYPTO`

#### Response

Successfully removed firm product group limit

Example:

`"f81d4fae-7dec-11d0-a765-00a0c91e6bf6"`

Available options

:

`CURRENCY`,

`EQUITY`,

`ENERGY`,

`METALS`,

`INTEREST_RATE`,

`AGRICULTURE`,

`CRYPTO`