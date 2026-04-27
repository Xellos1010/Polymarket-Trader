# server signer

Server Wallet’s 2-of-2 MPC option uses the Coinbase Server-Signer to simplify key management with a no-code setup on your cloud provider. You can host your Server-Signer on AWS with a quick 10-minute setup. API Wallets created with Server-Signer and the 2-of-2 MPC configuration leverage advanced cryptographic techniques for enhanced usability and security. MPC splits private keys into two shares, one held by Coinbase and the other by the developer, ensuring two security domains protect your wallets. The Server-Signer manages your key share and collaborates with Coinbase to sign transactions using advanced cryptography. It is designed to be secure, scalable, and resilient and provides the following capabilities:

-   **Key Generation**: The Server-Signer generates private keys for the wallets you create using the CDP SDK. The keys are securely generated through an MPC protocol. One keyshare is stored in the developer’s AWS account and the other keyshare with Coinbase.
-   **Key Signing**: The Server-Signer connects to CDP backend to facilitate transaction signing. It only signs transactions that you, as a developer, create using the CDP SDK.
-   **Backup**: Key materials are backed-up by AWS with the Aurora DB backup policies.
-   **Security**: Private keys created by Server-Signer are always split between Coinbase servers and your own Server-Signer instance. No single party ever holds a full private key, meaning a signature requires cooperation between both parties. This provides security even if one’s systems are compromised.

The Server-Signer operates on a shared security model between Coinbase, the developer and their cloud provider, AWS.

The Server-Signer is currently supported only on AWS. You can easily deploy it using [AWS CloudFormation](https://console.aws.amazon.com/cloudformation) with just a few clicks. Once deployed, the Server-Signer continuously monitors the CDP backend for any new actions related to your CDP Project. It performs two types of operations:

-   **Distributed key generation** for wallets within your project.
-   **Creating a signature for a transaction** that is pending broadcast for wallets in your project.

![](https://mintcdn.com/coinbase-prod/XD49nliTOz77YuSk/server-wallets/images/signer_architecture-.png?fit=max&auto=format&n=XD49nliTOz77YuSk&q=85&s=bdb287ea7e8c4b4d604c164b8cb32d59)

## Cost estimation

Since the Server-Signer provisions AWS resources, it generates AWS costs. You can use [this AWS library](https://awscli.amazonaws.com/v2/documentation/api/latest/reference/cloudformation/estimate-template-cost.html) to estimate the AWS costs, based on the CloudFormation template. The estimate provided is approximately $60 (USD) per month in the us-east-1 region, with the largest contributor being RDS usage. This is a baseline estimate, and your AWS costs for Server-Signer may vary based on your usage. Example cost estimation call to AWS:

```
aws cloudformation estimate-template-cost \
  --template-url https://cdp-server-signer-public.s3.amazonaws.com/templates/0.0.15/server-signer.yaml \
  --region us-east-1 \
  --parameters '[{"ParameterKey": "CDPAPIKeyName", "ParameterValue": "test"}, {"ParameterKey": "SSHKeyName", "ParameterValue": "test"}, {"ParameterKey": "CDPAPIKeyPrivateKey", "ParameterValue": "test"}]'

```

## Deploy Server-Signer on AWS

### Step 1: Create CDP Secret API key and AWS SSH KeyPair

****CDP Secret API Key**** Download a Secret API key from the [Coinbase Developer Platform](https://portal.cdp.coinbase.com/projects/api-keys). See [CDP API Key Management](https://developer.chrome.com/get-started/authentication/cdp-api-keys) for detailed steps. This key is used by your Server-Signer to authenticate against CDP backend services.

****SSH Keypair for AWS**** Create an SSH KeyPair on your AWS account for debugging purposes following the instructions in the AWS doc, [Step 1: Create EC2 SSH Key Pair](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/create-key-pairs.html). The SSH key is required for accessing the EC2 instance to see the logs and for debugging.

### Step 2: Create a new stack

1.  Go to [CloudFormation stack](https://console.aws.amazon.com/cloudformation) on your AWS Console.
2.  Click on the **Create stack** button.
3.  Select “Choose an existing template” option.
4.  Select “Amazon S3 URL” as your template source.
5.  Paste the URL [https://cdp-server-signer-public.s3.amazonaws.com/templates/0.0.15/server-signer.yaml](https://cdp-server-signer-public.s3.amazonaws.com/templates/0.0.15/server-signer.yaml) in the **Amazon S3 URL** field.
6.  Click **Next** on this page.

  

> Configuration example

![](https://mintcdn.com/coinbase-prod/XD49nliTOz77YuSk/server-wallets/images/create_cloud_formation_stack.png?fit=max&auto=format&n=XD49nliTOz77YuSk&q=85&s=16e37a1b53e6e038f7bdc23ca4cae3f6)

### Step 3: Specify stack details

1.  Fill in the following parameters in the CloudFormation stack:
    -   **Stack name**: Enter a unique stack name to identify your stack.
    -   **CDPAPIKeyName**: Enter the name of your downloaded CDP secret API key. This corresponds to the `name` field in the JSON file.
    -   **CDPAPIKeyPrivateKey**: Enter the CDP secret API key itself. This corresponds to the `privateKey` field in the JSON file.
    -   **ImageID**: Enter the optional AMI ID if you prefer to use a different AMI. The default AMI used will be the Amazon Linux 2 AMI in your region.
    -   **SSHIPRange**: Configure the IP range from which you want to SSH to your Server-Signer.
    -   **SSHKeyName**: Select the EC2 key pair you created in step 1.
    -   **UseMPCServerSigner:** Set this to true.

  

> CloudFormation stack details example

![](https://mintcdn.com/coinbase-prod/XD49nliTOz77YuSk/server-wallets/images/configure_cloud_formation_stack.png?fit=max&auto=format&n=XD49nliTOz77YuSk&q=85&s=c25766c421254229a4ee0fe43fbedcd6)

2.  Click **Next** on this page.

### Step 4: Configure stack options

1.  Under “Stack failure options”, select the “Delete all newly created resources” option.
2.  Click **Next** on this page.

### Step 5: Review and create stack

1.  Check **I acknowledge that AWS CloudFormation might create IAM resources**.
2.  Click **Submit** on this page.

### Step 6: Wait for the stack to be created

See below for an example of stack creation in progress: ![](https://mintcdn.com/coinbase-prod/XD49nliTOz77YuSk/server-wallets/images/wait_stack_creation.png?fit=max&auto=format&n=XD49nliTOz77YuSk&q=85&s=f147cc059eaee455f6b994a5927d4947)  
Within ~10 minutes, your stack will be created. You will see `CREATE_COMPLETE` status for the stack as shown below.  
Once the stack is created, you will see similar output to the example below: ![](https://mintcdn.com/coinbase-prod/XD49nliTOz77YuSk/server-wallets/images/complete_stack_creation.png?fit=max&auto=format&n=XD49nliTOz77YuSk&q=85&s=34d1574be6a059edfb11511fccc88bf8)  
The stack creates the following resources created for your Server-Signer:

-   **EC2 instance** that hosts the Server-Signer.
-   **Aurora DB** that stores the encrypted MPC key share and other metadata for your Wallets.
-   **Symmetric AWS KMS key** that performs encryption operations for the MPC protocol.
-   **Asymmetric AWS KMS key** that is used as identity key for the Server-Signer.
-   **Security groups and IAM roles** for the functioning of the Server-Signer.

### Step 7: Disable auto-rotation of RDS password

The RDS cluster is created with auto-rotation enabled by default for the database password. However, the stack does not account for the default 7-day rotation policy, and it is necessary to disable it. To disable the rotation policy, follow the steps below:

-   Go to AWS Secrets Manager [here](https://us-east-1.console.aws.amazon.com/secretsmanager/listsecrets?region=us-east-1&search=all%3Drds).
-   Find the secret associated with primary RDS DB cluster for your CloudFormation stack.
-   Go to rotation tab, click on `Edit rotation` and disable `Automatic Rotation`

  

> Disable auto-rotation of RDS password

![](https://mintcdn.com/coinbase-prod/XD49nliTOz77YuSk/server-wallets/images/disable_auto_rotation.png?fit=max&auto=format&n=XD49nliTOz77YuSk&q=85&s=8f7c8e4855c31dc38dd832ec5b1090be)  
The Server-Signer is now ready to be used with the CDP SDK. To make the Server-Signer perform its actions, invoke the methods in CDP SDK.

## Use CDP SDK to create 2-of-2 API Wallets with Server-Signer

### Initialize the CDP SDK

Follow the [quickstart](https://developer.chrome.com/server-wallets/v1/introduction/quickstart) to get an overview of the CDP SDK. To initialize the CDP SDK, use a CDP secret API key that is created for the same project as the Server-Signer. This is important because Server-Signers are scoped to the specific project they are created for and can only create wallets within that project.

-   Typescript
    
-   Python
    

```
const coinbase = Coinbase.configureFromJson({ filePath: '~/Downloads/cdp_api_key.json' });

```

```
Cdp.configure_from_json("~/Downloads/cdp_api_key.json")

```

### Configure CDP SDK to use Server-Signer

-   Typescript
    
-   Python
    

```
Coinbase.useServerSigner = true;

```

```
Cdp.use_server_signer = True

```

### Verify that your CDP project has a Server-Signer assigned

-   Typescript
    

```
    import { ServerSigner } from "@coinbase/coinbase-sdk";
    let serverSigner = await ServerSigner.getDefault();

```

If you do not see a Server-Signer for your project, ensure the deployment of the Server-Signer was successful and follow the [troubleshooting guide](#troubleshooting-the-server-signer). From this point forward, all wallets created using the CDP SDK in the current session are managed by the Server-Signer.

### Create a wallet using Server-Signer

-   Typescript
    
-   Python
    

```
let wallet = await Wallet.create({ networkId: Coinbase.networks.BaseSepolia });

```

```
wallet = Wallet.create("base-sepolia")

```

Wallet creation will take a few seconds because the transaction is orchestrated using the Server-Signer.

### Sign a transaction using Server-Signer

Transfers, and other write verbs, require the Server-Signer’s participation. Transactions for wallets backed by a Server-Signer will be automatically signed. Refer to the [Transfers](https://developer.chrome.com/server-wallets/v1/concepts/transfers) section for more details on how to create a transfer.

## Troubleshooting the Server-Signer

### Accessing the Server-Signer logs on CloudWatch

Go to CloudWatch log group for your ServerSigner [here](https://console.aws.amazon.com/cloudwatch/home?region=us-west-2#logsV2:log-groups/log-group/%2Fvar%2Flog%2Fserver-signer.log). Click on log stream and you will see the logs for your Server-Signer.

### Server-Signer not participating in wallet creation / signing as expected

If your wallet creation / signing times out, it indicates the Server-Signer is down. Check the logs of your Server-Signer to see what went wrong. If you do not see logs, check if the process is running in the instance:

#### SSH to the EC2

Use the SSH key created in Step 1 for this. Follow the instructions [here](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/connect-linux-inst-ssh.html).

```
ssh -i /path/my-key-pair.pem ubuntu@{PUBLIC_IP_OF_YOUR_EC2_INSTANCE}

```

#### Restart the process if required

Check the status of the service with the following command:

```
systemctl status cdp-signer.service
# You should see an active service like
## cdp-signer.service - Coinbase Developer Platform Signer Service
###     Loaded: loaded (/etc/systemd/system/cdp-signer.service; disabled; vendor preset: enabled)
###     Active: active (running)

```

If you do not see the process running, restart the system service with the following command:

```
sudo systemctl start cdp-signer.service

```

If the issue is not resolved with above steps, reach us out on discord at [Coinbase Developers Discord](https://discord.com/invite/cdp) for help.

## Updating the Server-Signer binary to latest version

When a new version of Server-Signer is released, the existing binary in the CloudFormation stack can be updated to the latest version with the following steps:

### SSH to the instance

```
ssh -i /path/my-key-pair.pem ubuntu@{PUBLIC_IP_OF_YOUR_EC2_INSTANCE}

```

### Run the following script to update the Server-Signer binary

```
set -e
SERVICE_NAME="cdp-signer"
# Update the below URL with the latest version of the Server-Signer binary.
SERVER_SIGNER_BINARY_URL="https://api.cdp.coinbase.com/server-signer/assets/0.0.16/cdp-signer.deb.zip"
TEMP_DIR="/tmp/new-binary"
mkdir -p $TEMP_DIR && cd $TEMP_DIR
curl -o $TEMP_DIR/new-binary.zip $SERVER_SIGNER_BINARY_URL
unzip -o $TEMP_DIR/new-binary.zip
sudo dpkg -i $TEMP_DIR/cdp-signer_*_amd64.deb
sudo systemctl start $SERVICE_NAME
echo "Binary updated and service restarted successfully."

```

The binary is updated now and can be verified by creating a new wallet or signing with an existing one.