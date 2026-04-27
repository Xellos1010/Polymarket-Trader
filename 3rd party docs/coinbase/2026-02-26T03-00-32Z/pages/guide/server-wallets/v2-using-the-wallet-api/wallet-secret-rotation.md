# wallet secret rotation

## Overview

CDP Wallets are secured by a [Wallet Secret](https://developer.chrome.com/server-wallets/v2/introduction/security), which can only be accessed by the wallet owner/developer. Wallet Secrets are asymmetric key pairs used to authenticate with a [Trusted Execution Environment](https://developer.chrome.com/server-wallets/v2/introduction/security). If you lose your Wallet Secret, or you suspect it has been compromised, it is crucial to rotate it promptly to ensure safety of your funds. This action can be performed via the CDP portal.

1.  Navigate to the [CDP Portal](https://portal.cdp.coinbase.com/products/server-wallets) and access the Server Wallet for your project.
2.  Under **Configuration**, you should see when your current **Wallet Secret** was generated. Click the **Generate new secret** button to rotate your secret.

3.  A modal will appear prompting you to complete 2-step verification. Click **Complete** to receive a verification code.

4.  Enter the verification code to complete the process.
5.  A modal will appear confirming your request to delete the existing secret. Type `delete secret` and click the **Delete and generate new secret** button.

6.  A new secret will be generated and displayed in the modal.

For enhanced security, the secret is no longer automatically downloaded. Click the **Download** button to save the secret file if you need to reference it via file path, or copy the secret details directly from the modal. However, it is recommended to use environment variables for better security. Ensure you save it in a secure location. You will not be able to access the secret again without repeating this recovery process.

## 2FA management

For better security posture, you should have enabled two-factor authentication (2FA) when you created your wallet. If you need to change or enable 2FA for your account: Navigate to the [Security dashboard](https://accounts.coinbase.com/security/settings). Under the **2-step verification** tab, you can add additional 2FA methods or enable/disable current methods.

3.  If you lost access to your existing 2FA method, you can verify your identity by clicking **Get started** at the bottom under **Lost access to your 2-step verification?**

## What to read next

[Server Wallet v2 Security](https://developer.chrome.com/server-wallets/v2/introduction/security): Learn more about security features and architecture.