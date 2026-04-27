# list users

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
UsersService usersService = PrimeServiceFactory.createUsersService(client);
ListUsersRequest request = new ListUsersRequest.Builder()
    .entityId("ENTITY_ID_HERE")
    .build();
ListUsersResponse response = usersService.listUsers(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var usersService = new UsersService(client);
var request = new ListUsersRequest("ENTITY_ID_HERE");
var response = usersService.ListUsers(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
usersService := users.NewUsersService(client)
request := &users.ListEntityUsersRequest{
    EntityId: "ENTITY_ID_HERE",
}
response, err := usersService.ListEntityUsers(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = ListUsersRequest(
    entity_id="ENTITY_ID_HERE",
)
response = prime_client.list_users(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl list-entity-users --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const usersService = new UsersService(client);
usersService.listUsers({
    entityId: 'ENTITY_ID_HERE'
}).then(async (response) => {
    console.log('Users: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

#### Query Parameters

Cursor used for pagination (last consumed record)

Number of items to retrieve

Sorting order

-   DESC: (-- api-linter: core::0126::unspecified=disabled --)

Available options

:

`DESC`,

`ASC`

#### Response