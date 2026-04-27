# error messages

## Overview

All error messages return both machine (`id`) and a human readable (`message`) message. Except for `validation_error`, all error messages return only one error. Some errors include a link to the documentation (`url`). When a `POST` or `PUT` request fails validation, a `validation_error` with status code `400` is returned. The response contains an `errors` field with a list of errors.

Error id

Code

Description

`two_factor_required`

400

When sending money over 2fa limit

`param_required`

400

Missing parameter

`validation_error`

400

Unable to validate POST/PUT

`invalid_request`

400

Invalid request

`personal_details_required`

400

User’s personal detail required to complete this request

`identity_verification_required`

400

Identity verification is required to complete this request

`jumio_verification_required`

400

Document verification is required to complete this request

`jumio_face_match_verification_required`

400

Document verification including face match is required to complete this request

`unverified_email`

400

User has not verified their email

`authentication_error`

401

Invalid auth (generic)

`invalid_token`

401

Invalid Oauth token

`revoked_token`

401

Revoked Oauth token

`expired_token`

401

Expired Oauth token

`invalid_scope`

403

User hasn’t authenticated necessary scope

`not_found`

404

Resource not found

`rate_limit_exceeded`

429

Rate limit exceeded

`unauthorized`

401

Not authorized to perform this operation.

`resource_exhausted`

429

Resource has been exhausted

`internal_server_error`

500

Internal server error

> Generic error response (4xx, 5xx)

```
{
  "errors": [
    {
      "id": "not_found",
      "message": "Not found"
    }
  ]
}

```

> Validation failed (400)

```
{
  "errors": [
    {
      "id": "validation_error",
      "message": "Please enter a valid email or bitcoin address"
    }
  ]
}

```

> Error with document link

```
{
  "errors": [
    {
      "id": "invalid_scope",
      "message": "Invalid scope",
      "url": "http://developers.coinbase.com/api#permissions"
    }
  ]
}

```

### Other errors

#### OAuth2

When authenticating or refreshing access tokens, OAuth2, will follow different error format.

```
{
  "error": "invalid_request",
  "error_description": "The request is missing a required parameter, includes an unsupported parameter value, or is otherwise malformed."
}

```