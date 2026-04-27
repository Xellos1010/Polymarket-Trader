# enrollmfa readme

The `EnrollMfa` component provides a comprehensive solution for implementing multi-factor authentication enrollment flows. It guides users through setting up two-factor authentication with support for multiple MFA methods. The core features include:

-   TOTP (Time-based One-Time Password) enrollment via authenticator apps
    -   QR code generation for easy setup
    -   Manual secret entry as an alternative to QR scanning
-   SMS enrollment via phone number verification
-   Multi-step enrollment process (method selection, setup, verification)
-   Composable UI that gives developers full control over layout and styling
-   Session timeout handling for security
-   Centralized state management for the enrollment process

## Architecture

The component is built using a composition pattern that allows for maximum flexibility while providing sensible defaults when customization is not needed.

### Composition model

The `EnrollMfa` component is composed of several subcomponents that work together to create the complete enrollment experience. This approach allows developers to customize the UI structure while maintaining the underlying functionality. The main components are:

-   `EnrollMfa`: The root wrapper component that provides the `EnrollMfaContext` and manages state
-   `EnrollMfaTitle`: Renders the title for the current step
-   `EnrollMfaDescription`: Renders the description for the current step
-   `EnrollMfaImage`: Renders the image/icon for the current step
-   `EnrollMfaFlow`: Manages the multi-step flow with transitions between steps
-   `EnrollMfaFlowBackButton`: A button to navigate back in the flow
-   `EnrollMfaItems`: Renders the list of available MFA methods
-   `EnrollMfaItem`: Renders an individual MFA method item
-   `EnrollMfaError`: Displays error messages
-   `EnrollMfaFooter`: The “Secured by Coinbase” footer

### EnrollMfa

The `EnrollMfa` component accepts a `children` prop that can be either React nodes or a render function. When using a render function, it receives the current `EnrollMfaState` as an argument, providing access to all state values without needing to use the `useEnrollMfaContext` hook directly. **Example of `children` as a render function:**

```
function MyEnrollMfaPage() {
  return (
    <EnrollMfa onEnrollSuccess={() => console.log('MFA enrolled!')}>
      {(state) => (
        <>
          <h1>
            {state.step === "list" && "Add Security"}
            {state.step === "setup" && "Set Up Method"}
            {state.step === "setup-verification" && "Verify Code"}
          </h1>
          <EnrollMfaTitle />
          <EnrollMfaDescription />
          <EnrollMfaFlow />
        </>
      )}
    </EnrollMfa>
  );
}

```

### State management (EnrollMfaProvider and EnrollMfaContext)

The entire enrollment flow’s state is managed by `EnrollMfaProvider` and accessed via the `useEnrollMfaContext` hook. This context contains:

-   `method`: The currently selected MFA method (`"totp"` or `"sms"`)
-   `methods`: Array of available MFA methods for enrollment
-   `step`: The current step (`list`, `setup`, or `setup-verification`)
-   `flowDirection`: The direction of the transition animation
-   `mfaCode`: The verification code entered by the user
-   `phoneNumber`: The phone number entered by the user (SMS enrollment)
-   `authUrl`: The otpauth:// URL for QR code generation (TOTP enrollment)
-   `secret`: The base32-encoded secret for manual entry (TOTP enrollment)
-   `initiatedAt`: Timestamp when enrollment was initiated (for timeout)
-   `isExpired`: Whether the enrollment session has expired
-   `error`: Any error that occurred during the process
-   `isPending`: Whether an async operation is in progress
-   `isSuccess`: Whether the enrollment was successful

### EnrollMfaFlow

The `EnrollMfaFlow` component manages the display of different views based on the current step. It handles transitions between:

-   `list`: The initial view showing available MFA methods
-   `setup`: The method-specific setup screen (QR code for TOTP, phone number entry for SMS)
-   `setup-verification`: The code verification step

`EnrollMfaFlow` provides a `children` render prop that receives an object containing the current `step`, `method`, and the `Content` component.

```
<EnrollMfaFlow>
  {({ step, method, Content }) => (
    <div className={`enroll-step-${step}`}>
      {step !== "list" && (
        <>
          <EnrollMfaImage step={step} method={method} />
          <EnrollMfaTitle step={step} method={method} />
          <EnrollMfaDescription step={step} method={method} />
        </>
      )}
      {Content}
    </div>
  )}
</EnrollMfaFlow>

```

### EnrollMfaTitle & EnrollMfaDescription

These components work similarly. By default, they use `useEnrollMfaContext` to get the current `step` and `method` and render the appropriate title or description. They also accept `step` and `method` props, which will override the values from the context. This is useful during transitions when you need to display the correct content for each transitioning view.

### Enrollment lifecycle

The enrollment process varies by method: **TOTP enrollment:**

1.  **List**: User sees available MFA methods and selects TOTP
2.  **Setup**: User scans QR code or manually enters secret in their authenticator app
3.  **Setup-verification**: User enters the 6-digit code from their authenticator to verify setup

**SMS enrollment:**

1.  **List**: User sees available MFA methods and selects SMS
2.  **Setup**: User enters their phone number
3.  **Setup-verification**: User enters the OTP code sent to their phone

The component monitors enrollment sessions with a 5-minute timeout for security. SMS enrollment also supports resending the OTP code.

## Example: Basic usage

**Implement a simple MFA enrollment interface:**

```
import { EnrollMfa } from '@coinbase/cdp-react';
function SetupMfa() {
  return (
    <EnrollMfa 
      onEnrollSuccess={() => {
        console.log('MFA enrollment complete!');
      }}
    />
  );
}

```

## Example: Custom layout with page title

**Customize the layout and add a page-level title:**

```
import {
  EnrollMfa,
  EnrollMfaDescription,
  EnrollMfaError,
  EnrollMfaFlow,
  EnrollMfaFlowBackButton,
  EnrollMfaImage,
  EnrollMfaTitle,
  useEnrollMfaContext,
} from '@coinbase/cdp-react';
function CustomEnrollMfaPage() {
  return (
    <EnrollMfa onEnrollSuccess={() => console.log('Enrolled!')}>
      <CustomEnrollMfaContent />
    </EnrollMfa>
  );
}
function CustomEnrollMfaContent() {
  const { state } = useEnrollMfaContext();
  return (
    <>
      <div className="page-header">
        <EnrollMfaFlowBackButton />
        <h1>
          {state.step === "list" && "Secure Your Account"}
          {state.step === "setup" && "Set Up Method"}
          {state.step === "setup-verification" && "Verify Setup"}
        </h1>
      </div>
      {state.step === "list" && (
        <>
          <EnrollMfaTitle />
          <EnrollMfaDescription />
          <EnrollMfaError />
        </>
      )}
      <EnrollMfaFlow>
        {({ step, method, Content }) => (
          <div className="enrollment-content">
            {step !== "list" && (
              <>
                <EnrollMfaImage step={step} method={method} />
                <EnrollMfaTitle step={step} method={method} />
                <EnrollMfaDescription step={step} method={method} />
              </>
            )}
            {Content}
          </div>
        )}
      </EnrollMfaFlow>
    </>
  );
}

```

## Example: Accessing state

**Display messages based on the enrollment state:**

```
import {
  EnrollMfa,
  EnrollMfaError,
  EnrollMfaFlow,
  EnrollMfaFlowBackButton,
  EnrollMfaTitle,
  EnrollMfaDescription,
} from '@coinbase/cdp-react';
function App() {
  return (
    <EnrollMfa onEnrollSuccess={() => alert('MFA enabled!')}>
      {(state) => (
        <>
          <div className="header">
            <EnrollMfaFlowBackButton />
            {state.isExpired && (
              <span className="warning">Session expired. Please start over.</span>
            )}
          </div>
          {state.step === "list" ? (
            <>
              <EnrollMfaTitle />
              <EnrollMfaDescription />
              <EnrollMfaError />
            </>
          ) : (
            <p className="progress">
              Step {state.step === "setup" ? "1" : "2"} of 2
            </p>
          )}
          <EnrollMfaFlow />
          {state.isSuccess && (
            <div className="success-banner">
              MFA has been successfully enabled!
            </div>
          )}
        </>
      )}
    </EnrollMfa>
  );
}

```

## API Reference

### Optional Props

-   `onEnrollSuccess`: Callback function invoked when MFA enrollment is successful
-   `resetOnSuccess`: Whether to reset the enrollment state when successful (defaults to `true`)
-   `children`: React nodes for custom layout, or a function that receives `EnrollMfaState`
-   `className`: Additional CSS classes to apply to the root element

## Notes

-   **Session timeout**: Enrollment sessions expire after 5 minutes for security. If expired, the user must start over.
-   **Supported methods**: TOTP (authenticator app) and SMS (phone number) enrollment are supported. Available methods are determined by the project’s MFA configuration.
-   **TOTP enrollment**: Users need an authenticator app (like Google Authenticator, Authy, or 1Password) to complete TOTP enrollment.
-   **SMS enrollment**: Users enter their phone number and verify with an OTP code sent via SMS. A resend option is available with a cooldown timer.