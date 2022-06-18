mod rent;
mod nft_callbacks;

pub use crate::rent::*;

use near_sdk::{AccountId, BorshStorageKey, near_bindgen, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use crate::base::RentFactory;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  rent: RentFactory,
  owner_id: AccountId,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    RentsCurrent,
    RentsPending,
    RentTokensPerAccount,
    RentsById,
    RentsPerAccount,
    RentsAt,
    ApprovedOwners
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new_with_default_meta(owner_id: AccountId) -> Self {
    Self::new(
      owner_id
    )
  }

  #[init]
  pub fn new(owner_id: AccountId) -> Self {
      let this = Self {
        rent: RentFactory::new(
          owner_id.clone(),
          AccountId::new_unchecked("mfight-nft_v2.testnet".to_string()),
          Some(StorageKey::ApprovedOwners),
          StorageKey::RentsCurrent,
          StorageKey::RentsPending,
          StorageKey::RentsById,
          StorageKey::RentTokensPerAccount,
          StorageKey::RentsPerAccount,
          StorageKey::RentsAt,
        ),
        owner_id: owner_id.into(),
      };

      this
  }

  pub fn rent_is_approved(&self, token_id: TokenId, account_id: AccountId) -> bool {
    let approve_id = self.rent.approved_owner_by_id.as_ref().unwrap().get(&token_id);

    if let Some(approve_id) = approve_id {
      return account_id == approve_id
    }

    false
  }

  pub fn change_nft_contract(&mut self, token_id: TokenId) -> u64 {
    self.rent.assert_owner();

    self.rent.nft_contract_id = AccountId::new_unchecked("mfight-nft_v2.testnet".to_string());

    0
  }
}

impl_rent_core!(Contract, rent);
impl_rent_enumeration!(Contract, rent);
impl_rent_stats!(Contract, rent);

