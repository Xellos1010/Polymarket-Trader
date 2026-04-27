# requests

## Creating a Request

Requests and responses follow standard HTTP response status codes for success and failures. Request bodies should have content type `application/json` and be valid JSON. For instructions on creating a valid `X-CB-ACCESS-SIGNATURE`, see [API Authentication](https://developer.chrome.com/prime/rest-api/authentication).

```
curl --request GET \
    --url 'https://api.prime.coinbase.com/v1/<API_PATH>' \
    --header 'X-CB-ACCESS-KEY: <ACCESS_KEY>' \
    --header 'X-CB-ACCESS-PASSPHRASE: <PASSPHRASE>' \
    --header 'X-CB-ACCESS-SIGNATURE: <SIGNATURE>' \
    --header 'X-CB-ACCESS-TIMESTAMP: <TIMESTAMP>' \
    --header 'Content-Type: application/json'

```

### Required IDs

Many API requests require either a `portfolio_id` or an `entity_id` value:

-   Your `portfolio_id` can be retrieved by calling [List Portfolios](https://developer.chrome.com/api-reference/prime-api/rest-api/portfolios/list-portfolios).
-   Your `entity_id` can be retrieved by calling [List Portfolios](https://developer.chrome.com/api-reference/prime-api/rest-api/portfolios/list-portfolios), or [List Portfolio Users](https://developer.chrome.com/api-reference/prime-api/rest-api/users/list-portfolio-users) with your `portfolio_id`.

### Sample Requests

The following Python and JavaScript snippets demonstrate an end-to-end Prime API request.

#### Python

```
from urllib.parse import urlparse
import json, hmac, hashlib, time, os, base64, requests
API_KEY = os.environ.get('ACCESS_KEY')
SECRET_KEY = os.environ.get('SIGNING_KEY')
PASSPHRASE = os.environ.get('PASSPHRASE')
uri = 'https://api.prime.coinbase.com/v1/portfolios'
url_path = urlparse(uri).path
timestamp = str(int(time.time()))
message = timestamp + 'GET' + url_path
signature_b64 = base64.b64encode(hmac.digest(SECRET_KEY.encode(), message.encode(), hashlib.sha256))
headers = {
  'X-CB-ACCESS-SIGNATURE': signature_b64,
  'X-CB-ACCESS-timestamp': timestamp,
  'X-CB-ACCESS-KEY': API_KEY,
  'X-CB-ACCESS-PASSPHRASE': PASSPHRASE,
  'Accept': 'application/json'
}
response = requests.get(uri, headers=headers)
parsed_response = json.loads(response.text)
print(json.dumps(parsed_response, indent=3))

```

#### JavaScript

```
// native NodeJS https module
const https = require("https");
// Google's crypto-js package via https://www.npmjs.com/package/crypto-js
const CryptoJS = require("crypto-js");
// Derived from your Coinbase Prime API Key
// SIGNING_KEY: the signing key provided as a part of your API key
//  ACCESS_KEY: the access key provided as a part of your API key
//  PASSPHRASE: the PASSPHRASE key provided as a part of your API key
const SIGNING_KEY = process.env.SIGNING_KEY;
const ACCESS_KEY = process.env.ACCESS_KEY;
const PASSPHRASE = process.env.PASSPHRASE;
const REST_METHODS = {
  GET: "GET",
  POST: "POST",
  PUT: "PUT",
  DELETE: "DELETE",
};
// Your unique entity ID
const ENTITY_ID = process.env.ENTITY_ID;
// A specific portfolio ID (only necessary if relevant to the request you're making)
const PORTFOLIO_ID = process.env.PORTFOLIO_ID;
// The base URL of the API
const PROD_URL = "api.prime.coinbase.com";
// The path of the API endpoint being called
let requestPath = `/v1/portfolios/${PORTFOLIO_ID}`;
// The method of the request: GET, POST, PUT, DELETE, etc
let method = REST_METHODS.GET;
// Request signatures require a current UNIX timestamp in seconds that is
// embedded in the signed payload to verify against server clock.
const currentTimeInSecs = Math.floor(Date.now() / 1000);
// Body will be JSON (POST) or empty string (GET)
const body = "";
// Function to generate a signature using CryptoJS
function sign(str, secret) {
  const hash = CryptoJS.HmacSHA256(str, secret);
  return hash.toString(CryptoJS.enc.Base64);
}
// Function to build the payload required to sign
function buildPayload(ts, method, requestPath, body) {
  return `${ts}${method}${requestPath}${body}`;
}
// Build the string we want to sign using information defined above
const strToSign = buildPayload(currentTimeInSecs, method, requestPath, body);
// Sign it!
const sig = sign(strToSign, SIGNING_KEY);
// Use Postman's scripting objects to append the header values
const headers = new Map();
headers.set("X-CB-ACCESS-KEY", ACCESS_KEY);
headers.set("X-CB-ACCESS-PASSPHRASE", PASSPHRASE);
headers.set("X-CB-ACCESS-SIGNATURE", sig);
headers.set("X-CB-ACCESS-TIMESTAMP", currentTimeInSecs);
headers.set("Content-Type", "application/json");
const requestOptions = {
  hostname: PROD_URL,
  path: requestPath,
  method: REST_METHODS.GET,
  headers: Object.fromEntries(headers),
};
https
  .get(requestOptions, (res) => {
    let data = [];
    console.log("Status Code:", res.statusCode);
    res.on("data", (chunk) => {
      data.push(chunk);
    });
    res.on("end", () => {
      console.log("Response ended: ");
      const parsedResponse = JSON.parse(Buffer.concat(data).toString());
      console.log(parsedResponse);
    });
  })
  .on("error", (err) => {
    console.log("Error: ", err.message);
  });

```

## Errors

Unless otherwise stated, bad requests respond with HTTP `4xx` or `5xx` status codes. The body also contains a `message` parameter indicating the cause. Your implementation language’s HTTP library should be configured to provide message bodies for non-`2xx` requests so that you can read the message field from the body.

### Common error codes

Error Code

Meaning

Description

400

Bad Request

Invalid request format

401

Unauthorized

Invalid API Key

403

Forbidden

You do not have access to the requested resource

404

Not Found

Requested resource could not be found

429

Too Many Requests

Too many requests in a given amount of time

500

Internal Server Error

Server-side error occurred

## Success

A successful response is indicated by HTTP status code `200` and may contain an optional body. If the response has a body, it will be documented under the related endpoint resource below.