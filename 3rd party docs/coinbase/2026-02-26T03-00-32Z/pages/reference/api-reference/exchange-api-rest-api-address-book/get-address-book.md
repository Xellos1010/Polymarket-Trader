# get address book

#### Authorizations

#### Response

Asset symbol for the saved address (e.g., `BTC`, `ETH`, `USDC`). `_ALL_ASSETS_` indicates that this address is stored globally for all assets, rather than a specific one. The `network` field determines which blockchain network the address applies to.

address\_book\_added\_at

string<date-time>

required

is\_verified\_self\_hosted\_wallet

Flag to indicate if the crypto addresses has previously been digitally signed and verified when added in the Address Book UI tab

The VASP identifier if the address is owned by one of the supported Virtual Asset Service Providers

Business name of the originator's account - only populated for travel rules regions

The country code (ISO 3166-1 alpha-2) of the originator's account location - only populated for travel rules regions

Blockchain network of the address. If omitted or `null`, the address is available on all supported networks compatible with the asset (e.g., both `ethereum` and `arbitrum` for an ERC-20 token). When `currency` is `_ALL_ASSETS_` and network is `_ALL_EVM_NETWORKS_`, the address is available on all assets with EVM-compatible networks.