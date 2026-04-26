# create travel rule

#### Authorizations

#### Body

Crypto address where funds will be deposited from

Name of the originator of the funds

ISO 3166-1 alpha-2 formatted country code of the originator of the funds

ISO 3166-1 alpha-2 formatted country code of the VASP

Legal Entity Identifier (LEI) of the VASP

wallet\_type

enum<string>

default:UNKNOWN\_WALLET\_TYPE

Available options

:

`UNKNOWN_WALLET_TYPE`,

`EXCHANGE`,

`SELF_HOSTED`

True if the user owns the wallet

#### Response

Timestamp of when entry was added

Crypto address where funds will be deposited from

Name of the originator of the funds

country code (ISO 3166-1 alpha-2) of the originator of the funds

ISO 3166-1 alpha-2 formatted country code of the VASP

Legal Entity Identifier (LEI) of the VASP