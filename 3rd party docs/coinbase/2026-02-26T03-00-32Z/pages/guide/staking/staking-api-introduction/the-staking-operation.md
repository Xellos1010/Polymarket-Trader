# the staking operation

The Staking API simplifies management of staking transactions for multiple networks through the ***Staking Operation***.

### Overview

A Staking Operation is a list of transactions that need to be signed and broadcasted sequentially in order to complete a staking action such as stake, unstake etc.

### Staking Operation Lifecycle

Performing any staking activity within the Staking API initiates a Staking Operation. This triggers an asynchronous process behind the scenes that constructs the required transactions for the staking action you specified (ex: stake, unstake, etc). For some networks, you might be presented with a single transaction, while for others, there might be multiple. These transactions might be available immediately or after necessary infrastructure setup that can take longer. Regardless of the network or the type of staking, your interaction with the Staking Operation remains consistent. **Your only responsibility is to sign and broadcast the presented transactions as they become available**. Here’s an example of how you can start a staking operation.

```
// Create a new external address on the `ethereum-hoodi` network.
let address = new ExternalAddress(Coinbase.networks.EthereumHoodi, "YOUR_WALLET_ADDRESS");
// Build a stake operation. For Dedicated ETH Staking this results in
// standing up the necessary infrastructure, and then creating deposit transactions.
let stakingOperation = await address.buildStakeOperation(96, Coinbase.assets.Eth, StakeOptionsMode.NATIVE);

```

Once the staking operation is built, you can keep polling on the staking operation until it reaches a terminal state. During successful operation, this means the transactions are fully constructed and ready to be signed and broadcasted.

```
// Example of polling the stake operation status until it reaches a terminal state.
await stakingOperation.wait();

```

Now, simply sign and broadcast the transactions that were created as part of the staking operation.

```
// Load your wallet's private key from which you initiated the above stake operation.
const wallet = new ethers.Wallet("YOUR_WALLET_PRIVATE_KEY");
// Sign the transactions within staking operation resource with your wallet.
await stakingOperation.sign(wallet);
// For Hoodi, publicly available RPC URL's can be found here https://chainlist.org/chain/560048
const provider = new ethers.JsonRpcProvider("HOODI_RPC_URL");
// Broadcast each of the signed transactions to the network.
stakingOperation.getTransactions().forEach(async tx => {
    let resp = await provider.broadcastTransaction(tx.getSignedPayload()!);
    console.log(resp);
});

```

### Staking Operation States

A staking operation can have the following states:

State

Definition

Terminal State

initialized

The staking operation has been initialized. Behind the scenes,  
the necessary transactions and/or infrastructure are being created.

False

complete

The staking operation is now complete and has no more transactions to create.  
In addition, for Wallet Address model this also means that the transactions  
have been successfully signed and broadcasted.

True

failed

The staking operation has failed.

True

### FAQs

#### How do I know when a staking operation is complete?

You can poll the staking operation until it reaches a terminal state of `complete` or `failed`. The SDK provides the `wait` helper method to make this process easier.

#### What happens if a staking operation fails?

If initiating a staking operation fails, an error with a meaningful message will be returned. You can retry the staking operation as needed. If a staking operation fails after a successful initialization, it typically means something in the backend failed with an internal error. There isn’t any action you can take to recover from this, and you’ll need to retry the staking operation. You should be able to retry as often as needed until the staking operation completes successfully without worrying about any side effects.

#### When should I sign and broadcast the transactions?

Currently, our supported networks don’t require transactions to be signed and broadcasted on the fly. As a result, we can wait for the staking operation to complete before proceeding ahead. In the future, this will change to accommodate more complex networks where a set of transactions need to be signed and broadcasted before the next round of transactions can be created as they are dependent on the successful broadcast of previous transactions. We will provide examples and documentation on how to handle when this change occurs.

#### Who should sign the transactions?

Staking API supports two address models. Answer depends on the address model you choose:

1.  **External Address Model**: In this model, the private keys are not managed by the SDK. As a result, transactions need to be relayed to the owner of the external address for signing.
2.  **Wallet Address Model**: In this model, the private keys are managed by the SDK. As a result, the SDK can sign and broadcast the transactions on behalf of the user.

#### Where can I reach out for help?

Feel free to reach out to us in the **#staking** channel of the [CDP Discord](https://discord.com/invite/cdp) if you have any questions or need help with the Staking API.