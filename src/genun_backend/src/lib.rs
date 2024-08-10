
/*
 * @title CertificationNFT
 * @dev A simple implementation of a certification NFT system on the Internet Computer.
 * This contract manages the creation, transfer, and management of NFTs representing certifications.
 * The owner of the contract can grant and revoke manager roles to other Principals.
 * Managers can mint NFTs, transfer them, and set base URIs for token metadata.
 * The contract includes functionality for batch minting and transferring NFTs.
 */

 
// Import necessary modules and types from external crates.

use candid::{CandidType, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use icrc_nft_types::Account;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;


use icrc_nft_types::icrc7::transfer::TransferArg;
use ic_cdk::api::call::call;

type TokenId = u64;



// Enum to represent the result of a transfer operation.
#[derive(CandidType, Deserialize, Debug)]
pub enum TransferResult {
    Ok(u128),
    Err(String),
}

// Enum to represent custom results with success and error cases.
#[derive(CandidType, Deserialize, Debug)]
pub enum CustomResult {
    Ok(u128),
    Err(String),
}

// Enum to represent custom batch results for batch operations.
#[derive(CandidType, Deserialize, Debug)]
pub enum CustomBatchResult {
    Ok(Vec<u128>),
    Err(String),
}

// Struct to define the arguments for setting a base URI.
#[derive(CandidType, Deserialize, Debug)]
pub struct SetBaseUriArgs {
    pub uri: String,    // The base URI to be set.
}

// Struct to define the arguments for retrieving a token URI.
#[derive(CandidType, Deserialize, Debug)]
pub struct TokenUriArgs {
    pub token_id: u128,     // The ID of the token for which to retrieve the URI.
}


// Struct to define the arguments for minting a single NFT.
/**
 * @dev Struct to define the arguments for minting a single NFT.
 * @param owner The account that will own the minted NFT.
 * @param name The name of the NFT.
 * @param description An optional description of the NFT.
 * @param logo An optional logo URL for the NFT.
 */
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MintArgs {
    pub owner: Account,     
    pub name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
}


 // Struct to define the arguments for minting a batch of NFTs.
 /**
 * @dev Struct to define the arguments for minting a batch of NFTs.
 * @param owners A vector of accounts that will own the minted NFTs.
 * @param names A vector of names for the NFTs.
 * @param descriptions A vector of optional descriptions for the NFTs.
 * @param logos A vector of optional logos for the NFTs.
 */
 #[derive(CandidType, Deserialize, Clone, Debug)]
 pub struct MintBatchArgs {
     pub owners: Vec<Account>,
     pub names: Vec<String>,
     pub descriptions: Vec<Option<String>>,
     pub logos: Vec<Option<String>>,
 }

// Struct representing the CertificationNFT, which includes various mappings for managing NFTs.
 /**
 * @dev Struct representing the CertificationNFT, which includes various mappings for managing NFTs.
 * @param owner The owner of the contract (canister).
 * @param is_manager Mapping of managers (Principals) with their management status.
 * @param token_owner Mapping of Token IDs to their respective owners.
 * @param owned_tokens Mapping of Principals to sets of owned Token IDs.
 * @param tokens Tracks the number of tokens for each Principal.
 * @param next_token_id Tracks the next available Token ID.
 */
#[derive(Clone)]
struct CertificationNFT {
    owner: Principal,
    is_manager: HashMap<Principal, bool>,
    token_owner: HashMap<TokenId, Principal>,
    owned_tokens: HashMap<Principal, HashSet<TokenId>>,
    tokens: HashMap<u64, Principal>, // Tracks the number of tokens for each principal
    next_token_id: u64, // Tracks the next token ID

}


// Implementation of the default behavior for CertificationNFT.
impl Default for CertificationNFT {
        /**
     * @dev Initializes the default state for the CertificationNFT contract.
     * The owner is set to an anonymous Principal, and all other mappings are initialized as empty.
     */
    fn default() -> Self {
        Self {
            owner: Principal::anonymous(),
            is_manager: HashMap::new(),
            token_owner: HashMap::new(),
            owned_tokens: HashMap::new(),
            tokens: HashMap::new(),
            next_token_id: 1,

        }
    }
}




// Implementation of the CertificationNFT struct with various methods.
impl CertificationNFT {
        /**
     * @dev Returns the starting Token ID, which is always 1.
     */
    fn _start_token_id(&self) -> u64 {
        1
    }

    /**
     * @dev Grants manager rights to a Principal. Only the owner can grant these rights.
     * @param manager The Principal to be granted manager rights.
     * @return Result<(), String> Returns an error if the caller is not the owner or if the Principal is already a manager.
     */
    fn grant_manager(&mut self, manager: Principal) -> Result<(), String> {
        let caller = ic_cdk::caller();
        if self.owner != caller {
            return Err("UnauthorizedGrantManager".to_string());
        }
        if let Some(true) = self.is_manager.get(&manager) {
            return Err("Error: Already a manager".to_string());
        }
        self.is_manager.insert(manager, true);
        Ok(())
    }
    


    /**
     * @dev Revokes manager rights from a Principal. Only the owner can revoke these rights.
     * @param manager The Principal to be revoked manager rights.
     * @return Result<(), String> Returns an error if the caller is not the owner, if the manager is the owner, or if the manager does not exist.
     */
    fn revoke_manager(&mut self, manager: Principal) -> Result<(), String> {
        let caller = ic_cdk::caller();
        
        // Check if caller is the owner
        if self.owner != caller {
            return Err("UnauthorizedRevokeManager".to_string());
        }
        
        // Check if trying to revoke the owner
        if self.owner == manager {
            return Err("CannotRevokeOwnerFromManager".to_string());
        }
        
        // Check if manager exists
        if !self.is_manager.contains_key(&manager) || !self.is_manager[&manager] {
            return Err("InvalidRevokeManager".to_string());
        }
        
        // Revoke manager rights
        self.is_manager.insert(manager, false);
        
        Ok(())
    }



    /**
     * @dev Retrieves a list of all managers in the system.
     * @return Vec<Principal> A vector containing all manager Principals.
     */
    fn get_managers(&self) -> Vec<Principal> {
        self.is_manager
            .iter()
            .filter_map(|(principal, is_manager)| if *is_manager { Some(*principal) } else { None })
            .collect()
    }

    /**
     * @dev Checks if the caller is a manager.
     * @return Result<(), String> Returns an error if the caller is not a manager.
     */
    fn is_caller_manager(&self, custom_error: &str) -> Result<(), String> {
        let caller = ic_cdk::caller();
        if self.is_manager.get(&caller).copied().unwrap_or(false) {
            Ok(())
        } else {
            Err(custom_error.to_string())
        }
    }




    /**
     * @dev Asynchronously calls the `base_uri` method on another canister to retrieve the base URI.
     * @param canister_id The Principal of the canister to call.
     * @return Result<String, String> Returns the base URI or an error message.
     */
    async fn call_icrc7_base_uri(
        &self,
        canister_id: Principal,
    ) -> Result<String, String> {
        //self.is_caller_manager()?;

        ic_cdk::println!("Calling base_uri on canister: {:?}", canister_id);

        let result: Result<(Result<String, String>,), _> = call(canister_id, "base_uri", ()).await;

        match result {
            Ok((Ok(uri),)) => {
                ic_cdk::println!("Base URI retrieval successful: {}", uri);
                Ok(uri)
            },
            Ok((Err(err_msg),)) => {
                ic_cdk::println!("Base URI retrieval failed: {}", err_msg);
                Err(err_msg)
            },
            Err(e) => {
                ic_cdk::println!("Failed to call base_uri: {:?}", e);
                Err(format!("Failed to call base_uri: {:?}", e))
            },
        }
    }

    /**
     * @dev Asynchronously calls the `set_base_uri` method on another canister to set a new base URI.
     * @param canister_id The Principal of the canister to call.
     * @param args The arguments containing the new base URI.
     * @return Result<(), String> Returns Ok if successful, or an error message.
     */
    async fn call_icrc7_set_base_uri(
        &self,
        canister_id: Principal,
        args: SetBaseUriArgs,
    ) -> Result<(), String> {
        self.is_caller_manager("UnauthorizedSetBaseURI")?;  // Ensures that the caller is manager.

        ic_cdk::println!("Calling set_base_uri on canister: {:?}", canister_id);
        ic_cdk::println!("SetBaseUriArgs: {:?}", args);

        let result: Result<(Result<(), String>,), _> = call(canister_id, "set_base_uri", (args,)).await;

        match result {
            Ok((Ok(()),)) => {
                ic_cdk::println!("Set base URI successful");
                Ok(())
            },
            Ok((Err(err_msg),)) => {
                ic_cdk::println!("Set base URI failed: {}", err_msg);
                Err(err_msg)
            },
            Err(e) => {
                ic_cdk::println!("Failed to call set_base_uri: {:?}", e);
                Err(format!("Failed to call set_base_uri: {:?}", e))
            },
        }
    }

    /**
     * @dev Asynchronously calls the `token_uri` method on another canister to retrieve the token URI for a specific token ID.
     * @param canister_id The Principal of the canister to call.
     * @param token_id The ID of the token for which to retrieve the URI.
     * @return Result<String, String> Returns the token URI or an error message.
     */
    async fn call_icrc7_token_uri(
        &self,
        canister_id: Principal,
        token_id: u128,
    ) -> Result<String, String> {
        ic_cdk::println!("Calling token_uri on canister: {:?}", canister_id);
        ic_cdk::println!("Token ID: {:?}", token_id);

        let result: Result<(Result<String, String>,), _> = call(canister_id, "token_uri", (token_id,)).await;

        match result {
            Ok((Ok(uri),)) => {
                ic_cdk::println!("Token URI retrieval successful: {}", uri);
                Ok(uri)
            },
            Ok((Err(err_msg),)) => {
                ic_cdk::println!("Token URI retrieval failed: {}", err_msg);
                Err(err_msg)
            },
            Err(e) => {
                ic_cdk::println!("Failed to call token_uri: {:?}", e);
                Err(format!("Failed to call token_uri: {:?}", e))
            },
        }
    }






    /**
     * @dev Asynchronously calls the `mint` method on another canister to mint a new NFT.
     * Only managers can mint NFTs.
     * @param canister_id The Principal of the canister to call.
     * @param args The arguments containing the details of the NFT to mint.
     * @return Result<u128, String> Returns the minted token ID or an error message.
     */
    async fn call_icrc7_mint(
        &self,
        canister_id: Principal,
        args: MintArgs
    ) -> Result<u128, String> {
        self.is_caller_manager("UnauthorizedMint")?;  // Ensures that the caller is manager.

        ic_cdk::println!("Calling mint on canister: {:?}", canister_id);
        ic_cdk::println!("MintArgs: {:?}", args);

        let result: Result<(CustomResult,), _> = call(canister_id, "mint", (args,)).await;

        match result {
            Ok((CustomResult::Ok(token_id),)) => {
                ic_cdk::println!("Mint successful: token_id = {}", token_id);
                Ok(token_id)
            },
            Ok((CustomResult::Err(err_msg),)) => {
                ic_cdk::println!("Mint failed: {}", err_msg);
                Err(err_msg)
            },
            Err(e) => {
                ic_cdk::println!("Failed to call mint: {:?}", e);
                Err(format!("Failed to call mint: {:?}", e))
            },
        }
    }


    /**
     * @dev Asynchronously calls the `mint_batch` method on another canister to mint a batch of NFTs.
     * Only managers can mint NFTs.
     * @param canister_id The Principal of the canister to call.
     * @param args The arguments containing the details of the NFTs to mint in a batch.
     * @return Result<Vec<u128>, String> Returns a vector of minted token IDs or an error message.
     */
    async fn call_icrc7_mint_batch(
        &self,
        canister_id: Principal,
        args: MintBatchArgs
    ) -> Result<Vec<u128>, String> {
        self.is_caller_manager("UnauthorizedMint")?;  // Ensure the caller is a manager.

        ic_cdk::println!("Calling mint_batch on canister: {:?}", canister_id);
        ic_cdk::println!("MintBatchArgs: {:?}", args);

        let result: Result<(CustomBatchResult,), _> = call(canister_id, "mint_batch", (args,)).await;

        match result {
            Ok((CustomBatchResult::Ok(token_ids),)) => {
                ic_cdk::println!("Mint batch successful: token_ids = {:?}", token_ids);
                Ok(token_ids)
            },
            Ok((CustomBatchResult::Err(err_msg),)) => {
                ic_cdk::println!("Mint batch failed: {}", err_msg);
                Err(err_msg)
            },
            Err(e) => {
                ic_cdk::println!("Failed to call mint_batch: {:?}", e);
                Err(format!("Failed to call mint_batch: {:?}", e))
            },
        }
    }


    
    
    



    /**
     * @dev Asynchronously calls the `icrc7_transfer` method on another canister to transfer NFTs.
     * @param canister_id The Principal of the canister to call.
     * @param caller The account performing the transfer.
     * @param args The arguments containing the details of the transfer, including token IDs.
     * @return Result<Vec<Result<u128, String>>, String> Returns a vector of results for each transfer or an error message.
     */
    async fn call_icrc7_transfer(
        &self,
        canister_id: Principal,
        caller: Account,
        args: Vec<TransferArg>,
    ) -> Result<Vec<Result<u128, String>>, String> {
        self.is_caller_manager("UnauthorizedTransfer")?; // Works similar to _beforeTokenTransfer function Ensures that the caller is manager

        ic_cdk::println!("Calling icrc7_transfer on canister: {:?}", canister_id);
        ic_cdk::println!("TransferArgs: {:?}", args);

        let result: Result<(Vec<Result<u128, String>>,), _> = call(canister_id, "icrc7_transfer", (caller, args)).await;

        match result {
            Ok((transfer_results,)) => {
                ic_cdk::println!("Transfer successful: {:?}", transfer_results);
                Ok(transfer_results)
            },
            Err(e) => {
                ic_cdk::println!("Transfer failed: {:?}", e);
                Err(format!("Failed to call icrc7_transfer: {:?}", e))
            }
        }
    }






}





// Thread-local storage to hold the state of the CertificationNFT.
thread_local! {
    static STATE: std::cell::RefCell<Option<CertificationNFT>> = std::cell::RefCell::new(None);
}



/**
 * @dev Initialization function for the canister. This function is called when the canister is first created.
 * It sets up the initial state, including setting the caller as the owner and granting them manager rights.
 * It also initializes various mappings for managing NFTs.
 */
#[ic_cdk::init]
fn init() {
    let caller = ic_cdk::caller();
    let mut is_manager = HashMap::new();
    is_manager.insert(caller, true);    // Grant manager rights to the caller.

    let contract = CertificationNFT {
        owner: caller,
        is_manager,
        token_owner: HashMap::new(),
        owned_tokens: HashMap::new(),

        tokens: HashMap::new(),
        next_token_id: 1,

    };

    STATE.with(|state| {
        *state.borrow_mut() = Some(contract);
    });
}


/**
 * @dev Grants manager rights to a given Principal. Can only be called by the owner.
 * @param manager The Principal to be granted manager rights.
 * @return Result<(), String> Returns Ok if successful, or an error message.
 */
#[ic_cdk::update]
fn grantManager(manager: Principal) -> Result<(), String> {
    STATE.with(|state| {

        if let Some(contract) = state.borrow_mut().as_mut() {
            contract.grant_manager(manager)     // Call the grant_manager method.
        } else {
            Err("Contract not initialized".to_string())
        }
    })
}


/**
 * @dev Revokes manager rights from a given Principal. Can only be called by the owner.
 * @param manager The Principal to be revoked manager rights.
 * @return Result<(), String> Returns Ok if successful, or an error message.
 */
#[ic_cdk::update]
fn revokeManager(manager: Principal) -> Result<(), String> {
    STATE.with(|state| {
        if let Some(contract) = state.borrow_mut().as_mut() {
            contract.revoke_manager(manager)        // Call the revoke_manager method.
        } else {
            Err("Contract not initialized".to_string())
        }
    })
}







/**
 * @dev Asynchronously calls the `base_uri` method on another canister to retrieve the base URI.
 * @param canister_id The Principal of the canister to call.
 * @return Result<String, String> Returns the base URI or an error message.
 */
#[ic_cdk::update]
async fn baseURI(canister_id: Principal) -> Result<String, String> {
    let state_clone = STATE.with(|state| state.borrow().clone());
    if let Some(contract) = state_clone {
        contract.call_icrc7_base_uri(canister_id).await     // Call the async base_uri method.
    } else {
        Err("Contract not initialized".to_string())
    }
}

/**
 * @dev Asynchronously calls the `set_base_uri` method on another canister to set a new base URI.
 * @param canister_id The Principal of the canister to call.
 * @param uri The new base URI to be set.
 * @return Result<(), String> Returns Ok if successful, or an error message.
 */
#[ic_cdk::update]
async fn setBaseURI(canister_id: Principal, uri: String) -> Result<(), String> {
    let args = SetBaseUriArgs { uri };
    let state_clone = STATE.with(|state| state.borrow().clone());
    if let Some(contract) = state_clone {
        contract.call_icrc7_set_base_uri(canister_id, args).await       // Call the async set_base_uri method.
    } else {
        Err("Contract not initialized".to_string())
    }
}

/**
 * @dev Asynchronously calls the `token_uri` method on another canister to retrieve the token URI for a specific token ID.
 * @param canister_id The Principal of the canister to call.
 * @param token_id The ID of the token for which to retrieve the URI.
 * @return Result<String, String> Returns the token URI or an error message.
 */
#[ic_cdk::update]
async fn tokenURI(canister_id: Principal, token_id: u128) -> Result<String, String> {
    let state_clone = STATE.with(|state| state.borrow().clone());
    if let Some(contract) = state_clone {
        contract.call_icrc7_token_uri(canister_id, token_id).await      // Call the async token_uri method.
    } else {
        Err("Contract not initialized".to_string())
    }
}




/**
 * @dev Retrieves the owner of the contract.
 * @return Principal The Principal ID of the contract owner.
 */
#[ic_cdk::query]
fn get_owner() -> Principal {
    STATE.with(|state| {
        state
            .borrow()
            .as_ref()
            .map(|contract| contract.owner)
            .unwrap_or_else(|| Principal::anonymous())      // Return the owner or anonymous if not initialized.
    })
}


/**
 * @dev Retrieves a list of all managers in the system.
 * @return Vec<Principal> A vector containing all manager Principals.
 */
#[ic_cdk::query]
fn get_managers() -> Vec<Principal> {
    STATE.with(|state| {
        state.borrow().as_ref().map_or(vec![], |contract| {
            contract.get_managers()         // Call the get_managers method.
        })
    })
}

/**
 * @dev Asynchronously calls the `mint` method on another canister to mint a new NFT.
 * @param canister_id The Principal of the canister to call.
 * @param owner The Principal ID of the owner of the NFT.
 * @param name The name of the NFT.
 * @param description An optional description of the NFT.
 * @param logo An optional logo for the NFT.
 * @return Result<u128, String> Returns the minted token ID or an error message.
 */
#[ic_cdk::update]
async fn mint(canister_id: Principal, owner: Principal, name: String, description: Option<String>, logo: Option<String>) -> Result<u128, String> {
    let account = Account {
        owner,
        subaccount: None,
    };
    let args = MintArgs {
        owner: account,
        name,
        description,
        logo,
    };

    let state_clone = STATE.with(|state| {
        state.borrow().clone()
    });

    if let Some(contract) = state_clone {
        contract.call_icrc7_mint(canister_id, args).await       // Call the async mint method.
    } else {
        Err("Contract not initialized".to_string())
    }
}




/**
 * @dev Asynchronously calls the `mint_batch` method on another canister to mint a batch of NFTs.
 * @param canister_id The Principal of the canister to call.
 * @param owners A vector of Principal IDs representing the owners of the NFTs.
 * @param names A vector of names for the NFTs.
 * @param descriptions A vector of optional descriptions for the NFTs.
 * @param logos A vector of optional logos for the NFTs.
 * @return Result<Vec<u128>, String> Returns a vector of minted token IDs or an error message.
 */
#[ic_cdk::update]
async fn mintBatch(canister_id: Principal, owners: Vec<Principal>, names: Vec<String>, descriptions: Vec<Option<String>>, logos: Vec<Option<String>>) -> Result<Vec<u128>, String> {
    let accounts = owners.into_iter().map(|owner| Account { owner, subaccount: None }).collect();
    let args = MintBatchArgs {
        owners: accounts,
        names,
        descriptions,
        logos,
    };

    let state_clone = STATE.with(|state| {
        state.borrow().clone()
    });

    if let Some(contract) = state_clone {
        contract.call_icrc7_mint_batch(canister_id, args).await     // Call the async mint_batch method.
    } else {
        Err("Contract not initialized".to_string())
    }
}



/**
 * @dev Asynchronously calls the `icrc7_transfer` method on another canister to transfer NFTs.
 * @param canister_id The Principal of the canister to call.
 * @param caller The Principal ID of the caller initiating the transfer.
 * @param token_ids A vector of token IDs to be transferred.
 * @param to_principal The Principal ID of the recipient.
 * @param from_subaccount An optional subaccount from which the tokens are being transferred.
 * @return Result<Vec<Result<u128, String>>, String> Returns a vector of results for each transfer or an error message.
 */
#[ic_cdk::update]
async fn transfer(
    canister_id: Principal,
    caller: Principal,
    token_ids: Vec<u128>,
    to_principal: Principal,
    from_subaccount: Option<[u8; 32]>,
) -> Result<Vec<Result<u128, String>>, String> {
    let account = Account {
        owner: caller,
        subaccount: from_subaccount,
    };

    let transfer_args: Vec<TransferArg> = token_ids.into_iter().map(|token_id| TransferArg {
        token_id,
        from_subaccount,
        to: Account {
            owner: to_principal,
            subaccount: None,
        },
        memo: None,
        created_at_time: None,
    }).collect();

    let state_clone = STATE.with(|state| state.borrow().clone());

    if let Some(contract) = state_clone {
        contract.call_icrc7_transfer(canister_id, account, transfer_args).await     // Call the async transfer method.
    } else {
        Err("Contract not initialized".to_string())
    }
}



ic_cdk::export_candid!();