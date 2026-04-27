# quickstart

This quickstart walks through creating an API key, setting up the Prime Go SDK, and making your first few API calls.

## Initial Setup

1.  **Create a Coinbase Prime Account:** Sign up at [Coinbase Prime](https://prime.coinbase.com/).
2.  **Generate an API Key:** From the web UI, navigate to Settings -> APIs.
3.  **Authenticate:** Ensure you authenticate all API requests. Detailed guidance is available at [API Authentication](https://developer.chrome.com/prime/rest-api/authentication).

## Using the Prime SDKs

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TypeScript
    

### Installation

The [Coinbase Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java) supports Java versions 11+.Check your Java version:

#### Install the Maven Dependency

```
<dependency>
    <groupId>com.coinbase.prime</groupId>
    <artifactId>coinbase-prime-sdk-java</artifactId>
   <version>1.0.0</version>
</dependency>

```

### Making your first API call

#### Initialize Prime Client

The following code snippet demonstrates how to initialize the Prime client.

```
package com.coinbase.examples;
import com.coinbase.prime.client.CoinbasePrimeClient;
import com.coinbase.prime.credentials.CoinbasePrimeCredentials;
import com.fasterxml.jackson.databind.ObjectMapper;
public class Main {
    public static void main(String[] args) {
        String credsStringBlob = System.getenv("COINBASE_PRIME_CREDENTIALS");
        ObjectMapper mapper = new ObjectMapper();
        CoinbasePrimeCredentials credentials = new CoinbasePrimeCredentials(credsStringBlob);
        CoinbasePrimeClient client = new CoinbasePrimeClient(credentials);
    }
}

```

#### Listing Portfolios

Update the code snippet with the service invocation and call to make your first API call with Prime to List Portfolios.

```
package com.coinbase.examples;
import com.coinbase.prime.client.CoinbasePrimeClient;
import com.coinbase.prime.credentials.CoinbasePrimeCredentials;
import com.coinbase.prime.factory.PrimeServiceFactory;
import com.coinbase.prime.model.portfolios.ListPortfoliosResponse;
import com.coinbase.prime.portfolios.PortfoliosService;
import com.fasterxml.jackson.databind.ObjectMapper;
public class Main {
    public static void main(String[] args) {
        String credsStringBlob = System.getenv("COINBASE_PRIME_CREDENTIALS");
        ObjectMapper mapper = new ObjectMapper();
        CoinbasePrimeCredentials credentials = new CoinbasePrimeCredentials(credsStringBlob);
        CoinbasePrimeClient client = new CoinbasePrimeClient(credentials);
        PortfoliosService portfoliosService = PrimeServiceFactory.createPortfoliosService(client);
        ListPortfoliosResponse listPortfoliosResponse = portfoliosService.listPortfolios();
        System.out.println(mapper.writeValueAsString(listPortfoliosResponse));
    }
}

```

### Installation

The [Coinbase Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet) supports .NET version 8.0+.Check your .NET version:

The SDK is vended through [NuGet](https://www.nuget.org/packages/CoinbaseSdk.Prime) and available for installation via the `dotnet` CLI.

```
dotnet add package CoinbaseSdk.Prime --version x.y.z

```

or if using [paket](https://fsprojects.github.io/Paket/):

```
paket add CoinbaseSdk.Prime --version x.y.z

```

### Making your first API call

#### Initialize Prime Client

The following code snippet demonstrates how to initialize the Prime client:

```
namespace CoinbaseSdk.PrimeExample.Example
{
  using CoinbaseSdk.Core.Credentials;
  using CoinbaseSdk.Core.Serialization;
  using CoinbaseSdk.Prime.Client;
  class Example
  {
    static void Main()
    {
      string? credentialsBlob = Environment.GetEnvironmentVariable("COINBASE_PRIME_CREDENTIALS");
      if (credentialsBlob == null)
      {
        Console.WriteLine("COINBASE_PRIME_CREDENTIALS environment variable not set");
        return;
      }
      var serializer = new JsonUtility();
      var credentials = serializer.Deserialize<CoinbaseCredentials>(credentialsBlob);
      if (credentials == null)
      {
        Console.WriteLine("Failed to parse COINBASE_PRIME_CREDENTIALS environment variable");
        return;
      }
      var client = new CoinbasePrimeClient(credentials!);
    }
  }
}

```

#### Listing Portfolios

Update the code snippet with the service invocation and call to make your first API call with Prime to List Portfolios.

```
namespace CoinbaseSdk.PrimeExample.Example
{
  using CoinbaseSdk.Core.Credentials;
  using CoinbaseSdk.Core.Serialization;
  using CoinbaseSdk.Prime.Client;
  using CoinbaseSdk.Prime.Model;
  using CoinbaseSdk.Prime.Orders;
  using CoinbaseSdk.Prime.Portfolios;
  class Example
  {
    static void Main()
    {
      string? credentialsBlob = Environment.GetEnvironmentVariable("COINBASE_PRIME_CREDENTIALS");
      if (credentialsBlob == null)
      {
        Console.WriteLine("COINBASE_PRIME_CREDENTIALS environment variable not set");
        return;
      }
      string? portfolioId = Environment.GetEnvironmentVariable("COINBASE_PRIME_PORTFOLIO_ID");
      if (portfolioId == null)
      {
        Console.WriteLine("COINBASE_PRIME_PORTFOLIO_ID environment variable not set");
        return;
      }
      var serializer = new JsonUtility();
      var credentials = serializer.Deserialize<CoinbaseCredentials>(credentialsBlob);
      if (credentials == null)
      {
        Console.WriteLine("Failed to parse COINBASE_PRIME_CREDENTIALS environment variable");
        return;
      }
      var client = new CoinbasePrimeClient(credentials!);
      var portfoliosService = new PortfoliosService(client);
      var listPortfoliosResponse = portfoliosService.ListPortfolios();
      Console.WriteLine($"Portfolio: {serializer.Serialize(portfolio)}");
    }
  }
}

```

### Installation

The [Coinbase Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go) works with [Go](https://go.dev/) 1.19+Check your Golang version:

Initialize a new Go module and tidy dependencies. Run the following commands in your project directory, replacing `example.com/test` with a proper project directory:

```
go mod init example.com/test
go mod tidy
go build

```

Install the Golang SDK:

```
go get github.com/coinbase-samples/prime-sdk-go

```

To use the Prime Go SDK, initialize the `Credentials` struct and create a new client. The `Credentials` struct is JSON enabled. Ensure that Prime API credentials are stored in a secure manner.There are convenience functions to read the credentials as an environment variable (prime.ReadEnvCredentials) and to deserialize the JSON structure (prime.UnmarshalCredentials) if pulled from a different source. The JSON format expected for running this SDK is:

```
export PRIME_CREDENTIALS='{ "accessKey": "", "passphrase": "", "signingKey": "", "portfolioId": "", "svcAccountId": "" }'

```

These should be stored in `~/.zshrc` or `~/.bashrc`. You may proceed initially without knowing your Portfolio ID or Entity ID. The SDK will return these values in the response when you make your first API call below.

### Making your first API call

#### Initialize Prime Client

The following code snippet demonstrates how to initialize the Prime client.

```
package main
import (
    "net/http"
    "github.com/coinbase-samples/prime-sdk-go"
)
func main() {
    primeCredentials, err := credentials.ReadEnvCredentials("PRIME_CREDENTIALS")
    if err != nil {
        log.Fatalf("unable to load prime credentials: %v", err)
    }
    httpClient, err := client.DefaultHttpClient()
    if err != nil {
        log.Fatalf("unable to load default http client: %v", err)
    }
    client := prime.NewRestClient(primeCredentials, httpClient)
}

```

#### Listing Portfolios

Update the code snippet with the service invocation and call to make your first API call with Prime to List Portfolios.

```
package main
import (
    "context"
    "encoding/json"
    "log"
    "net/http"
    "github.com/coinbase-samples/prime-sdk-go"
)
func main() {
    primeCredentials, err := credentials.ReadEnvCredentials("PRIME_CREDENTIALS")
    if err != nil {
        log.Fatalf("unable to load prime credentials: %v", err)
    }
    httpClient, err := client.DefaultHttpClient()
    if err != nil {
        log.Fatalf("unable to load default http client: %v", err)
    }
    client := prime.NewRestClient(primeCredentials, httpClient)
    ctx := context.Background()
    response, err := client.ListPortfolios(ctx, &prime.ListPortfoliosRequest{})
    if err != nil {
        fmt.Println("Could not make request:", err)
        return
    }
    jsonResponse, err := json.MarshalIndent(response, "", "  ")
    if err != nil {
        log.Fatalf("Could not marshall response to JSON: %v", err)
    }
    log.Println(string(jsonResponse))
}

```

### Installation

The [Coinbase Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py) supports Python versions 3.9+.Check your Python version:

**Install the Python Dependency**

### Making your first API call

#### Initialize Prime Client

The following code snippet demonstrates how to initialize the Prime client.

```
from credentials import Credentials
from client import Client
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)

```

#### Listing Portfolios

Update the code snippet with the service invocation and call to make your first API call with Prime to List Portfolios.

```
from list_portfolios import PrimeClient, ListPortfoliosRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
prime_client = PrimeClient(credentials)
request = ListPortfoliosRequest()
try:
    response = prime_client.list_portfolios(request)
    print(response)
except Exception as e:
    print(f"failed to list portfolios: {e}")

```

**The TypeScript SDK is currently in development. Please check back soon for updates.**

For technical support, see our [Help Center](https://help.coinbase.com/en/prime). If you do not have a Coinbase Prime account, or want to learn more, visit [coinbase.com/prime](https://www.coinbase.com/prime).