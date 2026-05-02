# update address book

#### Authorizations

#### Path Parameters

#### Body

is\_verified\_self\_hosted\_wallet

Flag to indicate if the crypto addresses has previously been digitally signed and verified when added in the Address Book UI tab

The VASP identifier if the address is owned by one of the supported Virtual Asset Service Providers

The country code (ISO 3166-1 alpha-2) of the originator's account location.

Flag to indicate if the user owns the crypto address

Blockchain network of the address. Provide a single network name (e.g., `ethereum`, `bitcoin`, `_ALL_EVM_NETWORKS_`). If omitted or `null`, the network field remains unchanged. Use `*` to make the address available on all supported networks compatible with the asset (e.g., both `ethereum` and `arbitrum` for an ERC-20 token). When `currency` is `_ALL_ASSETS_`, `network` is required. Use `_ALL_EVM_NETWORKS_` only with `_ALL_ASSETS_` to apply the address to all EVM-compatible networks.

wallet\_verification\_network

Blockchain network used to verify ownership of the wallet address

#### Response

Asset symbol for the saved address (e.g., `BTC`, `ETH`, `USDC`). `_ALL_ASSETS_` indicates that this address is stored globally for all assets, rather than a specific one. The `network` field determines which blockchain network the address applies to.

address\_book\_added\_at

string<date-time>

required

address\_book\_entry\_pending\_until

is\_verified\_self\_hosted\_wallet

Flag to indicate if the crypto addresses has previously been digitally signed and verified when added in the Address Book UI tab

The VASP identifier if the address is owned by one of the supported Virtual Asset Service Providers

Business name of the originator's account

The country code (ISO 3166-1 alpha-2) of the originator's account location.

Blockchain network of the address. Indicates the network scope the address is associated with (e.g., `ethereum`, `bitcoin`). If omitted or `null`, the address is available on all supported networks compatible with the asset (e.g., both `ethereum` and `arbitrum` for an ERC-20 token). `_ALL_EVM_NETWORKS_` indicates the address applies to all EVM-compatible networks.