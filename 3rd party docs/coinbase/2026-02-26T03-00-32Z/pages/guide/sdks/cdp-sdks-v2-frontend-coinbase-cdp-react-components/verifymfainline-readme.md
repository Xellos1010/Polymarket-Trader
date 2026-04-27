# verifymfainline readme

The `VerifyMfaInline` component provides a wrapper for embedding MFA verification within existing UI flows. It handles the transition between protected content and MFA verification, supporting both “verify first” and “verify on demand” patterns. The core features include:

-   Inline MFA verification within existing UI contexts (modals, pages, etc.)
-   Support for “verify first” mode (MFA before showing content)
-   Support for “verify on demand” mode (content first, MFA when triggered)
-   Smooth slide or fade transitions between views
-   Automatic MFA detection and handling via event-based scoping
-   Composable UI that gives developers control over layout

## Architecture

The component is built using a composition pattern with a context provider that coordinates transitions between the verification view and protected content.

### Composition model

The `VerifyMfaInline` component is composed of several subcomponents that work together:

-   `VerifyMfaInline`: The root wrapper component that provides context and handles MFA event listening
-   `VerifyMfaInlineFlow`: Manages transitions between “verify” and “content” views
-   `VerifyMfaInlineBackButton`: A back button for navigating from verify view to content

### VerifyMfaInline

The `VerifyMfaInline` component wraps your protected content and automatically handles MFA verification when needed. It accepts props to control whether verification happens first or on demand. **Key props:**

-   `verifyFirst`: If `true`, shows MFA verification before content. If `false`, shows content first and triggers MFA when a protected action is called.
-   `onVerified`: Called when MFA verification completes successfully
-   `onCancel`: Called when the user cancels MFA verification
-   `successDelay`: Delay before transitioning to content after successful verification

### VerifyMfaInlineFlow

The `VerifyMfaInlineFlow` component manages the display of either the MFA verification form or your protected content. It supports both simple children (used as protected content) and a render function for full control. **Render function signature:**

```
children:
  | ReactNode
  | ((props: {
      view: "verify" | "content";
      Content: ReactNode | null;
    }) => ReactNode);

```

When using a render function:

-   For `view === "verify"`: `Content` is the default MFA verification UI
-   For `view === "content"`: `Content` is `null` (you provide the content)

### State management (useVerifyMfaInlineContext)

The `useVerifyMfaInlineContext` hook provides access to the inline verification state:

-   `view`: Current view (“verify” or “content”)
-   `goToVerify`: Function to navigate to the verify view
-   `goToContent`: Function to navigate to the content view
-   `goBack`: Function to go back (handles cancellation)

## Example: Verify first with ExportWallet

**Show MFA verification before allowing wallet export:**

```
import { VerifyMfaInline, VerifyMfaInlineFlow } from '@coinbase/cdp-react';
import { ExportWallet } from '@coinbase/cdp-react';
function ExportWalletModal({ address, onClose }) {
  return (
    <Modal>
      <VerifyMfaInline 
        verifyFirst
        onVerified={() => console.log('MFA verified!')}
      >
        <VerifyMfaInlineFlow>
          <ExportWallet address={address} skipMfa />
        </VerifyMfaInlineFlow>
      </VerifyMfaInline>
    </Modal>
  );
}

```

## Example: With back button

**Add a back button outside the transition for consistent placement:**

```
import {
  VerifyMfaInline,
  VerifyMfaInlineBackButton,
  VerifyMfaInlineFlow,
} from '@coinbase/cdp-react';
function ProtectedActionModal() {
  return (
    <Modal>
      <VerifyMfaInline verifyFirst>
        <div className="modal-header">
          <VerifyMfaInlineBackButton />
          <h2>Protected Action</h2>
        </div>
        <VerifyMfaInlineFlow transition="fade">
          <MyProtectedContent />
        </VerifyMfaInlineFlow>
      </VerifyMfaInline>
    </Modal>
  );
}

```

## Example: Render function for full control

**Use the render function to customize each view:**

```
import {
  VerifyMfaInline,
  VerifyMfaInlineBackButton,
  VerifyMfaInlineFlow,
} from '@coinbase/cdp-react';
function AdvancedModal() {
  return (
    <VerifyMfaInline verifyFirst onVerified={() => console.log('Verified!')}>
      <VerifyMfaInlineBackButton />
      <VerifyMfaInlineFlow>
        {({ view, Content }) => (
          <div className={`view-${view}`}>
            {view === "verify" ? (
              <>
                <h2>Verify Your Identity</h2>
                {Content}
              </>
            ) : (
              <>
                <h2>Protected Content</h2>
                <MyProtectedContent />
              </>
            )}
          </div>
        )}
      </VerifyMfaInlineFlow>
    </VerifyMfaInline>
  );
}

```

## Example: On-demand MFA verification

**Let MFA be triggered automatically when a protected action is called:**

```
import { VerifyMfaInline, VerifyMfaInlineFlow } from '@coinbase/cdp-react';
function SignTransactionFlow() {
  return (
    <VerifyMfaInline 
      verifyFirst={false}
      onVerified={() => console.log('MFA complete, transaction will proceed')}
      onCancel={() => console.log('User cancelled MFA')}
    >
      <VerifyMfaInlineBackButton />
      <VerifyMfaInlineFlow>
        <TransactionForm />
      </VerifyMfaInlineFlow>
    </VerifyMfaInline>
  );
}
function TransactionForm() {
  const { signTransaction } = useSignTransaction();
  
  const handleSign = async () => {
    // If MFA is required, VerifyMfaInline will automatically
    // transition to the verify view. Once verified, this
    // action will complete automatically.
    await signTransaction(txData);
  };
  
  return (
    <form onSubmit={handleSign}>
      {/* Transaction form fields */}
      <button type="submit">Sign Transaction</button>
    </form>
  );
}

```

## Example: Using the context hook

**Access the inline MFA context for custom navigation:**

```
import {
  VerifyMfaInline,
  VerifyMfaInlineFlow,
  useVerifyMfaInlineContext,
} from '@coinbase/cdp-react';
function CustomContent() {
  const { view, goToVerify, goBack } = useVerifyMfaInlineContext();
  
  return (
    <div>
      <p>Current view: {view}</p>
      {view === "content" && (
        <button onClick={goToVerify}>
          Verify MFA Manually
        </button>
      )}
    </div>
  );
}
function App() {
  return (
    <VerifyMfaInline verifyFirst={false}>
      <VerifyMfaInlineFlow>
        <CustomContent />
      </VerifyMfaInlineFlow>
    </VerifyMfaInline>
  );
}

```

## API Reference

### VerifyMfaInline Props

Prop

Type

Default

Description

`children`

`ReactNode`

required

Should include `VerifyMfaInlineFlow` and optionally `VerifyMfaInlineBackButton`

`verifyFirst`

`boolean`

`true`

If true, shows MFA verification before content

`onVerified`

`() => void`

\-

Called when MFA verification completes

`onCancel`

`() => void`

\-

Called when MFA verification is cancelled

`successDelay`

`number`

`500`

Delay (ms) before transitioning to content after success

`className`

`string`

\-

Additional CSS classes

### VerifyMfaInlineFlow Props

Prop

Type

Default

Description

`children`

`ReactNode | ((props) => ReactNode)`

required

Protected content or render function

`className`

`string`

\-

Additional CSS classes

`animateHeight`

`boolean`

`true`

Whether to animate height during transitions

`autoFocus`

`boolean`

`true`

Whether to auto-focus forms

`transition`

`"slide" | "fade"`

`"slide"`

Transition animation type

### VerifyMfaInlineBackButton Props

Prop

Type

Default

Description

`size`

`ButtonSize`

`"md"`

Button size

`variant`

`ButtonVariant`

`"transparentSecondary"`

Button variant

## Notes

-   **Scoped MFA handling**: The component uses event-based scoping to only handle MFA requests that originate from within its container. This allows multiple `VerifyMfaInline` instances on a page without conflicts.
-   **Automatic MFA detection**: When `verifyFirst` is `false`, the component listens for MFA requirements from protected actions and automatically transitions to the verify view when needed.
-   **Back button visibility**: The `VerifyMfaInlineBackButton` coordinates with the inner MFA verification flow. In `verifyFirst` mode, it only renders when the user has navigated within the inner MFA flow (e.g., to the method list or an alternate method). In non-`verifyFirst` mode, it renders on the verify view and handles both inner flow navigation and outer view transitions.
-   **For enrolled users only**: This component is designed for users who have already completed MFA enrollment. If MFA is not required for the user, content is shown directly without the verification step.