# verifymfa readme

The `VerifyMfa` component provides a solution for prompting users to verify their identity using multi-factor authentication. It is designed for users who have already enrolled in MFA and need to enter a verification code to complete a sensitive action. The core features include:

-   TOTP (Time-based One-Time Password) verification via authenticator apps
-   SMS verification via phone number
-   Method selection when the user has multiple methods enrolled
-   Clean, focused verification interface
-   Composable UI that gives developers full control over layout and styling
-   Success and error callbacks for handling verification results
-   Centralized state management for the verification process

## Architecture

The component is built using a composition pattern that allows for maximum flexibility while providing sensible defaults when customization is not needed.

### Composition model

The `VerifyMfa` component is composed of several subcomponents that work together to create the verification experience. This approach allows developers to customize the UI structure while maintaining the underlying functionality. The main components are:

-   `VerifyMfa`: The root wrapper component that provides the `VerifyMfaContext` and manages state
-   `VerifyMfaTitle`: Renders the title for the verification interface
-   `VerifyMfaDescription`: Renders the description/instructions
-   `VerifyMfaImage`: Renders the verification icon/image
-   `VerifyMfaFlow`: Manages the verification form and multi-method navigation
-   `VerifyMfaFlowBackButton`: A button to navigate back within the multi-method flow
-   `VerifyMfaItems`: Renders the list of alternate MFA methods
-   `VerifyMfaItem`: Renders an individual MFA method item
-   `VerifyMfaFooter`: The “Secured by Coinbase” footer

### VerifyMfa

The `VerifyMfa` component accepts a `children` prop that can be either React nodes or a render function. When using a render function, it receives the current `VerifyMfaState` as an argument, providing access to all state values without needing to use the `useVerifyMfaContext` hook directly. **Example of `children` as a render function:**

```
function MyVerifyMfaPage() {
  return (
    <VerifyMfa onSuccess={(code) => console.log('Verified with code:', code)}>
      {(state) => (
        <>
          <VerifyMfaTitle />
          <VerifyMfaDescription />
          <VerifyMfaFlow />
          {state.error && <p className="error">{state.error}</p>}
          {state.isSuccess && <p className="success">Verified!</p>}
        </>
      )}
    </VerifyMfa>
  );
}

```

### State management (VerifyMfaProvider and VerifyMfaContext)

The entire verification flow’s state is managed by `VerifyMfaProvider` and accessed via the `useVerifyMfaContext` hook. This context contains:

-   `method`: The currently selected MFA method (`"totp"` or `"sms"`)
-   `methods`: Array of the user’s enrolled MFA methods
-   `step`: The current step (`"verification"` or `"list"`)
-   `flowDirection`: The direction of the transition animation
-   `mfaCode`: The verification code entered by the user
-   `error`: Any error that occurred during verification
-   `isPending`: Whether a verification request is in progress
-   `isSuccess`: Whether the verification was successful

### VerifyMfaFlow

The `VerifyMfaFlow` component manages the verification form and multi-method navigation. When the user has multiple enrolled methods, a “Use a different method” button is shown, allowing the user to switch between methods. The flow handles transitions between the verification screen and the method selection list. `VerifyMfaFlow` provides a `children` render prop that receives the current `step`, `method`, and the `Content` component.

```
<VerifyMfaFlow>
  {({ step, method, Content }) => (
    <div className={`verify-step-${step}`}>
      <VerifyMfaImage step={step} method={method} />
      <VerifyMfaTitle step={step} method={method} />
      <VerifyMfaDescription step={step} method={method} />
      {Content}
    </div>
  )}
</VerifyMfaFlow>

```

## Example: Basic usage

**Implement a simple MFA verification prompt:**

```
import { VerifyMfa } from '@coinbase/cdp-react';
function VerifyIdentity() {
  const handleSuccess = (mfaCode: string) => {
    console.log('Verified successfully!');
    // Proceed with the sensitive action
  };
  const handleError = (error: Error) => {
    console.error('Verification failed:', error);
  };
  return (
    <VerifyMfa 
      onSuccess={handleSuccess}
      onError={handleError}
    />
  );
}

```

## Example: Custom layout

**Customize the verification interface:**

```
import {
  VerifyMfa,
  VerifyMfaDescription,
  VerifyMfaFlow,
  VerifyMfaFlowBackButton,
  VerifyMfaImage,
  VerifyMfaTitle,
  useVerifyMfaContext,
} from '@coinbase/cdp-react';
function CustomVerifyMfa() {
  return (
    <VerifyMfa onSuccess={(code) => console.log('Verified!')}>
      <CustomVerifyContent />
    </VerifyMfa>
  );
}
function CustomVerifyContent() {
  const { state } = useVerifyMfaContext();
  return (
    <div className="custom-verify-container">
      <VerifyMfaFlowBackButton />
      <VerifyMfaFlow className="verify-form">
        {({ step, method, Content }) => (
          <>
            <VerifyMfaImage step={step} method={method} />
            <VerifyMfaTitle step={step} method={method} />
            <VerifyMfaDescription step={step} method={method} />
            {Content}
          </>
        )}
      </VerifyMfaFlow>
      {state.isPending && (
        <p className="status">Verifying...</p>
      )}
      {state.error && (
        <div className="error-message">
          {typeof state.error === 'string' ? state.error : state.error.message}
        </div>
      )}
    </div>
  );
}

```

## Example: Accessing state with render function

**Use the render function pattern for state access:**

```
import {
  VerifyMfa,
  VerifyMfaFlow,
  VerifyMfaTitle,
  VerifyMfaDescription,
} from '@coinbase/cdp-react';
function App() {
  return (
    <VerifyMfa 
      onSuccess={handleSuccess}
      onError={handleError}
    >
      {(state) => (
        <>
          <h2>Enter your code</h2>
          <VerifyMfaFlow />
          
          {state.isPending && <span>Verifying...</span>}
          {state.isSuccess && <span className="success">Verified!</span>}
          {state.error && (
            <span className="error">
              {typeof state.error === 'string' ? state.error : state.error.message}
            </span>
          )}
        </>
      )}
    </VerifyMfa>
  );
}

```

## API Reference

### Optional Props

-   `onSuccess`: Callback function invoked when verification is successful. Receives the MFA code as an argument.
-   `onError`: Callback function invoked when verification fails. Receives the error object.
-   `children`: React nodes for custom layout, or a function that receives `VerifyMfaState`
-   `className`: Additional CSS classes to apply to the root element

## Notes

-   **For enrolled users only**: This component is designed for users who have already completed MFA enrollment. Use `EnrollMfa` for the enrollment flow.
-   **Supported methods**: TOTP (authenticator app) and SMS verification are supported. The available methods are determined by the user’s enrolled MFA methods.
-   **Multi-method navigation**: When a user has multiple enrolled methods, a “Use a different method” button allows switching between them.
-   **SMS resend**: SMS verification includes a resend option with a cooldown timer.
-   **Automatic form submission**: The verification form automatically submits when the user enters a complete 6-digit code.