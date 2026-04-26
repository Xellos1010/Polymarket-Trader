# pagination

The Coinbase Prime REST API uses cursor pagination for most requests that return arrays. Endpoints that support pagination have a `cursor` query parameter, such as [List Portfolio Allocations](https://developer.chrome.com/api-reference/prime-api/rest-api/allocations/get-portfolio-allocations). For an end-to-end example of authoring pagination logic, see [Locate Order By Client Order ID](https://github.com/coinbase-samples/prime-scripts-py/blob/main/REST/prime_locate_order_by_client_order_id.py). Cursor pagination lets you fetch additional results after the current page of results, and is well suited for realtime data. To retrieve more results, subsequent requests should specify which direction to paginate based on the data previously returned. The JSON response for all relevant endpoints includes a `pagination` object with cursor information. If your initial request returns a pagination value with a non-null `next_cursor`, you can retrieve the next page of results by sending the same request with the `cursor=<next_cursor>` query parameter appended. Repeat until is `next_cursor=''`.

Object

Description

`next_cursor`

Cursor to navigate to next page

`sort_direction`

The page sorting direction, either `ASC` (ascending) or `DESC` (descending)

`has_next`

A boolean flag indicating whether there is more data available to page through

Parameter

Description

Default if not specified

`cursor`

Cursor to navigate to next page using `next_cursor` from response

`sort_direction`

The page sorting direction, either `ASC` (ascending) or `DESC` (descending)

`DESC`

### Example

For an end-to-end example of authoring pagination logic, see [Locate Order By Client Order ID](https://github.com/coinbase-samples/prime-scripts-py/blob/main/REST/prime_locate_order_by_client_order_id.py).