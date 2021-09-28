mod nft;
mod action;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], 
) -> ProgramResult {
    msg!("GENERATUS GREETS YOU.");
    msg!("GENERATUS IS READING YOUR INSTRUCTIONS.");

    let action = action::GeneratusAction::try_from_slice(instruction_data)?;
    match action {
      action::GeneratusAction::CreateProject { name, description, website} => {
        msg!("GENERATUS READ CREATE PROJECT: name = {}, description = {:?}, website = {:?}", name, description, website);
        
      },
      _ => msg!("GENERATUS IS CONFUSED.")
    }

    /*    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

*/
    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );


        let mut instruction_data: Vec<u8> = Vec::new();
        action::GeneratusAction::CreateProject { name: "test".to_string(), description: None, website: None }.serialize(&mut instruction_data).unwrap();
        let accounts = vec![account];
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    }
}