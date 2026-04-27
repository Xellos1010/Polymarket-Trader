# time

## 

[​](#table-of-endpoints)

Table of Endpoints

Name

Method

Endpoint

Scope

[Get Current Time](#get-current-time)

GET

`/v2/time`

N/A

## 

[​](#get-current-time)

Get Current Time

Get the API server time. **This endpoint doesn’t require authentication.**

### 

[​](#http-request)

HTTP Request

`GET https://api.coinbase.com/v2/time`

### 

[​](#scopes)

Scopes

-   *No permission required*

### 

[​](#examples)

Examples

#### 

[​](#request)

Request

```
curl https://api.coinbase.com/v2/time

```

#### 

[​](#response)

Response

```
{
  "data": {
    "iso": "2015-06-23T18:02:51Z",
    "epoch": 1435082571
  }
}

```