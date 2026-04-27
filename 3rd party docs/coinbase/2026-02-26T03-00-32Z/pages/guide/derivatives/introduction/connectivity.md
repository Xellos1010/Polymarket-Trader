# connectivity

## Data Center Locations

The Coinbase Derivatives trading platform currently operates out of two locations: Chicago, Illinois and Secaucus, New Jersey. **Production:** [Equinix CH4 Data Center](https://www.equinix.com/data-centers/americas-colocation/united-states-colocation/chicago-data-centers/ch4)  
**Address:** 350 E Cermak, Chicago, Illinois, 60616 **DR/Integration:** [Equinix NY5 Data Center](https://www.equinix.com/data-centers/americas-colocation/united-states-colocation/new-york-data-centers/ny5)  
**Address:** 800 Secaucus Road, Secaucus, New Jersey, 07094 Internet connectivity is hosted via AWS.

## Environments

Coinbase Derivatives operates the following environments:

-   Production
-   Disaster Recovery (DR)
-   Integration (Production Parallel / UAT environment)

## Connectivity Protocols

Type

Protocol

Integration Internet Connection

Production Internet Connection

Links

Order Entry

FIX 4.4

Available

Available

[FIX Order Entry](https://developer.chrome.com/derivatives/fix/order-entry)

Market Data

FIX 4.4

Available

Available

[FIX Market Data](https://developer.chrome.com/derivatives/fix/market-data)

Order Entry

SBE

Available

Cross Connect Only

[SBE Order Entry](https://developer.chrome.com/derivatives/sbe/order-entry)

Market Data

UDP

Cross Connect Only

Cross Connect Only

[UDP Market Data](https://developer.chrome.com/derivatives/udp/overview)

Drop Copy

FIX 4.4

Available

Available

[FIX Drop Copy](https://developer.chrome.com/derivatives/fix/drop-copy)

REST Gateway

HTTPS

Available

Available

[REST API](https://developer.chrome.com/api-reference/derivatives-api/rest-api/introduction)

## Coinbase Derivatives Points of Presence

Environment

Physical Location

Types of Connectivity available

Production

Equinix CH4

Cross Connect, AWS PrivateLink, Internet.

Disaster Recovery

Equinix NY5

Cross Connect, AWS PrivateLink, Internet.

Integration

Equinix NY5

Cross Connect, AWS PrivateLink, Internet.

## Connectivity via Cross Connect

Coinbase Derivatives Exchange (CDE) participants can [establish cross-connects](#how-to-connect) in the facilities detailed under [CDE Locations](#data-center-locations).

#### How to Connect

1.  **Contact the CDE team** about establishing a private co-located fiber connection.  
    *The CDE team will:* a. Issue a letter of authorization (LOA) allowing you, the participant, to connect into CDE equipment. b. Assign 2 IP address per side:
    
    -   1 address range for BGP peering.
    -   1 address range for connecting to CDE that is advertised by the participant.
    
    c. Assign a private ASN for BGP peering. Participants can use a public ASN as long as it is owned by them.
2.  **Configure BGP** with all parameters provided by CDE.
3.  **Optionally, configure PIM and RP addresses** to receive multicast market data. [RP Addresses](#cross-connect-multicast-rps) for each connection are provided by CDE.

## Connectivity via Internet

Clients may connect to select Coinbase Derivatives APIs via the internet. This solution is recommended for testing or non-latency sensitive systems. Please note that Coinbase Derivatives only accepts SSL/TLS1 encrypted connections for internet-based connections. See IP Addressing section for target host names. Clients are encouraged to whitelist both Production and DR public addresses on their firewalls.

### SSL/TLS Details

-   Preferred: TLSv1.2 128 bits ECDHE-RSA-AES128-GCM-SHA256 Curve P-256 DHE 256
-   Accepted: TLSv1.2 128 bits ECDHE-RSA-AES128-SHA256 Curve P-256 DHE 256
-   Accepted: TLSv1.2 256 bits ECDHE-RSA-AES256-GCM-SHA384 Curve P-256 DHE 256
-   Accepted: TLSv1.2 256 bits ECDHE-RSA-AES256-SHA384 Curve P-256 DHE 256
-   Accepted: TLSv1.2 128 bits AES128-GCM-SHA256
-   Accepted: TLSv1.2 128 bits AES128-SHA256
-   Accepted: TLSv1.2 256 bits AES256-GCM-SHA384
-   Accepted: TLSv1.2 256 bits AES256-SHA256

## Connectivity via AWS PrivateLink

Clients may connect to Coinbase Derivatives Exchange via AWS PrivateLink. This allows for some of the benefits of private connectivity without having to cross connect directly in the datacenter. Private connectivity via AWS PrivateLink offers reduced latency and a more stable connection when compared to the Public Internet. It should be noted that for the lowest latency and most stable connection possible, Colocation Cross Connects are still recommended. Each PrivateLink has multiple availability zones and regions that are supported. However, for the most optimal results using the primary region and availability zone is recommended.

### Exchange Endpoint PrivateLink Details

These endpoints support the following services:

-   **FIX Marketdata**
-   **FIX Orders**
-   **FIX Drop Copy**
-   **SBE Orders (Integration)**

Refer to the service names and supported availability zones below for each environment:

#### Production Connectivity

-   PrivateLink offering in AWS Region US-EAST-2 and US-EAST-1
-   Service Name: `com.amazonaws.vpce.us-east-2.vpce-svc-01bf36c2a63eaf006`
-   Availability Zones supported:
    -   az1 (Recommended)
    -   az2 (Recommended)
    -   az3

#### Disaster Recovery Connectivity

-   PrivateLink offering in AWS Region US-EAST-1 and US-EAST-2
-   Service Name: `com.amazonaws.vpce.us-east-1.vpce-svc-0051aaaf1479eae00`
-   Availability Zones supported:
    -   az2 (Recommended)
    -   az4
    -   az6

#### Integration Connectivity

-   PrivateLink offering in AWS Region US-EAST-1 and US-EAST-2
-   Service Name: `com.amazonaws.vpce.us-east-1.vpce-svc-0766c510bc2c236a8`
-   Availability Zones supported:
    -   az1
    -   az4 (Recommended)
    -   az6 (Recommended)

### REST API Endpoint PrivateLink Details

These endpoints support the following services:

-   **REST API**

Refer to the service names and supported availability zones below for each environment:

#### Production Connectivity

-   PrivateLink offering in AWS Region US-EAST-2 and US-EAST-1
-   Service Name: `com.amazonaws.vpce.us-east-2.vpce-svc-0c5e0f74520c6fc4e`
-   Availability Zones supported:
    -   az1 (Recommended)
    -   az2 (Recommended)
    -   az3
-   DNS Name: api.exchange.fairx.net

#### Disaster Recovery Connectivity

-   PrivateLink offering in AWS Region US-EAST-1 and US-EAST-2
-   Service Name: `com.amazonaws.vpce.us-east-1.vpce-svc-00581f836e5178638`
-   Availability Zones supported:
    -   az2 (Recommended)
    -   az4
    -   az6
-   DNS Name: api.exchange-dr.fairx.net

#### Integration Connectivity

-   PrivateLink offering in AWS Region US-EAST-1 and US-EAST-2
-   Service Name: `com.amazonaws.vpce.us-east-1.vpce-svc-0ca0bc38a677c27a2`
-   Availability Zones supported:
    -   az1
    -   az4 (Recommended)
    -   az6 (Recommended)
-   DNS Name: api.integration.fairx.net

### Provisioning PrivateLinks Using AWS Console

To provision a PrivateLink to Coinbase Derivatives Exchange using the AWS Console, follow these steps:

1.  **Navigate to VPC > Endpoints**
2.  **Click ‘Create Endpoint’**
3.  **Select ‘Endpoint Services that use NLBs and GWLBs’ under Type**
    -   Enter the PrivateLink service name provided by Coinbase Derivatives (see above for service names) into Service Name field.
    -   If using cross region connectivity check ‘Enable Cross Region Endpoint’ and select the region of the endpoint.
4.  **Choose your VPC, subnets, and security groups**
    -   Select the VPC you want to use, then choose the appropriate subnets and security groups for your environment.
5.  **Enable Private DNS (optional)**

-   If you want to use private DNS records added to your VPC, check the ‘Enable DNS name’ (see above for service support).

6.  **Review and create the endpoint**
    -   Review your selections and click “Create endpoint” to finish.

## Networks

### Cross Connect Unicast Networks

Environment

A Feed Subnet

B Feed Subnet

Production

208.52.130.0/27

208.52.130.32/27

Disaster Recovery

208.52.130.64/27

208.52.130.96/27

Integration

208.52.130.128/27

208.52.130.160/27

### Cross Connect Multicast Networks

Environment

A Feed Subnet

B Feed Subnet

Production

233.246.250.0/27

233.246.250.32/27

Disaster Recovery

233.246.250.64/27

233.246.250.96/27

Integration

233.246.250.128/27

233.246.250.160/27

### Cross Connect Multicast RPs

Environment

A Feed RP

B Feed RP

Production

208.52.130.16

208.52.130.48

Disaster Recovery

208.52.130.80

208.52.130.112

Integration

208.52.130.144

208.52.130.176

## IP Addressing

### Production

#### Unicast (TCP)

Service

A Feed

B Feed

Internet Host Name

Port

SBE Order

208.52.130.17

208.52.130.49

N/A

6210

FIX Market Data

208.52.130.18

208.52.130.50

fix-marketdata.exchange.fairx.net

6120

FIX Order

208.52.130.20

208.52.130.52

fix-orders.exchange.fairx.net

6110

FIX Drop Copy

208.52.130.23

208.52.130.55

fix-drop-copy.exchange.fairx.net

6130

#### Unicast (UDP)

Service

A Feed

B Feed

Port

SBE Market Data Retransmit Equity

208.52.130.19

208.52.130.51

6220

SBE Market Data Retransmit Non-Equity

208.52.130.19

208.52.130.51

6221

#### Multicast

Service

A Feed

B Feed

Port

SBE Market Data Incremental Equity

233.246.250.17

233.246.250.39

6222

SBE Market Data Snapshot Equity

233.246.250.18

233.246.250.40

6224

SBE Market Data Incremental Non-Equity

233.246.250.19

233.246.250.41

6223

SBE Market Data Snapshot Non-Equity

233.246.250.20

233.246.250.42

6225

### Disaster Recovery

#### Unicast (TCP)

Service

A Feed

B Feed

Internet Host Name

Port

SBE Order

208.52.130.81

208.52.130.113

N/A

6210

FIX Market Data

208.52.130.82

208.52.130.114

fix-marketdata.exchange-dr.fairx.net

6120

FIX Order

208.52.130.84

208.52.130.116

fix-orders.exchange-dr.fairx.net

6110

FIX Drop Copy

208.52.130.87

208.52.130.119

fix-drop-copy.exchange-dr.fairx.net

6130

#### Unicast (UDP)

Service

A Feed

B Feed

Port

SBE Market Data Retransmit Equity

208.52.130.83

208.52.130.115

6220

SBE Market Data Retransmit Non-Equity

208.52.130.83

208.52.130.115

6221

#### Multicast

Service

A Feed

B Feed

Port

SBE Market Data Incremental Equity

233.246.250.81

233.246.250.103

6222

SBE Market Data Snapshot Equity

233.246.250.82

233.246.250.104

6224

SBE Market Data Incremental Non-Equity

233.246.250.83

233.246.250.105

6223

SBE Market Data Snapshot Non-Equity

233.246.250.84

233.246.250.106

6225

### Integration

#### Unicast (TCP)

Service

A Feed

B Feed

Internet Host Name

Port

SBE Order

208.52.130.135

208.52.130.167

sbe-orders.integration.fairx.net

5210

FIX Market Data

208.52.130.136

208.52.130.168

fix-marketdata.integration.fairx.net

5120

FIX Order

208.52.130.138

208.52.130.170

fix-orders.integration.fairx.net

5110

FIX Drop Copy

N/A

N/A

fix-drop-copy.integration.fairx.net

6130

#### Unicast (UDP)

Service

A Feed

B Feed

Port

SBE Market Data Retransmit Equity

208.52.130.137

208.52.130.169

5220

SBE Market Data Retransmit Non-Equity

208.52.130.137

208.52.130.169

5221

#### Multicast

Service

A Feed

B Feed

Port

SBE Market Data Incremental Equity

233.246.250.135

233.246.250.167

5222

SBE Market Data Snapshot Equity

233.246.250.136

233.246.250.168

5224

SBE Market Data Incremental Non-Equity

233.246.250.137

233.246.250.169

5223

SBE Market Data Snapshot Non-Equity

233.246.250.138

233.246.250.170

5225

### Targeting Specific Feeds Over Internet and AWS PrivateLink

When connecting over the internet or AWS PrivateLink, participants can target a specific feed (A or B) by adjusting the port number:

-   **Aggregate:** The port listed in the tables refers to the aggregate port, which will load balance traffic across all available feeds.
-   **A Feed:** Use the aggregate(base) port **+1**
-   **B Feed:** Use the aggregate(base) port **+2**

For example, if the aggregate port is `6110`:

-   Use `6111` to target the A Feed directly.
-   Use `6112` to target the B Feed directly.

This allows participants to explicitly select which feed to connect to when required.

## UDP Multicast Market Data Channel IDs

Product Group

Channel ID

Equities

0xaf31

Non Equities

0xaf32