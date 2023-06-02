use arrayref::array_ref;
use solana_program::{
    account_info::AccountInfo, clock::Clock, program_error::ProgramError, sysvar::Sysvar,
};

use crate::error::BglShootoutError;

pub fn rand_choice<T: Clone>(
    choices: &Vec<T>,
    slot_hashes: &AccountInfo,
) -> Result<T, ProgramError> {
    assert!(*slot_hashes.key == solana_program::sysvar::slot_hashes::ID);

    let data = slot_hashes.data.borrow();
    let most_recent = array_ref![data, 12, 8];

    let clock = Clock::get()?;
    // seed for the random number is a combination of the slot_hash - timestamp
    let seed = usize::from_le_bytes(*most_recent).saturating_sub(clock.unix_timestamp as usize);

    let remainder: usize = seed
        .checked_rem(choices.len())
        .ok_or(BglShootoutError::NumericalOverflowError)?;

    Ok(choices[remainder].clone())
}
