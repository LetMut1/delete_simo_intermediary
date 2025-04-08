use {
    crate::{
        error::Error,
        processor::Processor,
    },
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        program_error::PrintProgramError,
        pubkey::Pubkey,
    },
};
solana_program::entrypoint!(process_instruction);
fn process_instruction<'a>(program_id: &'a Pubkey, accounts: &'a [AccountInfo], instruction_data: &'a [u8]) -> ProgramResult {
    if let Err(program_error) = Processor::process(
        program_id,
        accounts,
        instruction_data,
    ) {
        program_error.print::<Error>();
        return Err(program_error);
    }
    Ok(())
}
