# authentication

This page explains how to sign and authenticate REST API endpoints. For Prime FIX connectivity, refer to [FIX API Connectivity](https://developer.chrome.com/prime/fix-api/connectivity) on specific authentication scheme details.

## Generating an API Key

All API endpoints require authentication to access. You will need to create an API key via the Coinbase Prime web UI to interact with these resources. API keys created in the Prime UI are scoped to individual portfolios.

## Signing Requests

Prime REST API requests must include an access signature header:

-   `X-CB-ACCESS-KEY`: The API key as a string
-   `X-CB-ACCESS-PASSPHRASE`: The Passphrase shown when creating the API key
-   `X-CB-ACCESS-SIGNATURE`: The base64-encoded signature
-   `X-CB-ACCESS-TIMESTAMP`: A timestamp for your request

### Selecting a Timestamp

The `X-CB-ACCESS-TIMESTAMP` header must be number of seconds since [Unix Epoch](http://en.wikipedia.org/wiki/Unix_time) in UTC. Decimal values are not allowed, so make sure to use an integer. Your timestamp must be within 30 seconds of your request, or it will be considered expired and be rejected.

### Creating a Signature

To generate the `X-CB-ACCESS-SIGNATURE`, first compute the SHA256 HMAC of the concatenated message string using the secret key. The process for GET requests may look like the following:

```
message = timestamp + 'GET' + url_path
hmac_message = hmac.digest(SECRET_KEY.encode(), message.encode(), hashlib.sha256)

```

Then, base64-encode the HMAC output to create the signature value for the API request header. In Python, this can be accomplished with the following line:

```
base64.b64encode(hmac_message)

```

### Signature Examples

The following end-to-end examples demonstrate how to generate a signature in [Python](#python), [Ruby](#ruby), and [JavaScript](#javascript):

##### Python GET Request

```
uri = 'https://api.prime.coinbase.com/v1/portfolios'
url_path = urlparse(uri).path
timestamp = str(int(time.time()))
message = timestamp + 'GET' + url_path
signature_b64 = base64.b64encode(hmac.digest(SECRET_KEY.encode(), message.encode(), hashlib.sha256))

```

##### Python POST Request

```
uri = f'https://api.prime.coinbase.com/v1/portfolios/{PORTFOLIO_ID}/order'
url_path = urlparse(uri).path
timestamp = str(int(time.time()))
message = timestamp + 'POST' + url_path + json.dumps(payload)
signature_b64 = base64.b64encode(hmac.digest(SECRET_KEY.encode(), message.encode(), hashlib.sha256))

```

##### Ruby

```
timestamp = Time.now.to_i
what = "#{timestamp}#{method}#{request_path}#{body}";
# create a sha256 hmac with the secret
secret = Base64.decode64(@secret)
hash = OpenSSL::HMAC.digest('sha256', secret, what)
Base64.strict_encode64(hash)

```

##### JavaScript

```
// Function to generate a signature using Google's crypto-js package
function sign(str, secret) {
  const hash = CryptoJS.HmacSHA256(str, secret);
  return hash.toString(CryptoJS.enc.Base64);
}
// Function to build the payload required to sign
function buildPayload(ts, method, requestPath, body) {
  return `${ts}${method}${requestPath}${body}`;
}
// Build the string we want to sign using information defined above
const strToSign = buildPayload(
  Math.floor(Date.now() / 1000),
  "GET",
  "/v1/portfolios/<YOUR_PORTFOLIO_ID_HERE>/orders",
  "",
);
// Sign it!
const sig = sign(strToSign, SIGNING_KEY);

```