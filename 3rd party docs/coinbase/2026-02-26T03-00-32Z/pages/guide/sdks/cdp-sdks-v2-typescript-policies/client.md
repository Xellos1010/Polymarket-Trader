# client

## Classes

### PoliciesClient

Defined in: [src/client/policies/policies.ts:25](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/policies/policies.ts#L25) The namespace containing all Policy methods.

#### Implements

-   [`PoliciesClientInterface`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/policies/Types#policiesclientinterface)

#### Constructors

##### Constructor

```
new PoliciesClient(): PoliciesClient;

```

###### Returns

[`PoliciesClient`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/policies/Client#policiesclient)

#### Methods

##### createPolicy()

```
createPolicy(options: CreatePolicyOptions): Promise<Policy>;

```

Defined in: [src/client/policies/policies.ts:181](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/policies/policies.ts#L181) Creates a new policy that can be used to govern the behavior of projects and accounts.

###### Parameters

###### options

[`CreatePolicyOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/policies/Types#createpolicyoptions) Options for creating the policy

###### Returns

`Promise`<`Policy`\> The created policy

###### Throws

When the policy is invalid

###### Examples

```
         const policy = await cdp.policies.createPolicy({
           policy: {
             scope: "account",
             description: "Limits the amount of ETH in transaction",
             rules: [
               {
                 action: "reject",
                 operation: "signEvmTransaction",
                 criteria: [
                   {
                     type: "ethValue",
                     ethValue: "1000000000000000000",
                     operator: ">",
                   },
                 ],
               },
             ],
           }
         });

```

```
         const policy = await cdp.policies.createPolicy({
           policy: {
             scope: "account",
             description: "Limits SOL transfers and SPL token operations",
             rules: [
               {
                 action: "reject",
                 operation: "signSolTransaction",
                 criteria: [
                   {
                     type: "solValue",
                     solValue: "1000000000", // 1 SOL in lamports
                     operator: ">",
                   },
                   {
                     type: "solAddress",
                     addresses: ["9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"],
                     operator: "in",
                   },
                 ],
               },
               {
                 action: "accept",
                 operation: "sendSolTransaction",
                 criteria: [
                   {
                     type: "mintAddress",
                     addresses: ["EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"], // USDC mint
                     operator: "in",
                   },
                 ],
               },
             ],
           }
         });

```

```
         const idempotencyKey = uuidv4();
         // First call creates the policy
         const policy = await cdp.policies.createPolicy({
           policy: {
             scope: "account",
             description: "Limits the amount of ETH in transaction",
             rules: [
               {
                 action: "reject",
                 operation: "signEvmTransaction",
                 criteria: [
                   {
                     type: "ethValue",
                     ethValue: "1000000000000000000",
                     operator: ">",
                   },
                 ],
               },
             ],
           },
           idempotencyKey
         });
         // Second call with same key returns the same policy
         const samePolicy = await cdp.policies.createPolicy({
           policy: { ... },
           idempotencyKey
         });

```

###### Implementation of

```
PoliciesClientInterface.createPolicy

```

##### deletePolicy()

```
deletePolicy(options: DeletePolicyOptions): Promise<void>;

```

Defined in: [src/client/policies/policies.ts:252](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/policies/policies.ts#L252) Deletes a policy by its unique identifier. If a policy is referenced by an active project or account, this operation will fail.

###### Parameters

###### options

[`DeletePolicyOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/policies/Types#deletepolicyoptions) Options containing the policy ID to delete

###### Returns

`Promise`<`void`\> Void on successful deletion

###### Examples

```
         await cdp.policies.deletePolicy({
           id: "__ID__"
         });

```

```
         const idempotencyKey = uuidv4();
         // This operation is idempotent with the key
         await cdp.policies.deletePolicy({
           id: "__ID__",
           idempotencyKey
         });

```

###### Implementation of

```
PoliciesClientInterface.deletePolicy

```

##### getPolicyById()

```
getPolicyById(options: GetPolicyByIdOptions): Promise<Policy>;

```

Defined in: [src/client/policies/policies.ts:216](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/policies/policies.ts#L216) Retrieves a policy by its unique identifier.

###### Parameters

###### options

[`GetPolicyByIdOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/policies/Types#getpolicybyidoptions) Options containing the policy ID to retrieve

###### Returns

`Promise`<`Policy`\> The requested policy

###### Example

```
         const policy = await cdp.policies.getPolicyById({
           id: "__ID__"
         });
         console.log(policy.name);
         console.log(policy.rules);

```

###### Implementation of

```
PoliciesClientInterface.getPolicyById

```

##### listPolicies()

```
listPolicies(options?: ListPoliciesOptions): Promise<ListPoliciesResult>;

```

Defined in: [src/client/policies/policies.ts:63](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/policies/policies.ts#L63) Lists policies belonging to the developer’s CDP Project. Can be filtered by scope (project or account).

###### Parameters

###### options?

[`ListPoliciesOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/policies/Types#listpoliciesoptions) = `{}` Options for filtering and paginating the results

###### Returns

`Promise`<[`ListPoliciesResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/policies/Types#listpoliciesresult)\> A paginated list of policies

###### Examples

```
         const { policies } = await cdp.policies.listPolicies();

```

```
         const { policies } = await cdp.policies.listPolicies({
           scope: 'project'
         });

```

```
         // Get first page
         const firstPage = await cdp.policies.listPolicies({
           pageSize: 10
         });
         // Get next page using cursor
         const nextPage = await cdp.policies.listPolicies({
           pageSize: 10,
           pageToken: firstPage.pageToken
         });

```

###### Implementation of

```
PoliciesClientInterface.listPolicies

```

##### updatePolicy()

```
updatePolicy(options: UpdatePolicyOptions): Promise<Policy>;

```

Defined in: [src/client/policies/policies.ts:340](https://github.com/coinbase/cdp-sdk/blob/8794662b60e721852bfb60801a1d0bb1bb6e4c59/typescript/src/client/policies/policies.ts#L340) Updates an existing policy by its unique identifier. This will apply the updated policy to any project or accounts that are currently using it.

###### Parameters

###### options

[`UpdatePolicyOptions`](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript/policies/Types#updatepolicyoptions) Options containing the policy ID and updated policy data

###### Returns

`Promise`<`Policy`\> The updated policy

###### Throws

When the updated policy is invalid

###### Examples

```
         const updatedPolicy = await cdp.policies.updatePolicy({
           id: "__ID__",
           policy: {
             description: "Now with lower transaction limits",
             rules: [
               {
                 action: "reject",
                 operation: "signEvmTransaction",
                 criteria: [
                   {
                     type: "ethValue",
                     ethValue: "1000000000",
                     operator: ">",
                   },
                 ],
               },
             ],
           },
         });

```

```
         const updatedPolicy = await cdp.policies.updatePolicy({
           id: "__ID__",
           policy: {
             description: "Updated Solana transaction limits",
             rules: [
               {
                 action: "reject",
                 operation: "signSolTransaction",
                 criteria: [
                   {
                     type: "splValue",
                     splValue: "1000000", // SPL token amount
                     operator: ">=",
                   },
                   {
                     type: "mintAddress",
                     addresses: ["EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"], // USDC mint
                     operator: "in",
                   },
                 ],
               },
             ],
           },
         });

```

```
         const idempotencyKey = uuidv4();
         // This operation is idempotent with the key
         await cdp.policies.updatePolicy({
           id: "__ID__",
           policy: {
             description: "Modified Policy",
             rules: { ... }
           },
           idempotencyKey
         });

```

###### Implementation of

```
PoliciesClientInterface.updatePolicy

```