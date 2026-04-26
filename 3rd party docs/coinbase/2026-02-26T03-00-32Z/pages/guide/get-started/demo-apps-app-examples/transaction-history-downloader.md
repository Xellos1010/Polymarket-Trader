# transaction history downloader

Address HistoryOnchain Data

## Overview

This tool uses the Coinbase Developer Platform (CDP) SDK to:

1.  Fetch all transactions for a specified address
2.  Process both native transfers and token transfers
3.  Export the data to a CSV file with timestamp
4.  Track progress for long-running downloads

## Prerequisites

-   Python 3.x
-   CDP API Key (see Authentication section)
-   Install the CDP SDK

## Authentication

1.  Sign up at [CDP Portal](https://portal.cdp.coinbase.com/projects/api-keys)
2.  Generate an API key from your dashboard
3.  Optionally download the API key JSON file, or copy the key details
4.  Place it in your file system (e.g., ~/Downloads/cdp\_api\_key.json) or use environment variables

## Usage

1.  Copy the code below to a Python file (e.g., app.py)
    
    ```
        from cdp import *
        from typing import Dict, List
        import csv
        from datetime import datetime
        def process_transaction(tx: Transaction, address: str) -> List[Dict]:
            """
            Process transaction and return rows for CSV export.
            
            Args:
                tx: Transaction object
                address: Address to filter by (case-insensitive)
            
            Returns:
                List of dictionaries containing transaction data
            """
            address = address.lower()
            tx_content = tx.content.actual_instance
            rows = []
            
            # Process main transaction (native transfer)
            if tx_content.value != '0':  # Only include if there's a value transfer
                # Determine direction
                direction = 'send' if address == tx_content.var_from.lower() else 'receive'
                
                rows.append({
                    'type': 'native',
                    'direction': direction,
                    'contract_address': '',  # Empty for native transfers
                    'from_address': tx_content.var_from,
                    'to_address': tx_content.to,
                    'value': tx_content.value,
                    'token_id': '',
                    'log_index': '',
                    'transaction_hash': tx_content.hash,
                    'block_number': tx.block_height
                })
            
            # Process token transfers
            for transfer in tx_content.token_transfers:
                # Only include transfers involving the target address
                if (address == transfer.from_address.lower() or 
                    address == transfer.to_address.lower()):
                    
                    # Determine direction
                    direction = 'send' if address == transfer.from_address.lower() else 'receive'
                    
                    rows.append({
                        'type': transfer.token_transfer_type.value,  # 'erc20' or 'erc721'
                        'direction': direction,
                        'contract_address': transfer.contract_address,
                        'from_address': transfer.from_address,
                        'to_address': transfer.to_address,
                        'value': transfer.value if transfer.value is not None else '',
                        'token_id': transfer.token_id if transfer.token_id is not None else '',
                        'log_index': transfer.log_index,
                        'transaction_hash': tx_content.hash,
                        'block_number': tx.block_height
                    })
            
            return rows
        def main():
            # Configure CDP
            Cdp.configure_from_json("~/Downloads/cdp_api_key.json")
            print("CDP SDK has been successfully configured from JSON file.")
            
            # The network to filter by
            network = "base-mainnet"
            # The address to filter by
            target_address = "0x0e73fc61bb9d6b7588910c2d14e83bae68222c5d"
            
            # Create CSV filename with timestamp
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            csv_filename = f"transactions_{target_address[2:8]}_{timestamp}.csv"
            
            # CSV header
            headers = [
                'type',
                'direction',
                'contract_address',
                'from_address',
                'to_address',
                'value',
                'token_id',
                'log_index',
                'transaction_hash',
                'block_number'
            ]
            
            # Get transactions and write to CSV
            try:
                with open(csv_filename, 'w', newline='') as csvfile:
                    writer = csv.DictWriter(csvfile, fieldnames=headers)
                    writer.writeheader()
                    
                    items = Transaction.list(network, target_address)
                    transaction_count = 0
                    row_count = 0
                    
                    for item in items:
                        transaction_count += 1
                        rows = process_transaction(item, target_address)
                        for row in rows:
                            writer.writerow(row)
                            row_count += 1
                        
                        # Print progress every 100 transactions
                        if transaction_count % 100 == 0:
                            print(f"Processed {transaction_count} transactions...")
                    
                    print(f"\nComplete! Processed {transaction_count} transactions.")
                    print(f"Generated {row_count} rows in {csv_filename}")
                    
            except Exception as e:
                print(f"Error: {str(e)}")
        if __name__ == "__main__":
            main()
    
    
    
    ```
    
2.  Set your target address and configure the CDP client

```
# Configure CDP
Cdp.configure_from_json("~/Downloads/cdp_api_key.json")
# The network to filter by
network = "base-mainnet"
# The address to filter by
target_address = "0x0e73fc61bb9d6b7588910c2d14e83bae68222c5d"

```

3.  Run the script

4.  The script will generate a CSV file named `transactions_{address}_{timestamp}.csv`

#### Output Format

The CSV includes the following columns:

Column

Description

type

Transaction type (native/erc20/erc721/erc1155)

direction

send/receive

contract\_address

Token contract address (empty for native transfers)

from\_address

Sender address

to\_address

Recipient address

value

Transfer amount

token\_id

NFT token ID (for ERC721/ERC1155)

log\_index

Event log index

transaction\_hash

Transaction hash

block\_number

Block number