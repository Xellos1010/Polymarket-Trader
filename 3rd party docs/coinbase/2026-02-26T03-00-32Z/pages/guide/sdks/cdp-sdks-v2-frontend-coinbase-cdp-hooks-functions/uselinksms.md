# uselinksms

```
function useLinkSms(): {
  linkSms: (phoneNumber: string) => Promise<{
     flowId: string;
     message: string;
  }>;
};

```

A hook for linking a SMS account to the current user.

## 

[​](#returns)

Returns

```
{
  linkSms: (phoneNumber: string) => Promise<{
     flowId: string;
     message: string;
  }>;
}

```

An object containing the linkSms function.

### 

[​](#linksms)

linkSms()

```
linkSms: (phoneNumber: string) => Promise<{
  flowId: string;
  message: string;
}>;

```

#### 

[​](#parameters)

Parameters

Parameter

Type

`phoneNumber`

`string`

#### 

[​](#returns-2)

Returns

`Promise`<{ `flowId`: `string`; `message`: `string`; }>