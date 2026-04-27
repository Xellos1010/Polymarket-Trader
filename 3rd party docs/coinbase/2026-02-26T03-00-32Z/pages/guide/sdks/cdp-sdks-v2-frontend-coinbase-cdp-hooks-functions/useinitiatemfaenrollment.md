# useinitiatemfaenrollment

```
function useInitiateMfaEnrollment(): {
  initiateMfaEnrollment: (options: InitiateMfaOptions) => Promise<InitiateMfaEnrollmentResult>;
};

```

A hook for initiating MFA enrollment for the current user.

## Returns

```
{
  initiateMfaEnrollment: (options: InitiateMfaOptions) => Promise<InitiateMfaEnrollmentResult>;
}

```

An object containing the initiateMfaEnrollment function.

### initiateMfaEnrollment()

```
initiateMfaEnrollment: (options: InitiateMfaOptions) => Promise<InitiateMfaEnrollmentResult>;

```

#### Parameters

Parameter

Type

`options`

`InitiateMfaOptions`

#### Returns

`Promise`<`InitiateMfaEnrollmentResult`\>