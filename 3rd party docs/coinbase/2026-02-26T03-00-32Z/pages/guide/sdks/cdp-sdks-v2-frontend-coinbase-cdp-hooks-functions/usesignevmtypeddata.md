# usesignevmtypeddata

```
function useSignEvmTypedData(): {
  signEvmTypedData: (options: SignEvmTypedDataOptions) => Promise<SignEvmTypedDataResult>;
};

```

Hook that provides a wrapped function to sign EIP-712 typed data with authentication checks. This hook uses useEnforceAuthenticated to ensure the user is signed in before attempting to sign.

## Returns

```
{
  signEvmTypedData: (options: SignEvmTypedDataOptions) => Promise<SignEvmTypedDataResult>;
}

```

### signEvmTypedData()

```
signEvmTypedData: (options: SignEvmTypedDataOptions) => Promise<SignEvmTypedDataResult>;

```

#### Parameters

#### Returns

`Promise`<[`SignEvmTypedDataResult`](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend/@coinbase/cdp-hooks/Type-Aliases/SignEvmTypedDataResult)\>

## Example

```
function SignTypedData() {
  const { signEvmTypedData } = useSignEvmTypedData();
  const { evmAddress } = useEvmAddress();
  const handleSign = async () => {
    if (!evmAddress) return;
    try {
      const result = await signEvmTypedData({
        evmAccount: evmAddress,
        typedData: {
          domain: {
            name: "USDC",
            version: "2",
            chainId: 84532,
            verifyingContract: "0x036CbD53842c5426634e7929541eC2318f3dCF7e"
          },
          types: {
            EIP712Domain: [
              { name: "name", type: "string" },
              { name: "version", type: "string" },
              { name: "chainId", type: "uint256" },
              { name: "verifyingContract", type: "address" }
            ],
            TransferWithAuthorization: [
              { name: "from", type: "address" },
              { name: "to", type: "address" },
              { name: "value", type: "uint256" },
              { name: "validAfter", type: "uint256" },
              { name: "validBefore", type: "uint256" },
              { name: "nonce", type: "bytes32" }
            ]
          },
          primaryType: "TransferWithAuthorization",
          message: {
            from: evmAddress,
            to: "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB",
            value: "1000000", // 1 USDC
            validAfter: 0,
            validBefore: 2524604400,
            nonce: 0
          }
        }
      });
      console.log("Signature:", result.signature);
    } catch (error) {
      console.error("Failed to sign typed data:", error);
    }
  };
  return (
    <button onClick={handleSign}>Sign Typed Data</button>
  );
}

```