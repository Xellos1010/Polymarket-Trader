# create wallet

POST

/

v1

/

portfolios

/

{portfolio\_id}

/

wallets

```
curl --request POST \
  --url https://api.prime.coinbase.com/v1/portfolios/{portfolio_id}/wallets \
  --header 'Content-Type: application/json' \
  --data '
{
  "name": "<string>",
  "symbol": "<string>",
  "wallet_type": "VAULT",
  "idempotency_key": "<string>",
  "network_family": "NETWORK_FAMILY_UNSPECIFIED",
  "network": {
    "id": "<string>",
    "type": "<string>"
  }
}
'
```

```
{
  "activity_id": "<string>",
  "name": "<string>",
  "symbol": "<string>",
  "wallet_type": "VAULT",
  "network_family": "NETWORK_FAMILY_UNSPECIFIED"
}
```

**Supported Types**Currently, this endpoint can be used only to create vault wallets and onchain wallets that do not require key generation. The first EVM and first Solana onchain wallet in a portfolio must be created prior to creating subsequent EVM or Solana wallets in a portfolio via API.

Use the Prime SDK or CLI to test this endpoint by following the [quickstart](https://developer.chrome.com/prime/introduction/quickstart) guide and running with the following examples

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
WalletsService walletsService = PrimeServiceFactory.createWalletsService(client);
CreateWalletRequest request = new CreateWalletRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .type(WalletType.VAULT)
    .name("PRIME_API_EXAMPLE")
    .symbol("ETH")
    .build();
CreateWalletResponse response = walletsService.createWallet(request);

```

For more information, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var walletsService = new WalletsService(client);
var request = new CreateWalletRequest("PORTFOLIO_ID_HERE")
{
    Type = WalletType.VAULT,
    Name = "PRIME_API_EXAMPLE",
    Symbol = "ETH",
}
var response = walletsService.CreateWallet(request);

```

For more information, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
walletsService := users.NewWalletsService(client)
request := &users.CreateWalletRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    Type: "VAULT",
    Name: "PRIME_API_EXAMPLE",
    Symbol: "ETH",
}
response, err := walletsService.CreateWallet(context.Background(), request)

```

For more information, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
prime_client = PrimeClient(credentials)
request = CreateWalletRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    name="PRIME_API_EXAMPLE",
    symbol="ETH",
    wallet_type="VAULT",
)
response = prime_client.create_wallet(request)

```

For more information, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-wallet --help

```

For more information, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const walletsService = new WalletsService(client);
walletsService.getWalletDepositInstructions({
    portfolioId: 'PORTFOLIO_ID_HERE',
    type: WalletType.VAULT,
    name: "PRIME_API_EXAMPLE",
    symbol: "ETH",
}).then(async (response) => {
    console.log('Wallet: ', response);
})

```

For more information, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

#### Path Parameters

#### Body

-   UNKNOWN\_WALLET\_TYPE: (-- api-linter: core::0126::unspecified=disabled --)
-   VAULT: A crypto vault
-   TRADING: A trading wallet
-   WALLET\_TYPE\_OTHER: Other wallet types (like consumer, etc)
-   QC: A QC Wallet
-   ONCHAIN: An Onchain wallet

Available options

:

`VAULT`,

`TRADING`,

`WALLET_TYPE_OTHER`,

`QC`,

`ONCHAIN`

network\_family

enum<string>

default:NETWORK\_FAMILY\_UNSPECIFIED

Available options

:

`NETWORK_FAMILY_UNSPECIFIED`,

`NETWORK_FAMILY_EVM`,

`NETWORK_FAMILY_SOLANA`

#### Response

-   UNKNOWN\_WALLET\_TYPE: (-- api-linter: core::0126::unspecified=disabled --)
-   VAULT: A crypto vault
-   TRADING: A trading wallet
-   WALLET\_TYPE\_OTHER: Other wallet types (like consumer, etc)
-   QC: A QC Wallet
-   ONCHAIN: An Onchain wallet

Available options

:

`VAULT`,

`TRADING`,

`WALLET_TYPE_OTHER`,

`QC`,

`ONCHAIN`

network\_family

enum<string>

default:NETWORK\_FAMILY\_UNSPECIFIED

Available options

:

`NETWORK_FAMILY_UNSPECIFIED`,

`NETWORK_FAMILY_EVM`,

`NETWORK_FAMILY_SOLANA`