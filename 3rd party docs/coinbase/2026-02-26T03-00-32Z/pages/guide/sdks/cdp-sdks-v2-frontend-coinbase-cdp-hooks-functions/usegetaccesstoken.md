# usegetaccesstoken

```
function useGetAccessToken(): {
  getAccessToken: () => Promise<null | string>;
};

```

Hook to get the access token for the current user.

## Returns

```
{
  getAccessToken: () => Promise<null | string>;
}

```

Function to get the access token for the current user

### getAccessToken()

```
getAccessToken: () => Promise<null | string>;

```

#### Returns

`Promise`<`null` | `string`\>

## Example

```
const { getAccessToken } = useGetAccessToken();
const handleGetAccessToken = async () => {
  const accessToken = await getAccessToken();
  console.log("Access Token:", accessToken);
};

```