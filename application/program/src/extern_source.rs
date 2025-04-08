use solana_program::{
    instruction::{
        AccountMeta,
        Instruction,
    },
    pubkey::Pubkey,
};

use crate::SPL_ASSOCIATED_TOKEN_ACCOUNT_ID;
// https://github.com/raydium-io/raydium-amm/blob/d10a8e9fab9f7a3d87b4ae3891e3e4c24b75c041/program/src/instruction.rs#L1045
pub fn create_raydium_swap_base_in_instruction<'a>(
    amm_program: &'a Pubkey,
    amm_pool: &'a Pubkey,
    amm_authority: &'a Pubkey,
    amm_open_orders: &'a Pubkey,
    amm_coin_vault: &'a Pubkey,
    amm_pc_vault: &'a Pubkey,
    market_program: &'a Pubkey,
    market: &'a Pubkey,
    market_bids: &'a Pubkey,
    market_asks: &'a Pubkey,
    market_event_queue: &'a Pubkey,
    market_coin_vault: &'a Pubkey,
    market_pc_vault: &'a Pubkey,
    market_vault_signer: &'a Pubkey,
    user_token_source: &'a Pubkey,
    user_token_destination: &'a Pubkey,
    user_source_owner: &'a Pubkey,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Instruction {
    Instruction {
        program_id: *amm_program,
        accounts: vec![
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new(*amm_pool, false),
            AccountMeta::new_readonly(*amm_authority, false),
            AccountMeta::new(*amm_open_orders, false),
            AccountMeta::new(*amm_coin_vault, false),
            AccountMeta::new(*amm_pc_vault, false),
            AccountMeta::new_readonly(*market_program, false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*market_bids, false),
            AccountMeta::new(*market_asks, false),
            AccountMeta::new(*market_event_queue, false),
            AccountMeta::new(*market_coin_vault, false),
            AccountMeta::new(*market_pc_vault, false),
            AccountMeta::new_readonly(*market_vault_signer, false),
            AccountMeta::new(*user_token_source, false),
            AccountMeta::new(*user_token_destination, false),
            AccountMeta::new_readonly(*user_source_owner, true),
        ],
        data: raydium_swap_base_in_pack(
            amount_in,
            minimum_amount_out,
        ),
    }
}
// https://github.com/raydium-io/raydium-amm/blob/d10a8e9fab9f7a3d87b4ae3891e3e4c24b75c041/program/src/instruction.rs#L646
// https://github.com/raydium-io/raydium-amm/blob/d10a8e9fab9f7a3d87b4ae3891e3e4c24b75c041/program/src/instruction.rs#L758
pub fn raydium_swap_base_in_pack(amount_in: u64, minimum_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::<u8>::with_capacity(17);
    buffer.push(9);
    buffer.extend_from_slice(amount_in.to_le_bytes().as_slice());
    buffer.extend_from_slice(minimum_amount_out.to_le_bytes().as_slice());
    buffer
}
pub fn create_pumpswap_buy_instruction<'a>(
    program_id: &'a Pubkey,
    pool: &'a Pubkey,
    user: &'a Pubkey,
    global_config: &'a Pubkey,
    base_mint: &'a Pubkey,
    quote_mint: &'a Pubkey,
    user_base_token_account: &'a Pubkey,
    user_quote_token_account: &'a Pubkey,
    pool_base_token_account: &'a Pubkey,
    pool_quote_token_account: &'a Pubkey,
    protocol_fee_recipient: &'a Pubkey,
    protocol_fee_recipient_token_account: &'a Pubkey,
    base_token_program: &'a Pubkey,
    quote_token_program: &'a Pubkey,
    event_authority: &'a Pubkey,
    base_amount_out: u64,
    maximum_quote_amount_in: u64,
) -> Instruction {
    let mut data = Vec::<u8>::with_capacity(24);
    // From IDL (https://www.npmjs.com/package/@pump-fun/pump-swap-sdk)
    const DESCRIMINATOR: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];
    data.extend(DESCRIMINATOR);
    data.extend(base_amount_out.to_le_bytes());
    data.extend(maximum_quote_amount_in.to_le_bytes());
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*pool, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*global_config, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(*user_base_token_account, false),
            AccountMeta::new(*user_quote_token_account, false),
            AccountMeta::new(*pool_base_token_account, false),
            AccountMeta::new(*pool_quote_token_account, false),
            AccountMeta::new_readonly(*protocol_fee_recipient, false),
            AccountMeta::new(*protocol_fee_recipient_token_account, false),
            AccountMeta::new_readonly(*base_token_program, false),
            AccountMeta::new_readonly(*quote_token_program, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
            AccountMeta::new_readonly(SPL_ASSOCIATED_TOKEN_ACCOUNT_ID, false),
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(*program_id, false),
        ],
        data,
    }
}
pub fn create_pumpswap_sell_instruction<'a>(
    program_id: &'a Pubkey,
    pool: &'a Pubkey,
    user: &'a Pubkey,
    global_config: &'a Pubkey,
    base_mint: &'a Pubkey,
    quote_mint: &'a Pubkey,
    user_base_token_account: &'a Pubkey,
    user_quote_token_account: &'a Pubkey,
    pool_base_token_account: &'a Pubkey,
    pool_quote_token_account: &'a Pubkey,
    protocol_fee_recipient: &'a Pubkey,
    protocol_fee_recipient_token_account: &'a Pubkey,
    base_token_program: &'a Pubkey,
    quote_token_program: &'a Pubkey,
    event_authority: &'a Pubkey,
    base_amount_in: u64,
    min_quote_amount_out: u64,
) -> Instruction {
    let mut data = Vec::<u8>::with_capacity(24);
    // From IDL (https://www.npmjs.com/package/@pump-fun/pump-swap-sdk)
    const DESCRIMINATOR: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];
    data.extend(DESCRIMINATOR);
    data.extend(base_amount_in.to_le_bytes());
    data.extend(min_quote_amount_out.to_le_bytes());
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*pool, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*global_config, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(*user_base_token_account, false),
            AccountMeta::new(*user_quote_token_account, false),
            AccountMeta::new(*pool_base_token_account, false),
            AccountMeta::new(*pool_quote_token_account, false),
            AccountMeta::new_readonly(*protocol_fee_recipient, false),
            AccountMeta::new(*protocol_fee_recipient_token_account, false),
            AccountMeta::new_readonly(*base_token_program, false),
            AccountMeta::new_readonly(*quote_token_program, false),
            AccountMeta::new_readonly(solana_program::system_program::ID, false),
            AccountMeta::new_readonly(SPL_ASSOCIATED_TOKEN_ACCOUNT_ID, false),
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(*program_id, false),
        ],
        data,
    }
}