# uselinkemail

```
function useLinkEmail(): {
  linkEmail: (email: string) => Promise<{
     flowId: string;
     message: string;
  }>;
};

```

A hook for linking an email account to the current user.

## 

[​](#returns)

Returns

```
{
  linkEmail: (email: string) => Promise<{
     flowId: string;
     message: string;
  }>;
}

```

An object containing the linkEmail function.

### 

[​](#linkemail)

linkEmail()

```
linkEmail: (email: string) => Promise<{
  flowId: string;
  message: string;
}>;

```

#### 

[​](#parameters)

Parameters

Parameter

Type

`email`

`string`

#### 

[​](#returns-2)

Returns

`Promise`<{ `flowId`: `string`; `message`: `string`; }>