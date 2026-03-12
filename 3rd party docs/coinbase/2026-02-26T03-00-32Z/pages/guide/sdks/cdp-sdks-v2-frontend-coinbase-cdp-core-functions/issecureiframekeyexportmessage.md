# issecureiframekeyexportmessage

```
function isSecureIframeKeyExportMessage(message: unknown): message is SecureIframeOutgoingMessage | SecureIframeKeyExportIncomingMessage;

```

Checks if the message is a key export message.

## 

[​](#parameters)

Parameters

Parameter

Type

Description

`message`

`unknown`

The message to check.

## 

[​](#returns)

Returns

message is SecureIframeOutgoingMessage | SecureIframeKeyExportIncomingMessage True if the message is a key export message, false otherwise.