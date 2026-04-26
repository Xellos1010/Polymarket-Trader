# get sql grammar

Retrieve the SQL grammar for the SQL API.

The SQL queries that are supported by the SQL API are defined via an ANTLR4 grammar which is evaluated by server before executing the query. This ensures the safety and soundness of the SQL API.

This endpoint returns the ANTLR4 grammar that is used to evaluate the SQL queries so that developers can understand the SQL API and build SQL queries with high confidence and correctness. LLMs interact well with ANTLR4 grammar as well.

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Generate Bearer Token](https://docs.cdp.coinbase.com/api-reference/v2/authentication#2-generate-bearer-token) section of our Authentication docs for information on how to generate your Bearer Token.

#### Response

SQL grammar retrieved successfully.

The ANTLR4 grammar for the SQL API.

Example:

`"grammar SqlQuery; query: cteClause? unionStatement SEMICOLON? EOF;"`