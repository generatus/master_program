use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GeneratusAction {
  CreateProject { name: String, description: Option<String>, website: Option<String>, }
}