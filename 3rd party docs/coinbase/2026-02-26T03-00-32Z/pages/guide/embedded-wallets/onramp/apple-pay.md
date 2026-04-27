# apple pay

## Overview

Enable native Apple Pay onramp in your React Native app, allowing users to purchase crypto directly without leaving your app. This provides the fastest onramp experience available, with a fully native feel on iOS devices.

## Prerequisites

-   A free [CDP Portal](https://portal.cdp.coinbase.com/) account and project
-   A React Native app using the [CDP React Native SDK](https://developer.chrome.com/embedded-wallets/react-native/quickstart)
-   iOS development environment (Xcode 16.1+, iOS 15.1+)
-   Users must be located in the United States with a valid US phone number

## Testing and Production Access

## User Verification Requirements

Apple Pay onramp requires users to have verified contact information:

-   **Email verification**: Users must have a verified email address
-   **SMS verification**: Users must have a verified US phone number (a real mobile number, not VoIP).

CDP handles the verification process, but you are responsible for building the UI to prompt users when verification is needed. The `useApplePay` hook returns an error with a specific code when verification is required, allowing you to display the appropriate verification flow.

## Integration

The React Native SDK provides three main components for Apple Pay integration:

Component

Description

`ApplePayProvider`

Context provider that manages Apple Pay state and order creation

`ApplePayButton`

Renders the Apple Pay payment WebView when an order is created

`useApplePay`

Hook that provides order creation, payment status, and error handling

### Installation

Install the `@coinbase/cdp-react-native` package, along with its peer dependency on `react-native-webview` (tested on version `13.13.5`):

### Setup

Wrap your app or the relevant screen with the `ApplePayProvider`:

```
import { ApplePayProvider } from '@coinbase/cdp-react-native';
function ApplePayScreen() {
  return (
    <ApplePayProvider>
      <ApplePayFlow />
    </ApplePayProvider>
  );
}

```

### Using the useApplePay Hook

The `useApplePay` hook provides access to order creation and payment status:

```
const { status, data, error, createOrder, reset } = useApplePay();

```

### Hook Return Values

The `useApplePay` hook returns the following:

Property

Type

Description

`createOrder`

`(options: CreateOrderOptions) => Promise<void>`

Creates an Apple Pay onramp order

`data`

`EndUserApplePayOnrampOrderCreateResponse | undefined`

The created order data, including the payment link

`status`

`'idle' | 'pending' | 'error' | 'success'`

Current status of the onramp flow

`error`

`OnrampError | undefined`

Error details if status is `'error'`

`reset`

`() => void`

Resets the state back to `'idle'`

### CreateOrderOptions

Property

Type

Description

`destination.address`

`string`

The blockchain address receiving the purchased crypto

`destination.network`

`Network`

The blockchain network (e.g., `'base'`, `'ethereum'`)

`purchase.amount`

`string`

The crypto amount to purchase, exclusive of fees (e.g., `'10.00'`)

`purchase.currency`

`string`

The cryptocurrency to purchase (e.g., `'usdc'`)

`payment.currency`

`string`

The fiat currency for payment (e.g., `'usd'`)

`isSandbox`

`boolean`

Set to `true` for sandbox testing (default: `false`)

### Error Codes

When `status` is `'error'`, check the `error.code` to determine the appropriate action:

Error Code

Description

Action Required

`requires_email`

User needs to verify their email

Display email verification UI

`requires_sms`

User needs to verify their phone number

Display SMS verification UI

`user_not_authenticated`

User is not logged in

Redirect to authentication flow

`api_error`

API error occurred

Display error message and retry option

### Rendering the Apple Pay Button

The `ApplePayButton` component automatically reads the payment URL from the `ApplePayProvider` context and renders the Apple Pay WebView. It only renders when order data is available:

```
import { ApplePayButton } from '@coinbase/cdp-react-native';
function PaymentView() {
  const { data, reset } = useApplePay();
  // ApplePayButton only renders when data exists
  return (
    <View style={styles.container}>
      <ApplePayButton style={{ width: '100%', height: 48 }} />
      <Button title="Cancel" onPress={reset} />
    </View>
  );
}

```

## Handling Verification

When the `useApplePay` hook returns an error indicating missing verification, guide users through the verification process. You can use the CDP hooks to handle email and SMS verification:

-   [`useLinkEmail`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Functions/useLinkEmail) - Link and verify an email address
-   [`useLinkSms`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Functions/useLinkSms) - Link and verify a phone number

```
import { useApplePay } from '@coinbase/cdp-react-native';
import { Platform } from 'react-native';
function ApplePayFlow() {
  const { status, error, reset } = useApplePay();
  // Platform check - Apple Pay is iOS only
  if (Platform.OS !== 'ios') {
    return (
      <View>
        <Text>Apple Pay is only available on iOS devices.</Text>
      </View>
    );
  }
  if (status === 'error') {
    if (error?.code === 'requires_email') {
      // Render your email verification component
      // After successful verification, the user can retry
      return <EmailVerificationFlow onComplete={reset} />;
    }
    if (error?.code === 'requires_sms') {
      // Render your SMS verification component
      // After successful verification, the user can retry
      return <SmsVerificationFlow onComplete={reset} />;
    }
    // Handle other errors
    return (
      <View>
        <Text>Error: {error?.message || 'An error occurred'}</Text>
        <Button title="Try Again" onPress={reset} />
      </View>
    );
  }
  // Continue with Apple Pay flow...
}

```

Complete Example

Here’s a complete example showing the full Apple Pay onramp flow with verification handling:

```
import { useState, useCallback } from 'react';
import { View, Text, TextInput, Button, Platform } from 'react-native';
import { ApplePayProvider, ApplePayButton, useApplePay } from '@coinbase/cdp-react-native';
import { useEvmAddress } from '@coinbase/cdp-hooks';
export default function ApplePayScreen() {
  return (
    <ApplePayProvider>
      <ApplePayFlow />
    </ApplePayProvider>
  );
}
function ApplePayFlow() {
  const { status, data, error, createOrder, reset } = useApplePay();
  const { evmAddress } = useEvmAddress();
  const [amount, setAmount] = useState('');
  const handleCreateOrder = useCallback(async () => {
    const parsedAmount = parseFloat(amount);
    if (parsedAmount < 1 || parsedAmount > 10000 || !evmAddress) {
      return;
    }
    await createOrder({
      destination: { address: evmAddress, network: 'base' },
      purchase: { amount, currency: 'usdc' },
      payment: { currency: 'usd' },
      isSandbox: true,
    });
  }, [amount, evmAddress, createOrder]);
  const handleDone = useCallback(() => {
    reset();
    setAmount('');
  }, [reset]);
  // Platform check - Apple Pay is iOS only
  if (Platform.OS !== 'ios') {
    return (
      <View>
        <Text>Apple Pay is only available on iOS devices.</Text>
      </View>
    );
  }
  // Handle errors including verification requirements
  if (status === 'error') {
    if (error?.code === 'requires_email') {
      return <EmailVerificationView />;
    }
    if (error?.code === 'requires_sms') {
      return <SmsVerificationView />;
    }
    return (
      <View>
        <Text>Error: {error?.message || 'An error occurred'}</Text>
        <Button title="Try Again" onPress={reset} />
      </View>
    );
  }
  // Show success state
  if (status === 'success') {
    return (
      <View>
        <Text>Purchase complete!</Text>
        <Button title="Done" onPress={handleDone} />
      </View>
    );
  }
  // Show Apple Pay button when order is ready
  if (status === 'pending' && data) {
    return (
      <View>
        <Text>Complete your purchase</Text>
        <ApplePayButton />
        <Button title="Cancel" onPress={reset} />
      </View>
    );
  }
  // Show order form
  return (
    <View>
      <TextInput
        value={amount}
        onChangeText={setAmount}
        placeholder="Amount in USD"
        keyboardType="decimal-pad"
      />
      <Button
        title={status === 'pending' ? 'Creating order...' : 'Buy USDC with Apple Pay'}
        onPress={handleCreateOrder}
        disabled={status === 'pending' || !amount}
      />
    </View>
  );
}

```

## Legal Requirements

Your users must accept Coinbase’s legal agreements before using Apple Pay onramp:

-   [Guest Checkout Terms of Service](https://www.coinbase.com/legal/guest-checkout/us)
-   [User Agreement](https://www.coinbase.com/legal/user_agreement)
-   [Privacy Policy](https://www.coinbase.com/legal/privacy)

Ensure your app clearly informs users that by proceeding with payment, they are agreeing to these policies.

## Reference Implementation

For a complete React Native reference implementation showcasing the Apple Pay integration with CDP Embedded Wallets, check out the [example app on GitHub](https://github.com/coinbase/cdp-wallet-demo-apps/tree/main/apps/react-native-apple-pay).

## What to read next

-   **[React Native Quickstart](https://developer.chrome.com/embedded-wallets/react-native/quickstart)**: Get started with embedded wallets in React Native
-   **[Apple Pay Onramp API](https://developer.chrome.com/onramp/headless-onramp/overview)**: Learn about the underlying API and advanced configuration
-   **[Cross-Platform Onramp](https://developer.chrome.com/embedded-wallets/onramp/cross-platform)**: Explore other onramp options including the FundModal component