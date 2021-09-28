use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Project {
  pub name: String,
  pub description: Option<String>,
  pub website: Option<String>,
  pub verified: bool
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Item {
  pub name: String,
  pub script: String,
}

