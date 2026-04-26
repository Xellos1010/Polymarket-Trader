# channels

## Heartbeats Channel

```
// Request -> heartbeats channel
{
  "type": "subscribe",
  "channel": "heartbeats",
  "access_key": "ACCESS_KEY",
  "api_key_id": "SVC_ACCOUNTID",
  "timestamp": "TIMESTAMP",
  "passphrase": "PASSPHRASE",
  "signature": "SIGNATURE",
  "portfolio_id": "PORTFOLIO_ID",
  "product_ids": ["BTC-USD"]
}

```

Heartbeats indicate the current timestamp, as well as the number of messages sent.

```
// Response ->  heartbeats channel
{
  "channel": "subscriptions",
  "timestamp": "2022-01-25T20:52:59.353824785Z",
  "sequence_num": 0,
  "events": [
    {
      "subscriptions": {
        "heartbeats": ["heartbeats"]
      }
    }
  ]
}

```

## Products Channel

```
// Request -> products channel
{
  "type": "subscribe",
  "channel": "products",
  "access_key": "ACCESS_KEY",
  "api_key_id": "SVC_ACCOUNTID",
  "timestamp": "TIMESTAMP",
  "passphrase": "PASSPHRASE",
  "signature": "SIGNATURE",
  "portfolio_id": "PORTFOLIO_ID",
  "product_ids": []
}

```

Returns the full list of products available on Prime, along the corresponding product metadata:

```
// Response ->  products channel
{
   "channel":"subscriptions",
   "timestamp":"2026-01-28T21:36:58.668273009Z",
   "sequence_num":0,
   "events":[
      {
         "subscriptions":{
            "products":[
               "products"
            ]
         }
      }
   ]
}
{
   "channel":"products",
   "timestamp":"2026-01-28T21:36:58.668338999Z",
   "sequence_num":1,
   "events":[
      {
         "type":"snapshot",
         "products":[
            {
               "product_id":"BTC-USD",
               "base_min_size":"0.00000001",
               "base_max_size":"3400",
               "base_increment":"0.00000001",
               "quote_min_size":"1",
               "quote_max_size":"150000000",
               "quote_increment":"0.01",
               "price_increment":"",
               "permissions":"None",
               "rfq_product_details":{
                  "tradable":true,
                  "min_notional_size":"",
                  "max_notional_size":"",
                  "min_base_size":"0.00000001",
                  "max_base_size":"8.77",
                  "min_quote_size":"1",
                  "max_quote_size":"900000"
               }
            }
         ]
      }
      ...
      ...
      ...
   ]
}

```

## Orders Channel

The `orders` channel provides real-time updates on orders you’ve made.

```
//Request -> orders channel
{
  "type": "subscribe",
  "channel": "orders",
  "access_key": "ACCESS_KEY",
  "api_key_id": "SVC_ACCOUNTID",
  "timestamp": "TIMESTAMP",
  "passphrase": "PASSPHRASE",
  "signature": "SIGNATURE",
  "portfolio_id": "PORTFOLIO_ID",
  "product_ids": ["BTC-USD"]
}

```

```
//Response -> orders channel
{
  "channel": "orders",
  "timestamp": "2022-01-25T21:16:07.366595573Z",
  "sequence_num": 0,
  "events": [
    {
      "type": "snapshot",
      "orders": [
        {
          "order_id": "4c62681b-be8a-439c-af2b-5f0100386cc0",
          "client_order_id": "20a8cbe0-7680-4eba-9ffd-f9c2de89035d",
          "cum_qty": "0",
          "leaves_qty": "3",
          "avg_px": "0",
          "net_avg_px": "0",
          "fees": "0",
          "status": "OPEN",
          "product_id": "BTC-USD",
          "user_context": "", //field not present if empty
          "limit_px": "", //field not present if empty
          "order_type": "market",
          "side": "buy"
        }
      ]
    }
  ]
}

```

```
{
  "channel": "subscriptions",
  "timestamp": "2022-01-25T21:16:07.366625018Z",
  "sequence_num": 1,
  "events": [
    {
      "subscriptions": {
        "orders": ["PORTFOLIO_ID"]
      }
    }
  ]
}

```

### Python Example

Below is a detailed end to end Python script for subscribing to the `orders` channel:

```
import asyncio, base64, hashlib, hmac, json, os, sys, time, websockets
PASSPHRASE = os.environ.get('PASSPHRASE')
ACCESS_KEY = os.environ.get('ACCESS_KEY')
SECRET_KEY = os.environ.get('SIGNING_KEY')
SVC_ACCOUNTID = os.environ.get('SVC_ACCOUNTID')
PORTFOLIO_ID = os.environ.get('PORTFOLIO_ID')
uri = 'wss://ws-feed.prime.coinbase.com'
timestamp = str(int(time.time()))
channel = 'orders'
product_id = 'ETH-USD'
async def main_loop():
    while True:
      try:
        async with websockets.connect(uri, ping_interval=None, max_size=None) as websocket:
          signature = sign(channel, ACCESS_KEY, SECRET_KEY, SVC_ACCOUNTID, PORTFOLIO_ID, product_id)
          auth_message = json.dumps({
              'type': 'subscribe',
              'channel': channel,
              'access_key': ACCESS_KEY,
              'api_key_id': SVC_ACCOUNTID,
              'timestamp': timestamp,
              'passphrase': PASSPHRASE,
              'signature': signature,
              'portfolio_id': PORTFOLIO_ID,
              'product_ids': [product_id]
          })
          await websocket.send(auth_message)
          while True:
            response = await websocket.recv()
            parsed_response = json.loads(response)
            print(json.dumps(parsed_response, indent=3))
      except websockets.ConnectionClosed:
        continue
def sign(channel, key, secret, account_id, PORTFOLIO_ID, product_ids):
    message = channel + key + account_id + timestamp + PORTFOLIO_ID + product_ids
    signature = hmac.new(SECRET_KEY.encode('utf-8'), message.encode('utf-8'), digestmod=hashlib.sha256).digest()
    signature_b64 = base64.b64encode(signature).decode()
    return signature_b64
try:
    asyncio.get_event_loop().run_until_complete(main_loop())
except KeyboardInterrupt:
    print('\nClosing Prime websocket feed')
    sys.exit()

```

This script is open source and available on [GitHub](https://github.com/coinbase-samples/prime-scripts-py/blob/main/REST/prime_ws_orders.py).

## Level2 Data Channel

The `l2_data` channel guarantees delivery of all updates and is the easiest way to keep a snapshot of the order book.

```
// Request -> l2_data channel
{
  "type": "subscribe",
  "channel": "l2_data",
  "access_key": "ACCESS_KEY",
  "api_key_id": "SVC_ACCOUNTID",
  "timestamp": "TIMESTAMP",
  "passphrase": "PASSPHRASE",
  "signature": "SIGNATURE",
  "venue_filtering": false,
  "portfolio_id": "",
  "product_ids": ["BTC-USD"]
}

```

When sending a subscription message, Coinbase returns a snapshot of current channel subscriptions:

```
//Response -> subscriptions channel
[
  {
    "channel": "subscriptions",
    "client_id: "",
    "timestamp": "2022-02-16T22:56:34.770593004Z",
    "sequence_num": 0,
    "events": [
      {
        "subscriptions": {
          "l2_data": ["BTC-USD"]
        }
      }
    ]
  }
]

```

In addition, Coinbase returns an order book snapshot, an array of dictionaries containing the order’s side, event time, price, and quantity:

```
//Response -> l2_data channel
{
  "channel": "l2_data",
  "client_id: "",
  "timestamp": "2022-06-15T01:52:23.408309238Z",
  "sequence_num": 1,
  "events": [
    {
      "type": "snapshot",
      "product_id": "BTC-USD",
      "updates": [
        {
          "side": "bid",
          "event_time": "2022-06-15T01:52:23.385605Z",
          "px": "22145.3",
          "qty": "0.01083825"
        },
        {
          "side": "offer",
          "event_time": "2022-06-15T01:52:23.385605Z",
          "px": "22149.26",
          "qty": "0.03450782"
        }
      ]
    }
  ]
}

```

To return a filtered websocket feed based on venue configuration, change the `venue_filtering` request field to `true` and provide a `portfolio_id`.

```
// Request -> l2_data channel
{
  "type": "subscribe",
  "channel": "l2_data",
  "access_key": "ACCESS_KEY",
  "api_key_id": "SVC_ACCOUNTID",
  "timestamp": "TIMESTAMP",
  "passphrase": "PASSPHRASE",
  "signature": "SIGNATURE",
  "venue_filtering": true,
  "portfolio_id": "PORTFOLIO_ID",
  "product_ids": ["BTC-USD", "ETH-USD"]
}

```

A `venue_configuration` field will be included in each response

```
//Response -> l2_data channel
{
  "channel": "l2_data",
  "timestamp": "2022-06-15T01:52:23.408309238Z",
  "sequence_num": 2,
  "events": [
    {
      "type": "update",
      "product_id": "BTC-USD",
      "venue_configuration": "Filtered",
      "updates": [
        {
          "side": "bid",
          "event_time": "2022-06-15T01:52:23.385605Z",
          "px": "22145.3",
          "qty": "0.01083825"
        },
        {
          "side": "offer",
          "event_time": "2022-06-15T01:52:23.385605Z",
          "px": "22149.26",
          "qty": "0.03450782"
        }
      ]
    },
    {
      "type": "update",
      "product_id": "ETH-USD",
      "venue_configuration": "Filtered",
      "updates": [
        {
          "side": "bid",
          "event_time": "2022-06-15T01:52:23.385605Z",
          "px": "3145.30",
          "qty": "0.09394852"
        },
        {
          "side": "offer",
          "event_time": "2022-06-15T01:52:23.385605Z",
          "px": "3149.26",
          "qty": "0.05493869"
        }
      ]
    }
  ]
}

```

### Python Example

Below is a detailed end to end Python script for subscribing to the `l2_data` channel:

```
import asyncio, base64, hashlib, hmac, json, os, sys, time, websockets
PASSPHRASE = os.environ.get('PASSPHRASE')
ACCESS_KEY = os.environ.get('ACCESS_KEY')
SECRET_KEY = os.environ.get('SIGNING_KEY')
SVC_ACCOUNTID = os.environ.get('SVC_ACCOUNTID')
uri = 'wss://ws-feed.prime.coinbase.com'
timestamp = str(int(time.time()))
channel = 'l2_data'
product_ids = ['ETH-USD']
async def main_loop():
    while True:
        try:
            async with websockets.connect(uri, ping_interval=None, max_size=None) as websocket:
                signature = sign(channel, ACCESS_KEY, SECRET_KEY, SVC_ACCOUNTID, product_ids)
                auth_message = json.dumps({
                    'type': 'subscribe',
                    'channel': channel,
                    'access_key': ACCESS_KEY,
                    'api_key_id': SVC_ACCOUNTID,
                    'timestamp': timestamp,
                    'passphrase': PASSPHRASE,
                    'signature': signature,
                    'venue_filtering': false,
                    'product_ids': product_ids
                })
                await websocket.send(auth_message)
                while True:
                    response = await websocket.recv()
                    parsed_response = json.loads(response)
                    print(json.dumps(parsed_response, indent=3))
        except websockets.ConnectionClosed:
            continue
def sign(channel, key, secret, account_id, product_ids):
    message = channel + key + account_id + timestamp + "".join(product_ids)
    signature = hmac.new(SECRET_KEY.encode('utf-8'), message.encode('utf-8'), digestmod=hashlib.sha256).digest()
    signature_b64 = base64.b64encode(signature).decode()
    return signature_b64
try:
    asyncio.get_event_loop().run_until_complete(main_loop())
except KeyboardInterrupt:
    print('\nClosing Prime websocket feed')
    sys.exit()

```

This script is open source and available on [GitHub](https://github.com/coinbase-samples/prime-scripts-py/blob/main/REST/prime_ws_market.py).

## Maintaining an Order Book

For a detailed look at building and maintaining an order book using the L2 data market feed, refer to this [reference application](https://github.com/coinbase-samples/prime-order-book-py).

## Calculating Slippage

The following code sample demonstrates how you can use the WebSocket feed to calculate slippage.

```
#PYTHON EXAMPLE
#!/usr/bin/env python
import asyncio
import datetime
import json, hmac, hashlib, time, base64
import asyncio
import time
import websockets
import unittest
import logging
import sys
PASSPHRASE = "<API key passphrase here>"
ACCESS_KEY = "<API access key here>"
SIGNING_KEY = "<API signing key here>"
SVC_ACCOUNTID = "<API account ID passphrase here>"
s = time.gmtime(time.time())
TIMESTAMP = time.strftime("%Y-%m-%dT%H:%M:%SZ",s)
"""
A processor maintains an in-memory price book used for analytics
"""
class PriceBookProcessor:
    def __init__(self, snapshot):
        """
        Instantiate a processor using a snapshot from the WebSocket feed
        """
        self.bids = []
        self.offers = []
        snapshot_data = json.loads(snapshot)
        px_levels = snapshot_data["events"][0]["updates"]
        for i in range(len(px_levels)):
            level = px_levels[i]
            if level["side"] == "bid":
                self.bids.append(level)
            elif level["side"] == "offer":
                self.offers.append(level)
            else:
                raise IOError()
        self._sort()
    def apply_update(self, data):
        """
        Update in-memory state with a single update from the WebSocket feed
        """
        event = json.loads(data)
        if event["channel"] != "l2_data":
            return
        events = event["events"]
        for e in events:
            updates = e["updates"]
            for update in updates:
                self._apply(update)
        self._filter_closed()
        self._sort()
    def _apply(self, level):
        if level["side"] == "bid":
            found = False
            for i in range(len(self.bids)):
                if self.bids[i]["px"] == level["px"]:
                    self.bids[i] = level
                    found = True
                    break
            if not found:
                self.bids.append(level)
        else:
            found = False
            for i in range(len(self.offers)):
                if self.offers[i]["px"] == level["px"]:
                    self.offers[i] = level
                    found = True
                    break
            if not found:
                self.offers.append(level)
    def _filter_closed(self):
        self.bids = [x for x in self.bids if abs(float(x["qty"])) > 0]
        self.offers = [x for x in self.offers if abs(float(x["qty"])) > 0]
    def _sort(self):
        self.bids = sorted(self.bids, key=lambda x: float(x["px"]) * -1)
        self.offers = sorted(self.offers, key=lambda x: float(x["px"]))
    def stringify(self):
        """
        Return a string summary of the contents of the price book
        """
        best_bid = self.bids[0]["px"]
        best_offer = self.offers[0]["px"]
        spread = str(float(best_offer) - float(best_bid))
        l1 = f"{best_bid} =>{spread}<= {best_offer} ({len(self.bids)} bids, {len(self.offers)} offers)\n"
        bids = self.bids[:5]
        offers = self.offers[:5]
        l2, l3 = "", ""
        if len(bids) > 0:
            l2 = "Bids: " + ", ".join([b["qty"] + " " + b["px"] for b in bids]) + "\n"
        if len(offers) > 0:
            l3 = "Offers: " + ", ".join([b["qty"] + " " + b["px"] for b in offers]) + "\n"
        l4 = "Buy 1 BTC: " + str(self.estimate_aggressive_px(1.0, True)) + " USD\n"
        return l1 + l2 + l3 + l4
    def estimate_aggressive_px(self, qty, is_bid=True):
        """
        Estimate the average price of an aggressive order of a given size/side
        """
        orders = self.bids
        if is_bid:
            orders = self.offers
        total, total_value = 0.0, 0.0
        idx = 0
        while total < qty and idx < len(orders):
            px = float(orders[idx]["px"])
            this_level = min(qty-total, float(orders[idx]["qty"]))
            value = this_level * px
            total_value += value
            total += this_level
            idx += 1
        return total_value / total
"""
Sign a subscription for the WebSocket API
"""
async def sign(channel, key, secret, account_id, portfolio_id, product_ids):
    message = channel + key + account_id + TIMESTAMP + portfolio_id + product_ids
    print(message)
    signature = hmac.new(secret.encode('utf-8'), message.encode('utf-8'), digestmod=hashlib.sha256).digest()
    signature_b64 = base64.b64encode(signature).decode()
    print(signature_b64)
    return signature_b64
"""
Main loop for consuming from the WebSocket feed
"""
async def main_loop():
    uri = "wss://ws-feed.prime.coinbase.com"
    async with websockets.connect(uri, ping_interval=None, max_size=None) as websocket:
        signature = await sign('l2_data', ACCESS_KEY, SIGNING_KEY, SVC_ACCOUNTID, "", "BTC-USD")
        print(signature)
        auth_message = json.dumps({
            "type": "subscribe",
            "channel": "l2_data",
            "access_key": ACCESS_KEY,
            "api_key_id": SVC_ACCOUNTID,
            "timestamp": TIMESTAMP,
            "passphrase": PASSPHRASE,
            "signature": signature,
            "portfolio_id": "",
            "product_ids": ["BTC-USD"],
        })
        print(type(auth_message))
        print(auth_message)
        await websocket.send(auth_message)
        try:
            processor = None
            while True:
                response = await websocket.recv()
                #print(f"<<< {response}")
                parsed = json.loads(response)
                if parsed["channel"] == "l2_data" and parsed["events"][0]["type"] == "snapshot":
                    processor = PriceBookProcessor(response)
                elif processor != None:
                    processor.apply_update(response)
                if processor != None:
                    print(processor.stringify())
                    sys.stdout.flush()
        except websockets.exceptions.ConnectionClosedError:
            print("Error caught")
            sys.exit(1)
if __name__ == '__main__':
    asyncio.run(main_loop())

```