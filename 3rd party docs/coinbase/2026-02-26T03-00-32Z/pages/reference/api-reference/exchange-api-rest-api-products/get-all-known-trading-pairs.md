# get all known trading pairs

Get all known trading pairs

The `base_min_size` and `base_max_size` fields define the min and max order size. The `min_market_funds` and `max_market_funds` fields define the min and max funds allowed in a market order. `status_message` provides any extra information regarding the status if available. The `quote_increment` field specifies the min order price as well as the price increment. The order price must be a multiple of this increment (i.e. if the increment is 0.01, order prices of 0.001 or 0.021 would be rejected). The `base_increment` field specifies the minimum increment for the `base_currency`. `trading_disabled` indicates whether trading is currently restricted on this product, this includes whether both new orders and order cancellations are restricted. `cancel_only` indicates whether this product only accepts cancel requests for orders. `post_only` indicates whether only maker orders can be placed. No orders will be matched when post\_only mode is active. `limit_only` indicates whether this product only accepts limit orders. Only a maximum of one of `trading_disabled`, `cancel_only`, `post_only`, `limit_only` can be true at once. If none are true, the product is trading normally. `fx_stablecoin` indicates whether the currency pair is a Stable Pair. `auction_mode` boolean which indicates whether or not the book is in auction mode. For more details on the auction mode see [Get product book](https://developer.chrome.com/api-reference/exchange-api/rest-api/products/get-product-book) describing the level 1 book which contains information pertaining to products in auction mode.

#### Query Parameters

#### Response

Min order price (a.k.a. price increment

status

enum<string>

default:online

required

Available options

:

`online`,

`offline`,

`internal`,

`delisted`

high\_bid\_limit\_percentage

Percentage to calculate highest price for limit buy order (Stable coin trading pair only)