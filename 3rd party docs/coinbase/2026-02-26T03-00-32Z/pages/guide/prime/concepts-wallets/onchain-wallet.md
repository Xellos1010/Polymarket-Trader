# onchain wallet

The Prime Onchain Wallet REST API provides programmatic access to the Onchain Wallet, enabling workflow automation for tasks such as sending, receiving, and onchain trading. This enhances operational efficiency, minimizes manual intervention, and ensures secure, seamless transactions across supported blockchains.

## Creating an Onchain Wallet

An onchain wallet can be created using the same [Create Wallet](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet) endpoint detailed in the [Wallets](https://developer.chrome.com/prime/concepts/wallets/wallets-overview) concepts page. See below for examples on how to process this creation:

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
.type(WalletType.ONCHAIN)
.name("ONCHAIN_WALLET_EXAMPLE")
.build();
CreateWalletResponse response = walletsService.createWallet(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var request = new CreateWalletRequest("PORTFOLIO_ID_HERE")
{
    Type = WalletType.ONCHAIN,
    Name = "ONCHAIN_WALLET_EXAMPLE",
}
var response = walletsService.CreateWallet(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
walletsService := users.NewWalletsService(client)
request := &users.CreateWalletRequest{
PortfolioId: "PORTFOLIO_ID_HERE",
Type: model.WalletTypeOnchain,
Name: "ONCHAIN_WALLET_EXAMPLE",
}
response, err := walletsService.CreateWallet(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.wallets import WalletsService, CreateWalletRequest, WalletType
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
wallets_service = WalletsService(client)
request = CreateWalletRequest(
portfolio_id="PORTFOLIO_ID_HERE",
name="ONCHAIN_WALLET_EXAMPLE",
wallet_type=WalletType.ONCHAIN.value,
)
response = wallets_service.create_wallet(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-wallet --help

```

```
const walletsService = new WalletsService(client);
walletsService.getWalletDepositInstructions({
portfolioId: 'PORTFOLIO_ID_HERE',
type: WalletType.ONCHAIN,
name: "ONCHAIN_WALLET_EXAMPLE",
}).then(async (response) => {
console.log('Wallet: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Receiving funds into Prime Onchain Wallet

This functionality works identically to deriving a deposit address for any other Prime wallet. See [Deposits](https://developer.chrome.com/prime/concepts/transactions/deposits) for more information.

## Creating an Onchain Transaction

Creating an onchain transaction is accomplished via the [Create Onchain Transaction](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/create-onchain-transaction) endpoint. While this endpoint processes the initiation of a transaction, please note:

-   The transaction still requires signing through the standard Onchain wallet flows.
-   Consensus approval still applies if required by the portfolio’s transfer rules.

When specifying the chain ID, it is important to remember that it is a numeric code representing the blockchain network. For example, the chain ID for Base is 8453. Other network codes can be looked up at [Chainlist](https://chainlist.org/). See below for an example request.

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
TransactionsService transactionsService = PrimeServiceFactory.createOnchainTransaction(client);
EvmParams evmParams = new EvmParams.Builder()
.chainId("CHAIN_ID_HERE")
.build();
CreateOnchainTransactionRequest request = new CreateOnchainTransactionRequest.Builder()
.portfolioId("PORTFOLIO_ID_HERE")
.walletId("WALLET_ID_HERE")
.rawUnsignedTransaction("RAW_UNSIGNED_TRANSACTION_HERE")
.evmParams(evmParams)
.amount("AMOUNT")
.build();
CreateOnchainTransactionResponse response = transactionsService.createOnchainTransaction(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var transactionsService = new OnchainTransactionsService(client);
var evmParams = new EvmParams
{
    ChainId               = "CHAIN_ID_HERE"
};
var request = new CreateOnchainTransactionRequest
{
    PortfolioId           = "PORTFOLIO_ID_HERE",
    WalletId              = "WALLET_ID_HERE",
    RawUnsignedTransaction = "RAW_UNSIGNED_TRANSACTION_HERE",
    EvmParams             = evmParams,
    Amount                = "AMOUNT"
};
var response = transactionsService.CreateOnchainTransaction(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
func createOnchainTransactionExample() {
transactionsService := transactions.NewOnchainTransactionsService(client)
evmParams := &transactions.EvmParams{
ChainId:               "CHAIN_ID_HERE",
}
request := &transactions.CreateOnchainTransactionRequest{
PortfolioId:           "PORTFOLIO_ID_HERE",
WalletId:              "WALLET_ID_HERE",
RawUnsignedTransaction: "RAW_UNSIGNED_TRANSACTION_HERE",
EvmParams:            evmParams,
Amount:               "AMOUNT",
}
response, err := transactionsService.CreateOnchainTransaction(context.Background(), request)
}

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.transactions import OnchainTransactionsService, CreateOnchainTransactionRequest, EvmParams
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
transactions_service = OnchainTransactionsService(client)
evm_params = EvmParams(
    chain_id="CHAIN_ID_HERE"
)
request = CreateOnchainTransactionRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
    raw_unsigned_transaction="RAW_UNSIGNED_TRANSACTION_HERE",
    evm_params=evm_params,
    amount="AMOUNT"
)
response = transactions_service.create_onchain_transaction(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-onchain-transaction --help

```

```
const transactionsService = new OnchainTransactionsService(client);
const evmParams = {
chainId: "CHAIN_ID_HERE"
};
const request = {
portfolioId: "PORTFOLIO_ID_HERE",
walletId: "WALLET_ID_HERE",
rawUnsignedTransaction: "RAW_UNSIGNED_TRANSACTION_HERE",
evmParams,
amount: "AMOUNT"
};
transactionsService.createOnchainTransaction(request)
.then(response => {
console.log('Onchain Transaction Response:', response);
})

```

## Address Groups

Prime Onchain Wallet utilizes address groups, which are collections of addresses organized for specific purposes, such as grouping addresses associated with a particular Uniswap pool. To list the available Onchain Address Groups, use the [List Onchain Address Groups](https://developer.chrome.com/api-reference/prime-api/rest-api/onchain-address-groups/list-onchain-address-groups) endpoint. An example request is shown below:

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
OnchainAddressBookService onchainAddressBookService = PrimeServiceFactory.createOnchainAddressBookService(client);
ListOnchainAddressBookRequest request = new ListOnchainAddressBookRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .build();
ListOnchainAddressBookResponse response = onchainAddressBookService.listOnchainAddressBook(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var onchainAddressBookService = new OnchainAddressBookService(client);
var request = new ListOnchainAddressBookRequest("PORTFOLIO_ID_HERE");
var response = onchainAddressBookService.ListOnchainAddressBook(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
onchainAddressBookService := onchainaddressbook.NewOnchainAddressBookService(client)
request := &onchainaddressbook.ListOnchainAddressBookRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
}
response, err := onchainAddressBookService.ListOnchainAddressBook(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.onchain_address_book import OnchainAddressBookService, ListOnchainAddressBookRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
onchain_address_book_service = OnchainAddressBookService(client)
request = ListOnchainAddressGroupsRequest(
    portfolio_id="PORTFOLIO_ID_HERE"
)
response = onchain_address_book_service.list_onchain_address_groups(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const onchainAddressBookService = new OnchainAddressBookService(client);
onchainAddressBookService.listOnchainAddressBook({
    portfolioId: 'PORTFOLIO_ID_HERE'
}).then(async (response) => {
    console.log('Onchain Address Groups: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl list-onchain-address-groups --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).