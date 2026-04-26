# websocket authentication

This guide explains how to authenticate requests to the Advanced Trade [WebSocket API](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels) server channels. It assumes that you have already [created API keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication).

## Sending Messages with API Keys

### Making Requests

Use the code samples below to generate/export a JSON Web Token (JWT) and make an authenticated request.

### Generating a JWT

Regardless of which [code snippet](#code-samples) you use, follow these steps:

1.  Replace `key name` and `key secret` with your key name and private key. `key secret` is a multi-line key and newlines must be preserved to properly parse the key. Do this on one line with `\n` escaped newlines, or with a multi-line string.
2.  Run the generation script that prints the command `export JWT=...`.
3.  Run the generated command to save your JWT.

### Code samples

The easiest way to generate a JWT is to use the built-in functions in our Python SDK as described below. Otherwise, use the code samples below to generate/export a JWT and make an authenticated request.

-   Python SDK
    
-   Python
    
-   Go
    
-   JavaScript
    
-   PHP
    
-   Java
    
-   C++
    
-   TypeScript
    
-   C#
    
-   Ruby
    

1.  Install the SDK.
    
    ```
    pip3 install coinbase-advanced-py
    
    ```
    
2.  In the console, run: `python main.py` (or whatever your file name is).
3.  Set the JWT to that output, or export the JWT to the environment with `export JWT=$(python main.py)`.

```
from coinbase import jwt_generator
api_key = "organizations/{org_id}/apiKeys/{key_id}"
api_secret = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n"
def main():
    jwt_token = jwt_generator.build_ws_jwt(api_key, api_secret)
    print(f"export JWT={jwt_token}")
if __name__ == "__main__":
    main()

```

1.  Install dependencies PyJWT and cryptography.
    
    ```
    pip install PyJWT
    pip install cryptography
    
    ```
    
2.  In the console, run: `python main.py` (or whatever your file name is).
3.  Set JWT to that output, or export the JWT to the environment with `export JWT=$(node main.py)`.

```
import jwt
from cryptography.hazmat.primitives import serialization
import time
import secrets
key_name     = "organizations/{org_id}/apiKeys/{key_id}"
key_secret   = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n"
def build_jwt():
    private_key_bytes = key_secret.encode('utf-8')
    private_key = serialization.load_pem_private_key(private_key_bytes, password=None)
    jwt_payload = {
        'sub': key_name,
        'iss': "cdp",
        'nbf': int(time.time()),
        'exp': int(time.time()) + 120,
    }
    jwt_token = jwt.encode(
        jwt_payload,
        private_key,
        algorithm='ES256',
        headers={'kid': key_name, 'nonce': secrets.token_hex()},
    )
    return jwt_token
def main():
    jwt_token = build_jwt()
    print(f"export JWT={jwt_token}")
if __name__ == "__main__":
    main()

```

1.  Create a new directory and generate a Go file called `main.go`.
2.  Paste the Go snippet below into `main.go`.
3.  Run `go mod init jwt-generator` and `go mod tidy` to generate `go.mod` and `go.sum` and manage your dependencies.
4.  In the console, run: go run `main.go`.
5.  Set your JWT with that output, or export the JWT to environment with `export JWT=$(node main.go)`.

```
package main
import (
    "crypto/rand"
    "crypto/x509"
    "encoding/pem"
    "fmt"
    "math"
    "math/big"
    "time"
    log "github.com/sirupsen/logrus"
    "gopkg.in/go-jose/go-jose.v2"
    "gopkg.in/go-jose/go-jose.v2/jwt"
)
const (
    keyName     = "organizations/{org_id}/apiKeys/{key_id}"
    keySecret   = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n"
)
type APIKeyClaims struct {
    *jwt.Claims
}
func buildJWT() (string, error) {
    block, _ := pem.Decode([]byte(keySecret))
    if block == nil {
        return "", fmt.Errorf("jwt: Could not decode private key")
    }
    key, err := x509.ParseECPrivateKey(block.Bytes)
    if err != nil {
        return "", fmt.Errorf("jwt: %w", err)
    }
    sig, err := jose.NewSigner(
        jose.SigningKey{Algorithm: jose.ES256, Key: key},
        (&jose.SignerOptions{NonceSource: nonceSource{}}).WithType("JWT").WithHeader("kid", keyName),
    )
    if err != nil {
        return "", fmt.Errorf("jwt: %w", err)
    }
    cl := &APIKeyClaims{
        Claims: &jwt.Claims{
            Subject:   keyName,
            Issuer:    "cdp",
            NotBefore: jwt.NewNumericDate(time.Now()),
            Expiry:    jwt.NewNumericDate(time.Now().Add(2 * time.Minute)),
        },
    }
    jwtString, err := jwt.Signed(sig).Claims(cl).CompactSerialize()
    if err != nil {
        return "", fmt.Errorf("jwt: %w", err)
    }
    return jwtString, nil
}
var max = big.NewInt(math.MaxInt64)
type nonceSource struct{}
func (n nonceSource) Nonce() (string, error) {
    r, err := rand.Int(rand.Reader, max)
    if err != nil {
        return "", err
    }
    return r.String(), nil
}
func main() {
    jwt, err := buildJWT()
    if err != nil {
        log.Errorf("error building jwt: %v", err)
    }
    fmt.Println("export JWT=" + jwt)
}

```

1.  Install jsonwebtoken.
2.  In the console, run: `node main.js` (or whatever your file name is).
3.  Set JWT to that output, or export the JWT to environment with `export JWT=$(node main.js)`.

```
const { sign } = require('jsonwebtoken');
const crypto = require('crypto');
const key_name       = 'organizations/{org_id}/apiKeys/{key_id}';
const key_secret = '-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n';
const algorithm = 'ES256';
const token = sign(
        {
           iss: 'cdp',
           nbf: Math.floor(Date.now() / 1000),
           exp: Math.floor(Date.now() / 1000) + 120,
           sub: key_name,
        },
        key_secret,
        {
           algorithm,
           header: {
              kid: key_name,
              nonce: crypto.randomBytes(16).toString('hex'),
           },
        }
);
console.log('export JWT=' + token);

```

1.  Add PHP dependencies with Composer (for JWT and environment variable management):
    
    ```
    composer require firebase/php-jwt
    composer require vlucas/phpdotenv
    
    ```
    
2.  Run `generate_jwt.php` (or a filename of your choice).
3.  Output the JWT to the command line and use a shell script to export it:
    
    ```
    export JWT=$(php generate_jwt.php)
    
    ```
    

  

> Code Snippet

```
<?php
require 'vendor/autoload.php';
use Firebase\JWT\JWT;
use \Dotenv\Dotenv;
// Load environment variables
$dotenv = Dotenv::createImmutable(__DIR__);
$dotenv->load();
function buildJwt() {
    $keyName = $_ENV['NAME'];
    $keySecret = str_replace('\\n', "\n", $_ENV['PRIVATE_KEY']);
    $privateKeyResource = openssl_pkey_get_private($keySecret);
    if (!$privateKeyResource) {
        throw new Exception('Private key is not valid');
    }
    $time = time();
    $nonce = bin2hex(random_bytes(16));  // Generate a 32-character hexadecimal nonce
    $jwtPayload = [
        'sub' => $keyName,
        'iss' => 'cdp',
        'nbf' => $time,
        'exp' => $time + 120,  // Token valid for 120 seconds from now
    ];
    $headers = [
        'typ' => 'JWT',
        'alg' => 'ES256',
        'kid' => $keyName,  // Key ID header for JWT
        'nonce' => $nonce  // Nonce included in headers for added security
    ];
    $jwtToken = JWT::encode($jwtPayload, $privateKeyResource, 'ES256', $keyName, $headers);
    return $jwtToken;
}

```

1.  Add Java Dependencies to your project’s Maven or Gradle configuration:
    
    ```
    nimbus-jose-jwt (version 9.39), bcpkix-jdk18on (version 1.78), and java-dotenv (version 5.2.2)
    
    ```
    
2.  Compile your Java application to generates a JWT, for example:
3.  Capture and export the JWT output from your Java application to an environment variable:
    
    ```
    export JWT=$(mvn exec:java -Dexec.mainClass=Main)
    
    ```
    

  

> Code Snippet

```
import com.nimbusds.jose.*;
import com.nimbusds.jose.crypto.*;
import com.nimbusds.jwt.*;
import java.security.interfaces.ECPrivateKey;
import java.util.Map;
import java.util.HashMap;
import java.time.Instant;
import java.util.Base64;
import org.bouncycastle.jce.provider.BouncyCastleProvider;
import org.bouncycastle.openssl.PEMParser;
import org.bouncycastle.openssl.jcajce.JcaPEMKeyConverter;
import java.security.spec.PKCS8EncodedKeySpec;
import java.security.KeyFactory;
import java.io.StringReader;
import java.security.PrivateKey;
import java.security.Security;
import io.github.cdimascio.dotenv.Dotenv;
public class Main {
    public static void main(String[] args) throws Exception {
        // Register BouncyCastle as a security providerx
        Security.addProvider(new BouncyCastleProvider());
        // Load environment variables
        Dotenv dotenv = Dotenv.load();
        String privateKeyPEM = dotenv.get("PRIVATE_KEY").replace("\\n", "\n");
        String name = dotenv.get("NAME");
        // create header object
        Map<String, Object> header = new HashMap<>();
        header.put("alg", "ES256");
        header.put("typ", "JWT");
        header.put("kid", name);
        header.put("nonce", String.valueOf(Instant.now().getEpochSecond()));
        // create data object
        Map<String, Object> data = new HashMap<>();
        data.put("iss", "cdp");
        data.put("nbf", Instant.now().getEpochSecond());
        data.put("exp", Instant.now().getEpochSecond() + 120);
        data.put("sub", name);
        // Load private key
        PEMParser pemParser = new PEMParser(new StringReader(privateKeyPEM));
        JcaPEMKeyConverter converter = new JcaPEMKeyConverter().setProvider("BC");
        Object object = pemParser.readObject();
        PrivateKey privateKey;
        if (object instanceof PrivateKey) {
            privateKey = (PrivateKey) object;
        } else if (object instanceof org.bouncycastle.openssl.PEMKeyPair) {
            privateKey = converter.getPrivateKey(((org.bouncycastle.openssl.PEMKeyPair) object).getPrivateKeyInfo());
        } else {
            throw new Exception("Unexpected private key format");
        }
        pemParser.close();
        // Convert to ECPrivateKey
        KeyFactory keyFactory = KeyFactory.getInstance("EC");
        PKCS8EncodedKeySpec keySpec = new PKCS8EncodedKeySpec(privateKey.getEncoded());
        ECPrivateKey ecPrivateKey = (ECPrivateKey) keyFactory.generatePrivate(keySpec);
        // create JWT
        JWTClaimsSet.Builder claimsSetBuilder = new JWTClaimsSet.Builder();
        for (Map.Entry<String, Object> entry : data.entrySet()) {
            claimsSetBuilder.claim(entry.getKey(), entry.getValue());
        }
        JWTClaimsSet claimsSet = claimsSetBuilder.build();
        JWSHeader jwsHeader = new JWSHeader.Builder(JWSAlgorithm.ES256).customParams(header).build();
        SignedJWT signedJWT = new SignedJWT(jwsHeader, claimsSet);
        JWSSigner signer = new ECDSASigner(ecPrivateKey);
        signedJWT.sign(signer);
        String sJWT = signedJWT.serialize();
        System.out.println(sJWT);
    }
}

```

1.  Install C++ project dependencies like so:
    
    ```
    apt-get update
    apt-get install libcurlpp-dev libssl-dev
    git clone https://github.com/Thalhammer/jwt-cpp
    cd jwt-cpp
    mkdir build && cd build
    cmake ..
    make
    make install
    
    ```
    
2.  After you’ve saved your code to a file name, for example main.cpp, compile the program:
    
    ```
    g++ main.cpp -o myapp -lcurlpp -lcurl -lssl -lcrypto -I/usr/local/include -L/usr/local/lib -ljwt -std=c++17
    
    ```
    
3.  Capture and export the JWT output from your C++ application to an environment variable:

  

> Code Snippet

```
#include <iostream>
#include <sstream>
#include <string>
#include <curlpp/cURLpp.hpp>
#include <curlpp/Easy.hpp>
#include <curlpp/Options.hpp>
#include <jwt-cpp/jwt.h>
#include <openssl/evp.h>
#include <openssl/ec.h>
#include <openssl/pem.h>
#include <openssl/rand.h>
std::string create_jwt() {
    // Set request parameters
    std::string key_name = "organizations/{org_id}/apiKeys/{key_id}";
    std::string key_secret = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n";
    // Generate a random nonce
    unsigned char nonce_raw[16];
    RAND_bytes(nonce_raw, sizeof(nonce_raw));
    std::string nonce(reinterpret_cast<char*>(nonce_raw), sizeof(nonce_raw));
    // Create JWT token
    auto token = jwt::create()
        .set_subject(key_name)
        .set_issuer("cdp")
        .set_not_before(std::chrono::system_clock::now())
        .set_expires_at(std::chrono::system_clock::now() + std::chrono::seconds{120})
        .set_header_claim("kid", jwt::claim(key_name))
        .set_header_claim("nonce", jwt::claim(nonce))
        .sign(jwt::algorithm::es256(key_name, key_secret));
    return token;
};
int main() {
    try {
        std::string token = create_jwt();
        std::cout << "Generated JWT Token: " << token << std::endl;
    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
    return 0;
};

```

1.  Install the JSON Web Token and TypeScript dependencies:
    
    ```
    npm install jsonwebtoken
    npm install @types/jsonwebtoken
    npm install -g typescript
    
    ```
    
2.  Create a TypeScript file named `main.ts` and add the following code:
    
    ```
    import * as jwt from 'jsonwebtoken';
    import * as crypto from 'crypto';
    const keyName = 'organizations/{org_id}/apiKeys/{key_id}';
    const keySecret = `-----BEGIN EC PRIVATE KEY-----
    YOUR PRIVATE KEY
    -----END EC PRIVATE KEY-----`;
    const algorithm = 'ES256';
    const generateJWT = (): string => {
      const payload = {
        iss: 'cdp',
        nbf: Math.floor(Date.now() / 1000),
        exp: Math.floor(Date.now() / 1000) + 120,
        sub: keyName,
      };
      const header = {
        alg: algorithm,
        kid: keyName,
        nonce: crypto.randomBytes(16).toString('hex'),
      };
      return jwt.sign(payload, keySecret, { algorithm, header });
    };
    const main = () => {
      const token = generateJWT();
      console.log(token);
    };
    main();
    
    ```
    
3.  Compile the TypeScript file to JavaScript: This will generate a `main.js` file.
4.  Run the generated JavaScript file:
5.  Set the JWT to the output, or export the JWT to the environment with:
    
    ```
    export JWT=$(node main.js)
    
    ```
    

> Code Snippet

```
import * as jwt from 'jsonwebtoken';
import * as crypto from 'crypto';
const keyName = 'organizations/{org_id}/apiKeys/{key_id}';
const keySecret = `-----BEGIN EC PRIVATE KEY-----
YOUR PRIVATE KEY
-----END EC PRIVATE KEY-----`;
const algorithm = 'ES256';
const generateJWT = (): string => {
  const payload = {
    iss: 'cdp',
    nbf: Math.floor(Date.now() / 1000),
    exp: Math.floor(Date.now() / 1000) + 120,
    sub: keyName,
  };
  const header = {
    alg: algorithm,
    kid: keyName,
    nonce: crypto.randomBytes(16).toString('hex'),
  };
  return jwt.sign(payload, keySecret, { algorithm, header });
};
const main = () => {
  const token = generateJWT();
  console.log(token);
};
main();

```

1.  Create a new console project by running the following command:
2.  Open the Program.cs file in a text editor or IDE (e.g., Visual Studio Code, Visual Studio, or any text editor). Replace the contents of Program.cs with the provided bellow in the Code Snippet.
3.  Install C# project dependencies like so:
    
    ```
    dotnet add package Microsoft.IdentityModel.Tokens
    dotnet add package System.IdentityModel.Tokens.Jwt
    dotnet add package Jose-JWT
    
    ```
    
4.  Build the project by running the following command:
5.  Run the project by running the following command:

  

> Code Snippet

```
// Environment is .NET 6.0 C#
using Microsoft.IdentityModel.Tokens;
using System.IdentityModel.Tokens.Jwt;
using System.Security.Claims;
using System.Security.Cryptography;
using Jose;
namespace JwtTest {
    internal class Program {
        static Random random = new Random();
        static void Main(string[] args) {
            string name = "organizations/{org_id}/apiKeys/{key_id}";
            string cbPrivateKey = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n";
            string key = parseKey(cbPrivateKey);
            string token = generateToken(name, key);
            Console.WriteLine($"Token is valid? {isTokenValid(token, name, key)}");
        }
        static string generateToken(string name, string secret) {
             var privateKeyBytes = Convert.FromBase64String(secret); // Assuming PEM is base64 encoded
             using var key = ECDsa.Create();
             key.ImportECPrivateKey(privateKeyBytes, out _);
             var payload = new Dictionary<string, object>
             {
                 { "sub", name },
                 { "iss", "coinbase-cloud" },
                 { "nbf", Convert.ToInt64((DateTime.UtcNow - new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc)).TotalSeconds) },
                 { "exp", Convert.ToInt64((DateTime.UtcNow.AddMinutes(1) - new DateTime(1970, 1, 1, 0, 0, 0, DateTimeKind.Utc)).TotalSeconds) },
             };
             var extraHeaders = new Dictionary<string, object>
             {
                 { "kid", name },
                 // add nonce to prevent replay attacks with a random 10 digit number
                 { "nonce", randomHex(10) },
                 { "typ", "JWT"}
             };
             var encodedToken = JWT.Encode(payload, key, JwsAlgorithm.ES256, extraHeaders);
            // print token
            Console.WriteLine(encodedToken);
            return encodedToken;
        }
        static bool isTokenValid(string token, string tokenId, string secret) {
            if (token == null)
                return false;
            var key = ECDsa.Create();
            key?.ImportECPrivateKey(Convert.FromBase64String(secret), out _);
            var securityKey = new ECDsaSecurityKey(key) { KeyId = tokenId };
            try {
                var tokenHandler = new JwtSecurityTokenHandler();
                tokenHandler.ValidateToken(token, new TokenValidationParameters {
                    ValidateIssuerSigningKey = true,
                    IssuerSigningKey = securityKey,
                    ValidateIssuer = false,
                    ValidateAudience = false,
                    ClockSkew = TimeSpan.Zero
                }, out var validatedToken);
                return true;
            } catch {
                return false;
            }
        }
        static string parseKey(string key) {
            List<string> keyLines = new List<string>();
            keyLines.AddRange(key.Split('\n', StringSplitOptions.RemoveEmptyEntries));
            keyLines.RemoveAt(0);
            keyLines.RemoveAt(keyLines.Count - 1);
            return String.Join("", keyLines);
        }
        static string randomHex(int digits) {
            byte[] buffer = new byte[digits / 2];
            random.NextBytes(buffer);
            string result = String.Concat(buffer.Select(x => x.ToString("X2")).ToArray());
            if (digits % 2 == 0)
                return result;
            return result + random.Next(16).ToString("X");
        }
    }
}

```

1.  Install dependencies `JWT` and `OpenSSL`.
    
    ```
    gem install JWT
    gem install OpenSSL
    
    ```
    
2.  In the console, run: `ruby main.rb` (or whatever your file name is).
3.  Set the JWT to that output, or export the JWT to the environment with `export JWT=$(ruby main.rb)`.

```
require 'jwt'
require 'openssl'
require 'time'
require 'securerandom'
Key_name = "organizations/{org_id}/apiKeys/{key_id}"
Key_secret = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n"
def build_jwt()
    header = {
      typ: 'JWT',
      kid: Key_name,
      nonce: SecureRandom.hex(16)
    }
    claims = {
      sub: Key_name,
      iss: 'cdp',
      aud: ['cdp_service'],
      nbf: Time.now.to_i,
      exp: Time.now.to_i + 120, # Expiration time: 2 minute from now.
    }
    private_key = OpenSSL::PKey::read(Key_secret)
    JWT.encode(claims, private_key, 'ES256', header)
  end
token = build_jwt()
puts token

```

## Sending Messages without API Keys

### Subscribing

```
// Request
// Subscribe to ETH-USD and ETH-EUR with the level2 channel
{
    "type": "subscribe",
    "product_ids": [
        "ETH-USD",
        "ETH-EUR"
    ],
    "channel": "level2"
}

```

### Unsubscribing

```
// Request
{
    "type": "unsubscribe",
    "product_ids": [
        "ETH-USD",
        "ETH-EUR"
    ],
    "channel": "level2"
}

```

## Sequence Numbers

Most feed messages contain a sequence number. Sequence numbers are increasing integer values for each product, with each new message being exactly one sequence number greater than the one before it. Sequence numbers that are *greater than one integer value* from the previous number indicate that a message has been dropped. Sequence numbers that are *less* than the previous number can be ignored or represent a message that has arrived out of order. In either situation you may need to perform logic to make sure your system is in the correct state.

**See Also:**

-   [WebSocket Channels](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels)