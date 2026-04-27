# button

```
const Button: ForwardRefExoticComponent<ButtonProps & RefAttributes<HTMLButtonElement>>;

```

A themed button component with loading state.

## Param

The props for the component.

## Returns

The Button component.

## Examples

```
// Render a submit button
<Button type="submit" onClick={() => console.log("Button clicked")}>Click me</Button>

```

```
// Render a small secondary button
<Button type="button" variant="secondary" size="sm">Secondary button</Button>

```

```
// Render a full width button
<Button fullWidth>Full width button</Button>

```

```
// Render a button with a pending state
<Button isPending pendingLabel="Sending...">Send transaction</Button>

```

```
// Render a button with a custom button element
<Button onClick={() => console.log("Button clicked")} asChild>
  <button>Send transaction</button>
</Button>

```