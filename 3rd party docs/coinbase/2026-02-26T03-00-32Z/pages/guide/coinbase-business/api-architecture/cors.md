# cors

The Coinbase Business API v2 supports cross-origin HTTP requests, commonly referred as [CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/Access_control_CORS). This means that you can call API resources using Javascript from any browser. While CORS allows for many interesting use cases, it’s important to remember that you should never expose private API keys to 3rd parties. CORS is mainly useful with unauthenticated endpoints (e.g., Bitcoin price information) and OAuth2 client side applications.

## What is Cross-Origin Resource Sharing (CORS)?

Cross-Origin Resource Sharing (CORS) is an HTTP-header-based mechanism that allows a server to indicate which origins (domains, schemes, or ports) are permitted to access its resources. CORS overcomes the browser’s **same-origin policy**, which restricts web applications from accessing resources hosted on a different origin for security reasons. For example, a frontend app hosted on `https://domain-a.com` may need to fetch resources from an API hosted at `https://domain-b.com`. With CORS enabled, the browser permits this interaction if the API server provides the appropriate CORS headers.

## Functional Overview of CORS

CORS works by adding new HTTP headers that let servers describe the origins and methods permitted for resource access. For requests with potential side effects on server data (e.g., POST or DELETE), the browser performs a **preflight request** using the `OPTIONS` method to ensure the server approves the actual request.

## How CORS Works with Coinbase API v2

The Coinbase Business API v2 uses CORS to allow secure, client-side access to its endpoints directly from browsers, enabling various use cases like fetching live cryptocurrency prices. Here’s how it simplifies integration:

-   **Ease of Use**: JavaScript can directly fetch API data without needing server-side intermediaries.
-   **Support for Modern Applications**: Suitable for building real-time dashboards or widgets.

1.  **Access-Control-Allow-Origin**: Specifies which origin(s) are permitted access. Example:
    
    ```
    Access-Control-Allow-Origin: https://your-domain.com
    
    ```
    
    A value of `*` allows access from all origins but is unsuitable for credentialed requests.
2.  **Access-Control-Allow-Methods**: Lists the HTTP methods permitted for cross-origin requests:
    
    ```
    Access-Control-Allow-Methods: GET, POST, OPTIONS
    
    ```
    
3.  **Access-Control-Allow-Headers**: Specifies the custom headers allowed in the request:
    
    ```
    Access-Control-Allow-Headers: X-Custom-Header, Content-Type
    
    ```
    
4.  **Access-Control-Allow-Credentials**: Indicates whether credentials (cookies or authentication tokens) can be included:
    
    ```
    Access-Control-Allow-Credentials: true
    
    ```
    
5.  **Access-Control-Max-Age**: Indicates how long the preflight response can be cached:
    
    ```
    Access-Control-Max-Age: 86400
    
    ```
    

## Security Considerations

1.  **Never Expose Private API Keys**: Storing private keys in client-side code risks unauthorized access.
    -   Use server-side authentication for sensitive operations.
    -   Leverage OAuth2 for user authorization.
2.  **Understand the Risks of Wildcards (`*`)**:
    -   Using `*` for `Access-Control-Allow-Origin` can lead to unintended exposure.
    -   For credentialed requests, explicitly specify the origin.
3.  **Use Preflight Requests for Sensitive Actions**:
    -   Preflight ensures the server explicitly approves actions like modifying data (e.g., POST or DELETE).

## Common CORS Scenarios

1.  **Simple Requests**:
    -   Simple requests (GET, POST with safe headers) do not require preflight.
    -   Example:
        
        ```
        fetch("https://api.coinbase.com/v2/prices/BTC-USD/spot")
          .then((response) => response.json())
          .then((data) => console.log(data));
        
        ```
        
2.  **Preflighted Requests**:
    -   Required when using custom headers, non-simple methods, or credentials.
    -   Example:
        
        ```
        fetch("https://api.coinbase.com/v2/data", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            "X-Custom-Header": "value",
          },
          body: JSON.stringify({ key: "value" }),
        }).then((response) => console.log(response));
        
        ```
        
3.  **Requests with Credentials**:
    -   To send cookies or authentication tokens:
        
        ```
        fetch("https://api.coinbase.com/v2/secure-data", {
          credentials: "include",
        }).then((response) => console.log(response));
        
        ```
        

## Common Errors and Troubleshooting

-   **Error**: `"No 'Access-Control-Allow-Origin' header is present"`
    -   **Solution**: Verify the API server includes the correct CORS headers.
-   **Error**: `"CORS preflight did not succeed"`
    -   **Solution**: Ensure the server handles `OPTIONS` requests correctly.
-   **Error**: `"The request was redirected but preflight does not allow redirects"`
    -   **Solution**: Avoid redirects or rewrite the client request to point to the final URL.

## Real-World Use Cases with Coinbase API

-   **Cryptocurrency Dashboard**: Build an app that fetches live Bitcoin prices for a portfolio tracker.
-   **Interactive Charts**: Use API data for real-time price visualizations.
-   **Secure Transactions**: Implement OAuth2 workflows for client-side apps.

## Additional Resources

For more information or assistance, feel free to reach out to us on the [CDP Discord](https://discord.com/invite/cdp).