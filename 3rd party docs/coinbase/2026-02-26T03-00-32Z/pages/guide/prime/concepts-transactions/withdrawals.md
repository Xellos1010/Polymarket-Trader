# withdrawals

**Onchain withdrawals** are supported for all wallet types in Coinbase Prime. By default, all withdrawals require consensus approval in the Prime UI before completion. This ensures control over all withdrawal actions. For fully automated use cases, these approval requirements can be adjusted. All withdrawals are further governed by Prime’s address book.

## Adding to the address book

By default, Coinbase Prime leverages an [address book](https://developer.chrome.com/api-reference/prime-api/rest-api/address-book/get-address-book) to prevent withdrawals to unknown or unauthorized destinations. New address entries can be submitted via [Create Address Book Entry](https://developer.chrome.com/api-reference/prime-api/rest-api/address-book/create-address-book-entry); however, each entry still requires consensus approval in the UI. If desired, the address book feature can be disabled in the UI for more streamlined workflows. When submitting an address book entry, provide:

1.  A **name** for the address
2.  The **address** itself
3.  The **asset** (symbol) associated with that address

Prime will validate the address format to ensure it matches the specified asset. Currently, only the **default network** for each asset is supported, but additional network parameters may be enabled in a future release. For details on which networks are available, refer to [Account Structure](https://developer.chrome.com/prime/concepts/account-structure).

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
AddressBookService addressBookService = PrimeServiceFactory.createAddressBookService(client);
CreateAddressBookEntryRequest request = new CreateAddressBookEntryRequest.Builder("portfolio_id")
    .accountIdentifier("account_identifier")
    .address("address")
    .currencySymbol("currency_symbol")
    .name("name")
    .build();
CreateAddressBookEntryResponse response = addressBookService.createAddressBookEntry(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var addressBookService = new AddressBookService(client);
var request = new CreateAddressBookEntryRequest("portfolio_id")
{
    Address = "address",
    CurrencySymbol = "currency_symbol",
    Name = "name",
    AccountIdentifier = "account_identifier",
};
var response = addressBookService.CreateAddressBookEntry(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
addressBookService := addressbook.NewAddressBookService(client)
request := &addressbook.CreateAddressBookEntryRequest{
    PortfolioId: "portfolio-id",
    Address: "address",
    Symbol: "currency_symbol",
    Name: "name",
    AccountIdentifier: "account_identifier",
}
response, err := addressBookService.CreateAddressBookEntry(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.addressbook import AddressBookService, CreateAddressBookEntryRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
address_book_service = AddressBookService(client)
request = CreateAddressBookEntryRequest(
    portfolio_id="portfolio_id",
    address="address",
    currency_symbol="currency_symbol",
    name="name",
    account_identifier="account_identifier",
)
response = address_book_service.create_address_book_entry(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl create-address-book-entry --help

```

To learn more about this CLI, please visit the [Prime CLI](https://github.com/coinbase-samples/prime-cli).

```
const addressBooksService = new AddressBooksService(client);
addressBooksService.createAddressBook({
    portfolioId: 'PORTFOLIO_ID_HERE',
    address: 'ADDRESS_HERE',
    currencySymbol: 'ETH',
    name: 'XYZ Address',
}).then(async (response) => {
    console.log('Address Book: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Creating a Crypto Withdrawal

Onchain crypto withdrawals to an allowlisted address are created via the [Create Withdrawal](https://developer.chrome.com/api-reference/prime-api/rest-api/wallets/create-wallet) endpoint. Specify the **Wallet ID** to withdraw from; for a refresher on obtaining Wallet IDs, see the [Wallets](https://developer.chrome.com/prime/concepts/wallets/wallets-overview) page. Even though withdrawals can be created through the API, the default behavior requires UI approval to finalize the transaction. The API response will include:

-   A **Transaction ID**, which can be used to track the transaction
-   An **Activity ID** specific to the consensus process in Prime

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   TS/JS
    
-   CLI
    

```
TransactionsService transactionsService = PrimeServiceFactory.createTransactionsService(client);
CreateWithdrawalRequest request = new CreateWithdrawalRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .walletId("WALLET_ID_HERE")
    .amount("0.001")
    .destinationType(DestinationType.DESTINATION_BLOCKCHAIN)
    .idempotencyKey(UUID.randomUUID().toString())
    .currencySymbol("ETH")
    .blockchainAddress(new BlockchainAddress.Builder()
    .address("DESTINATION_WALLET_ADDRESS")
    .build();
CreateWithdrawalResponse response = transactionsService.createWithdrawal(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var transactionsService = new TransactionsService(client);
var request = new CreateWithdrawalRequest("PORTFOLIO_ID_HERE", "WALLET_ID_HERE")
{
    Amount = "0.001",
    DestinationType = DestinationType.DESTINATION_BLOCKCHAIN,
    IdempotencyKey = Guid.NewGuid().ToString(),
    CurrencySymbol = "ETH",
    BlockchainAddress = new BlockchainAddress
{
    Address = "DESTINATION_WALLET_ADDRESS",
},
};
var response = transactionsService.CreateWithdrawal(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
transactionsService := transactions.NewTransactionsService(client)
request := &transactions.CreateWalletWithdrawalRequest{
    PortfolioId: "PORTFOLIO_ID_HERE",
    WalletId: "WALLET_ID_HERE",
    Amount: "0.001",
    DestinationType: "DESTINATION_BLOCKCHAIN",
    IdempotencyKey: uuid.New().String(),
    Symbol: "ETH",
    BlockchainAddress: &transactions.BlockchainAddress{
    Address: "DESTINATION_WALLET_ADDRESS",
},
}
response, err := transactionsService.CreateWalletWithdrawal(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.transactions import TransactionsService, CreateWithdrawalRequest, BlockchainAddress
import uuid
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
transactions_service = TransactionsService(client)
request = CreateWithdrawalRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    wallet_id="WALLET_ID_HERE",
    amount = '0.001',
    destination_type = 'DESTINATION_BLOCKCHAIN',
    idempotency_key = str(uuid.uuid4()),
    currency_symbol = 'ETH',
    blockchain_address = BlockchainAddress(
    address='DESTINATION_WALLET_ADDRESS',
)
response = transactions_service.create_withdrawal(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
const transactionsService = new TransactionsService(client);
transactionsService.createWithdrawal({
    portfolioId: 'PORTFOLIO_ID_HERE',
    walletId: 'WALLET_ID_HERE',
    amount: "0.001",
    idempotencyKey: uuidv4(),
    currencySymbol: "ETH",
    destinationType: DestinationType.DestinationBlockchain,
    blockchainAddress: {
    address: 'DESTINATION_WALLET_ADDRESS',
}
}).then(async (response) => {
    console.log('Withdrawal: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

```
primectl create-withdrawal --help

```

## Tracking withdrawals

Use the **Transaction ID** returned by the Create Withdrawal endpoint to track the transaction’s status:

-   **[Get Transaction by ID](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/get-transaction-by-transaction-id)**: Retrieves detailed information for a single transaction.
-   **[List Transactions](https://developer.chrome.com/api-reference/prime-api/rest-api/transactions/list-portfolio-transactions)**: Can filter by `WITHDRAWAL` to list all withdrawal transactions.

The transaction `STATUS` field in these responses indicates the current stage of withdrawal processing (e.g., pending approval, approved, completed).

-   Java
    
-   .NET
    
-   Go
    
-   Python
    
-   CLI
    
-   TS/JS
    

```
TransactionsService transactionsService = PrimeServiceFactory.createTransactionsService(client);
GetTransactionByTransactionIdRequest request = new GetTransactionByTransactionIdRequest.Builder()
    .portfolioId("PORTFOLIO_ID_HERE")
    .transactionId("TRANSACTION_ID_HERE")
    .build();
GetTransactionByTransactionIdResponse response = transactionsService.getTransactionByTransactionId(request);

```

To learn more about this SDK, please visit the [Prime Java SDK](https://github.com/coinbase-samples/prime-sdk-java).

```
var transactionsService = new TransactionsService(client);
var request = new GetTransactionByTransactionIdRequest("PORTFOLIO_ID_HERE", "TRANSACTION_ID_HERE");
var response = transactionsService.GetPortfolioById(request);

```

To learn more about this SDK, please visit the [Prime .NET SDK](https://github.com/coinbase-samples/prime-sdk-dotnet).

```
transactionsService := transactions.NewTransactionsService(client)
request := &transactions.GetPortfolio{
    PortfolioId: "PORTFOLIO_ID_HERE",
    TransactionId: "TRANSACTION_ID_HERE",
}
response, err := transactionsService.GetTransactions(context.Background(), request)

```

To learn more about this SDK, please visit the [Prime Go SDK](https://github.com/coinbase-samples/prime-sdk-go).

```
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.transactions import TransactionsService, GetTransactionRequest
credentials = Credentials.from_env("PRIME_CREDENTIALS")
client = Client(credentials)
transactions_service = TransactionsService(client)
request = GetTransactionRequest(
    portfolio_id="PORTFOLIO_ID_HERE",
    transaction_id="TRANSACTION_ID_HERE",
)
response = transactions_service.get_transaction(request)

```

To learn more about this SDK, please visit the [Prime Python SDK](https://github.com/coinbase-samples/prime-sdk-py).

```
primectl get-transaction --help

```

```
const transactionsService = new TransactionsService(client);
transactionsService.getTransaction({
    portfolioId: 'PORTFOLIO_ID_HERE',
    transactionId: 'TRANSACTION_ID_HERE',
}).then(async (response) => {
    console.log('Transaction: ', response);
})

```

To learn more about this SDK, please visit the [Prime TS SDK](https://github.com/coinbase-samples/prime-sdk-ts).

## Fiat withdrawals

The **Create Withdrawal** endpoint can also be used to withdraw fiat. Before doing so, link a bank account in the Prime UI. Once linked, retrieve the bank account’s `payment_method_id` via the [List Entity Payment Methods](https://developer.chrome.com/api-reference/prime-api/rest-api/payment-methods/list-entity-payment-methods) endpoint. This endpoint requires the **entity ID**, which can be found by following instructions in the [Account Structure](https://developer.chrome.com/prime/concepts/account-structure) page. Once the correct `payment_method_id` is obtained, call **Create Withdrawal** again, specifying the fiat amount, the payment method, and the destination type `DESTINATION_PAYMENT_METHOD`. For a straightforward example, see [Create Withdrawal To Payment Method](https://github.com/coinbase-samples/prime-scripts-py/blob/main/REST/prime_create_withdrawal_to_payment_method.py). Please note: All requests discussed above require proper authentication. For more information, visit [REST API Authentication](https://developer.chrome.com/prime/rest-api/authentication).