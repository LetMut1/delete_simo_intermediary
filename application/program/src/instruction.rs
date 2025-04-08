use {
    crate::SPL_ASSOCIATED_TOKEN_ACCOUNT_ID, solana_program::{
        instruction::{
            AccountMeta,
            Instruction as Instruction_,
        },
        pubkey::Pubkey,
    }, std::error::Error
};
#[repr(C)]
#[derive(Debug, borsh::BorshSerialize, borsh::BorshDeserialize)]
pub enum Instruction {
    Initialize {
        lamports_to_treasury: u64,
        w_sol_token_account_pubkey_bump_seed: u8,
        w_sol_token_account_authority_pubkey_bump_seed: u8,
        temporary_w_sol_token_account_pubkey_bump_seed: u8,
        temporary_w_sol_token_account_authority_pubkey_bump_seed: u8,
    },
    DepositFunds {
        lamports_to_treasury: u64,
    },
    WithdrawFunds {
        lamports_from_treasury: u64,
    },
    BuyOnRaydium {
        input_token_amount: u64,
        minimum_output_token_amount: u64,
        any_mint_token_account_pubkey_bump_seed: u8,
        any_mint_token_account_authority_pubkey_bump_seed: u8,
    },
    SellOnRaydium {
        input_token_amount: u64,
        minimum_output_token_amount: Option<u64>,
        any_mint_token_account_pubkey_bump_seed: u8,
        any_mint_token_account_authority_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_authority_pubkey_bump_seed: u8,
    },
    BuyOnPumpswap {
        base_amount_out: u64,
        maximum_quote_amount_in: u64,
        any_mint_token_account_pubkey_bump_seed: u8,
    },
    SellOnPumpswap {
        input_token_amount: u64,
        minimum_output_token_amount: Option<u64>,
        any_mint_token_account_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_pubkey_bump_seed: u8,
    },
    ChangeManager,
    ChangeTrader,
}
impl Instruction {
    pub fn initialize<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_investor_pubkey: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        intermediary_manager_pubkey: &'a Pubkey,
        intermediary_trader_pubkey: &'a Pubkey,
        w_sol_token_account_pubkey: &'a Pubkey,
        w_sol_token_account_authority_pubkey: &'a Pubkey,
        temporary_w_sol_token_account_pubkey: &'a Pubkey,
        temporary_w_sol_token_account_authority_pubkey: &'a Pubkey,
        lamports_to_treasury: u64,
        w_sol_token_account_pubkey_bump_seed: u8,
        w_sol_token_account_authority_pubkey_bump_seed: u8,
        temporary_w_sol_token_account_pubkey_bump_seed: u8,
        temporary_w_sol_token_account_authority_pubkey_bump_seed: u8,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new_readonly(*intermediary_investor_pubkey, true),
                    AccountMeta::new(*intermediary_pubkey, true),
                    AccountMeta::new_readonly(*intermediary_manager_pubkey, false),
                    AccountMeta::new_readonly(*intermediary_trader_pubkey, false),
                    AccountMeta::new(*w_sol_token_account_pubkey, false),
                    AccountMeta::new_readonly(*w_sol_token_account_authority_pubkey, false),
                    AccountMeta::new_readonly(*temporary_w_sol_token_account_pubkey, false),
                    AccountMeta::new_readonly(*temporary_w_sol_token_account_authority_pubkey, false),
                    AccountMeta::new_readonly(spl_token::native_mint::ID, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                    AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
                    AccountMeta::new_readonly(spl_token::ID, false),
                ],
                data: borsh::to_vec(
                    &Self::Initialize {
                        lamports_to_treasury,
                        w_sol_token_account_pubkey_bump_seed,
                        w_sol_token_account_authority_pubkey_bump_seed,
                        temporary_w_sol_token_account_pubkey_bump_seed,
                        temporary_w_sol_token_account_authority_pubkey_bump_seed,
                    },
                )?,
            },
        )
    }
    pub fn deposit_funds<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_investor_pubkey: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        w_sol_token_account_pubkey: &'a Pubkey,
        lamports_to_treasury: u64,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new(*intermediary_investor_pubkey, true),
                    AccountMeta::new_readonly(*intermediary_pubkey, false),
                    AccountMeta::new(*w_sol_token_account_pubkey, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                    AccountMeta::new_readonly(spl_token::ID, false),
                ],
                data: borsh::to_vec(
                    &Self::DepositFunds {
                        lamports_to_treasury,
                    },
                )?,
            },
        )
    }
    pub fn withdraw_funds<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_investor_pubkey: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        w_sol_token_account_pubkey: &'a Pubkey,
        w_sol_token_account_authority_pubkey: &'a Pubkey,
        temporary_w_sol_token_account_pubkey: &'a Pubkey,
        temporary_w_sol_token_account_authority_pubkey: &'a Pubkey,
        lamports_from_treasury: u64,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new(*intermediary_investor_pubkey, true),
                    AccountMeta::new_readonly(*intermediary_pubkey, false),
                    AccountMeta::new(*w_sol_token_account_pubkey, false),
                    AccountMeta::new_readonly(*w_sol_token_account_authority_pubkey, false),
                    AccountMeta::new(*temporary_w_sol_token_account_pubkey, false),
                    AccountMeta::new_readonly(*temporary_w_sol_token_account_authority_pubkey, false),
                    AccountMeta::new_readonly(spl_token::native_mint::ID, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                    AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
                    AccountMeta::new_readonly(spl_token::ID, false),
                ],
                data: borsh::to_vec(
                    &Self::WithdrawFunds {
                        lamports_from_treasury,
                    },
                )?,
            },
        )
    }
    pub fn buy_on_raydium<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_trader_pubkey: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        w_sol_token_account_pubkey: &'a Pubkey,
        w_sol_token_account_authority_pubkey: &'a Pubkey,
        temporary_w_sol_token_account_pubkey: &'a Pubkey,
        temporary_w_sol_token_account_authority_pubkey: &'a Pubkey,
        any_mint_token_account_pubkey: &'a Pubkey,
        any_mint_token_account_authority_pubkey: &'a Pubkey,
        any_mint_token_mint_account_pubkey: &'a Pubkey,
        amm_programm_id_pubkey: &'a Pubkey,
        amm_pool_pubkey: &'a Pubkey,
        amm_authority_pubkey: &'a Pubkey,
        amm_open_orders_pubkey: &'a Pubkey,
        amm_coin_vault_pubkey: &'a Pubkey,
        amm_pc_vault_pubkey: &'a Pubkey,
        market_program_id_pubkey: &'a Pubkey,
        market_pubkey: &'a Pubkey,
        market_bids_pubkey: &'a Pubkey,
        market_asks_pubkey: &'a Pubkey,
        market_event_queue_pubkey: &'a Pubkey,
        market_coin_vault_pubkey: &'a Pubkey,
        market_pc_vault_pubkey: &'a Pubkey,
        market_vault_signer_pubkey: &'a Pubkey,
        input_token_amount: u64,
        minimum_output_token_amount: u64,
        any_mint_token_account_pubkey_bump_seed: u8,
        any_mint_token_account_authority_pubkey_bump_seed: u8,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new(*intermediary_trader_pubkey, true),
                    AccountMeta::new_readonly(*intermediary_pubkey, false),
                    AccountMeta::new(*w_sol_token_account_pubkey, false),
                    AccountMeta::new_readonly(*w_sol_token_account_authority_pubkey, false),
                    AccountMeta::new(*temporary_w_sol_token_account_pubkey, false),
                    AccountMeta::new_readonly(*temporary_w_sol_token_account_authority_pubkey, false),
                    AccountMeta::new(*any_mint_token_account_pubkey, false),
                    AccountMeta::new_readonly(*any_mint_token_account_authority_pubkey, false),
                    AccountMeta::new_readonly(spl_token::native_mint::ID, false),
                    AccountMeta::new_readonly(*any_mint_token_mint_account_pubkey, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                    AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
                    AccountMeta::new_readonly(spl_token::ID, false),
                    AccountMeta::new_readonly(*amm_programm_id_pubkey, false),
                    AccountMeta::new(*amm_pool_pubkey, false),
                    AccountMeta::new_readonly(*amm_authority_pubkey, false),
                    AccountMeta::new(*amm_open_orders_pubkey, false),
                    AccountMeta::new(*amm_coin_vault_pubkey, false),
                    AccountMeta::new(*amm_pc_vault_pubkey, false),
                    AccountMeta::new_readonly(*market_program_id_pubkey, false),
                    AccountMeta::new(*market_pubkey, false),
                    AccountMeta::new(*market_bids_pubkey, false),
                    AccountMeta::new(*market_asks_pubkey, false),
                    AccountMeta::new(*market_event_queue_pubkey, false),
                    AccountMeta::new(*market_coin_vault_pubkey, false),
                    AccountMeta::new(*market_pc_vault_pubkey, false),
                    AccountMeta::new_readonly(*market_vault_signer_pubkey, false),
                ],
                data: borsh::to_vec(
                    &Self::BuyOnRaydium {
                        input_token_amount,
                        minimum_output_token_amount,
                        any_mint_token_account_pubkey_bump_seed,
                        any_mint_token_account_authority_pubkey_bump_seed,
                    },
                )?,
            },
        )
    }
    pub fn sell_on_raydium<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_trader_pubkey: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        w_sol_token_account_pubkey: &'a Pubkey,
        any_mint_token_account_pubkey: &'a Pubkey,
        any_mint_token_account_authority_pubkey: &'a Pubkey,
        temporary_any_mint_token_account_pubkey: &'a Pubkey,
        temporary_any_mint_token_account_authority_pubkey: &'a Pubkey,
        any_mint_token_mint_account_pubkey: &'a Pubkey,
        amm_programm_id_pubkey: &'a Pubkey,
        amm_pool_pubkey: &'a Pubkey,
        amm_authority_pubkey: &'a Pubkey,
        amm_open_orders_pubkey: &'a Pubkey,
        amm_coin_vault_pubkey: &'a Pubkey,
        amm_pc_vault_pubkey: &'a Pubkey,
        market_program_id_pubkey: &'a Pubkey,
        market_pubkey: &'a Pubkey,
        market_bids_pubkey: &'a Pubkey,
        market_asks_pubkey: &'a Pubkey,
        market_event_queue_pubkey: &'a Pubkey,
        market_coin_vault_pubkey: &'a Pubkey,
        market_pc_vault_pubkey: &'a Pubkey,
        market_vault_signer_pubkey: &'a Pubkey,
        input_token_amount: u64,
        minimum_output_token_amount: Option<u64>,
        any_mint_token_account_pubkey_bump_seed: u8,
        any_mint_token_account_authority_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_authority_pubkey_bump_seed: u8,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new(*intermediary_trader_pubkey, true),
                    AccountMeta::new_readonly(*intermediary_pubkey, false),
                    AccountMeta::new(*w_sol_token_account_pubkey, false),
                    AccountMeta::new(*any_mint_token_account_pubkey, false),
                    AccountMeta::new_readonly(*any_mint_token_account_authority_pubkey, false),
                    AccountMeta::new(*temporary_any_mint_token_account_pubkey, false),
                    AccountMeta::new_readonly(*temporary_any_mint_token_account_authority_pubkey, false),
                    AccountMeta::new_readonly(*any_mint_token_mint_account_pubkey, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                    AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
                    AccountMeta::new_readonly(spl_token::ID, false),
                    AccountMeta::new_readonly(*amm_programm_id_pubkey, false),
                    AccountMeta::new(*amm_pool_pubkey, false),
                    AccountMeta::new_readonly(*amm_authority_pubkey, false),
                    AccountMeta::new(*amm_open_orders_pubkey, false),
                    AccountMeta::new(*amm_coin_vault_pubkey, false),
                    AccountMeta::new(*amm_pc_vault_pubkey, false),
                    AccountMeta::new_readonly(*market_program_id_pubkey, false),
                    AccountMeta::new(*market_pubkey, false),
                    AccountMeta::new(*market_bids_pubkey, false),
                    AccountMeta::new(*market_asks_pubkey, false),
                    AccountMeta::new(*market_event_queue_pubkey, false),
                    AccountMeta::new(*market_coin_vault_pubkey, false),
                    AccountMeta::new(*market_pc_vault_pubkey, false),
                    AccountMeta::new_readonly(*market_vault_signer_pubkey, false),
                ],
                data: borsh::to_vec(
                    &Self::SellOnRaydium {
                        input_token_amount,
                        minimum_output_token_amount,
                        any_mint_token_account_pubkey_bump_seed,
                        any_mint_token_account_authority_pubkey_bump_seed,
                        temporary_any_mint_token_account_pubkey_bump_seed,
                        temporary_any_mint_token_account_authority_pubkey_bump_seed,
                    },
                )?,
            },
        )
    }
    pub fn buy_on_pumpswap<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_trader_pubkey: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        w_sol_token_account_pubkey: &'a Pubkey,
        w_sol_token_account_authority_pubkey: &'a Pubkey,
        temporary_w_sol_token_account_pubkey: &'a Pubkey,
        any_mint_token_account_pubkey: &'a Pubkey,
        any_mint_token_mint_account_pubkey: &'a Pubkey,
        pumpfan_program_id_pubkey: &'a Pubkey,
        pool_pubkey: &'a Pubkey,
        global_config_pubkey: &'a Pubkey,
        pool_base_token_account_pubkey: &'a Pubkey,
        pool_quote_token_account_pubkey: &'a Pubkey,
        protocol_fee_recipient_pubkey: &'a Pubkey,
        protocol_fee_recipient_token_account_pubkey: &'a Pubkey,
        base_token_program_pubkey: &'a Pubkey,
        quote_token_program_pubkey: &'a Pubkey,
        event_authority_pubkey: &'a Pubkey,
        base_amount_out: u64,
        maximum_quote_amount_in: u64,
        any_mint_token_account_pubkey_bump_seed: u8,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new(*intermediary_trader_pubkey, true),
                    AccountMeta::new_readonly(*intermediary_pubkey, false),
                    AccountMeta::new(*w_sol_token_account_pubkey, false),
                    AccountMeta::new_readonly(*w_sol_token_account_authority_pubkey, false),
                    AccountMeta::new(*temporary_w_sol_token_account_pubkey, false),
                    AccountMeta::new(*any_mint_token_account_pubkey, false),
                    AccountMeta::new_readonly(spl_token::native_mint::ID, false),
                    AccountMeta::new_readonly(*any_mint_token_mint_account_pubkey, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                    AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
                    AccountMeta::new_readonly(spl_token::ID, false),
                    AccountMeta::new_readonly(*pumpfan_program_id_pubkey, false),
                    AccountMeta::new_readonly(*pool_pubkey, false),
                    AccountMeta::new_readonly(*global_config_pubkey, false),
                    AccountMeta::new(*pool_base_token_account_pubkey, false),
                    AccountMeta::new(*pool_quote_token_account_pubkey, false),
                    AccountMeta::new_readonly(*protocol_fee_recipient_pubkey, false),
                    AccountMeta::new(*protocol_fee_recipient_token_account_pubkey, false),
                    AccountMeta::new_readonly(*base_token_program_pubkey, false),
                    AccountMeta::new_readonly(*quote_token_program_pubkey, false),
                    AccountMeta::new_readonly(SPL_ASSOCIATED_TOKEN_ACCOUNT_ID, false),
                    AccountMeta::new_readonly(*event_authority_pubkey, false),
                ],
                data: borsh::to_vec(
                    &Self::BuyOnPumpswap {
                        base_amount_out,
                        maximum_quote_amount_in,
                        any_mint_token_account_pubkey_bump_seed,
                    },
                )?,
            },
        )
    }
    pub fn sell_on_pumpswap<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_trader_pubkey: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        w_sol_token_account_pubkey: &'a Pubkey,
        temporary_w_sol_token_account_pubkey: &'a Pubkey,
        any_mint_token_account_pubkey: &'a Pubkey,
        temporary_any_mint_token_account_pubkey: &'a Pubkey,
        any_mint_token_mint_account_pubkey: &'a Pubkey,
        pumpfan_program_id_pubkey: &'a Pubkey,
        pool_pubkey: &'a Pubkey,
        global_config_pubkey: &'a Pubkey,
        pool_base_token_account_pubkey: &'a Pubkey,
        pool_quote_token_account_pubkey: &'a Pubkey,
        protocol_fee_recipient_pubkey: &'a Pubkey,
        protocol_fee_recipient_token_account_pubkey: &'a Pubkey,
        base_token_program_pubkey: &'a Pubkey,
        quote_token_program_pubkey: &'a Pubkey,
        event_authority_pubkey: &'a Pubkey,
        input_token_amount: u64,
        minimum_output_token_amount: Option<u64>,
        any_mint_token_account_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_pubkey_bump_seed: u8,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new(*intermediary_trader_pubkey, true),
                    AccountMeta::new_readonly(*intermediary_pubkey, false),
                    AccountMeta::new(*w_sol_token_account_pubkey, false),
                    AccountMeta::new(*temporary_w_sol_token_account_pubkey, false),
                    AccountMeta::new(*any_mint_token_account_pubkey, false),
                    AccountMeta::new(*temporary_any_mint_token_account_pubkey, false),
                    AccountMeta::new_readonly(spl_token::native_mint::ID, false),
                    AccountMeta::new_readonly(*any_mint_token_mint_account_pubkey, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                    AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false),
                    AccountMeta::new_readonly(spl_token::ID, false),
                    AccountMeta::new_readonly(*pumpfan_program_id_pubkey, false),
                    AccountMeta::new_readonly(*pool_pubkey, false),
                    AccountMeta::new_readonly(*global_config_pubkey, false),
                    AccountMeta::new(*pool_base_token_account_pubkey, false),
                    AccountMeta::new(*pool_quote_token_account_pubkey, false),
                    AccountMeta::new_readonly(*protocol_fee_recipient_pubkey, false),
                    AccountMeta::new(*protocol_fee_recipient_token_account_pubkey, false),
                    AccountMeta::new_readonly(*base_token_program_pubkey, false),
                    AccountMeta::new_readonly(*quote_token_program_pubkey, false),
                    AccountMeta::new_readonly(SPL_ASSOCIATED_TOKEN_ACCOUNT_ID, false),
                    AccountMeta::new_readonly(*event_authority_pubkey, false),
                ],
                data: borsh::to_vec(
                    &Self::SellOnPumpswap {
                        input_token_amount,
                        minimum_output_token_amount,
                        any_mint_token_account_pubkey_bump_seed,
                        temporary_any_mint_token_account_pubkey_bump_seed,
                    },
                )?,
            },
        )
    }
    pub fn change_manager<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_investor_pubkey: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        intermediary_manager_pubkey: &'a Pubkey,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new_readonly(*intermediary_investor_pubkey, true),
                    AccountMeta::new(*intermediary_pubkey, false),
                    AccountMeta::new_readonly(*intermediary_manager_pubkey, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                ],
                data: borsh::to_vec(&Self::ChangeManager)?,
            },
        )
    }
    pub fn change_trader<'a>(
        program_id_pubkey: &'a Pubkey,
        intermediary_manager: &'a Pubkey,
        intermediary_pubkey: &'a Pubkey,
        intermediary_trader_pubkey: &'a Pubkey,
    ) -> Result<Instruction_, Box<dyn Error + 'static>> {
        Ok(
            Instruction_ {
                program_id: *program_id_pubkey,
                accounts: vec![
                    AccountMeta::new_readonly(*intermediary_manager, true),
                    AccountMeta::new(*intermediary_pubkey, false),
                    AccountMeta::new_readonly(*intermediary_trader_pubkey, false),
                    AccountMeta::new_readonly(solana_program::system_program::ID, false),
                ],
                data: borsh::to_vec(&Self::ChangeTrader)?,
            },
        )
    }
}
