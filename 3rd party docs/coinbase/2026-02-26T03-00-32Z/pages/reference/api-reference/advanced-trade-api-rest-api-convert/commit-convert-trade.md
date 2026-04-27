# commit convert trade

```
{
  "trade": {
    "id": "<string>",
    "status": "TRADE_STATUS_UNSPECIFIED",
    "user_entered_amount": {
      "value": "<string>",
      "currency": "<string>"
    },
    "amount": {
      "value": "<string>",
      "currency": "<string>"
    },
    "subtotal": {
      "value": "<string>",
      "currency": "<string>"
    },
    "total": {
      "value": "<string>",
      "currency": "<string>"
    },
    "fees": [
      {
        "title": "<string>",
        "description": "<string>",
        "amount": {
          "value": "<string>",
          "currency": "<string>"
        },
        "label": "<string>",
        "disclosure": {
          "title": "<string>",
          "description": "<string>",
          "link": {
            "text": "<string>",
            "url": "<string>"
          }
        },
        "waived_details": {
          "amount": {
            "value": "<string>",
            "currency": "<string>"
          },
          "source": "WAIVED_FEE_SOURCE_UNSPECIFIED"
        }
      }
    ],
    "total_fee": {
      "title": "<string>",
      "description": "<string>",
      "amount": {
        "value": "<string>",
        "currency": "<string>"
      },
      "label": "<string>",
      "disclosure": {
        "title": "<string>",
        "description": "<string>",
        "link": {
          "text": "<string>",
          "url": "<string>"
        }
      },
      "waived_details": {
        "amount": {
          "value": "<string>",
          "currency": "<string>"
        },
        "source": "WAIVED_FEE_SOURCE_UNSPECIFIED"
      }
    },
    "source": {
      "type": "INVALID",
      "network": "<string>",
      "payment_method_id": "<string>",
      "blockchain_address": {
        "address": "<string>",
        "destination_tag": "<string>"
      },
      "coinbase_account": {
        "account_id": "<string>"
      },
      "blockchain_transaction": {
        "hsh": "<string>",
        "height": 123,
        "normalized_hash": "<string>"
      },
      "fedwire": {
        "routing_number": "<string>",
        "account_holder": {
          "legal_name": "<string>",
          "account_number": "<string>",
          "address": {
            "lines": [
              "<string>"
            ],
            "country_code": "<string>",
            "city": "<string>",
            "state": "<string>",
            "postal_code": "<string>"
          }
        },
        "bank": {
          "name": "<string>",
          "address": {
            "lines": [
              "<string>"
            ],
            "country_code": "<string>",
            "city": "<string>",
            "state": "<string>",
            "postal_code": "<string>"
          },
          "identifier": "<string>",
          "type": "CHIPS_IDENTIFIER",
          "identifier_code": "<string>"
        },
        "intermediary_bank": {
          "name": "<string>",
          "address": {
            "lines": [
              "<string>"
            ],
            "country_code": "<string>",
            "city": "<string>",
            "state": "<string>",
            "postal_code": "<string>"
          },
          "identifier": "<string>",
          "type": "CHIPS_IDENTIFIER",
          "identifier_code": "<string>"
        },
        "currency": "<string>",
        "fiat_id": "<string>",
        "rejection_count_v2": 123
      },
      "swift": {
        "account_holder": {
          "legal_name": "<string>",
          "iban": "<string>",
          "bban": "<string>",
          "domestic_account_id": "<string>",
          "customer_payment_address1": "<string>",
          "customer_payment_address2": "<string>",
          "customer_payment_address3": "<string>",
          "customer_payment_country_code": "<string>",
          "iban_generated": true
        },
        "institution": {
          "bic": "<string>",
          "name": "<string>",
          "bank_address1": "<string>",
          "bank_address2": "<string>",
          "bank_address3": "<string>",
          "bank_country_code": "<string>",
          "domestic_bank_id": "<string>",
          "international_bank_id": "<string>"
        },
        "intermediary": {
          "bic": "<string>",
          "name": "<string>",
          "bank_address1": "<string>",
          "bank_address2": "<string>",
          "bank_address3": "<string>",
          "bank_country_code": "<string>",
          "domestic_bank_id": "<string>",
          "international_bank_id": "<string>"
        },
        "currency": "<string>",
        "rejection_count": 123
      },
      "card": {
        "first_data_token": {
          "value": "<string>",
          "store_id": "<string>",
          "id": "<string>"
        },
        "merchant": {
          "mid": "<string>"
        },
        "vault_token": {
          "value": "<string>",
          "vault_id": "<string>",
          "attempted_at": "<string>",
          "encrypted_data": "<string>",
          "encrypted_cvv": "<string>"
        },
        "worldpay_params": {
          "token_value": "<string>",
          "token_event": "<string>",
          "uses_merchant_token": true,
          "accept_header": "<string>",
          "user_agent_header": "<string>",
          "shopper_ip": "<string>",
          "shopper_session_id": "<string>"
        },
        "checkout_token": {
          "source_id": "<string>"
        },
        "stripe_params": {
          "stripe_id": "<string>",
          "radar_session_id": "<string>",
          "address_line1_check": "STRIPE_CARD_CHECK_STATUS_UNKNOWN",
          "address_postal_code_check": "STRIPE_CARD_CHECK_STATUS_UNKNOWN",
          "cvc_check": "STRIPE_CARD_CHECK_STATUS_UNKNOWN"
        },
        "six_digit_bin": "<string>",
        "last_four": "<string>",
        "eight_digit_bin": "<string>",
        "card_sub_brand": "<string>",
        "card_brand": "<string>",
        "obfuscated_pan": "<string>",
        "bin_info": {
          "match_found": true,
          "type": "UNKNOWN",
          "issuer": "<string>",
          "issuer_country_code": "<string>",
          "prepaid": true,
          "corporate": true,
          "consumer": true,
          "visa": {
            "oct_supported": true,
            "fast_funds_supported": true
          },
          "mastercard": {
            "fast_funds_supported": true
          },
          "use_3ds": true
        },
        "scheme": "UNKNOWN",
        "issuer": "<string>",
        "issuer_country_code": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "phone_number": "<string>",
        "user_id": "<string>",
        "postal_code": "<string>",
        "customer_info": {
          "full_name": "<string>",
          "first_name": "<string>",
          "last_name": "<string>",
          "dob": {
            "month": "<string>",
            "day": "<string>",
            "year": "<string>"
          },
          "nationality": "<string>",
          "email": "<string>"
        },
        "expiry_date": {
          "month": "<string>",
          "year": "<string>"
        },
        "cdv_method": "<string>",
        "cdv_amount_1": {
          "amount": "<string>",
          "currency": "<string>"
        },
        "cdv_amount_2": {
          "amount": "<string>",
          "currency": "<string>"
        },
        "worldpay_card_cdv_status": {
          "cdv_required": true,
          "cdv_failed": true
        },
        "previous_scheme_tx_id": "<string>",
        "initial_scheme_tx_id": "<string>",
        "rejected": true,
        "rejected_at": "<string>",
        "rejection_count": 123,
        "rejected_via_purchase": true,
        "account_updater_updated_at": "<string>",
        "account_updater_reason": "<string>",
        "account_updater_status": "<string>",
        "almost_expired_notified": true,
        "is_migrated_card": true,
        "migrated_created_at": "<string>",
        "card_source": "SOURCE_UNKNOWN",
        "added_via_buy_widget": true,
        "encrypted_data": "<string>",
        "coinbase_credit_card_params": {
          "credit_card_account_id": "<string>"
        },
        "customer_name": "<string>",
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "customer_dob": {
          "month": "<string>",
          "day": "<string>",
          "year": "<string>"
        }
      },
      "zengin": {
        "account_holder": {
          "legal_name": "<string>",
          "identifier": "<string>",
          "type": "<string>"
        },
        "institution": {
          "bank_code": "<string>",
          "branch_code": "<string>"
        }
      },
      "uk": {
        "account_holder": {
          "legal_name": "<string>",
          "bban": "<string>",
          "sort_code": "<string>",
          "account_number": "<string>",
          "address": {
            "address1": "<string>",
            "address2": "<string>",
            "city": "<string>",
            "state": "<string>",
            "postal_code": "<string>",
            "country": "<string>",
            "uuid": "<string>",
            "id": "<string>"
          },
          "account_type": "UK_ACCOUNT_TYPE_UNSPECIFIED"
        },
        "institution": {
          "name": "<string>"
        },
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "currency": "<string>",
        "open_banking_details": {
          "true_layer_bank_provider_id": "<string>",
          "open_banking_provider_id": "<string>"
        }
      },
      "sepa": {
        "account_holder": {
          "legal_name": "<string>",
          "iban": "<string>",
          "bban": "<string>"
        },
        "institution": {
          "bic": "<string>",
          "name": "<string>",
          "bank_country_code": "<string>",
          "bank_country_name": "<string>",
          "bank_address": "<string>"
        },
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>"
      },
      "paypal": {
        "account_holder": {
          "paypal_id": "<string>",
          "paypal_pm_id": "<string>",
          "paypal_verified": true,
          "account_type": "<string>",
          "account_age": "<string>"
        },
        "merchant": {
          "merchant_account_id": "<string>"
        },
        "metadata": {
          "paypal_correlation_id": "<string>",
          "authorization_code": "<string>"
        },
        "email": "<string>",
        "owner_name": "<string>",
        "birth_date": "<string>",
        "phone_number": "<string>",
        "phone_type": "<string>",
        "billing_address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "braintree_customer_id": "<string>",
        "payment_method_nonce": "<string>",
        "payer_id": "<string>"
      },
      "ledger_account": {
        "account_id": "<string>",
        "currency": "<string>",
        "owner": {
          "id": "<string>",
          "uuid": "<string>",
          "user_uuid": "<string>",
          "type": "UNKNOWN"
        },
        "account_uuid": "<string>",
        "account_name": "<string>"
      },
      "external_payment_method": {
        "payment_method_id": "<string>"
      },
      "pro_account": {
        "account_id": "<string>",
        "coinbase_account_id": "<string>",
        "user_id": "<string>",
        "currency": "<string>",
        "portfolio_id": "<string>"
      },
      "rtp": {
        "account_holder": {
          "legal_name": "<string>",
          "identifier": "<string>",
          "bank_account_id": "<string>"
        },
        "institution": {
          "routing_number": "<string>"
        },
        "currency": "<string>"
      },
      "venue": {
        "name": "<string>"
      },
      "ledger_named_account": {
        "name": "<string>",
        "currency": "<string>",
        "foreign_network": "<string>"
      },
      "custodial_pool": {
        "name": "<string>",
        "network": "<string>",
        "fiat_id": "<string>"
      },
      "apple_pay": {
        "braintree": {
          "nonce": "<string>",
          "correlation_id": "<string>"
        },
        "apple_pay": {
          "nonce": "<string>",
          "correlation_id": "<string>",
          "version": "<string>"
        },
        "user_id": "<string>",
        "postal_code": "<string>",
        "customer_name": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "six_digit_bin": "<string>",
        "last_four": "<string>",
        "issuing_country": "<string>",
        "issuing_bank": "<string>",
        "product_id": "<string>",
        "customer_dob": "<string>",
        "billing_address_id": "<string>",
        "currency": "<string>",
        "scheme": "UNKNOWN",
        "prepaid": "UNCERTAIN",
        "debit": "UNCERTAIN"
      },
      "default_account": {
        "user_uuid": "<string>",
        "currency": "<string>"
      },
      "remitly": {
        "account_holder": {
          "recipient_id": "<string>",
          "payout_method_type": "<string>",
          "recipient_version": "<string>",
          "currency": "<string>"
        }
      },
      "pro_internal_account": {
        "user_id": "<string>",
        "currency": "<string>"
      },
      "dapp_wallet_account": {
        "user_uuid": "<string>",
        "network": "<string>",
        "cohort_id": "<string>",
        "signing_backend": "<string>",
        "currency": "<string>"
      },
      "google_pay": {
        "braintree": {
          "nonce": "<string>",
          "correlation_id": "<string>"
        },
        "google_pay": {
          "nonce": "<string>",
          "correlation_id": "<string>",
          "version": "<string>"
        },
        "user_id": "<string>",
        "postal_code": "<string>",
        "customer_name": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "six_digit_bin": "<string>",
        "last_four": "<string>",
        "issuing_country": "<string>",
        "issuing_bank": "<string>",
        "product_id": "<string>",
        "customer_dob": "<string>",
        "billing_address_id": "<string>",
        "currency": "<string>",
        "scheme": "UNKNOWN",
        "prepaid": "UNCERTAIN",
        "debit": "UNCERTAIN"
      },
      "dapp_wallet_blockchain_address": {
        "network": "<string>",
        "address": "<string>",
        "cohort_id": "<string>",
        "user_uuid": "<string>",
        "pool": "<string>"
      },
      "zaakpay_mobikwik": {
        "phone_number": "<string>"
      },
      "deneb_upi": {
        "vpa_id": "<string>",
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "pan": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        }
      },
      "bank_account": {
        "customer_account_type": "<string>",
        "customer_account_number": "<string>",
        "customer_routing_number": "<string>",
        "customer_name": "<string>",
        "currency": "<string>",
        "bank_name": "<string>",
        "ach_entry_class": "<string>",
        "verification_method": "<string>",
        "cdv_amount_1": {
          "amount": "<string>",
          "currency": "<string>"
        },
        "cdv_amount_2": {
          "amount": "<string>",
          "currency": "<string>"
        },
        "ach_invalid_account": true,
        "ach_bad_account": true,
        "received_non_nsf_return": true,
        "giactInfo": {
          "giact_item_id": "<string>",
          "giact_bank_result": "<string>",
          "giact_customer_result": "<string>",
          "giact_status": "<string>",
          "giact_last_updated_at": "<string>",
          "giact_account_added_date": "<string>",
          "giact_account_last_updated_date": "<string>"
        },
        "vendor_verification_status": {
          "iav_status": "<string>",
          "md_status": "<string>",
          "email_verification_status": "<string>"
        },
        "plaid_verification_info": {
          "plaid_bank_name": "<string>",
          "plaid_access_token": "<string>",
          "plaid_item_id": "<string>",
          "plaid_old_access_token": "<string>",
          "plaid_response": "<string>",
          "plaid_info": "<string>",
          "plaid_account_id": "<string>",
          "plaid_test_account": true,
          "plaid_link_account": true,
          "plaid_link_bank_name": "<string>",
          "balance": {
            "amount": "<string>",
            "currency": "<string>"
          },
          "balance_updated_at": "<string>",
          "plaid_name_check": true,
          "plaid_public_token": "<string>",
          "account_number_mask": "<string>"
        },
        "stripe_verification_info": {
          "stripe_response": "<string>",
          "stripe_info": "<string>",
          "stripe_financial_connections_id": "<string>",
          "stripe_payment_method_id": "<string>",
          "stripe_test_account": true,
          "stripe_link_account": true,
          "stripe_link_bank_name": "<string>",
          "stripe_institution_id": "<string>",
          "balance": {
            "amount": "<string>",
            "currency": "<string>"
          },
          "balance_updated_at": "<string>",
          "stripe_name_check": true
        }
      },
      "identity_contract_call": {
        "network": "<string>",
        "address": "<string>"
      },
      "deneb_imps": {
        "ifsc_code": "<string>",
        "account_number": "<string>",
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "pan": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        }
      },
      "allocation": {
        "id": "<string>",
        "legs": [
          {
            "id": "<string>",
            "movements": [
              {
                "id": "<string>",
                "source_account": {
                  "account_id": "<string>",
                  "currency": "<string>",
                  "owner": {
                    "id": "<string>",
                    "uuid": "<string>",
                    "user_uuid": "<string>",
                    "type": "UNKNOWN"
                  },
                  "account_uuid": "<string>",
                  "account_name": "<string>"
                },
                "destination_account": {
                  "account_id": "<string>",
                  "currency": "<string>",
                  "owner": {
                    "id": "<string>",
                    "uuid": "<string>",
                    "user_uuid": "<string>",
                    "type": "UNKNOWN"
                  },
                  "account_uuid": "<string>",
                  "account_name": "<string>"
                },
                "source_currency": "<string>",
                "target_currency": "<string>",
                "amount": {
                  "amount": "<string>",
                  "currency": "<string>"
                },
                "fee": {
                  "amount": "<string>",
                  "currency": "<string>"
                }
              }
            ],
            "is_netted": true
          }
        ],
        "is_netted": true
      },
      "liquidity_pool": {
        "network": "<string>",
        "pool": "<string>",
        "currency": "<string>",
        "account_id": "<string>",
        "from_address": "<string>"
      },
      "zengin_v2": {
        "account_holder": {
          "legal_name": "<string>",
          "identifier": "<string>",
          "type": "<string>"
        },
        "institution": {
          "bank_code": "<string>",
          "branch_code": "<string>"
        }
      },
      "direct_deposit": {
        "direct_deposit_account": "<string>"
      },
      "sepa_v2": {
        "account": {
          "legal_name": "<string>",
          "iban": "<string>",
          "bban": "<string>",
          "account_address": "<string>"
        },
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "customer_country": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "supports_open_banking": true,
        "banking_circle_token": {
          "mandate_id": "<string>",
          "active": true,
          "balance_check_supported": true,
          "hosted_page_id": "<string>"
        },
        "institution": {
          "bic": "<string>",
          "name": "<string>",
          "bank_country_code": "<string>",
          "bank_country_name": "<string>",
          "bank_address": "<string>"
        },
        "currency": "<string>",
        "fiat_id": "<string>",
        "direct_debit_ineligible": true,
        "open_banking_details": {
          "true_layer_bank_provider_id": "<string>",
          "open_banking_provider_id": "<string>"
        }
      },
      "zepto": {
        "account": {
          "contact_id": "<string>",
          "bank_account_id": "<string>",
          "account_number": "<string>",
          "customer_payment_name": "<string>"
        },
        "institution": {
          "branch_code": "<string>"
        }
      },
      "pix_ebanx": {
        "payment_method_id": "<string>",
        "user_uuid": "<string>",
        "deposit": {
          "transaction_id": "<string>",
          "account_id": "<string>",
          "account_number": "<string>",
          "account_type": "<string>",
          "bank_code": "<string>",
          "bank_name": "<string>",
          "branch_number": "<string>",
          "customer_payment_name": "<string>",
          "sender_document": "<string>"
        },
        "withdrawal": {
          "account_number": "<string>",
          "account_type": "<string>",
          "bank_code": "<string>",
          "branch_number": "<string>",
          "pix_key": "<string>",
          "customer_payment_name": "<string>",
          "sender_document": "<string>",
          "bank_name": "<string>"
        }
      },
      "signet": {
        "signet_wallet_id": "<string>"
      },
      "derivative_settlement": {
        "account_settlements": [
          {
            "amount": {
              "amount": "<string>",
              "currency": "<string>"
            },
            "source_ledger_account": {
              "account_id": "<string>",
              "currency": "<string>",
              "owner": {
                "id": "<string>",
                "uuid": "<string>",
                "user_uuid": "<string>",
                "type": "UNKNOWN"
              },
              "account_uuid": "<string>",
              "account_name": "<string>"
            },
            "source_ledger_named_account": {
              "name": "<string>",
              "currency": "<string>",
              "foreign_network": "<string>"
            },
            "target_ledger_account": {
              "account_id": "<string>",
              "currency": "<string>",
              "owner": {
                "id": "<string>",
                "uuid": "<string>",
                "user_uuid": "<string>",
                "type": "UNKNOWN"
              },
              "account_uuid": "<string>",
              "account_name": "<string>"
            },
            "target_ledger_named_account": {
              "name": "<string>",
              "currency": "<string>",
              "foreign_network": "<string>"
            },
            "hold_id_to_replace": "<string>",
            "new_hold_id": "<string>",
            "new_hold_amount": {
              "amount": "<string>",
              "currency": "<string>"
            },
            "existing_hold_id": "<string>"
          }
        ],
        "equity_reset": {
          "amount": {
            "amount": "<string>",
            "currency": "<string>"
          },
          "equity_account": {
            "account_id": "<string>",
            "currency": "<string>",
            "owner": {
              "id": "<string>",
              "uuid": "<string>",
              "user_uuid": "<string>",
              "type": "UNKNOWN"
            },
            "account_uuid": "<string>",
            "account_name": "<string>"
          }
        }
      },
      "user": {
        "user_uuid": "<string>"
      },
      "sg_fast": {
        "account": {
          "customer_name": "<string>",
          "account_number": "<string>"
        },
        "institution": {
          "bank_code": "<string>"
        },
        "direct_debit_auth_state": "STATE_UNSPECIFIED",
        "direct_debit_token": "<string>"
      },
      "interac": {
        "account": {
          "account_name": "<string>",
          "institution_number": "<string>",
          "transit_number": "<string>",
          "account_number": "<string>"
        },
        "pmsvc_id": "<string>",
        "withdrawal_only": true,
        "coinbase_email": "<string>"
      },
      "intra_bank": {
        "currency": "<string>",
        "account_number": "<string>",
        "routing_number": "<string>",
        "customer_name": "<string>",
        "fiat_id": "<string>"
      },
      "cbit": {
        "cbit_wallet_address": "<string>",
        "customers_bank_account_id": "<string>",
        "currency": "<string>"
      },
      "ideal": {
        "currency": "<string>",
        "iban": "<string>",
        "bic": "<string>",
        "bank_name": "<string>",
        "customer_payment_name": "<string>",
        "customer_country_code": "<string>"
      },
      "sofort": {
        "currency": "<string>",
        "iban": "<string>",
        "bic": "<string>",
        "bank_name": "<string>",
        "customer_payment_name": "<string>",
        "customer_country_code": "<string>"
      },
      "sg_paynow": {
        "identifier_type": "TYPE_UNSPECIFIED",
        "identifier": "<string>",
        "customer_name": "<string>"
      },
      "checkout_payment_link": {
        "payment_link_id": "<string>"
      },
      "email_address": {
        "value": "<string>"
      },
      "phone_number": {
        "value": "<string>"
      },
      "vendor_payment": {
        "vendor_name": "<string>",
        "vendor_payment_id": "<string>"
      },
      "ctn": {
        "id": "<string>"
      },
      "bancomat_pay": {
        "customer_name": "<string>",
        "account": {
          "phone_number": "<string>"
        }
      },
      "hot_wallet": {
        "network": "<string>",
        "address": "<string>"
      },
      "nova_account": {
        "network": "<string>",
        "nova_account_id": "<string>",
        "pool_name": "<string>",
        "account_idempotency_key": "<string>"
      },
      "magic_spend_blockchain_address": {
        "address": "<string>",
        "destination_tag": "<string>"
      },
      "transfer_pointer": {
        "idem": "<string>"
      },
      "eft": {
        "account": {
          "account_name": "<string>",
          "account_phone_number": "<string>",
          "account_email": "<string>",
          "institution_number": "<string>",
          "transit_number": "<string>",
          "account_number": "<string>",
          "provider_id": "<string>",
          "provider_name": "<string>",
          "trustly_account_token_status": "TRUSTLY_TOKEN_STATUS_UNSPECIFIED",
          "account_type": "<string>"
        }
      },
      "wallace_account": {
        "wallace_account_id": "<string>",
        "pool_name": "<string>"
      },
      "manual": {
        "settlement_bank_name": "<string>",
        "settlement_account_number": "<string>",
        "reference": "<string>"
      },
      "argentine_bank_account": {
        "tax_id": "<string>",
        "cbu": "<string>"
      },
      "representment": {
        "currency": "<string>"
      },
      "banking_circle_now": {
        "iban": "<string>",
        "currency": "<string>",
        "customer_payment_name": "<string>"
      },
      "trustly": {
        "country": "<string>",
        "iban": "<string>",
        "account_holder": "<string>",
        "bank_code": "<string>",
        "account_number": "<string>",
        "partial_account_number": "<string>",
        "bank_name": "<string>",
        "email": "<string>"
      },
      "blik": {
        "email": "<string>",
        "country": "<string>",
        "account_holder": "<string>"
      },
      "mb_way": {
        "account_holder": "<string>",
        "country": "<string>",
        "phone_number": "<string>"
      },
      "pix": {
        "account_number": "<string>",
        "account_type": "<string>",
        "bank_code": "<string>",
        "bank_name": "<string>",
        "branch_number": "<string>",
        "customer_payment_name": "<string>",
        "sender_document": "<string>",
        "pix_key": "<string>",
        "pix_account_type": "ACCOUNT_TYPE_UNSPECIFIED",
        "branch_code": "<string>"
      },
      "magic_wallet": {
        "user_uuid": "<string>",
        "network_name": "<string>",
        "wallet_name": "<string>"
      },
      "samsung_pay": {
        "samsung_pay": {
          "nonce": "<string>",
          "correlation_id": "<string>",
          "version": "<string>"
        },
        "user_id": "<string>",
        "postal_code": "<string>",
        "customer_name": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "six_digit_bin": "<string>",
        "last_four": "<string>",
        "issuing_country": "<string>",
        "issuing_bank": "<string>",
        "product_id": "<string>",
        "customer_dob": "<string>",
        "currency": "<string>",
        "billing_address_id": "<string>",
        "scheme": "UNKNOWN",
        "prepaid": "UNCERTAIN",
        "debit": "UNCERTAIN"
      },
      "cad_wire": {
        "account_number": "<string>",
        "customer_payment_name": "<string>",
        "account_address": "<string>",
        "institution_number": "<string>",
        "transit_number": "<string>",
        "bank_address1": "<string>",
        "institution_name": "<string>"
      },
      "wab_intra_bank": {
        "account_id": "<string>",
        "customer_name": "<string>"
      },
      "india_bank_account": {
        "account_number": "<string>",
        "ifsc_code": "<string>",
        "bank_name": "<string>",
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "pan_number": "<string>",
        "customer_payment_name": "<string>"
      },
      "jpmorgan_kinexys": {
        "account_number": "<string>",
        "bic": "<string>"
      },
      "sic": {
        "legal_name": "<string>",
        "iban": "<string>",
        "swift": "<string>",
        "customer_payment_address1": "<string>",
        "customer_payment_address2": "<string>",
        "customer_payment_address3": "<string>"
      },
      "spei": {
        "account_number": "<string>",
        "customer_name": "<string>"
      }
    },
    "target": {
      "type": "INVALID",
      "network": "<string>",
      "payment_method_id": "<string>",
      "blockchain_address": {
        "address": "<string>",
        "destination_tag": "<string>"
      },
      "coinbase_account": {
        "account_id": "<string>"
      },
      "blockchain_transaction": {
        "hsh": "<string>",
        "height": 123,
        "normalized_hash": "<string>"
      },
      "fedwire": {
        "routing_number": "<string>",
        "account_holder": {
          "legal_name": "<string>",
          "account_number": "<string>",
          "address": {
            "lines": [
              "<string>"
            ],
            "country_code": "<string>",
            "city": "<string>",
            "state": "<string>",
            "postal_code": "<string>"
          }
        },
        "bank": {
          "name": "<string>",
          "address": {
            "lines": [
              "<string>"
            ],
            "country_code": "<string>",
            "city": "<string>",
            "state": "<string>",
            "postal_code": "<string>"
          },
          "identifier": "<string>",
          "type": "CHIPS_IDENTIFIER",
          "identifier_code": "<string>"
        },
        "intermediary_bank": {
          "name": "<string>",
          "address": {
            "lines": [
              "<string>"
            ],
            "country_code": "<string>",
            "city": "<string>",
            "state": "<string>",
            "postal_code": "<string>"
          },
          "identifier": "<string>",
          "type": "CHIPS_IDENTIFIER",
          "identifier_code": "<string>"
        },
        "currency": "<string>",
        "fiat_id": "<string>",
        "rejection_count_v2": 123
      },
      "swift": {
        "account_holder": {
          "legal_name": "<string>",
          "iban": "<string>",
          "bban": "<string>",
          "domestic_account_id": "<string>",
          "customer_payment_address1": "<string>",
          "customer_payment_address2": "<string>",
          "customer_payment_address3": "<string>",
          "customer_payment_country_code": "<string>",
          "iban_generated": true
        },
        "institution": {
          "bic": "<string>",
          "name": "<string>",
          "bank_address1": "<string>",
          "bank_address2": "<string>",
          "bank_address3": "<string>",
          "bank_country_code": "<string>",
          "domestic_bank_id": "<string>",
          "international_bank_id": "<string>"
        },
        "intermediary": {
          "bic": "<string>",
          "name": "<string>",
          "bank_address1": "<string>",
          "bank_address2": "<string>",
          "bank_address3": "<string>",
          "bank_country_code": "<string>",
          "domestic_bank_id": "<string>",
          "international_bank_id": "<string>"
        },
        "currency": "<string>",
        "rejection_count": 123
      },
      "card": {
        "first_data_token": {
          "value": "<string>",
          "store_id": "<string>",
          "id": "<string>"
        },
        "merchant": {
          "mid": "<string>"
        },
        "vault_token": {
          "value": "<string>",
          "vault_id": "<string>",
          "attempted_at": "<string>",
          "encrypted_data": "<string>",
          "encrypted_cvv": "<string>"
        },
        "worldpay_params": {
          "token_value": "<string>",
          "token_event": "<string>",
          "uses_merchant_token": true,
          "accept_header": "<string>",
          "user_agent_header": "<string>",
          "shopper_ip": "<string>",
          "shopper_session_id": "<string>"
        },
        "checkout_token": {
          "source_id": "<string>"
        },
        "stripe_params": {
          "stripe_id": "<string>",
          "radar_session_id": "<string>",
          "address_line1_check": "STRIPE_CARD_CHECK_STATUS_UNKNOWN",
          "address_postal_code_check": "STRIPE_CARD_CHECK_STATUS_UNKNOWN",
          "cvc_check": "STRIPE_CARD_CHECK_STATUS_UNKNOWN"
        },
        "six_digit_bin": "<string>",
        "last_four": "<string>",
        "eight_digit_bin": "<string>",
        "card_sub_brand": "<string>",
        "card_brand": "<string>",
        "obfuscated_pan": "<string>",
        "bin_info": {
          "match_found": true,
          "type": "UNKNOWN",
          "issuer": "<string>",
          "issuer_country_code": "<string>",
          "prepaid": true,
          "corporate": true,
          "consumer": true,
          "visa": {
            "oct_supported": true,
            "fast_funds_supported": true
          },
          "mastercard": {
            "fast_funds_supported": true
          },
          "use_3ds": true
        },
        "scheme": "UNKNOWN",
        "issuer": "<string>",
        "issuer_country_code": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "phone_number": "<string>",
        "user_id": "<string>",
        "postal_code": "<string>",
        "customer_info": {
          "full_name": "<string>",
          "first_name": "<string>",
          "last_name": "<string>",
          "dob": {
            "month": "<string>",
            "day": "<string>",
            "year": "<string>"
          },
          "nationality": "<string>",
          "email": "<string>"
        },
        "expiry_date": {
          "month": "<string>",
          "year": "<string>"
        },
        "cdv_method": "<string>",
        "cdv_amount_1": {
          "amount": "<string>",
          "currency": "<string>"
        },
        "cdv_amount_2": {
          "amount": "<string>",
          "currency": "<string>"
        },
        "worldpay_card_cdv_status": {
          "cdv_required": true,
          "cdv_failed": true
        },
        "previous_scheme_tx_id": "<string>",
        "initial_scheme_tx_id": "<string>",
        "rejected": true,
        "rejected_at": "<string>",
        "rejection_count": 123,
        "rejected_via_purchase": true,
        "account_updater_updated_at": "<string>",
        "account_updater_reason": "<string>",
        "account_updater_status": "<string>",
        "almost_expired_notified": true,
        "is_migrated_card": true,
        "migrated_created_at": "<string>",
        "card_source": "SOURCE_UNKNOWN",
        "added_via_buy_widget": true,
        "encrypted_data": "<string>",
        "coinbase_credit_card_params": {
          "credit_card_account_id": "<string>"
        },
        "customer_name": "<string>",
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "customer_dob": {
          "month": "<string>",
          "day": "<string>",
          "year": "<string>"
        }
      },
      "zengin": {
        "account_holder": {
          "legal_name": "<string>",
          "identifier": "<string>",
          "type": "<string>"
        },
        "institution": {
          "bank_code": "<string>",
          "branch_code": "<string>"
        }
      },
      "uk": {
        "account_holder": {
          "legal_name": "<string>",
          "bban": "<string>",
          "sort_code": "<string>",
          "account_number": "<string>",
          "address": {
            "address1": "<string>",
            "address2": "<string>",
            "city": "<string>",
            "state": "<string>",
            "postal_code": "<string>",
            "country": "<string>",
            "uuid": "<string>",
            "id": "<string>"
          },
          "account_type": "UK_ACCOUNT_TYPE_UNSPECIFIED"
        },
        "institution": {
          "name": "<string>"
        },
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "currency": "<string>",
        "open_banking_details": {
          "true_layer_bank_provider_id": "<string>",
          "open_banking_provider_id": "<string>"
        }
      },
      "sepa": {
        "account_holder": {
          "legal_name": "<string>",
          "iban": "<string>",
          "bban": "<string>"
        },
        "institution": {
          "bic": "<string>",
          "name": "<string>",
          "bank_country_code": "<string>",
          "bank_country_name": "<string>",
          "bank_address": "<string>"
        },
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>"
      },
      "paypal": {
        "account_holder": {
          "paypal_id": "<string>",
          "paypal_pm_id": "<string>",
          "paypal_verified": true,
          "account_type": "<string>",
          "account_age": "<string>"
        },
        "merchant": {
          "merchant_account_id": "<string>"
        },
        "metadata": {
          "paypal_correlation_id": "<string>",
          "authorization_code": "<string>"
        },
        "email": "<string>",
        "owner_name": "<string>",
        "birth_date": "<string>",
        "phone_number": "<string>",
        "phone_type": "<string>",
        "billing_address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "braintree_customer_id": "<string>",
        "payment_method_nonce": "<string>",
        "payer_id": "<string>"
      },
      "ledger_account": {
        "account_id": "<string>",
        "currency": "<string>",
        "owner": {
          "id": "<string>",
          "uuid": "<string>",
          "user_uuid": "<string>",
          "type": "UNKNOWN"
        },
        "account_uuid": "<string>",
        "account_name": "<string>"
      },
      "external_payment_method": {
        "payment_method_id": "<string>"
      },
      "pro_account": {
        "account_id": "<string>",
        "coinbase_account_id": "<string>",
        "user_id": "<string>",
        "currency": "<string>",
        "portfolio_id": "<string>"
      },
      "rtp": {
        "account_holder": {
          "legal_name": "<string>",
          "identifier": "<string>",
          "bank_account_id": "<string>"
        },
        "institution": {
          "routing_number": "<string>"
        },
        "currency": "<string>"
      },
      "venue": {
        "name": "<string>"
      },
      "ledger_named_account": {
        "name": "<string>",
        "currency": "<string>",
        "foreign_network": "<string>"
      },
      "custodial_pool": {
        "name": "<string>",
        "network": "<string>",
        "fiat_id": "<string>"
      },
      "apple_pay": {
        "braintree": {
          "nonce": "<string>",
          "correlation_id": "<string>"
        },
        "apple_pay": {
          "nonce": "<string>",
          "correlation_id": "<string>",
          "version": "<string>"
        },
        "user_id": "<string>",
        "postal_code": "<string>",
        "customer_name": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "six_digit_bin": "<string>",
        "last_four": "<string>",
        "issuing_country": "<string>",
        "issuing_bank": "<string>",
        "product_id": "<string>",
        "customer_dob": "<string>",
        "billing_address_id": "<string>",
        "currency": "<string>",
        "scheme": "UNKNOWN",
        "prepaid": "UNCERTAIN",
        "debit": "UNCERTAIN"
      },
      "default_account": {
        "user_uuid": "<string>",
        "currency": "<string>"
      },
      "remitly": {
        "account_holder": {
          "recipient_id": "<string>",
          "payout_method_type": "<string>",
          "recipient_version": "<string>",
          "currency": "<string>"
        }
      },
      "pro_internal_account": {
        "user_id": "<string>",
        "currency": "<string>"
      },
      "dapp_wallet_account": {
        "user_uuid": "<string>",
        "network": "<string>",
        "cohort_id": "<string>",
        "signing_backend": "<string>",
        "currency": "<string>"
      },
      "google_pay": {
        "braintree": {
          "nonce": "<string>",
          "correlation_id": "<string>"
        },
        "google_pay": {
          "nonce": "<string>",
          "correlation_id": "<string>",
          "version": "<string>"
        },
        "user_id": "<string>",
        "postal_code": "<string>",
        "customer_name": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "six_digit_bin": "<string>",
        "last_four": "<string>",
        "issuing_country": "<string>",
        "issuing_bank": "<string>",
        "product_id": "<string>",
        "customer_dob": "<string>",
        "billing_address_id": "<string>",
        "currency": "<string>",
        "scheme": "UNKNOWN",
        "prepaid": "UNCERTAIN",
        "debit": "UNCERTAIN"
      },
      "dapp_wallet_blockchain_address": {
        "network": "<string>",
        "address": "<string>",
        "cohort_id": "<string>",
        "user_uuid": "<string>",
        "pool": "<string>"
      },
      "zaakpay_mobikwik": {
        "phone_number": "<string>"
      },
      "deneb_upi": {
        "vpa_id": "<string>",
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "pan": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        }
      },
      "bank_account": {
        "customer_account_type": "<string>",
        "customer_account_number": "<string>",
        "customer_routing_number": "<string>",
        "customer_name": "<string>",
        "currency": "<string>",
        "bank_name": "<string>",
        "ach_entry_class": "<string>",
        "verification_method": "<string>",
        "cdv_amount_1": {
          "amount": "<string>",
          "currency": "<string>"
        },
        "cdv_amount_2": {
          "amount": "<string>",
          "currency": "<string>"
        },
        "ach_invalid_account": true,
        "ach_bad_account": true,
        "received_non_nsf_return": true,
        "giactInfo": {
          "giact_item_id": "<string>",
          "giact_bank_result": "<string>",
          "giact_customer_result": "<string>",
          "giact_status": "<string>",
          "giact_last_updated_at": "<string>",
          "giact_account_added_date": "<string>",
          "giact_account_last_updated_date": "<string>"
        },
        "vendor_verification_status": {
          "iav_status": "<string>",
          "md_status": "<string>",
          "email_verification_status": "<string>"
        },
        "plaid_verification_info": {
          "plaid_bank_name": "<string>",
          "plaid_access_token": "<string>",
          "plaid_item_id": "<string>",
          "plaid_old_access_token": "<string>",
          "plaid_response": "<string>",
          "plaid_info": "<string>",
          "plaid_account_id": "<string>",
          "plaid_test_account": true,
          "plaid_link_account": true,
          "plaid_link_bank_name": "<string>",
          "balance": {
            "amount": "<string>",
            "currency": "<string>"
          },
          "balance_updated_at": "<string>",
          "plaid_name_check": true,
          "plaid_public_token": "<string>",
          "account_number_mask": "<string>"
        },
        "stripe_verification_info": {
          "stripe_response": "<string>",
          "stripe_info": "<string>",
          "stripe_financial_connections_id": "<string>",
          "stripe_payment_method_id": "<string>",
          "stripe_test_account": true,
          "stripe_link_account": true,
          "stripe_link_bank_name": "<string>",
          "stripe_institution_id": "<string>",
          "balance": {
            "amount": "<string>",
            "currency": "<string>"
          },
          "balance_updated_at": "<string>",
          "stripe_name_check": true
        }
      },
      "identity_contract_call": {
        "network": "<string>",
        "address": "<string>"
      },
      "deneb_imps": {
        "ifsc_code": "<string>",
        "account_number": "<string>",
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "pan": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        }
      },
      "allocation": {
        "id": "<string>",
        "legs": [
          {
            "id": "<string>",
            "movements": [
              {
                "id": "<string>",
                "source_account": {
                  "account_id": "<string>",
                  "currency": "<string>",
                  "owner": {
                    "id": "<string>",
                    "uuid": "<string>",
                    "user_uuid": "<string>",
                    "type": "UNKNOWN"
                  },
                  "account_uuid": "<string>",
                  "account_name": "<string>"
                },
                "destination_account": {
                  "account_id": "<string>",
                  "currency": "<string>",
                  "owner": {
                    "id": "<string>",
                    "uuid": "<string>",
                    "user_uuid": "<string>",
                    "type": "UNKNOWN"
                  },
                  "account_uuid": "<string>",
                  "account_name": "<string>"
                },
                "source_currency": "<string>",
                "target_currency": "<string>",
                "amount": {
                  "amount": "<string>",
                  "currency": "<string>"
                },
                "fee": {
                  "amount": "<string>",
                  "currency": "<string>"
                }
              }
            ],
            "is_netted": true
          }
        ],
        "is_netted": true
      },
      "liquidity_pool": {
        "network": "<string>",
        "pool": "<string>",
        "currency": "<string>",
        "account_id": "<string>",
        "from_address": "<string>"
      },
      "zengin_v2": {
        "account_holder": {
          "legal_name": "<string>",
          "identifier": "<string>",
          "type": "<string>"
        },
        "institution": {
          "bank_code": "<string>",
          "branch_code": "<string>"
        }
      },
      "direct_deposit": {
        "direct_deposit_account": "<string>"
      },
      "sepa_v2": {
        "account": {
          "legal_name": "<string>",
          "iban": "<string>",
          "bban": "<string>",
          "account_address": "<string>"
        },
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "customer_country": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "supports_open_banking": true,
        "banking_circle_token": {
          "mandate_id": "<string>",
          "active": true,
          "balance_check_supported": true,
          "hosted_page_id": "<string>"
        },
        "institution": {
          "bic": "<string>",
          "name": "<string>",
          "bank_country_code": "<string>",
          "bank_country_name": "<string>",
          "bank_address": "<string>"
        },
        "currency": "<string>",
        "fiat_id": "<string>",
        "direct_debit_ineligible": true,
        "open_banking_details": {
          "true_layer_bank_provider_id": "<string>",
          "open_banking_provider_id": "<string>"
        }
      },
      "zepto": {
        "account": {
          "contact_id": "<string>",
          "bank_account_id": "<string>",
          "account_number": "<string>",
          "customer_payment_name": "<string>"
        },
        "institution": {
          "branch_code": "<string>"
        }
      },
      "pix_ebanx": {
        "payment_method_id": "<string>",
        "user_uuid": "<string>",
        "deposit": {
          "transaction_id": "<string>",
          "account_id": "<string>",
          "account_number": "<string>",
          "account_type": "<string>",
          "bank_code": "<string>",
          "bank_name": "<string>",
          "branch_number": "<string>",
          "customer_payment_name": "<string>",
          "sender_document": "<string>"
        },
        "withdrawal": {
          "account_number": "<string>",
          "account_type": "<string>",
          "bank_code": "<string>",
          "branch_number": "<string>",
          "pix_key": "<string>",
          "customer_payment_name": "<string>",
          "sender_document": "<string>",
          "bank_name": "<string>"
        }
      },
      "signet": {
        "signet_wallet_id": "<string>"
      },
      "derivative_settlement": {
        "account_settlements": [
          {
            "amount": {
              "amount": "<string>",
              "currency": "<string>"
            },
            "source_ledger_account": {
              "account_id": "<string>",
              "currency": "<string>",
              "owner": {
                "id": "<string>",
                "uuid": "<string>",
                "user_uuid": "<string>",
                "type": "UNKNOWN"
              },
              "account_uuid": "<string>",
              "account_name": "<string>"
            },
            "source_ledger_named_account": {
              "name": "<string>",
              "currency": "<string>",
              "foreign_network": "<string>"
            },
            "target_ledger_account": {
              "account_id": "<string>",
              "currency": "<string>",
              "owner": {
                "id": "<string>",
                "uuid": "<string>",
                "user_uuid": "<string>",
                "type": "UNKNOWN"
              },
              "account_uuid": "<string>",
              "account_name": "<string>"
            },
            "target_ledger_named_account": {
              "name": "<string>",
              "currency": "<string>",
              "foreign_network": "<string>"
            },
            "hold_id_to_replace": "<string>",
            "new_hold_id": "<string>",
            "new_hold_amount": {
              "amount": "<string>",
              "currency": "<string>"
            },
            "existing_hold_id": "<string>"
          }
        ],
        "equity_reset": {
          "amount": {
            "amount": "<string>",
            "currency": "<string>"
          },
          "equity_account": {
            "account_id": "<string>",
            "currency": "<string>",
            "owner": {
              "id": "<string>",
              "uuid": "<string>",
              "user_uuid": "<string>",
              "type": "UNKNOWN"
            },
            "account_uuid": "<string>",
            "account_name": "<string>"
          }
        }
      },
      "user": {
        "user_uuid": "<string>"
      },
      "sg_fast": {
        "account": {
          "customer_name": "<string>",
          "account_number": "<string>"
        },
        "institution": {
          "bank_code": "<string>"
        },
        "direct_debit_auth_state": "STATE_UNSPECIFIED",
        "direct_debit_token": "<string>"
      },
      "interac": {
        "account": {
          "account_name": "<string>",
          "institution_number": "<string>",
          "transit_number": "<string>",
          "account_number": "<string>"
        },
        "pmsvc_id": "<string>",
        "withdrawal_only": true,
        "coinbase_email": "<string>"
      },
      "intra_bank": {
        "currency": "<string>",
        "account_number": "<string>",
        "routing_number": "<string>",
        "customer_name": "<string>",
        "fiat_id": "<string>"
      },
      "cbit": {
        "cbit_wallet_address": "<string>",
        "customers_bank_account_id": "<string>",
        "currency": "<string>"
      },
      "ideal": {
        "currency": "<string>",
        "iban": "<string>",
        "bic": "<string>",
        "bank_name": "<string>",
        "customer_payment_name": "<string>",
        "customer_country_code": "<string>"
      },
      "sofort": {
        "currency": "<string>",
        "iban": "<string>",
        "bic": "<string>",
        "bank_name": "<string>",
        "customer_payment_name": "<string>",
        "customer_country_code": "<string>"
      },
      "sg_paynow": {
        "identifier_type": "TYPE_UNSPECIFIED",
        "identifier": "<string>",
        "customer_name": "<string>"
      },
      "checkout_payment_link": {
        "payment_link_id": "<string>"
      },
      "email_address": {
        "value": "<string>"
      },
      "phone_number": {
        "value": "<string>"
      },
      "vendor_payment": {
        "vendor_name": "<string>",
        "vendor_payment_id": "<string>"
      },
      "ctn": {
        "id": "<string>"
      },
      "bancomat_pay": {
        "customer_name": "<string>",
        "account": {
          "phone_number": "<string>"
        }
      },
      "hot_wallet": {
        "network": "<string>",
        "address": "<string>"
      },
      "nova_account": {
        "network": "<string>",
        "nova_account_id": "<string>",
        "pool_name": "<string>",
        "account_idempotency_key": "<string>"
      },
      "magic_spend_blockchain_address": {
        "address": "<string>",
        "destination_tag": "<string>"
      },
      "transfer_pointer": {
        "idem": "<string>"
      },
      "eft": {
        "account": {
          "account_name": "<string>",
          "account_phone_number": "<string>",
          "account_email": "<string>",
          "institution_number": "<string>",
          "transit_number": "<string>",
          "account_number": "<string>",
          "provider_id": "<string>",
          "provider_name": "<string>",
          "trustly_account_token_status": "TRUSTLY_TOKEN_STATUS_UNSPECIFIED",
          "account_type": "<string>"
        }
      },
      "wallace_account": {
        "wallace_account_id": "<string>",
        "pool_name": "<string>"
      },
      "manual": {
        "settlement_bank_name": "<string>",
        "settlement_account_number": "<string>",
        "reference": "<string>"
      },
      "argentine_bank_account": {
        "tax_id": "<string>",
        "cbu": "<string>"
      },
      "representment": {
        "currency": "<string>"
      },
      "banking_circle_now": {
        "iban": "<string>",
        "currency": "<string>",
        "customer_payment_name": "<string>"
      },
      "trustly": {
        "country": "<string>",
        "iban": "<string>",
        "account_holder": "<string>",
        "bank_code": "<string>",
        "account_number": "<string>",
        "partial_account_number": "<string>",
        "bank_name": "<string>",
        "email": "<string>"
      },
      "blik": {
        "email": "<string>",
        "country": "<string>",
        "account_holder": "<string>"
      },
      "mb_way": {
        "account_holder": "<string>",
        "country": "<string>",
        "phone_number": "<string>"
      },
      "pix": {
        "account_number": "<string>",
        "account_type": "<string>",
        "bank_code": "<string>",
        "bank_name": "<string>",
        "branch_number": "<string>",
        "customer_payment_name": "<string>",
        "sender_document": "<string>",
        "pix_key": "<string>",
        "pix_account_type": "ACCOUNT_TYPE_UNSPECIFIED",
        "branch_code": "<string>"
      },
      "magic_wallet": {
        "user_uuid": "<string>",
        "network_name": "<string>",
        "wallet_name": "<string>"
      },
      "samsung_pay": {
        "samsung_pay": {
          "nonce": "<string>",
          "correlation_id": "<string>",
          "version": "<string>"
        },
        "user_id": "<string>",
        "postal_code": "<string>",
        "customer_name": "<string>",
        "address": {
          "address1": "<string>",
          "address2": "<string>",
          "city": "<string>",
          "state": "<string>",
          "postal_code": "<string>",
          "country": "<string>",
          "uuid": "<string>",
          "id": "<string>"
        },
        "six_digit_bin": "<string>",
        "last_four": "<string>",
        "issuing_country": "<string>",
        "issuing_bank": "<string>",
        "product_id": "<string>",
        "customer_dob": "<string>",
        "currency": "<string>",
        "billing_address_id": "<string>",
        "scheme": "UNKNOWN",
        "prepaid": "UNCERTAIN",
        "debit": "UNCERTAIN"
      },
      "cad_wire": {
        "account_number": "<string>",
        "customer_payment_name": "<string>",
        "account_address": "<string>",
        "institution_number": "<string>",
        "transit_number": "<string>",
        "bank_address1": "<string>",
        "institution_name": "<string>"
      },
      "wab_intra_bank": {
        "account_id": "<string>",
        "customer_name": "<string>"
      },
      "india_bank_account": {
        "account_number": "<string>",
        "ifsc_code": "<string>",
        "bank_name": "<string>",
        "customer_first_name": "<string>",
        "customer_last_name": "<string>",
        "email": "<string>",
        "phone_number": "<string>",
        "pan_number": "<string>",
        "customer_payment_name": "<string>"
      },
      "jpmorgan_kinexys": {
        "account_number": "<string>",
        "bic": "<string>"
      },
      "sic": {
        "legal_name": "<string>",
        "iban": "<string>",
        "swift": "<string>",
        "customer_payment_address1": "<string>",
        "customer_payment_address2": "<string>",
        "customer_payment_address3": "<string>"
      },
      "spei": {
        "account_number": "<string>",
        "customer_name": "<string>"
      }
    },
    "unit_price": {
      "target_to_fiat": {
        "amount": {
          "value": "<string>",
          "currency": "<string>"
        },
        "scale": 123
      },
      "target_to_source": {
        "amount": {
          "value": "<string>",
          "currency": "<string>"
        },
        "scale": 123
      },
      "source_to_fiat": {
        "amount": {
          "value": "<string>",
          "currency": "<string>"
        },
        "scale": 123
      }
    },
    "user_warnings": [
      {
        "id": "<string>",
        "link": {
          "text": "<string>",
          "url": "<string>"
        },
        "context": {
          "details": [
            "<string>"
          ],
          "title": "<string>",
          "link_text": "<string>"
        },
        "code": "<string>",
        "message": "<string>"
      }
    ],
    "user_reference": "<string>",
    "source_currency": "<string>",
    "target_currency": "<string>",
    "cancellation_reason": {
      "message": "<string>",
      "code": "<string>",
      "error_code": "ERROR_CODES_UNSPECIFIED",
      "error_cta": "ERROR_CTA_UNSPECIFIED",
      "error_metadata": {
        "limit_amount": {
          "value": "<string>",
          "currency": "<string>"
        }
      },
      "title": "<string>"
    },
    "source_id": "<string>",
    "target_id": "<string>",
    "subscription_info": {
      "free_trading_reset_date": "<string>",
      "used_zero_fee_trading": {
        "value": "<string>",
        "currency": "<string>"
      },
      "remaining_free_trading_volume": {
        "value": "<string>",
        "currency": "<string>"
      },
      "max_free_trading_volume": {
        "value": "<string>",
        "currency": "<string>"
      },
      "has_benefit_cap": true,
      "applied_subscription_benefit": true,
      "fee_without_subscription_benefit": {
        "value": "<string>",
        "currency": "<string>"
      },
      "payment_method_fee_without_subscription_benefit": {
        "value": "<string>",
        "currency": "<string>"
      }
    },
    "exchange_rate": {
      "value": "<string>",
      "currency": "<string>"
    },
    "tax_details": [
      {
        "name": "<string>",
        "amount": {
          "value": "<string>",
          "currency": "<string>"
        }
      }
    ],
    "trade_incentive_info": {
      "applied_incentive": true,
      "user_incentive_id": "<string>",
      "code_val": "<string>",
      "ends_at": "<string>",
      "fee_without_incentive": {
        "value": "<string>",
        "currency": "<string>"
      },
      "redeemed": true
    },
    "total_fee_without_tax": {
      "title": "<string>",
      "description": "<string>",
      "amount": {
        "value": "<string>",
        "currency": "<string>"
      },
      "label": "<string>",
      "disclosure": {
        "title": "<string>",
        "description": "<string>",
        "link": {
          "text": "<string>",
          "url": "<string>"
        }
      },
      "waived_details": {
        "amount": {
          "value": "<string>",
          "currency": "<string>"
        },
        "source": "WAIVED_FEE_SOURCE_UNSPECIFIED"
      }
    },
    "fiat_denoted_total": {
      "value": "<string>",
      "currency": "<string>"
    }
  }
}
```