# quickstart

This quickstart walks through creating an API key, setting up the International Exchange Java SDK (INTX Java SDK), and making your first API call.

## Initial Setup

1.  **Generate an API Key:** From the web UI, navigate to API.
2.  **Authenticate:** Ensure you authenticate all API requests. Detailed guidance is available at [Authentication](https://developer.chrome.com/api-reference/international-exchange-api/rest-api/authentication).

## Using the INTX Java SDK

### Setting up the SDK

The INTX Java SDK is vended through Maven. To use these in your own project, install the dependencies using Maven, where the current version is `1.0.0`:

```
<dependency>
    <groupId>com.coinbase.intx</groupId>
    <artifactId>coinbase-intx-sdk-java</artifactId>
    <version>1.0.0</version>
</dependency>

```

To use this SDK, initialize the `Credentials` class and create a new client. The `Credentials` class is JSON enabled. Ensure that INTX API credentials are stored in a secure manner. The JSON format expected for credentials is:

```
INTX_CREDENTIALS='{ "accessKey": "", "passphrase": "", "signingKey": "", "portfolioId": ""}'

```

### Listing Portfolios

The following code snippet demonstrates how to make your first API call using the INTX Java SDK. This example lists all portfolios associated with your provided API key.

```
package com.coinbase.examples;
import com.coinbase.intx.client.CoinbaseIntxClient;
import com.coinbase.intx.credentials.CoinbaseIntxCredentials;
import com.coinbase.intx.errors.CoinbaseIntxException;
import com.coinbase.intx.factory.IntxServiceFactory;
import com.coinbase.intx.model.portfolios.*;
import com.coinbase.intx.portfolios.PortfoliosService;
import com.coinbase.intx.portfolios.PortfoliosServiceImpl;
import com.fasterxml.jackson.databind.ObjectMapper;
public class Main {
    public static void main(String[] args) throws CoinbaseIntxException {
        String credsStringBlob = System.getenv("INTX_CREDENTIALS");
        ObjectMapper mapper = new ObjectMapper();
        try {
            CoinbaseIntxCredentials credentials = new CoinbaseIntxCredentials(credsStringBlob);
            CoinbaseIntxClient client = new CoinbaseIntxClient(credentials);
            PortfoliosService portfoliosService = IntxServiceFactory.createPortfoliosService(client);
            ListPortfoliosResponse listResponse = portfoliosService.listPortfolios();
            System.out.println("List Portfolios Response:");
            System.out.println(mapper.writerWithDefaultPrettyPrinter().writeValueAsString(listResponse));
        } catch (Throwable e) {
            throw new CoinbaseIntxException("Failed to retrieve the list portfolios response", e);
        }
    }
}

```

### Creating an Order

This second example demonstrates how to use the INTX Java SDK to create a new order:

```
package com.coinbase.examples;
import com.coinbase.core.credentials.CoinbaseCredentials;
import com.coinbase.intx.errors.CoinbaseIntxException;
import com.coinbase.intx.model.orders.CreateOrderRequest;
import com.coinbase.intx.model.orders.CreateOrderResponse;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.coinbase.intx.client.CoinbaseIntxHttpClient;
public class Main {
    public static void main(String[] args) throws CoinbaseIntxException {
        String credsStringBlob = System.getenv("INTX_CREDENTIALS");
        ObjectMapper mapper = new ObjectMapper();
        try {
            CoinbaseCredentials credentials = new CoinbaseCredentials(credsStringBlob);
            CoinbaseIntxHttpClient client = new CoinbaseIntxHttpClient(credentials);
            CreateOrderRequest createReq = new CreateOrderRequest.Builder()
                    .clientOrderId("7f32e67e-8f7d-4b71-b22d-1c8b90b1aae4")
                    .side("BUY")
                    .size("0.005")
                    .tif("GTC")
                    .price("2000")
                    .portfolio("018a04e1-2006-7e1c-912e-23c2eb75f06f")
                    .instrument("ETH-USDC")
                    .type("LIMIT")
                    .build();
            CreateOrderResponse listResponse = client.createOrder(createReq);
            System.out.println("List Portfolios Response:");
            System.out.println(mapper.writerWithDefaultPrettyPrinter().writeValueAsString(listResponse));
        } catch (Throwable e) {
            throw new CoinbaseIntxException("Failed to retrieve the list portfolios response", e);
        }
    }
}

```