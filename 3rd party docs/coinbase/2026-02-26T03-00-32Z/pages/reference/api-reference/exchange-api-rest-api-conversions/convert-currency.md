# convert currency

```
curl --request POST \
  --url https://api.exchange.coinbase.com/conversions \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "from": "<string>",
  "to": "<string>",
  "amount": "<string>",
  "profile_id": "<string>",
  "nonce": "<string>",
  "idem": "<string>"
}
'
```

```
{
  "id": "c5aaf125-d99e-41fe-82ea-ad068038b278",
  "amount": "11.00000000",
  "from_account_id": "5dcc143c-fb96-4f72-aebf-a165e3d29b53",
  "to_account_id": "6100247f-90fc-4335-ac17-d99839f0c909",
  "from": "USDC",
  "to": "USD",
  "fee_amount": "0.0000000000000000"
}
```

Converts funds from `from` currency to `to` currency. Funds are converted on the `from` account in the `profile_id` profile.

```
curl --request POST \
  --url https://api.exchange.coinbase.com/conversions \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "from": "<string>",
  "to": "<string>",
  "amount": "<string>",
  "profile_id": "<string>",
  "nonce": "<string>",
  "idem": "<string>"
}
'
```

```
{
  "id": "c5aaf125-d99e-41fe-82ea-ad068038b278",
  "amount": "11.00000000",
  "from_account_id": "5dcc143c-fb96-4f72-aebf-a165e3d29b53",
  "to_account_id": "6100247f-90fc-4335-ac17-d99839f0c909",
  "from": "USDC",
  "to": "USD",
  "fee_amount": "0.0000000000000000"
}
```

**Caution**Users whose USD and USDC accounts are unified do not have access to the conversion endpoint, and conversions from USDC to USD are automatic upon deposit.

## API Key Permissions

This endpoint requires the “trade” permission.

## Response

A successful conversion is assigned a conversion ID. The corresponding ledger entries for a conversion reference this conversion ID.

#### Authorizations

#### Body

#### Response