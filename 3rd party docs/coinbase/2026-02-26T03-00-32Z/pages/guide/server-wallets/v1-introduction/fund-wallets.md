# fund wallets

Funding a wallet means transferring crypto to a wallet address. Having crypto in your wallet is key to many use cases such as using the wallet to make payments, transact onchain, or deploy smart contracts. There are several ways to fund an API Wallet:

1.  **Wallet Funding API (Alpha)**: With CDP SDK, you can fund your wallet from fiat currency in a bank account with a single API call. This method is currently limited to US-based individuals using US debit card (without 3DS verification) payment methods configured in a Coinbase account. We will add support for businesses, and for additional payment methods soon. This method currently supports funding wallets with USDC and ETH, with additional assets coming soon.
2.  **Send crypto from an external wallet**: If you already have crypto in another wallet, you can fund your API Wallet by transferring crypto from the other wallet to your API Wallet.
3.  **Use Coinbase Product APIs**: You can use Coinbase Product APIs (Retail / Exchange) to buy and transfer crypto to your API Wallet address.

## Wallet Funding API (Alpha)

For this method, you need a Coinbase account with a US debit card (non-3DS) payment method configured. CDP SDK provides methods to get a quote if desired, and call the Wallet Funding API to buy crypto and transfer it to the indicated API Wallet. If you have multiple debit cards set up, this method will use the first non-3DS, active, verified card you added. The sections below walk through how to use these SDK methods.

### Supported Assets

You can fund your wallet with assets that are supported in the Coinbase App for the applicable network. Not all assets are available to fund on every network; availability in the Wallet Funding API will mirror an asset’s availability in the Coinbase App. For all of our supported EVM networks `eth` and `usdc` will be supported, and you can [Trade](https://developer.chrome.com/server-wallets/v1/concepts/trades) those assets for your desired asset.

### Limits

Wallet funding limits are the same as your [Coinbase account limits](https://help.coinbase.com/en/coinbase/trading-and-funding/buying-selling-or-converting-crypto/limits-and-account-levels).

### Get a quote

You may want to obtain estimates for the network fee and Coinbase fee (if applicable - save on fees with [Coinbase One](https://www.coinbase.com/one)) associated with your wallet funding operation, before executing it. You can do this by calling the quoteFund method, as shown below. The quoteFund method returns a QuoteID and estimated network and Coinbase fees, as applicable.

### Fund your API Wallet

The wallet.fund() method will buy a specified amount of a given crypto asset using your Coinbase account, and transfer it to your API Wallet using the network you configured when creating your wallet. Fees, if applicable, will be charged in addition to the target amount. If you have already generated a quote, you can simply execute it to initiate wallet funding:

The operation shown above will execute the previously-quoted funding operation - in this example, buying 100 USDC, then transferring the 100 USDC to your API Wallet using the network associated with your API Wallet. **If you want to directly execute a wallet funding operation without first getting a quote**, you may directly call the wallet.fund method specifying the amount and crypto asset.

## Receive crypto from an external wallet

You can fund your API Wallet by receiving crypto from an external wallet. To do this, get the deposit address of the API Wallet by calling the `getDefaultAddress` API.

Once you have the deposit address, you can send crypto to the deposit address from an external wallet.

## Use Coinbase products to move funds to your API Wallets

Another way to move funds to your API Wallet is to use Coinbase products.

### Sign up on Coinbase

The first step in using Coinbase products to move funds to your API Wallet is to create a [Coinbase](https://coinbase.com/) account. If you are an individual, you can use a personal [Coinbase App](https://www.coinbase.com/) account. If you are a business, you can use [Coinbase Exchange](https://www.coinbase.com/exchange). The onboarding process will involve answering questions about yourself and your business. If you need assistance in business onboarding, reach out to us in the **#wallet-api** channel of the [CDP Discord](https://discord.com/invite/cdp).

### Send funds from Coinbase Retail / Exchange

Once you have onboarded to Retail / Exchange, you can buy crypto with payment methods of your choice on these platforms.

-   Supported payment methods on Coinbase Exchange can be found [here](https://help.coinbase.com/en/exchange/trading-and-funding/adding-a-payment-method).
-   Supported payment methods on Coinbase Retail can be found [here](https://help.coinbase.com/en/coinbase/getting-started/add-a-payment-method/add-and-verify-pm-namerica-latam#available-payment-methods-by-country).

You can then move the funds to your wallet with the code snippets below.

#### Move funds from Retail via Coinbase App APIs

Expand to see code snippet to move funds from retail to an API Wallet

```
const axios = require('axios');
const jwt = require('jsonwebtoken');
const cdpApiKeyName=""; // Replace with your API key name.
const cdpApiKeySecret=""; // Replace with your API key secret.
interface AccountResponse {
    data: RetailAccount[];
}
interface RetailAccount {
    id: string;
    name: string;
    primary: boolean;
    type: string;
    balance: {
        amount: string;
        currency: string;
    };
    created_at: string;
    updated_at: string;
    resource: string;
    resource_path: string;
    currency: {
        asset_id: string;
        code: string;
        color: string;
        exponent: number;
        name: string;
        slug: string;
        type: string;
        rewards: any;
    };
    allow_deposits: boolean;
    allow_withdrawals: boolean;
}
interface SendRequest {
    type: string;
    to: string;
    amount: string;
    currency: string;
    network: string;
}
interface SendResponse {
    data: {
        id: string;
        type: string;
        status: string;
        amount: {
            amount: string;
            currency: string;
        };
        native_amount: {
            amount: string;
            currency: string;
        };
        description: any;
        created_at: string;
        updated_at: string;
        resource: string;
        resource_path: string;
        network: {
            status: string;
            hash: string;
            name: string;
        };
        to: {
            resource: string;
            address: string;
        };
        details: {
            title: string;
            subtitle: string;
        };
    };
}
class RetailHTTPClient {
    private client: typeof axios;
    private url: string;
    constructor(url: string) {
        this.client = axios;
        this.url = url;
    }
    async request(method: string, urlPattern: string, content: any, responseContent: any): Promise<any> {
        const url = `${this.url}${urlPattern}`;
        let reqBody: string | undefined;
        if (content) {
            reqBody = JSON.stringify(content);
        }
        const uri = `${method} api.coinbase.com${urlPattern}`;
        const jwtToken = await buildJWT(uri);
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${jwtToken}`
        };
        try {
            const response = await this.client({
                method,
                url,
                data: reqBody,
                headers
            });
            if (responseContent) {
                return response.data;
            }
            return response;
        } catch (error) {
            console.error('Error in request:', error);
            throw error;
        }
    }
}
interface APIKeyClaims {
    sub: string;
    iss: string;
    nbf: number;
    exp: number;
    uri: string;
}
async function buildJWT(uri: string): Promise<string> {
    const privateKey = cdpApiKeySecret.replace(/\\n/g, '\n');
    const claims: APIKeyClaims = {
        sub: cdpApiKeyName,
        iss: 'cdp',
        nbf: Math.floor(Date.now() / 1000),
        exp: Math.floor(Date.now() / 1000) + 120,
        uri: uri
    };
    const options = {
        algorithm: 'ES256',
        header: { kid: cdpApiKeyName }
    };
    return new Promise((resolve, reject) => {
        jwt.sign(claims, privateKey, options, (err, token) => {
            if (err) reject(err);
            else resolve(token as string);
        });
    });
}
async function main() {
    const uri = "https://api.coinbase.com";
    const client = new RetailHTTPClient(uri);
    try {
        const accountResp = await client.request('GET', '/api/v2/accounts', null, {}) as AccountResponse;
        let accountID = '';
        for (const account of accountResp.data) {
            if (account.currency.name === 'Ethereum') {
                console.log('Ethereum account found', account);
                accountID = account.id;
                break;
            }
        }
        if (!accountID) {
            throw new Error('Ethereum account not found');
        }
        const sendPath = `/v2/accounts/${accountID}/transactions`;
        const sendRequest: SendRequest = {
            type: 'send',
            to: '0xWalletAddress', // Replace with your wallet address retrieved from wallet.getDefaultAddress()
            amount: '0.0001',
            currency: 'ETH',
            network:  "base",
        };
        const sendResp = await client.request('POST', sendPath, sendRequest, {}) as SendResponse;
        console.log('Send response:', sendResp);
    } catch (error) {
        console.error('Error in requesting to Coinbase Retail APIs:', error);
    }
}
main();

```

#### Move funds from Exchange via Exchange API

Expand to see code snippet to move funds from exchange to an API Wallet

## Withdrawing crypto to fiat

You may wish to convert crypto in your API Wallet to fiat in a bank account. You can do this using the following steps:

1.  Get your Coinbase App [deposit address](https://help.coinbase.com/en/coinbase/getting-started/crypto-education/where-is-my-crypto-address) for the same network as the API Wallet from which you want to withdraw
2.  Use the [Transfer API](https://developer.chrome.com/server-wallets/v1/concepts/transfers) to move funds from your API Wallet to your Coinbase account
3.  Once the crypto is in your Coinbase account, you can [sell crypto for cash](https://help.coinbase.com/en/coinbase/trading-and-funding/buying-selling-or-converting-crypto/how-do-i-sell-or-cash-out-my-digital-currency)
4.  Lastly, follow the steps in [cash out your funds](https://help.coinbase.com/en/coinbase/trading-and-funding/buying-selling-or-converting-crypto/cash-out-funds) for fiat in your selected payment method

If you are interested in an API-based crypto-to-fiat withdrawal feature, please reach out to us in the **#wallet-api** channel of the [CDP Discord](https://discord.com/invite/cdp).