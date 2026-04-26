# recordmfaenrollmentprompted

```
function recordMfaEnrollmentPrompted(): Promise<RecordMfaEnrollmentPrompted200>;

```

Records that the end user was prompted for MFA enrollment. This endpoint should be called when the user is shown the MFA enrollment prompt, regardless of whether they choose to enroll or skip. This helps track when users were last offered the opportunity to enroll in MFA.

## Returns

`Promise`<`RecordMfaEnrollmentPrompted200`\> The result containing the timestamp when enrollment was prompted.

## Example

```
const result = await recordMfaEnrollmentPrompted();
console.log("MFA enrollment prompt recorded at:", result.enrollmentPromptedAt);

```