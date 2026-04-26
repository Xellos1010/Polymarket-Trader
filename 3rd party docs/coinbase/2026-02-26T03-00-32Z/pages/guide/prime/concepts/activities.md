# activities

Activities are auditable events generated for any action within a Prime account. This includes transactions (e.g., withdrawals or transfers), user management (e.g., adding or removing users), API key creation, staking operations, and more. For API users, activities provide an extra layer of reconciliation by logging every significant event in the account. Additionally, each activity can link directly to the Prime UI, where consensus approval (or rejection) might be required before finalizing certain actions. Many API responses include both an Activity ID and a unique hyperlink to that activity in the Prime UI. To list activities, begin by calling [List Activities](https://developer.chrome.com/api-reference/prime-api/rest-api/activities/list-activities). The example 200 response in the documentation breaks down all possible activity types and fields returned.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```

ActivitiesService activitiesService = PrimeServiceFactory.createActivitiesService(client);  
ListActivitiesRequest request = new ListActivitiesRequest.Builder("portfolio_id").build();  
ListActivitiesResponse listActivitiesResponse = activitiesService.listActivities(request);  

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```

var activitiesService = new ActivitiesService(client);  
var list = new ListActivitiesRequest("portfolio_id");  
var response = activitiesService.ListActivities(list); 

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
activitiesService := activities.NewActivitiesService(client)  
  
request := &activities.ListActivitiesRequest{  
    PortfolioId: credentials.PortfolioId,  
}  
  
response, err := activitiesService.ListActivities(context.Background(), request)     

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.activities import ActivitiesService, ListActivitiesRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
activities_service = ActivitiesService(client)
request = ListActivitiesRequest(  
    portfolio_id="portfolio_id"  
)  
response = activities_service.list_activities(request)     

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const activitiesService = new ActivitiesService(client);  
  
activitiesService.listPortfolioActivities({  
    portfolioId: 'PORTFOLIO_ID_HERE'  
}).then(async (response) => {  
    console.log('Activities: ', response);  
})  

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```

primectl list-activities --help  

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

Please note: All requests discussed above require proper authentication. For more information, visit [REST API Authentication](https://developer.chrome.com/prime/rest-api/authentication).