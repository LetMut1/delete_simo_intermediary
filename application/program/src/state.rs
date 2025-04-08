use solana_program::pubkey::Pubkey;
#[repr(C)]
#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, borsh::BorshSchema)]
pub struct Intermediary {
    pub is_initialized: bool,
    pub investor_pubkey: Pubkey,
    pub manager_pubkey: Pubkey,
    pub trader_pubkey: Pubkey,
    pub w_sol_token_account_pubkey: Pubkey,
    pub w_sol_token_account_authority_pubkey: Pubkey,
    pub temporary_w_sol_token_account_pubkey: Pubkey,
    pub temporary_w_sol_token_account_authority_pubkey: Pubkey,
    pub w_sol_token_account_authority_pubkey_bump_seed: u8,
    pub temporary_w_sol_token_account_pubkey_bump_seed: u8,
    pub temporary_w_sol_token_account_authority_pubkey_bump_seed: u8,
}
