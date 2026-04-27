# pagination

## Overview

```
curl https://api.coinbase.com/v2/accounts \
  -H 'Authorization: Bearer abd90df5f27a7b170cd775abf89d632b350b7c1c9d53e08b340cd9832ce52c2c'

```

> Example response

```
{
  "pagination": {
    "ending_before": null,
    "starting_after": null,
    "limit": 25,
    "order": "desc",
    "previous_uri": null,
    "next_uri": "/v2/accounts?&limit=25&starting_after=5d5aed5f-b7c0-5585-a3dd-a7ed9ef0e414"
  },
  "data": [
    ...
  ]
}

```

`GET` endpoints that return an object list support cursor based pagination, with pagination information inside a `pagination` object. This means that to get all objects, you must paginate through the results by always using the `id` of the last resource in the list as a `starting_after` parameter for the next call. To make it easier, the API constructs the next call into `next_uri` together with all the currently used pagination parameters. You know that you have paginated all the results when the response’s `next_uri` is empty. Cursor based pagination protects you from the situation where the resulting object list changes during pagination (new resource gets added or removed). The default `limit` is set to 25, but values up to 100 are permitted. Due to permissions and access level control, the response list might in some cases return less objects than specified by the `limit` parameter. This is normal behaviour and should be expected. The result list is in descending order by default (newest item first) but it can be reversed by supplying `order=asc` instead.

### Arguments

Parameter

Description

`limit` *optional*

Number of results per call. Accepted values: 0 - 100 (default 25)

`order` *optional*

Result order. Accepted values: `asc`, `desc` (default)

`starting_after` *optional*

Pagination cursor. Resource ID that defines your place in the list.

`ending_before` *optional*

Pagination cursor. Resource ID that defines your place in the list.