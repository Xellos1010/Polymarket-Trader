# postman files

The following Postman collection and environment files are available for download.

-   [Coinbase Developer Platform Advanced Trade Postman Collection](https://developer.chrome.com/coinbase-business/advanced-trade-apis/files/coinbase_advanced_trading.postman_collection.json)
-   [Coinbase Developer Platform Postman Environment](https://developer.chrome.com/coinbase-business/advanced-trade-apis/files/coinbase_developer_platform.postman_environment.json)

We recommend testing authentication and relevant endpoints with these files as this will help our team troubleshoot any problems.

## Coinbase Developer Platform Collection

### Step 1: Download Postman

If don’t have Postman installed, download and install [Postman](https://www.postman.com/downloads/) from their website.

### Step 2: Download and Import Files

1.  **Download Collection:**
    -   Download the [Coinbase Developer Platform Advanced Trade Postman Collection](https://developer.chrome.com/coinbase-business/advanced-trade-apis/files/coinbase_advanced_trading.postman_collection.json)
2.  **Import Collection into Postman:**
    -   Open Postman.
    -   Click on **Import** in the upper left corner.
    -   Select the downloaded JSON file and import it.

### Step 3: Configure Environment Variables

Once the files are imported, you need to configure your environment variables.

1.  **Download Postman Environment:**
    -   [Coinbase Developer Platform Postman Environment](https://developer.chrome.com/coinbase-business/advanced-trade-apis/files/coinbase_developer_platform.postman_environment.json)
2.  **Import Environment into Postman:**
    -   Open Postman.
    -   Click on **Import** in the upper left corner.
    -   Select the downloaded JSON file and import it.
3.  **Select the Environment:**
    -   In Postman, click on the environment dropdown near the top right of the screen and select “Coinbase Developer Platform Postman Environment”.
4.  **Set Up Variables:**
    
    -   Click the Environments tab under My Workspace on the left of the screen
    -   Select the “Coinbase Developer Platform Postman Environment”.
    -   Configure the following variables:
    
    Variable
    
    Current value
    
    `name`
    
    `"organizations/{ORG_ID}/apiKeys/{KEY_ID}"` (Include quotes)
    
    `privateKey`
    
    `"-----BEGIN EC PRIVATE KEY-----\{KEY}\n-----END EC PRIVATE KEY-----\n"` (Include quotes)
    

### Step 4: Authenticate and Test Endpoints

1.  **Send Requests:**
    -   Navigate to the “Collections” tab in Postman.
    -   Expand the “Coinbase Developer Platform Postman Collection”.
    -   Select any request and click **Send** to test the endpoint.
2.  **Check Responses:**
    -   Ensure that the responses are as expected.
    -   If you encounter any issues, refer to the detailed response messages to understand the problem.

### Important Notes