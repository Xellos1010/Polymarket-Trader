# rename profile

```
curl --request PUT \
  --url https://api.exchange.coinbase.com/profiles/{profile_id} \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "profile_id": "<string>",
  "name": "<string>"
}
'
```

```
{
  "id": "8058d771-2d88-4f0f-ab6e-299c153d4308",
  "user_id": "5cf6e115aaf44503db300f1e",
  "name": "default",
  "active": true,
  "is_default": true,
  "created_at": "2019-06-04T21:22:32.226Z"
}
```

Rename a profile. Names ‘default’ and ‘margin’ are reserved.

```
curl --request PUT \
  --url https://api.exchange.coinbase.com/profiles/{profile_id} \
  --header 'Content-Type: application/json' \
  --header 'cb-access-key: <api-key>' \
  --header 'cb-access-passphrase: <api-key>' \
  --header 'cb-access-sign: <api-key>' \
  --header 'cb-access-timestamp: <api-key>' \
  --data '
{
  "profile_id": "<string>",
  "name": "<string>"
}
'
```

```
{
  "id": "8058d771-2d88-4f0f-ab6e-299c153d4308",
  "user_id": "5cf6e115aaf44503db300f1e",
  "name": "default",
  "active": true,
  "is_default": true,
  "created_at": "2019-06-04T21:22:32.226Z"
}
```

#### Authorizations

#### Path Parameters

#### Body

#### Response

created\_at

string<date-time>

required