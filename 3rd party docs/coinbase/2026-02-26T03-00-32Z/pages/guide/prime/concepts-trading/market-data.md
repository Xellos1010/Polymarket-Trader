# market data

## Order Book

Prime aggregates liquidity from multiple venues into a single, unified order book for each trading pair. This consolidated view provides clients with access to the best available prices and deeper liquidity than any individual venue could offer alone. The aggregated order book represents the current state of all listed buy and sell orders. This unified liquidity pool enables more efficient price discovery and better execution for all order types.

### Crossed Bids and Asks

Prime’s multi-venue liquidity aggregation may result in crossed bids and asks where the highest bid price exceeds the lowest ask price. Do not filter out crossed markets - they represent real liquidity available for execution and are an expected characteristic of aggregated order books. Prime’s execution engine handles these conditions appropriately during order processing.

### Real-time Order Book Updates

Prime provides a WebSocket feed that can be used to maintain a real-time copy of the order book. The `l2_data` provides both an initial snapshot and real-time updates, enabling you to maintain an accurate local representation of the order book state. This is essential for Crypto-as-a-Service (CaaS) applications requiring real-time pricing information. **Sequence Number Tracking**: Each `l2_data` message contains a monotonically increasing sequence number. You must track these sequence numbers to ensure no updates are missed. If you detect a gap in sequence numbers, immediately disconnect and reconnect to request a fresh snapshot. **Heartbeat Monitoring**: Implement a `heartbeats` channel subscription in parallel with `l2_data` to monitor connection health. If heartbeats stop arriving, treat this as a connection issue and reconnect. **Update Processing**: No delta math is required when processing updates. Simply replace the existing `px` level with the updated `qty` at that level. If you receive a quantity of `0` for a price level, this indicates there is no more liquidity at that price level - remove it from your local order book until you receive the next update at that price.

#### Example: Maintaining Best Bid and Ask

The following Java example demonstrates how to connect to the Prime WebSocket feed and maintain real-time best bid and best ask prices:

```
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.java_websocket.client.WebSocketClient;
import org.java_websocket.handshake.ServerHandshake;
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;
import java.net.URI;
import java.nio.charset.StandardCharsets;
import java.security.GeneralSecurityException;
import java.util.*;
public class BestBidAskPrinter {
    private static final String WS_URI       = "wss://ws-feed.prime.coinbase.com";
    private static final String CHANNEL      = "l2_data";
    private static final String[] PRODUCT_IDS = {"ETH-USD", "BTC-USD"};
    private static class Book {
        final TreeMap<Double, Double> bids = new TreeMap<>(Collections.reverseOrder());
        final TreeMap<Double, Double> asks = new TreeMap<>();
    }
    private final Map<String, Book> books = new HashMap<>();
    private static final ThreadLocal<ObjectMapper> MAPPER =
            ThreadLocal.withInitial(ObjectMapper::new);
    public static void main(String[] args) throws Exception {
        new BestBidAskPrinter().start();
    }
    private void start() throws Exception {
        WebSocketClient client = new WebSocketClient(new URI(WS_URI)) {
            @Override public void onOpen(ServerHandshake hs) { send(buildSubscribeMessage()); }
            @Override public void onMessage(String msg)      { handle(msg); }
            @Override public void onClose(int c,String r,boolean rem){ System.out.println("WebSocket closed: "+r+" – reconnecting…");reconnect();}
            @Override public void onError(Exception ex)      { System.err.println("WebSocket error: "+ex.getMessage()); }
        };
        client.connectBlocking();
    }
    /* ---------- message handling ---------- */
    private void handle(String raw) {
        try {
            JsonNode root = MAPPER.get().readTree(raw);
            if (!CHANNEL.equals(root.path("channel").asText())) return;
            JsonNode events = root.path("events");
            if (!events.isArray() || events.isEmpty()) return;
            JsonNode evt  = events.get(0);
            String type    = evt.path("type").asText();
            String product = evt.path("product_id").asText();
            if (product.isEmpty()) return;
            JsonNode updates = evt.path("updates");
            if (!updates.isArray()) return;
            books.computeIfAbsent(product, p -> new Book());
            Book book = books.get(product);
            if ("snapshot".equals(type)) { book.bids.clear(); book.asks.clear(); }
            applyUpdates(updates, book);
            printBBA(product, book);
        } catch (Exception ex) {
            System.err.println("Parse error: " + ex.getMessage());
        }
    }
    private void applyUpdates(JsonNode updates, Book book) {
        for (JsonNode u : updates) {
            String side = u.path("side").asText();
            double px   = u.path("px").asDouble();
            double qty  = u.path("qty").asDouble();
            TreeMap<Double, Double> depth = "bid".equals(side) ? book.bids : book.asks;
            if (qty == 0.0) depth.remove(px); else depth.put(px, qty);
        }
    }
    private void printBBA(String product, Book book) {
        Map.Entry<Double, Double> bid = book.bids.firstEntry();
        Map.Entry<Double, Double> ask = book.asks.firstEntry();
        if (bid != null && ask != null) {
            System.out.printf(
                    "%s → Best Bid: %.8f (qty %.6f) | Best Ask: %.8f (qty %.6f)%n",
                    product, bid.getKey(), bid.getValue(), ask.getKey(), ask.getValue());
        }
    }
    private String buildSubscribeMessage() {
        long currentTimeMillis = System.currentTimeMillis();
        String ts = String.valueOf(currentTimeMillis / 1000);
        String key = env("API_KEY"), sec = env("SECRET_KEY"),
                pas = env("PASSPHRASE"), acct = env("SVC_ACCOUNTID");
        String sig = sign(CHANNEL, key, sec, acct, ts, PRODUCT_IDS);
        String prods = String.join("\",\"", PRODUCT_IDS);
        return String.format(
                "{"
                        + "\"type\":\"subscribe\","
                        + "\"channel\":\"%s\","
                        + "\"access_key\":\"%s\","
                        + "\"api_key_id\":\"%s\","
                        + "\"timestamp\":\"%s\","
                        + "\"passphrase\":\"%s\","
                        + "\"signature\":\"%s\","
                        + "\"product_ids\":[\"%s\"]"
                        + "}",
                CHANNEL, key, acct, ts, pas, sig, prods
        );
    }
    private static String sign(String ch, String key, String secret,
                               String acct, String ts, String[] prods) {
        String joined = String.join("", prods);
        String msg    = ch + key + acct + ts + joined;
        try {
            Mac mac = Mac.getInstance("HmacSHA256");
            mac.init(new SecretKeySpec(secret.getBytes(StandardCharsets.UTF_8), "HmacSHA256"));
            return Base64.getEncoder()
                    .encodeToString(mac.doFinal(msg.getBytes(StandardCharsets.UTF_8)));
        } catch (GeneralSecurityException e) { throw new RuntimeException(e); }
    }
    private static String env(String n) {
        String v = System.getenv(n);
        if (v == null || v.isEmpty())
            throw new IllegalStateException("Missing env var: " + n);
        return v;
    }
}

```

For a comprehensive guide on maintaining a full order book with proper state management and error handling, refer to [Maintaining an Order Book](https://developer.chrome.com/prime/websocket-feed/channels#maintaining-an-order-book).

## Candles Data

Prime provides historical OHLCV (Open, High, Low, Close, Volume) candle data via REST API. This data is returned as an array based on two ISO8601 timestamps (e.g., `2025-01-01T00:00:00Z`), with support for up to 350 candles per request. You can specify granularity to establish the bin size, ranging from `ONE_MINUTE` to `ONE_DAY`. Candles data is especially useful for:

-   Creating price charts and technical analysis
-   Calculating price changes over time periods
-   Building trading algorithms based on historical patterns

### 24-Hour Price Change Calculation

The following example demonstrates how to calculate 24-hour price changes using the Prime Python SDK and Candles API:

```
import argparse
from datetime import datetime, timedelta, timezone
from prime_sdk.credentials import Credentials
from prime_sdk.client import Client
from prime_sdk.services.products import ProductsService, GetProductCandlesRequest
def calculate_24h_change(products_service, portfolio_id, product_id):
    """
    Calculate 24-hour price change for a given product.
    Uses single API call with FIVE_MINUTES granularity to get 24 hours of data.
    A more complex version of this script would use two separate API requests.
    Args:
        products_service: ProductsService instance
        portfolio_id: The portfolio ID
        product_id: The product to analyze (e.g., "BTC-USD")
    Returns:
        dict: Contains current_price, price_24h_ago, change_amount, change_percentage
    """
    # Calculate timestamps (API expects ISO8601 format)
    from datetime import datetime, timedelta, timezone
    current_time = datetime.now(timezone.utc)
    past_time = current_time - timedelta(hours=24)
    # Convert to ISO8601 format strings
    current_time_iso = current_time.isoformat()
    past_time_iso = past_time.isoformat()
    # Get 24 hours of 5-minute candles (288 candles total)
    request = GetProductCandlesRequest(
        portfolio_id=portfolio_id,
        product_id=product_id,
        granularity="FIVE_MINUTES",
        start_time=past_time_iso,
        end_time=current_time_iso
    )
    
    response = products_service.get_product_candles(request)
    
    if not response.candles or len(response.candles) < 2:
        raise Exception("Insufficient price data available")
        
    price_24h_ago = float(response.candles[0].close)
    current_price = float(response.candles[-1].close)
    
    change_amount = current_price - price_24h_ago
    change_percentage = (change_amount / price_24h_ago) * 100
    
    results = {
        'product_id': product_id,
        'current_price': current_price,
        'price_24h_ago': price_24h_ago,
        'change_amount': change_amount,
        'change_percentage': change_percentage,
        'candles_count': len(response.candles)
    }
    
    print(f"Product: {product_id}")
    print(f"Current Price: ${current_price:,.2f}")
    print(f"Price 24h Ago: ${price_24h_ago:,.2f}")
    print(f"Change Amount: ${change_amount:+,.2f}")
    print(f"Change Percentage: {change_percentage:+.2f}%")
    print(f"Data points: {len(response.candles)} candles")
        
    return results
def main():
    parser = argparse.ArgumentParser(description="Calculate 24-hour price change for a product")
    parser.add_argument("--product-id", nargs='+', required=True, help="Product ID(s) (e.g., BTC-USD or BTC-USD ETH-USD SOL-USD)")
    args = parser.parse_args()
    credentials = Credentials.from_env()
    client = Client(credentials)
    products_service = ProductsService(client)
    products_to_analyze = args.product_id
    for product in products_to_analyze:
        try:
            calculate_24h_change(products_service, credentials.portfolio_id, product)
            print(f"\n{'='*60}")
        except Exception as e:
            print(f"Failed to analyze {product}: {e}")
            print(f"\n{'='*60}")
if __name__ == "__main__":
    main()

```

### Plotting Candles Example

Here’s an example showing how to get candle data and create a candlestick chart using matplotlib:

```
import os
import time
import json
import base64
import hmac
import hashlib
import requests
import matplotlib.pyplot as plt
import matplotlib.dates as mdates
from datetime import datetime, timedelta, timezone
from urllib.parse import urlparse, quote
API_KEY = os.environ.get('ACCESS_KEY')
SECRET_KEY = os.environ.get('SIGNING_KEY')
PASSPHRASE = os.environ.get('PASSPHRASE')
PORTFOLIO_ID = os.environ.get('PORTFOLIO_ID')
end_time = datetime.now(timezone.utc)
start_time = end_time - timedelta(hours=350)
# Convert to ISO8601 format strings
end_timestamp = end_time.isoformat()
start_timestamp = start_time.isoformat()
uri = f'https://api.prime.coinbase.com/v1/portfolios/{PORTFOLIO_ID}/candles?product_id=BTC-USD&granularity=ONE_HOUR&start_time={quote(start_timestamp)}&end_time={quote(end_timestamp)}'
url_path = urlparse(uri).path
timestamp = str(int(time.time()))
message = timestamp + 'GET' + url_path
signature_b64 = base64.b64encode(hmac.digest(SECRET_KEY.encode(), message.encode(), hashlib.sha256))
headers = {
   'X-CB-ACCESS-SIGNATURE': signature_b64,
   'X-CB-ACCESS-TIMESTAMP': timestamp,
   'X-CB-ACCESS-KEY': API_KEY,
   'X-CB-ACCESS-PASSPHRASE': PASSPHRASE,
   'Accept': 'application/json'
}
response = requests.get(uri, headers=headers)
parsed_response = json.loads(response.text)
if 'candles' in parsed_response:
    candles = parsed_response['candles']
    
    timestamps = [datetime.fromisoformat(candle['timestamp'].replace('Z', '+00:00')) for candle in candles]
    opens = [float(candle['open']) for candle in candles]
    highs = [float(candle['high']) for candle in candles]
    lows = [float(candle['low']) for candle in candles]
    closes = [float(candle['close']) for candle in candles]
    
    fig, ax = plt.subplots(figsize=(12, 8))
    
    for i in range(len(timestamps)):
        color = 'green' if closes[i] >= opens[i] else 'red'
        
        ax.plot([timestamps[i], timestamps[i]], [lows[i], highs[i]], color='black', linewidth=1)
        
        body_height = abs(closes[i] - opens[i])
        body_bottom = min(opens[i], closes[i])
        
        ax.bar(timestamps[i], body_height, bottom=body_bottom, width=0.8/24, 
               color=color, alpha=0.7, edgecolor='black', linewidth=0.5)
    
    ax.set_title('BTC-USD Candlestick Chart', fontsize=16)
    ax.set_xlabel('Time', fontsize=12)
    ax.set_ylabel('Price (USD)', fontsize=12)
    
    ax.xaxis.set_major_formatter(mdates.DateFormatter('%m-%d %H:%M'))
    ax.xaxis.set_major_locator(mdates.HourLocator(interval=6))
    plt.xticks(rotation=45)
    
    plt.tight_layout()
    plt.grid(True, alpha=0.3)
    plt.show()
else:
    print("Error: No candles data in response")
    print(json.dumps(parsed_response, indent=3))

```