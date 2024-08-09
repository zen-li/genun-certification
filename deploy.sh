#!/bin/bash

./build.sh


TOKEN_SYMBOL="HSD"
TOKEN_NAME="Harman Singh Dhaliwal"
TOKEN_DESCRIPTION="ICRC7 token Rust ICP contract"
TOKEN_LOGO="https://internetcomputer.org/img/ethdenver/astronaut.webp"
SUPPLY_CAP=50
CALLER_PRINCIPAL=$(dfx identity get-principal)

BASE_URI="https://example.com/token/"

# Ensure the paths are correct and the files exist
if [ ! -f "wasm_files/icrc7.wasm" ]; then
    echo "Error: wasm_files/icrc7.wasm does not exist."
    exit 1
fi

dfx deploy icrc7 --argument "(
    record {
        icrc7_symbol = \"$TOKEN_SYMBOL\";
        icrc7_name = \"$TOKEN_NAME\";
        icrc7_description = opt \"$TOKEN_DESCRIPTION\";
        icrc7_logo = opt \"$TOKEN_LOGO\";
        icrc7_supply_cap = opt $SUPPLY_CAP;
        icrc7_allow_transfers = null;
        icrc7_max_query_batch_size = opt 100;
        icrc7_max_update_batch_size = opt 100;
        icrc7_default_take_value = opt 1000;
        icrc7_max_take_value = opt 10000;
        icrc7_max_memo_size = opt 512;
        icrc7_permitted_drift = null;
        icrc7_tx_window = null;
        icrc7_burn_account = null;
        icrc7_deployer = principal \"$CALLER_PRINCIPAL\";
        icrc7_supported_standards = null;
        base_token_uri = \"$BASE_URI\";
    }
)"

dfx deploy genun_backend 