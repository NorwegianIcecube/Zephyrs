use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};

entrypoint!(create_event);

fn create_event(
    program_id: &Pubkey, // Public key of the account the program was loaded into
    accounts: &[AccountInfo], // All the accounts that are passed to this program
    instruction_data: &[u8], // All instruction data
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter(); // Create an iterator over the accounts
    let event_account = next_account_info(account_info_iter)?; // Get the account that will hold the event data
    let organizer_account = next_account_info(account_info_iter)?; // Get the account of the organizer
    let system_program = next_account_info(account_info_iter)?; // Get the system program account

    // Process the instruction data into correct types
    let instruction_data:Vec<&str> = instruction_data.split(|&x| x == b',').map(|s| std::str::from_utf8(s).unwrap()).collect();

    // Check if the event account is owned by the programc
    if event_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if the organizer account is owned by the system program, which checks if the user is a valid Solana account
    if organizer_account.owner != system_program.key {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if the instruction data is valid
    if instruction_data.len() != 7 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Get the instruction data
    let event_name:String = instruction_data[0].to_string();
    let event_date:u64 = instruction_data[1].parse::<u64>().unwrap();
    let venue_name:String = instruction_data[2].to_string();
    let venue_address:String = instruction_data[3].to_string();
    let artists:Vec<String> = instruction_data[4].split(",").map(|s| s.to_string()).collect();
    let max_tickets:u64 = instruction_data[5].parse::<u64>().unwrap();
    let tickets_sold:u64 = instruction_data[6].parse::<u64>().unwrap();
    let ticket_price:u64 = instruction_data[7].parse::<u64>().unwrap();

    // Create the event data
    let event_data = EventData::new(
        *organizer_account.key,
        event_name,
        event_date,
        venue_name,
        venue_address,
        artists,
        max_tickets,
        tickets_sold,
        ticket_price,
    );

    // Serialize the event data
    let event_data_bytes = event_data.try_to_vec().unwrap();

    // Write the event data to the event account
    event_account.try_borrow_mut_data()?.copy_from_slice(&event_data_bytes);
    
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
    pub fn new(
        organizer: Pubkey,
        event_name: String,
        event_date: u64,
        venue_name: String,
        venue_address: String,
        artists: Vec<String>,
        max_tickets: u64,
        tickets_sold: u64,
        ticket_price: u64,
    ) -> Self {
        Self {
            organizer,
            event_name,
            event_date,
            venue_name,
            venue_address,
            artists,
            max_tickets,
            tickets_sold,
            ticket_price,
        }
    }

    pub fn try_to_vec(&self) -> Result<Vec<u8>, ProgramError> {
        let mut data = Vec::new();
        data.extend_from_slice(&self.organizer.as_ref());
        data.extend_from_slice("<s>".as_bytes());
        data.extend_from_slice(&self.event_name.as_bytes());
        data.extend_from_slice("<s>".as_bytes());
        data.extend_from_slice(&self.event_date.to_le_bytes());
        data.extend_from_slice(&self.venue_name.as_bytes());
        data.extend_from_slice("<s>".as_bytes());
        data.extend_from_slice(&self.venue_address.as_bytes());
        data.extend_from_slice("<s>".as_bytes());
        for artist in &self.artists {
            data.extend_from_slice(&artist.as_bytes());
            data.extend_from_slice("<sa>".as_bytes());
        }
        data.extend_from_slice(&self.max_tickets.to_le_bytes());
        data.extend_from_slice("<s>".as_bytes());
        data.extend_from_slice(&self.tickets_sold.to_le_bytes());
        data.extend_from_slice("<s>".as_bytes());
        data.extend_from_slice(&self.ticket_price.to_le_bytes());
        Ok(data)
    }
}