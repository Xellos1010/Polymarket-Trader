# admin messages

The baseline specification for this API is [FIX 4.2](https://www.fixtrading.org/standards/fix-4-2/). Below, we’ve noted the places in which the FIX API for Coinbase Prime extends (or clarifies) the FIX spec. For example, there are custom tags with a four-digit number range, as allowed by the standard, which are unique to Prime. A standard header must be present at the start of every message in both directions. You should configure your sessions so that

-   `SenderCompID` = the Service Account ID associated with the API key as your `SenderCompID`
-   `TargetCompID` = the string `COIN`

This is typically accomplished via your FIX client’s configuration file.

Tag

Name

Description

8

BeginString

Must be `FIX.4.2`

49

SenderCompID

The Service Account ID (on messages from the client)

56

TargetCompID

Must be `COIN` (on messages from the client)

## Logon (A)

Sent by the client to initiate a session and by the server as an acknowledgement. Only one session can exist per connection — sending a Logon message within an established session results in an error. The Logon message sent by the client must be signed for security. The prehash string has the following fields, each joined by the empty string: `{timestamp}A{seqNum}{apiKey}{targetComp}{passphrase}`. There is no trailing separator. The RawData field should be a `base64` encoding of the HMAC signature. To establish multiple FIX connections, you must generate a new API key for each one. All messages must have a `SendingTime` value within 5 seconds of server time in UTC or they are rejected. Contact [primeops@coinbase.com](mailto:primeops@coinbase.com) to create an API key for accessing multiple portfolios in a single fix connection.

Tag

Name

Description

96

RawData

Client message signature (see below)

554

Password

Client API passphrase

9406

DropCopyFlag

If set to Y, execution reports are generated for all user orders (defaults to Y), if set to N execution reports are only generated for orders from the FIX session

9407

Access Key

Client API key

### Python Example

```
def toAdmin(self, message, sessionID):
    if message.getHeader().getField(35) == "A":
        rawData = self.sign(message.getHeader().getField(52), message.getHeader().getField(35),
                            message.getHeader().getField(34), self.API_KEY, message.getHeader().getField(56),
                            self.PASSPHRASE)
        message.setField(fix.StringField(554, self.PASSPHRASE))
        message.setField(fix.StringField(96, rawData))
        message.setField(fix.StringField(9407, self.API_KEY))
        msg = message.toString().replace(__SOH__, "|")
        logfix.info("(Admin) S >> %s" % msg)
def sign(self, t, msg_type, seq_num, access_key, target_comp_id, passphrase):
    message = ''.join([t, msg_type, seq_num, access_key, target_comp_id, passphrase]).encode("utf-8")
    hmac_key = self.API_SECRET
    signature = hmac.new(hmac_key.encode('utf-8'), message, hashlib.sha256)
    sign_b64 = base64.b64encode(signature.digest()).decode()
    return sign_b64

```

### Java Example

```
import quickfix.*;
import quickfix.field.*;
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;
import java.nio.charset.StandardCharsets;
import java.util.Base64;
public class PrimeFixAuth implements Application {
    private final String apiKey, apiSecret, passphrase, portfolioId;
    public PrimeFixAuth(String apiKey, String apiSecret, String passphrase, String portfolioId) {
        this.apiKey = apiKey;
        this.apiSecret = apiSecret;
        this.passphrase = passphrase;
        this.portfolioId = portfolioId;
    }
    @Override
    public void toAdmin(Message message, SessionID sessionId) {
        try {
            if (MsgType.LOGON.equals(message.getHeader().getString(MsgType.FIELD))) {
                String timestamp = message.getHeader().getString(SendingTime.FIELD);
                String seq       = Integer.toString(message.getHeader().getInt(MsgSeqNum.FIELD));
                String target    = sessionId.getTargetCompID();
                buildLogon(message, timestamp, seq, apiKey, apiSecret, passphrase, target, portfolioId);
            }
        } catch (FieldNotFound ignored) {}
    }
    public static void buildLogon(Message m, String ts, String seq,
                                  String apiKey, String apiSecret, String passphrase,
                                  String targetCompId, String portfolioId) {
        String sig = sign(ts, MsgType.LOGON, seq, apiKey, targetCompId, passphrase, apiSecret);
        m.setField(new Account(portfolioId));              // tag 1
        m.setField(new RawDataLength(sig.length()));       // tag 95
        m.setField(new RawData(sig));                      // tag 96
        m.setField(new Password(passphrase));              // tag 554
        m.setField(new StringField(9406, "Y"));            // DropCopyFlag
        m.setField(new StringField(9407, apiKey));         // AccessKey
    }
    public static String sign(String timestamp, String msgType, String sequence,
                              String apiKey, String targetCompId, String passphrase,
                              String apiSecret) {
        try {
            String payload = timestamp + msgType + sequence + apiKey + targetCompId + passphrase;
            Mac mac = Mac.getInstance("HmacSHA256");
            byte[] key = apiSecret.getBytes(StandardCharsets.UTF_8);
            mac.init(new SecretKeySpec(key, "HmacSHA256"));
            return Base64.getEncoder().encodeToString(mac.doFinal(payload.getBytes(StandardCharsets.UTF_8)));
        } catch (Exception e) {
            throw new RuntimeException("Failed to generate HMAC signature", e);
        }
    }
}

```

## Reject (3)

Sent by either side upon receipt of a message which cannot be processed, e.g., due to missing fields or an unsupported message type.

Tag

Name

Description

45

RefSeqNum

MsgSeqNum of the rejected incoming message

58

Text

Human-readable description of the error (optional)

371

RefTagID

Tag number of the field which caused the reject (optional)

372

RefMsgType

MsgType of the rejected incoming message

373

SessionRejectReason

Code to identify reason for the reject (for session-level rejections only)

## Business Message Reject (j)

Sent to reject an application-level message which fulfills session-level rules but cannot be rejected via any other means. For example, Coinbase is undergoing system-wide maintenance and the FIX API is unavailable.

Tag

Name

Description

45

RefSeqNum

MsgSeqNum of the rejected incoming message

58

Text

Human-readable description of the error (optional)

372

RefMsgType

MsgType of the rejected incoming message

380

BusinessRejectReason

Code to identify reason for the reject (for application-level rejections only)