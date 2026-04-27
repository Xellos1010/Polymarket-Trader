# overview

The Multicast UDP Market Data API includes multiple groups of data channels/streams for unique sets of related products and instruments. Each group is comprised of 3 pairs of channels:

-   **INCREMENTAL UPDATES** - A/B UDP multicast groups with **real-time updates** for orders, trades, market state changes, and instrument definitions.
-   **SNAPSHOTS** - A/B UDP multicast groups with **periodic snapshots** of orders and instrument definitions/statuses at a regular interval.
-   **RETRANSMIT SERVICE** - UDP unicast with updates by **InstrSeqNum range** using request/response model.