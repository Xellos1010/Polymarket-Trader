# versioning

All API calls should be made with a `CB-VERSION` header which guarantees that your call is using the correct API version. Version is passed in as a date (UTC) of the implementation in `YYYY-MM-DD` format. If no version is passed, the version from the user’s [CDP API settings](https://portal.cdp.coinbase.com/access/oauth) will be used and a warning will be shown.

Do not pass in the current date, as that will return the current version which might break your implementation.

```
curl https://api.coinbase.com/v2/accounts \
    -H "CB-VERSION: 2022-01-06" \
    -H "Authorization: Bearer abd90df5f27a7b170cd775abf89d632b350b7c1c9d53e08b340cd9832ce52c2c"

```