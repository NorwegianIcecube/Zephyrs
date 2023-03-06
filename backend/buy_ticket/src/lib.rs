use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use arrayref::{array_ref};

entrypoint!(buy_ticket);

fn buy_ticket(
    program_id: &Pubkey, // Public key of the account the program was loaded into
    accounts: &[AccountInfo], // All the accounts that are passed to this program
    instruction_data: &[u8], // All instruction data
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter(); // Create an iterator over the accounts
    let customer_account = next_account_info(account_info_iter)?; // Get the account of the customer
    let event_account = next_account_info(account_info_iter)?; // Get the account that holds the event data
    let event_owner_account = next_account_info(account_info_iter)?; // Get the account of the event owner
    let system_program = next_account_info(account_info_iter)?; // Get the system program account

    // Check if the customer account is owned by the system program, which checks if the user is a valid Solana account
    if customer_account.owner != system_program.key {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if the event account is owned by the program
    if event_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if the event owner account is owned by the system program, which checks if the user is a valid Solana account
    if event_owner_account.owner != system_program.key {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get ticket price from the event account
    let event_data = EventData::try_from_slice(&event_account.data.borrow())?;
    let ticket_price = event_data.ticket_price;

    // TODO
    // Check if the customer has enough SOL to buy a ticket (including gas and commission fees)
    // Check if there are any tickets left
    // Transfer the SOL from the customer to the event owner and the program owner
    // Update the event data, maybe use a layer 2 solution as there will be a lot of customer transactions, this can cut fees and speed up the process
    // Create a ticket NFT for the customer


    Ok(())
}

struct EventData {
    organizer: Pubkey,
    event_name: String,
    event_date: u64,
    venue_name: String,
    venue_address: String,
    artists: Vec<String>,
    max_tickets: u64,
    tickets_sold: u64,
    ticket_price: u64,
}

impl EventData {
    pub fn try_from_slice(data: &[u8]) -> Result<Self, ProgramError> {
        let event_data = EventData::unpack(data)?;
        Ok(event_data)
    }

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // TODO
        // Unpack input into the struct, input is a byte array of the event data with u64 numbers, Pubkey, and variable length strings separated by <s>.as_bytes()
        // Use array_ref! macro to get the byte array of the variable length strings ?
        // Use str::from_utf8 to convert the byte array to a string ?
        // Split the input into the different fields by knowing fields are separated by <s>.as_bytes()
        // Convert the u64 numbers to u64
        // Convert the Pubkey to a Pubkey
        // Convert the variable length strings to strings
        // Return the struct


        Ok(EventData {
            organizer,
            event_name,
            event_date,
            venue_name,
            venue_address,
            artists,
            max_tickets,
            tickets_sold,
            ticket_price,
        })
    }
}