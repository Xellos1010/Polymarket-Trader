# fix api overview

[Financial Information eXchange](http://en.wikipedia.org/wiki/Financial_Information_eXchange), or FIX, is a standard protocol which can be used to enter orders, submit cancel requests, and receive fills. FIX API users typically have existing software that runs FIX for order management. The baseline specification for the INTX FIX API is [FIX 5.0](https://www.onixs.biz/fix-dictionary/5.0/index.html).

## Supported Endpoints

**Production**

-   Order Entry: `tcp+ssl://fix.international.coinbase.com:6110`  
    
-   Market Data: `tcp+ssl://fix.international.coinbase.com:6120`  
    
-   Drop Copy: `tcp+ssl://fix.international.coinbase.com:6130`  
    

**Sandbox**

-   Order Entry: `tcp+ssl://n5e2.coinbase.com:6110`  
    
-   Market Data: `tcp+ssl://n5e2.coinbase.com:6120`  
    
-   Drop Copy: `tcp+ssl://n5e2.coinbase.com:6130`  
    

## FIX Gateway

Before logging onto a FIX session, clients must establish a secure connection to the FIX gateway. See the [available endpoints](#endpoints) above. **TCP SSL** If your FIX implementation does not support establishing a **native TCP SSL connection**, you must setup a local proxy such as [stunnel](https://www.stunnel.org/) to establish a secure connection to the FIX gateway. **AWS IP** If connecting from servers **outside of AWS** which require firewall rules, use the [AWS provided resources](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html) to determine how best to allowlist AWS IP ranges.

## Ciphers

Coinbase Exchange supports **TLSv1.2** with the following server ciphers:

Recommend

Length

Cipher Suite

Elliptic Curve

Preferred

128 bits

ECDHE-RSA-AES128-GCM-SHA256

Curve P-256 DHE 256

Accepted

128 bits

ECDHE-RSA-AES128-SHA256

Curve P-256 DHE 256

Accepted

256 bits

ECDHE-RSA-AES256-GCM-SHA384

Curve P-256 DHE 256

Accepted

256 bits

ECDHE-RSA-AES256-SHA384

Curve P-256 DHE 256

## Sequence Numbers

The exchange resets the sequences weekly on Saturday at 12:00 CDT. Resets may also be initiated by a firm sending a Logon message with `ResetSeqNumFlag=Y`.