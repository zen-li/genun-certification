
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



#[derive(CandidType, Deserialize, Debug)]
pub enum TransferResult {
    Ok(u128),
    Err(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum CustomResult {
    Ok(u128),
    Err(String),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum CustomBatchResult {
    Ok(Vec<u128>),
    Err(String),
}


#[derive(CandidType, Deserialize, Debug)]
pub struct SetBaseUriArgs {
    pub uri: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct TokenUriArgs {
    pub token_id: u128,
}



#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MintArgs {
    pub owner: Account,
    pub name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
}


 // Add these functions
 #[derive(CandidType, Deserialize, Clone, Debug)]
 pub struct MintBatchArgs {
     pub owners: Vec<Account>,
     pub names: Vec<String>,
     pub descriptions: Vec<Option<String>>,
     pub logos: Vec<Option<String>>,
 }




 #[derive(CandidType, Deserialize, Clone, Debug)]
    pub struct MintSameBatchArgs {
        pub owner: Account,
        pub name: String,
        pub description: Option<String>,
        pub logo: Option<String>,
        pub amount: u32,
    }





#[derive(Clone)]
struct CertificationNFT {
    owner: Principal,
    is_manager: HashMap<Principal, bool>,
    token_owner: HashMap<TokenId, Principal>,
    owned_tokens: HashMap<Principal, HashSet<TokenId>>,
    tokens: HashMap<u64, Principal>, // Tracks the number of tokens for each principal
    next_token_id: u64, // Tracks the next token ID

}


impl Default for CertificationNFT {
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




impl CertificationNFT {
    fn _start_token_id(&self) -> u64 {
        1
    }


    fn grant_manager(&mut self, manager: Principal) -> Result<(), String> {
        let caller = ic_cdk::caller();
        if self.owner != caller {
            return Err("Unauthorized: only the owner can grant manager rights".to_string());
        }
        if let Some(true) = self.is_manager.get(&manager) {
            return Err("Error: Already a manager".to_string());
        }
        self.is_manager.insert(manager, true);
        Ok(())
    }
    



    fn revoke_manager(&mut self, manager: Principal) -> Result<(), String> {
        let caller = ic_cdk::caller();
        
        // Check if caller is the owner
        if self.owner != caller {
            return Err("Unauthorized: only the owner can revoke manager rights".to_string());
        }
        
        // Check if trying to revoke the owner
        if self.owner == manager {
            return Err("Cannot revoke owner from manager".to_string());
        }
        
        // Check if manager exists
        if !self.is_manager.contains_key(&manager) || !self.is_manager[&manager] {
            return Err("Invalid manager to revoke".to_string());
        }
        
        // Revoke manager rights
        self.is_manager.insert(manager, false);
        
        Ok(())
    }





    fn _before_token_transfers(
        from: Principal,
        to: Principal,
        start_token_id: u64,
        quantity: u64,
    ) -> Result<(), String> {
        let caller = ic_cdk::caller();
        
        // Check if caller is a manager directly from the hashmap
        if !STATE.with(|state| {
            state
                .borrow()
                .as_ref()
                .map_or(false, |contract| contract.is_manager.get(&caller).copied().unwrap_or(false))
        }) {
            return Err("Unauthorized transfer".to_string());
        }

        // Additional logic before token transfer (if any)
        // ...

        Ok(())
    }



    fn is_approved_for_all(&self, owner: &Principal, operator: &Principal) -> bool {
        // Check if the operator is a manager
        if self.is_manager.get(operator).copied().unwrap_or(false) {
            return true;
        }

        // Here, you would normally call the super class method. Since Rust does not have
        // an exact equivalent, this example assumes a placeholder for additional logic.
        // If there is a base implementation, it should be added here.
        // Example: self.base_is_approved_for_all(owner, operator)
        
        false // Replace with the actual call to the base implementation if needed
    }



    fn get_managers(&self) -> Vec<Principal> {
        self.is_manager
            .iter()
            .filter_map(|(principal, is_manager)| if *is_manager { Some(*principal) } else { None })
            .collect()
    }


    fn is_caller_manager(&self) -> Result<(), String> {
        let caller = ic_cdk::caller();
        if self.is_manager.get(&caller).copied().unwrap_or(false) {
            Ok(())
        } else {
            Err("Unauthorized: Only managers can perform this operation".to_string())
        }
    }








    async fn call_icrc7_base_uri(
        &self,
        canister_id: Principal,
    ) -> Result<String, String> {
        self.is_caller_manager()?;

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

    async fn call_icrc7_set_base_uri(
        &self,
        canister_id: Principal,
        args: SetBaseUriArgs,
    ) -> Result<(), String> {
        self.is_caller_manager()?;

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







    async fn call_icrc7_mint(
        &self,
        canister_id: Principal,
        args: MintArgs
    ) -> Result<u128, String> {
        self.is_caller_manager()?;

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



    async fn call_icrc7_mint_batch(
        &self,
        canister_id: Principal,
        args: MintBatchArgs
    ) -> Result<Vec<u128>, String> {
        self.is_caller_manager()?;

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


    
    
    




    async fn call_icrc7_transfer(
        &self,
        canister_id: Principal,
        caller: Account,
        args: Vec<TransferArg>,
    ) -> Result<Vec<Result<u128, String>>, String> {
        //self.is_caller_manager()?; // Authorization check

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






thread_local! {
    static STATE: std::cell::RefCell<Option<CertificationNFT>> = std::cell::RefCell::new(None);
}





#[ic_cdk::init]
fn init() {
    let caller = ic_cdk::caller();
    let mut is_manager = HashMap::new();
    is_manager.insert(caller, true);

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



#[ic_cdk::update]
fn grants_manager(manager: Principal) -> Result<(), String> {
    STATE.with(|state| {

        if let Some(contract) = state.borrow_mut().as_mut() {
            contract.grant_manager(manager)
        } else {
            Err("Contract not initialized".to_string())
        }
    })
}



#[ic_cdk::update]
fn revokes_manager(manager: Principal) -> Result<(), String> {
    STATE.with(|state| {
        if let Some(contract) = state.borrow_mut().as_mut() {
            contract.revoke_manager(manager)
        } else {
            Err("Contract not initialized".to_string())
        }
    })
}








#[ic_cdk::update]
async fn call_icrc7_base_uri_async(canister_id: Principal) -> Result<String, String> {
    let state_clone = STATE.with(|state| state.borrow().clone());
    if let Some(contract) = state_clone {
        contract.call_icrc7_base_uri(canister_id).await
    } else {
        Err("Contract not initialized".to_string())
    }
}

#[ic_cdk::update]
async fn call_icrc7_set_base_uri_async(canister_id: Principal, uri: String) -> Result<(), String> {
    let args = SetBaseUriArgs { uri };
    let state_clone = STATE.with(|state| state.borrow().clone());
    if let Some(contract) = state_clone {
        contract.call_icrc7_set_base_uri(canister_id, args).await
    } else {
        Err("Contract not initialized".to_string())
    }
}

#[ic_cdk::update]
async fn call_icrc7_token_uri_async(canister_id: Principal, token_id: u128) -> Result<String, String> {
    let state_clone = STATE.with(|state| state.borrow().clone());
    if let Some(contract) = state_clone {
        contract.call_icrc7_token_uri(canister_id, token_id).await
    } else {
        Err("Contract not initialized".to_string())
    }
}





#[ic_cdk::query]
fn get_owner() -> Principal {
    STATE.with(|state| {
        state
            .borrow()
            .as_ref()
            .map(|contract| contract.owner)
            .unwrap_or_else(|| Principal::anonymous())
    })
}



#[ic_cdk::query]
fn get_managers() -> Vec<Principal> {
    STATE.with(|state| {
        state.borrow().as_ref().map_or(vec![], |contract| {
            contract.get_managers()
        })
    })
}


#[ic_cdk::update]
async fn mint_token_to_icrc7(canister_id: Principal, owner: Principal, name: String, description: Option<String>, logo: Option<String>) -> Result<u128, String> {
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
        contract.call_icrc7_mint(canister_id, args).await
    } else {
        Err("Contract not initialized".to_string())
    }
}





#[ic_cdk::update]
async fn mint_batch_to_icrc7(canister_id: Principal, owners: Vec<Principal>, names: Vec<String>, descriptions: Vec<Option<String>>, logos: Vec<Option<String>>) -> Result<Vec<u128>, String> {
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
        contract.call_icrc7_mint_batch(canister_id, args).await
    } else {
        Err("Contract not initialized".to_string())
    }
}




#[ic_cdk::update]
async fn icrc7_transfer_async(
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
        contract.call_icrc7_transfer(canister_id, account, transfer_args).await
    } else {
        Err("Contract not initialized".to_string())
    }
}



ic_cdk::export_candid!();