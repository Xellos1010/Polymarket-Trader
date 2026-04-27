# userecordmfaenrollmentprompted

```
function useRecordMfaEnrollmentPrompted(): {
  recordMfaEnrollmentPrompted: () => Promise<RecordMfaEnrollmentPrompted200>;
};

```

A hook for recording when the end user was prompted for MFA enrollment. This should be called when the user is shown the MFA enrollment prompt, regardless of whether they choose to enroll or skip.

## Returns

```
{
  recordMfaEnrollmentPrompted: () => Promise<RecordMfaEnrollmentPrompted200>;
}

```

An object containing the recordMfaEnrollmentPrompted function.

### recordMfaEnrollmentPrompted()

```
recordMfaEnrollmentPrompted: () => Promise<RecordMfaEnrollmentPrompted200>;

```

#### Returns

`Promise`<`RecordMfaEnrollmentPrompted200`\>