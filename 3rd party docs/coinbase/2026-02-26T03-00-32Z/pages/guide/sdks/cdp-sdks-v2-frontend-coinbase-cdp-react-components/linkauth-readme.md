# linkauth readme

The `LinkAuth` component provides a comprehensive solution for managing linked authentication methods for the current user. It allows users to link or unlink multiple authentication methods (email, phone, OAuth providers) to their account, providing a flexible and composable interface for building custom authentication management experiences. The core features include:

-   Support for multiple authentication methods (email, phone, OAuth providers)
-   Multi-step authentication linking process
-   Composable UI that gives developers full control over layout and styling
-   Smooth transitions between authentication method views
-   Centralized state management for the link auth process

## Architecture

The component is built using a composition pattern that enables maximum flexibility while providing sensible defaults when customization is not needed.

### Composition model

The `LinkAuth` component is composed of several subcomponents that work together to create the complete authentication management experience. This approach allows developers to customize the UI structure while maintaining the underlying functionality. The main components are:

-   `LinkAuth`: The root wrapper component that provides the `LinkAuthContext` and manages state
-   `LinkAuthTitle`: Renders the title for the current view
-   `LinkAuthItems`: Renders the list of available authentication methods
-   `LinkAuthItem`: Renders an individual authentication method item
-   `LinkAuthFlow`: Manages the multi-view flow for linking authentication methods
-   `LinkAuthFlowBackButton`: A button to navigate back to the list view
-   `LinkAuthError`: Displays error messages when linking fails

### LinkAuth

The `LinkAuth` component accepts a `children` prop that can be either React nodes or a render function. When using a render function, it receives the current `LinkAuthState` as an argument, providing access to all state values without needing to use the `useLinkAuthContext` hook directly. **Render function signature:**

```
children?: ReactNode | ((state: LinkAuthState) => ReactNode)

```

The `LinkAuthState` object contains:

-   `methodToLink`: The authentication method currently being linked (or `null`)
-   `authMethods`: Array of available authentication methods with link status
-   `error`: Error message if linking failed (or `null`)
-   `isPending`: Boolean indicating if a linking operation is in progress

**Example of `children` as a render function:**

```
function MyLinkAuthPage() {
  return (
    <LinkAuth onLinkSuccess={(method) => console.log(`Linked ${method}`)}>
      {(state) => (
        <>
          <div className="header">
            <LinkAuthTitle />
            <LinkAuthFlowBackButton />
          </div>
          {state.methodToLink && (
            <p className="message">
              Linking {state.methodToLink}...
            </p>
          )}
          <div className="error">
            <LinkAuthError />
          </div>
          <LinkAuthFlow />
        </>
      )}
    </LinkAuth>
  );
}

```

This render function pattern is useful when you need to conditionally render UI based on the linking state without creating a separate component and calling `useLinkAuthContext`.

### State management (LinkAuthProvider and LinkAuthContext)

The entire link auth flow’s state is managed by `LinkAuthProvider` and accessed via the `useLinkAuthContext` hook. This context contains:

-   `methodToLink`: The authentication method currently being linked (if any)
-   `error`: Any error message that occurred during the linking process
-   `dispatch`: Function to update the link auth state

Actions that can be dispatched include:

-   `LINK_AUTH_METHOD`: Set the authentication method to link
-   `LINK_AUTH_METHOD_ERROR`: Set an error message
-   `RESET_STATE`: Reset the state back to the initial list view

### LinkAuthFlowProvider and useLinkAuthFlow

The `LinkAuthFlowProvider` manages the flow-specific context and provides helper functions for navigation and actions. The `useLinkAuthFlow` hook provides access to:

-   `link`: Function to initiate linking an authentication method
-   `back`: Function to navigate back in the flow
-   `onLinkSuccess`: Callback when linking is successful

### LinkAuthFlow

The `LinkAuthFlow` component manages the display of different views based on the state. It handles transitions between:

-   `list`: The main view showing all available authentication methods
-   `email`: The email linking flow
-   `sms`: The phone number linking flow

OAuth providers are handled differently: they open in a new window/tab rather than transitioning to a new view within the component. **Render function (optional):** `LinkAuthFlow` provides a `children` render prop that allows you to wrap or augment the content for each view. This is particularly useful for adding view-specific styling or additional UI elements. **Render function signature:**

```
children?: (props: { view: LinkAuthFlowView; Content: ReactNode }) => ReactNode

```

The render function receives:

-   `view`: The current view being rendered (`"list"`, `"email"`, or `"sms"`)
-   `Content`: The rendered content for that view

**Example:**

```
<LinkAuthFlow>
  {({ view, Content }) => (
    <div className={`link-auth-view-${view}`}>
      {view === "list" && <p className="instructions">Select a method to link to your account</p>}
      {Content}
      {view !== "list" && <p className="help-text">This will add an additional way to sign in</p>}
    </div>
  )}
</LinkAuthFlow>

```

This allows you to inject content before or after the main view content, or wrap it with custom styling based on which view is active.

### LinkAuthItems

The `LinkAuthItems` component renders the list of available authentication methods. By default, it uses the `authMethods` from the app config and automatically renders `LinkAuthItem` components for each method. It accepts an `onLink` callback to customize what happens when a user selects a method to link. **Render function (optional):** `LinkAuthItems` provides a `children` render prop that allows you to customize how individual authentication method items are rendered. This is useful if you want to use your own custom UI for the list items while still leveraging the LinkAuthItems logic for managing the list. **Render function signature:**

```
children?: (props: LinkAuthItemProps) => ReactNode

```

The render function receives props for each item including:

-   `authMethod`: The authentication method (`"email"`, `"sms"`, `"oauth:google"`, etc.)
-   `icon`: The icon element for the method
-   `label`: The display label for the method
-   `isLinked`: Boolean indicating if the method is already linked
-   `userAlias`: The linked identifier (email address, phone number, etc.)
-   `isPending`: Boolean indicating if this method is currently being linked
-   `onLink`: Function to call when linking this method

**Example:**

```
<LinkAuthItems>
  {(props) => (
    <div className="custom-auth-item">
      <span className="icon">{props.icon}</span>
      <div className="details">
        <strong>{props.label}</strong>
        {props.isLinked && <small>{props.userAlias}</small>}
      </div>
      {!props.isLinked && (
        <button onClick={props.onLink} disabled={props.isPending}>
          {props.isPending ? "Linking..." : "Link"}
        </button>
      )}
    </div>
  )}
</LinkAuthItems>

```

If no children are provided, `LinkAuthItems` will render the default `LinkAuthItem` component for each method.

### LinkAuthItem

The `LinkAuthItem` component renders an individual authentication method with:

-   An icon representing the authentication method
-   The method name
-   The user alias (e.g. email address, phone number, username) if already linked
-   A link button (if not already linked)
-   A disabled unlink button (if already linked)

## Example: Basic usage

Render a `LinkAuth` component with an `onLinkSuccess` handler:

```
import { LinkAuth } from '@coinbase/cdp-react';
function ManageAuthMethods() {
  return (
    <LinkAuth 
      onLinkSuccess={(method) => {
        console.log(`Successfully linked ${method}`);
      }}
    />
  );
}

```

## Example: Accessing state

Display a message based on the LinkAuth state:

```
import {
  LinkAuth,
  LinkAuthError,
  LinkAuthFlow,
  LinkAuthFlowBackButton,
  LinkAuthTitle,
} from '@coinbase/cdp-react';
function App() {
  return (
    <LinkAuth>
      {(state) => (
        <>
          <div className="header">
            <LinkAuthTitle />
            <LinkAuthFlowBackButton />
          </div>
          <div className="message">
            {state.methodToLink ? (
              <span>Currently linking: {state.methodToLink}</span>
            ) : (
              <span>Select a method to link</span>
            )}
          </div>
          <div className="error">
            <LinkAuthError />
          </div>
          <LinkAuthFlow />
        </>
      )}
    </LinkAuth>
  );
}

```

## Advanced example: Using SignInModal

Customize LinkAuthItems to show a modal for non-OAuth methods instead of transitioning in place:

```
import {
  LinkAuth,
  LinkAuthItems,
  SignInModal,
  SignInModalTrigger,
  useLinkAuthFlow,
  useAppConfig,
  type AuthMethod,
} from '@coinbase/cdp-react';
import { useCallback, useMemo, useState } from 'react';
function CustomLinkAuthItems() {
  const { link, back } = useLinkAuthFlow();
  const { authMethods } = useAppConfig();
  const [openModal, setOpenModal] = useState<AuthMethod | null>(null);
  const modalMethods = useMemo(
    () => authMethods.filter(method => !method.startsWith("oauth:")),
    [authMethods],
  );
  const handleClose = useCallback(() => {
    setOpenModal(null);
    back();
  }, [back, setOpenModal]);
  const handleLink = useCallback(
    (method: AuthMethod) => {
      link(method);
      if (!method.startsWith("oauth:")) {
        setOpenModal(method);
      }
    },
    [link, setOpenModal],
  );
  return (
    <>
      <LinkAuthItems onLink={handleLink} />
      {modalMethods.map(method => {
        return (
          <SignInModal
            key={method}
            open={openModal === method}
            authMethods={[method]}
            setIsOpen={isOpen => (isOpen ? setOpenModal(method) : handleClose())}
            onSuccess={() => setOpenModal(null)}
          >
            <SignInModalTrigger>null</SignInModalTrigger>
          </SignInModal>
        );
      })}
    </>
  );
}
function App() {
  return (
    <LinkAuth>
      <h2>Link a profile</h2>
      <CustomLinkAuthItems />
    </LinkAuth>
  );
}

```

## Notes

-   **Unlinking authentication methods is not yet implemented.** Users can add multiple authentication methods to their account, but cannot currently remove them.
-   OAuth providers perform a full page redirect for authentication. The user will temporarily leave your application to authenticate with the provider and then be redirected back.