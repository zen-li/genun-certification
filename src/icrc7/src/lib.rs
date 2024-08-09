pub mod memory;
pub mod state;
pub mod utils;

use crate::state::{query_metadata, query_token_map, update_metadata};
use candid::{CandidType, Nat, Principal};
use ic_cdk::{init, query, update};
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_nft_types::{
    icrc7::{
        metadata::{Icrc7CollectionMetadata, Icrc7TokenMetadata},
        transfer::{TransferArg, TransferResult},
    },
    Account,
};
use serde::Deserialize;
use state::{get_txn_id, SetBaseUriArgs, Token, COLLECTION_METADATA, TOKEN_MAP};

#[derive(CandidType, Deserialize, Debug)]
pub struct InitArg {
    pub minting_auth: Option<Account>,
    pub icrc7_name: String,
    pub icrc7_symbol: String,
    pub icrc7_supply_cap: Option<u128>,
    pub icrc7_description: Option<String>, // Add description field
    pub icrc7_logo: Option<String>,        // Add logo field
}

#[init]
pub fn init(arg: InitArg) {
    ic_cdk::println!("InitArg received: {:?}", arg); // Debugging log
    COLLECTION_METADATA.with_borrow_mut(|metadata| {
        let mut data = metadata.get().clone();
        data.icrc7_name = Some(arg.icrc7_name);
        data.icrc7_symbol = Some(arg.icrc7_symbol);
        data.icrc7_supply_cap = arg.icrc7_supply_cap;
        if let Some(minting_auth) = arg.minting_auth {
            data.minting_auth = Some(minting_auth);
        }
        // Set the description and logo fields
        if let Some(description) = arg.icrc7_description {
            data.icrc7_description = Some(description);
        }
        if let Some(logo) = arg.icrc7_logo {
            data.icrc7_logo = Some(logo);
        }

        data.base_token_uri = String::new(); // Initialize base_token_uri
        data.next_token_id = 1; // Initialize next_token_id

        ic_cdk::println!("Metadata after init: {:?}", data);
        metadata.set(data).unwrap();
    });
}

pub fn icrc7_collection_metadata() -> Icrc7CollectionMetadata {
    query_metadata(|metadata| {
        let mut map = Icrc7CollectionMetadata::new();
        if let Some(name) = &metadata.icrc7_name {
            map.insert("icrc7:name".into(), MetadataValue::Text(name.clone()));
        }
        if let Some(symbol) = &metadata.icrc7_symbol {
            map.insert("icrc7:symbol".into(), MetadataValue::Text(symbol.clone()));
        }
        if let Some(logo) = &metadata.icrc7_logo {
            map.insert("icrc7:logo".into(), MetadataValue::Text(logo.clone()));
        }
        if let Some(description) = &metadata.icrc7_description {
            map.insert(
                "icrc7:description".into(),
                MetadataValue::Text(description.clone()),
            );
        }
        map.insert(
            "icrc7:total_supply".into(),
            MetadataValue::Nat(Nat::from(query_token_map(|map| map.len() as u128))),
        );
        if let Some(supply_cap) = metadata.icrc7_supply_cap {
            map.insert(
                "icrc7:supply_cap".into(),
                MetadataValue::Nat(Nat::from(supply_cap)),
            );
        }
        if let Some(max_query_batch_size) = metadata.icrc7_max_query_batch_size {
            map.insert(
                "icrc7:max_query_batch_size".into(),
                MetadataValue::Nat(Nat::from(max_query_batch_size)),
            );
        }
        if let Some(max_update_batch_size) = metadata.icrc7_max_update_batch_size {
            map.insert(
                "icrc7:max_update_batch_size".into(),
                MetadataValue::Nat(Nat::from(max_update_batch_size)),
            );
        }
        if let Some(default_take_value) = metadata.icrc7_default_take_value {
            map.insert(
                "icrc7:default_take_value".into(),
                MetadataValue::Nat(Nat::from(default_take_value)),
            );
        }
        if let Some(max_take_value) = metadata.icrc7_max_take_value {
            map.insert(
                "icrc7:max_take_value".into(),
                MetadataValue::Nat(Nat::from(max_take_value)),
            );
        }
        if let Some(max_memo_size) = metadata.icrc7_max_memo_size {
            map.insert(
                "icrc7:max_memo_size".into(),
                MetadataValue::Nat(Nat::from(max_memo_size)),
            );
        }
        if let Some(permitted_drift) = metadata.icrc7_permitted_drift {
            map.insert(
                "icrc7:permitted_drift".into(),
                MetadataValue::Nat(Nat::from(permitted_drift)),
            );
        }
        if let Some(tx_window) = metadata.icrc7_tx_window {
            map.insert(
                "icrc7:tx_window".into(),
                MetadataValue::Nat(Nat::from(tx_window)),
            );
        }
        map
    })
}

#[query]
pub fn icrc7_name() -> Option<String> {
    query_metadata(|metadata| {
        let name = metadata.icrc7_name.clone();
        ic_cdk::println!("icrc7_name: {:?}", name); // Debug log
        name
    })
}

#[query]
pub fn icrc7_symbol() -> Option<String> {
    query_metadata(|metadata| metadata.icrc7_symbol.clone())
}

#[query]
pub fn icrc7_total_supply() -> u128 {
    query_token_map(|map| map.len() as u128)
}

#[query]
pub fn icrc7_supply_cap() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_supply_cap)
}

#[query]
pub fn icrc7_description() -> Option<String> {
    query_metadata(|metadata| metadata.icrc7_description.clone())
}

#[query]
pub fn icrc7_logo() -> Option<String> {
    query_metadata(|metadata| metadata.icrc7_logo.clone())
}

#[query]
pub fn icrc7_max_query_batch_size() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_max_query_batch_size)
}

#[query]
pub fn icrc7_max_update_batch_size() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_max_update_batch_size)
}

#[query]
pub fn icrc7_default_take_value() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_default_take_value)
}

#[query]
pub fn icrc7_max_take_value() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_max_take_value)
}

#[query]
pub fn icrc7_max_memo_size() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_max_memo_size)
}

#[query]
pub fn icrc7_atomic_batch_transfer() -> Option<bool> {
    query_metadata(|metadata| metadata.icrc7_atomic_batch_transfer)
}

#[query]
pub fn icrc7_tx_window() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_tx_window)
}

#[query]
pub fn icrc7_permitted_drift() -> Option<u128> {
    query_metadata(|metadata| metadata.icrc7_permitted_drift)
}

#[query]
pub fn icrc7_token_metadata(token_ids: Vec<u128>) -> Vec<Option<Icrc7TokenMetadata>> {
    query_token_map(|token_map| {
        token_ids
            .into_iter()
            .map(|id| {
                if let Some(token) = token_map.get(&id) {
                    Some(token.token_metadata())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[query]
pub fn icrc7_owner_of(token_ids: Vec<u128>) -> Vec<Option<Account>> {
    query_token_map(|token_map| {
        token_ids
            .into_iter()
            .map(|id| {
                if let Some(token) = token_map.get(&id) {
                    Some(token.owner)
                } else {
                    None
                }
            })
            .collect()
    })
}

#[query]
pub fn icrc7_balance_of(accounts: Vec<Account>) -> Vec<Nat> {
    query_token_map(|token_map| {
        accounts
            .into_iter()
            .map(|account| {
                let mut balance = Nat::from(0u128);
                token_map.iter().for_each(|(_k, v)| {
                    if v.owner == account {
                        balance += Nat::from(1u128);
                    }
                });
                balance
            })
            .collect()
    })
}

pub fn icrc7_tokens(prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    todo!()
}

pub fn icrc7_tokens_of(account: Account, prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    todo!()
}

#[query]
fn base_uri() -> Result<String, String> {
    query_metadata(|metadata| {
        if metadata.base_token_uri.is_empty() {
            Err("Base URI is not set".to_string())
        } else {
            Ok(metadata.base_token_uri.clone())
        }
    })
}

#[update]
fn set_base_uri(args: SetBaseUriArgs) -> Result<(), String> {
    let mut success = false;

    update_metadata(|metadata| {
        let old_base_uri = metadata.base_token_uri.clone();
        metadata.base_token_uri = args.uri.clone();
        ic_cdk::println!(
            "SetBaseURI event: oldBaseURI={}, newBaseURI={}",
            old_base_uri,
            args.uri
        );
        success = true;
    });

    if success {
        Ok(())
    } else {
        Err("Failed to set base URI".to_string())
    }
}

#[query]
fn token_uri(token_id: u128) -> Result<String, String> {
    query_token_map(|token_map| {
        if !token_map.contains_key(&token_id) {
            return Err("NonExistentToken".to_string());
        }
        query_metadata(|metadata| {
            if metadata.base_token_uri.is_empty() {
                Ok("".to_string())
            } else {
                Ok(format!("{}{}", metadata.base_token_uri, token_id))
            }
        })
    })
}

fn get_next_token_id() -> u128 {
    let next_token_id = COLLECTION_METADATA.with_borrow_mut(|metadata| {
        let mut data = metadata.get().clone();
        let id = data.next_token_id;
        data.next_token_id += 1;
        metadata.set(data).unwrap();
        id
    });
    next_token_id
}

// Define the input structs for minting functions
#[derive(CandidType, Deserialize)]
pub struct MintArgs {
    pub owner: Account,
    pub name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
}

#[update]
pub fn mint(args: MintArgs) -> Result<u128, String> {
    let MintArgs {
        owner,
        name,
        description,
        logo,
    } = args;

    let new_id: u128 = get_next_token_id();
    let token = Token::new(new_id, owner.clone(), name, logo, description);

    let mut success = false;
    TOKEN_MAP.with_borrow_mut(|map| {
        if map.insert(new_id, token).is_none() {
            success = true;
        } else {
            ic_cdk::println!("Failed to insert token");
        }
    });

    if success {
        Ok(new_id)
    } else {
        Err("Failed to insert token".into())
    }
}

// Add these functions
#[derive(CandidType, Deserialize)]
pub struct MintBatchArgs {
    pub owners: Vec<Account>,
    pub names: Vec<String>,
    pub descriptions: Vec<Option<String>>,
    pub logos: Vec<Option<String>>,
}

#[update]
pub fn mint_batch(args: MintBatchArgs) -> Result<Vec<u128>, String> {
    let MintBatchArgs {
        owners,
        mut names,
        mut descriptions,
        mut logos,
    } = args;

    let owners_len = owners.len();

    // Adjust the lengths of the other vectors to match the length of the owners vector
    if names.len() != owners_len {
        let last_name = names.last().cloned().unwrap_or_default();
        names.resize(owners_len, last_name);
    }
    if descriptions.len() != owners_len {
        let last_description = descriptions.last().cloned().unwrap_or(None);
        descriptions.resize(owners_len, last_description);
    }
    if logos.len() != owners_len {
        let last_logo = logos.last().cloned().unwrap_or(None);
        logos.resize(owners_len, last_logo);
    }

    // Ensure all vectors are of the same length
    if owners.len() != names.len()
        || owners.len() != descriptions.len()
        || owners.len() != logos.len()
    {
        return Err("Input vector lengths do not match".into());
    }

    let mut token_ids = Vec::with_capacity(owners.len());
    for i in 0..owners.len() {
        match mint(MintArgs {
            owner: owners[i].clone(),
            name: names[i].clone(),
            description: descriptions[i].clone(),
            logo: logos[i].clone(),
        }) {
            Ok(id) => token_ids.push(id),
            Err(err) => return Err(err),
        }
    }

    Ok(token_ids)
}

#[update]
pub fn icrc7_transfer(caller: Account, args: Vec<TransferArg>) -> Vec<Result<u128, String>> {
    let mut results = Vec::new();

    for arg in args {
        // Step 1: Retrieve the token and clone it for processing
        let token_opt =
            query_token_map(|token_map| token_map.get(&arg.token_id).map(|token| token.clone()));

        if let Some(mut token) = token_opt {
            // Step 2: Ensure that the caller owns the token and the subaccount matches
            if token.owner.owner != caller.owner || token.owner.subaccount != arg.from_subaccount {
                results.push(Err(
                    "Unauthorized: Only the token owner can transfer the token.".to_string(),
                ));
                continue;
            }

            // Step 3: Check if the destination is different from the source
            if token.owner == arg.to {
                results.push(Err(
                    "Invalid transfer: Cannot transfer to the same account.".to_string(),
                ));
                continue;
            }

            // Step 4: Perform the transfer
            token.transfer(arg.to.clone());

            // Step 5: Insert the updated token back into the map
            let token_id = token.id;
            TOKEN_MAP.with_borrow_mut(|map| {
                map.insert(token_id, token);
            });

            results.push(Ok(token_id));
        } else {
            results.push(Err(
                "NonExistingTokenId: The specified token does not exist.".to_string(),
            ));
        }
    }

    results
}

pub fn burn() {}

#[derive(CandidType, Debug)]
pub struct Standard {
    name: String,
    url: String,
}

#[query]
pub fn icrc10_supported_standards() -> Vec<Standard> {
    vec![
        Standard {
            name: "ICRC-7".into(),
            url: "https://github.com/dfinity/ICRC/ICRCs/ICRC-7".into(),
        },
        Standard {
            name: "ICRC-61".into(),
            url: "https://github.com/dfinity/ICRC/ICRCs/ICRC-61".into(),
        },
    ]
}

ic_cdk::export_candid!();
