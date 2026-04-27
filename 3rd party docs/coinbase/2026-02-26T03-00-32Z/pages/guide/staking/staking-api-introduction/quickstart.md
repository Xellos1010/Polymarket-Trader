# quickstart

Coinbase Staking API empowers developers to deliver a fully-featured staking experience in their applications using one common interface across protocols. This quickstart shows you how to stake Hoodi (an Ethereum testnet) ETH to our best-in-class staking infrastructure using the Coinbase Staking API.

### What you will learn

-   Installing the CDP SDK
-   How to check your stakeable balance
-   How to create a transaction to stake Hoodi ETH
-   How to sign and broadcast a staking transaction

### 1\. Prerequisites

-   TypeScript
    
-   Go
    

The Coinbase Typescript SDK requires [Node.js](https://nodejs.org/en/download) version 18+. To verify your Node version:

The Coinbase Go SDK requires Go 1.19 and recommends using 1.21+. To verify your Go version:

### 2\. API Key Setup

To use the CDP SDK, you need a CDP secret API key. If you need to create one, follow [this guide](https://developer.chrome.com/get-started/authentication/cdp-api-keys#creating-api-keys). Once you have the key, you can either save it as a file on your filesystem (if you downloaded it) or use the key details as environment variables. You will need to reference this later in your code.

### 3\. Create a Workspace

In your preferred shell, create a new directory:

```
mkdir staking-demo
cd staking-demo

```

### 4\. Install the CDP SDK

-   TypeScript
    
-   Go
    

Install the CDP SDK using your preferred package manager:

```
npm install @coinbase/coinbase-sdk

```

Install the Coinbase SDK:

```
go get github.com/coinbase/coinbase-sdk-go

```

* * *

### 5a. Create a Staking Transaction using an External Address

To proceed with the stake example below, you need some Hoodi ETH in your wallet. If you don’t have any, you can request some from the [Ethereum Hoodi Faucet](https://faucet.quicknode.com/ethereum/hoodi).

-   TypeScript
    
-   Go
    

Create a new file named `stake.ts` and paste the code block below:

```
import {
    Coinbase,
    ExternalAddress,
    StakeOptionsMode,
} from "@coinbase/coinbase-sdk";
// highlight-start
const apiKeyFilePath = "YOUR_API_KEY_FILE_PATH";
const walletAddress = "YOUR_WALLET_ADDRESS";
// highlight-end
/**
 * Stake 0.005 ETH on the ethereum-hoodi testnet network.
 */
async function stake() {
Coinbase.configureFromJson({ filePath: apiKeyFilePath });
// Create a new external address on the ethereum-hoodi testnet network.
const address = new ExternalAddress(
    Coinbase.networks.EthereumHoodi,
    walletAddress,
);
// Find out how much ETH is available to stake.
const stakeableBalance = await address.stakeableBalance(
    Coinbase.assets.Eth,
    StakeOptionsMode.PARTIAL,
);
console.log("Stakeable balance of address %s is %s ETH", walletAddress, stakeableBalance);
// Build a stake transaction for an amount <= stakeableBalance
process.stdout.write("Building a transaction to stake 0.005 ETH...");
const stakingOperation = await address.buildStakeOperation(
    0.005,
    Coinbase.assets.Eth,
    StakeOptionsMode.PARTIAL,
);
console.log("Staking Operation ID: %s", stakingOperation.getID())
console.log("Done.");
}
(async () => {
try {
    await stake();
} catch (error) {
    console.error("Error during stake operation", error);
}
})();

```

#### Note

-   Be sure to replace the placeholder values with your own for:

```
YOUR_API_KEY_FILE_PATH
YOUR_WALLET_ADDRESS

```

Then run the code to create a staking transaction:

Create a new file named stake.go

```
package main
import (
    "context"
    "fmt"
    "log"
    "math/big"
    "github.com/coinbase/coinbase-sdk-go/pkg/coinbase"
)
const (
    // highlight-start
    ApiKeyFilePath = "YOUR_API_KEY_FILE_PATH"
    WalletAddress  = "YOUR_WALLET_ADDRESS"
    // highlight-end
)
func main() {
    client, err := coinbase.NewClient(
        coinbase.WithAPIKeyFromJSON(ApiKeyFilePath),
    )
    if err != nil {
        log.Fatalf("error creating coinbase client: %v", err)
    }
    address := coinbase.NewExternalAddress(coinbase.EthereumHoodi, WalletAddress)
    stakeableBalance, err := client.GetStakeableBalance(
        context.Background(),
        coinbase.Eth,
        address,
        coinbase.WithStakingBalanceMode(coinbase.StakingOperationModePartial),
    )
    if err != nil {
        log.Fatal(err)
    }
    println(fmt.Sprintf("Stakeable balance of address %s is %f ETH", WalletAddress, stakeableBalance.Amount()))
    print("Building a transaction to stake 0.005 ETH...")
    stakeOperation, err := client.BuildStakeOperation(
        context.Background(),
        big.NewFloat(0.005),
        coinbase.Eth,
        address,
        coinbase.WithStakingOperationMode(coinbase.StakingOperationModePartial),
    )
    if err != nil {
        log.Fatalf("error building staking operation: %v", err)
    }
    println("Done.")
    println(fmt.Sprintf("Stake operation ID: %v", stakeOperation.ID()))
}

```

#### Note

-   Be sure to replace the placeholder values with your own for:

```
YOUR_API_KEY_FILE_PATH
YOUR_WALLET_ADDRESS

```

Run the code:

The transaction that was generated is an unsigned transaction. This still needs to be signed and broadcasted to the network to stake successfully. See the next section for instructions on how to sign and broadcast the transaction.

Sample output

```
Stakeable balance of address 0x87Bf57c3d7B211a100ee4d00dee08435130A62fA is 207.65555527344569 ETH
Building stake operation for 0.005 ETH ... Done.
Unsigned payloads: ["7b2274797065223a22307832222c22636861696e4964223a22307834323638222c226e6f6e6365223a223078313030222c22746f223a22307861353534313664653564653631613061633161613839373061323830653034333838623164653462222c22676173223a2230783364303930222c226761735072696365223a6e756c6c2c226d61785072696f72697479466565506572476173223a223078323534306265343030222c226d6178466565506572476173223a223078323534306265343065222c2276616c7565223a2230783131633337393337653038303030222c22696e707574223a2230783361346236366631222c226163636573734c697374223a5b5d2c2276223a22307830222c2272223a22307830222c2273223a22307830222c2279506172697479223a22307830222c2268617368223a22307832623335363130643637653936313864326338343739613638623362383163626232323734323933353935326331626334626536313364363965366662643037227d"]

```

### 5b. Create a Staking Transaction using a Wallet Address

-   TypeScript
    

Create a new file named `stake.ts` and paste the code block below:

```
import { Coinbase, Wallet, StakeOptionsMode } from "@coinbase/coinbase-sdk";
// highlight-start
const apiKeyFilePath = "YOUR_API_KEY_FILE_PATH";
// highlight-end
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
/**
 * Perform a stake operation.
 */
async function stake() {
// Import the CDP API key from the file.
const coinbase = Coinbase.configureFromJson({ filePath: apiKeyFilePath });
// Create a wallet on the `ethereum-hoodi` testnet network.
process.stdout.write("Creating wallet...");
const wallet = await Wallet.create({ networkId: Coinbase.networks.EthereumHoodi });
console.log(
    `Wallet created with ID: ${wallet.getId()} and default address: ${(await wallet.getDefaultAddress()).getId()}`,
);
// Save wallet seed locally for future use.
process.stdout.write("Saving wallet seed information to local directory...");
wallet.saveSeed("wallet-seed", true);
console.log("Done.");
// Fund your wallet from a faucet with some ethereum-hoodi testnet funds.
process.stdout.write("Funding wallet with testnet funds...");
const faucetTransaction = await wallet.faucet();
await faucetTransaction.wait();
console.log("Done.");
console.log(`Wallet funding transaction: ${faucetTransaction.getTransactionLink()}`);
// Find out how much ETH is available to stake from your wallet.
process.stdout.write("Getting stakeable balance of wallet...");
const stakeableBalance = await wallet.stakeableBalance(Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);
console.log("Done.");
console.log(`Stakeable balance of wallet: ${stakeableBalance} ETH`);
// Stake a small amount of ETH.
process.stdout.write("Staking 0.00001 ETH from your wallet...");
const stakingOperation = await wallet.createStake(0.00001, Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);
console.log("Done.");
console.log(
    `View your stake transaction: ${stakingOperation.getTransactions()[0].getTransactionLink()}`,
);
}
(async () => {
try {
    await stake();
} catch (error) {
    console.error("Error during stake operation", error);
}
})();

```

#### Note

-   Be sure to replace the placeholder values with your own for:

Then run the code to create a staking transaction:

Sample output

```
Creating wallet...Done.
Wallet created with ID: ab6efe3b-ea0d-4e1d-8360-cc7b47d4d024 and default address: 0x962e0561475a34e933a41b0dA6e5c4F4b9f39315
Saving wallet seed information to local directory...Done.
Funding wallet with testnet funds...Done.
Wallet funding transaction: https://holesky.etherscan.io/tx/0x1963584c3953f0cd72c31f1f3560ba20f2295a4ae87b173c778a633ca6535eea
Waiting for transaction to finalize.......Done.
Getting stakeable balance of wallet...Done.
Stakeable balance of wallet: 0.01 ETH
Staking 0.00001 ETH from your wallet...Done.
View your stake transaction: https://holesky.etherscan.io/tx/0xb6ef8dec0464569ba303d435cf5d32a5d1c9b27740fa42ebe57726f05029916e

```

#### Note

-   To reuse the wallet created in the steps above, see [Re-instantiating a Wallet](https://developer.chrome.com/server-wallets/v1/concepts/wallets#re-instantiating-a-wallet) with the CDP SDK.
-   Network congestion can sometimes delay a transaction, which can cause the funding or stake transaction to fail.

### 6\. \[Optional\]: Sign and Broadcast your Staking Transaction (External Address only)

The previous step generated an unsigned transaction. To stake successfully, the transaction needs to be signed and broadcasted to the network. Signing and broadcasting functionality is added to the example from above. The additional lines are highlighted for clarity.

-   TypeScript
    
-   Go
    

```
import { Coinbase, ExternalAddress, StakeOptionsMode } from "@coinbase/coinbase-sdk";
// highlight-start
import { ethers } from "ethers";
// highlight-end
const apiKeyFilePath = "YOUR_API_KEY_FILE_PATH";
const walletAddress = "YOUR_WALLET_ADDRESS";
/**
 * Stake 0.005 ETH on the ethereum-hoodi testnet network.
 */
async function stake() {
Coinbase.configureFromJson({ filePath: apiKeyFilePath });
// Create a new external address on the ethereum-hoodi testnet network.
const address = new ExternalAddress(Coinbase.networks.EthereumHoodi, walletAddress);
// Find out how much ETH is available to stake.
const stakeableBalance = await address.stakeableBalance(Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);
console.log("Stakeable balance of address %s is %s ETH", walletAddress, stakeableBalance);
// Build a stake transaction for an amount <= stakeableBalance
process.stdout.write("Building a transaction to stake 0.005 ETH... ");
const stakingOperation = await address.buildStakeOperation(0.005, Coinbase.assets.Eth, StakeOptionsMode.PARTIAL);
console.log("Done.");
// highlight-start
// Load your wallet's private key from which you initiated the above stake operation.
const walletPrivateKey = "YOUR_WALLET_PRIVATE_KEY";
const wallet = new ethers.Wallet(walletPrivateKey);
// Additional public Hoodi RPC endpoints can be found here https://chainlist.org/chain/560048
const hoodiNodeURL = "HOODI_NODE_URL";
// Sign the transactions within staking operation resource with your wallet.
process.stdout.write("Signing the stake operation... ");
await stakingOperation.sign(wallet);
console.log("Done.");
const provider = new ethers.JsonRpcProvider(hoodiNodeURL);
// Broadcast each of the signed transactions to the network.
process.stdout.write("Broadcasting the stake operation... ");
for (const tx of stakingOperation.getTransactions()) {
    const resp = await provider.broadcastTransaction(tx.getSignedPayload()!);
    console.log("Broadcasted transaction hash: %s", resp.hash);
}
// highlight-end
}
(async () => {
try {
    await stake();
} catch (error) {
    console.error("Error during stake operation", error);
}
})();

```

#### Note

-   Be sure to replace the placeholder values with your own for:

```
YOUR_API_KEY_FILE_PATH
YOUR_WALLET_ADDRESS

```

Run the code:

```
package main
import (
    "context"
    "fmt"
    "log"
    "math/big"
    "github.com/coinbase/coinbase-sdk-go/pkg/coinbase"
    // highlight-start
    "github.com/ethereum/go-ethereum/crypto"
    "github.com/ethereum/go-ethereum/ethclient"
    // highlight-end
)
const (
    ApiKeyFilePath = "YOUR_API_KEY_FILE_PATH"
    WalletAddress  = "YOUR_WALLET_ADDRESS"
    PrivateKey     = "YOUR_WALLET_PRIVATE_KEY"
    NodeURL        = "HOODI_NODE_URL"
)
func main() {
    client, err := coinbase.NewClient(
        coinbase.WithAPIKeyFromJSON(ApiKeyFilePath),
    )
    if err != nil {
        log.Fatalf("error creating coinbase client: %v", err)
    }
    address := coinbase.NewExternalAddress(coinbase.EthereumHoodi, WalletAddress)
    stakeableBalance, err := client.GetStakeableBalance(
        context.Background(),
        coinbase.Eth,
        address,
        coinbase.WithStakingBalanceMode(coinbase.StakingOperationModePartial),
    )
    if err != nil {
        log.Fatal(err)
    }
    println(fmt.Sprintf("Stakeable balance of address %s is %f ETH", WalletAddress, stakeableBalance.Amount()))
    print("Building a transaction to stake 0.005 ETH...")
    stakeOperation, err := client.BuildStakeOperation(
        context.Background(),
        big.NewFloat(0.0001),
        coinbase.Eth,
        address,
        coinbase.WithStakingOperationMode(coinbase.StakingOperationModePartial),
    )
    if err != nil {
        log.Fatalf("error building staking operation: %v", err)
    }
    println("Done.")
    // highlight-start
    // Load your wallet's private key from which you initiated the above stake operation.
    key, err := crypto.HexToECDSA(PrivateKey)
    if err != nil {
        log.Fatal(err)
    }
    // Sign the transactions within staking operation resource with your private key.
    print("Signing the stake transaction...")
    err = stakeOperation.Sign(key)
    if err != nil {
        log.Fatal(err)
    }
    println("Done.")
    // For Hoodi, publicly available RPC URL's can be found here https://chainlist.org/chain/560048
    ethClient, err := ethclient.Dial(NodeURL)
    if err != nil {
        log.Fatal(err)
    }
    // Broadcast each of the signed transactions to the network.
    print("Broadcasting the stake transaction...")
    for _, transaction := range stakeOperation.Transactions() {
        if err := ethClient.SendTransaction(context.Background(), transaction.Raw()); err != nil {
            log.Fatal(err)
        }
        println(fmt.Sprintf("Broadcasted transaction hash: %s", transaction.Raw().Hash().Hex()))
    }
    // highlight-end
}

```

#### Note

-   Be sure to replace the placeholder values with your own for:

```
YOUR_API_KEY_FILE_PATH
YOUR_WALLET_ADDRESS
YOUR_WALLET_PRIVATE_KEY
HOODI_NODE_URL

```

Run the code:

Sample output

You should see the transaction being created, signed, and then broadcast to the network:

```
Stakeable balance of address 0x87Bf57c3d7B211a100ee4d00dee08435130A62fA is 207.64830262344410791 ETH
Building a transaction to stake 0.005 ETH... Done.
Unsigned payloads: [
'7b2274797065223a22307832222c22636861696e4964223a22307834323638222c226e6f6e6365223a223078313031222c22746f223a22307861353534313664653564653631613061633161613839373061323830653034333838623164653462222c22676173223a2230783364303930222c226761735072696365223a6e756c6c2c226d61785072696f72697479466565506572476173223a223078323534306265343030222c226d6178466565506572476173223a223078323534306265343065222c2276616c7565223a2230783131633337393337653038303030222c22696e707574223a2230783361346236366631222c226163636573734c697374223a5b5d2c2276223a22307830222c2272223a22307830222c2273223a22307830222c2279506172697479223a22307830222c2268617368223a22307839346364373935376334373962396266396464623233326561333939393366653234636661313464663839396563343938386234613038663862613065623936227d'
]
Signing the stake operation... Done.
Broadcasting the stake operation... Broadcasted transaction hash: 0x2c66c1d716ceadeef25115cc5c2834c600cd9c35292195d9e2511c7f8c89a123

```

Visit the Etherscan block explorer to view the finalized transaction after it has been broadcasted. Testnet transactions may take up to a minute to be confirmed by a block explorer.

```
https://holesky.etherscan.io/tx/{BROADCASTED_TX_HASH}

```

## Next Steps

Congratulations! You’ve used the CDP SDK to stake your first ETH on the Holesky testnet. See also:

-   [Shared ETH Staking (no minimum)](https://developer.chrome.com/staking/staking-api/protocols/shared-eth/overview)
-   [Dedicated ETH Staking (32 ETH minimum)](https://developer.chrome.com/staking/staking-api/protocols/dedicated-eth/overview)