use {
    crate::{
        error::Error, instruction::Instruction, state::Intermediary, AnyMintTokenAccountAddressResolver, AnyMintTokenAccountAuthorityAddressResolver, ProgramDerivedAddress, TemporaryAnyMintTokenAccountAddressResolver, TemporaryAnyMintTokenAccountAuthorityAddressResolver, TemporaryWSolTokenAccountAddressResolver, TemporaryWSolTokenAccountAuthorityAddressResolver, WSolTokenAccountAddressResolver, WSolTokenAccountAuthorityAddressResolver, FEE_BUY_ON_PUMPSWAP, FEE_BUY_ON_RAYDIUM, FEE_CHANGE_MANAGER, FEE_CHANGE_TRADER, FEE_DEPOSIT_FUNDS, FEE_INITIALIZE, FEE_SELL_ON_PUMPSWAP, FEE_SELL_ON_RAYDIUM, FEE_WITHDRAW_FUNDS, PROGRAM_PUBKEY, PUMPSWAP_PROGRAM_PUBKEY, RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_PUBKEY, SPL_ASSOCIATED_TOKEN_ACCOUNT_ID
    },
    borsh::BorshDeserialize,
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
    },
    spl_token::state::Account,
    std::collections::HashSet,
};
pub struct Processor;
impl Processor {
    pub fn process<'a>(program_id: &'a Pubkey, accounts: &'a [AccountInfo], input: &'a [u8]) -> ProgramResult {
        match Instruction::try_from_slice(input)? {
            Instruction::Initialize {
                lamports_to_treasury,
                w_sol_token_account_pubkey_bump_seed,
                w_sol_token_account_authority_pubkey_bump_seed,
                temporary_w_sol_token_account_pubkey_bump_seed,
                temporary_w_sol_token_account_authority_pubkey_bump_seed,
            } => {
                Self::initialize(
                    program_id,
                    accounts,
                    lamports_to_treasury,
                    w_sol_token_account_pubkey_bump_seed,
                    w_sol_token_account_authority_pubkey_bump_seed,
                    temporary_w_sol_token_account_pubkey_bump_seed,
                    temporary_w_sol_token_account_authority_pubkey_bump_seed,
                )
            }
            Instruction::DepositFunds {
                lamports_to_treasury,
            } => Self::deposit_funds(
                program_id,
                accounts,
                lamports_to_treasury,
            ),
            Instruction::WithdrawFunds {
                lamports_from_treasury,
            } => Self::withdraw_funds(
                program_id,
                accounts,
                lamports_from_treasury,
            ),
            Instruction::BuyOnRaydium {
                input_token_amount,
                minimum_output_token_amount,
                any_mint_token_account_pubkey_bump_seed,
                any_mint_token_account_authority_pubkey_bump_seed,
            } => Self::buy_on_raydium(
                program_id,
                accounts,
                input_token_amount,
                minimum_output_token_amount,
                any_mint_token_account_pubkey_bump_seed,
                any_mint_token_account_authority_pubkey_bump_seed,
            ),
            Instruction::SellOnRaydium {
                input_token_amount,
                minimum_output_token_amount,
                any_mint_token_account_pubkey_bump_seed,
                any_mint_token_account_authority_pubkey_bump_seed,
                temporary_any_mint_token_account_pubkey_bump_seed,
                temporary_any_mint_token_account_authority_pubkey_bump_seed,
            } => Self::sell_on_raydium(
                program_id,
                accounts,
                input_token_amount,
                minimum_output_token_amount,
                any_mint_token_account_pubkey_bump_seed,
                any_mint_token_account_authority_pubkey_bump_seed,
                temporary_any_mint_token_account_pubkey_bump_seed,
                temporary_any_mint_token_account_authority_pubkey_bump_seed,
            ),
            Instruction::BuyOnPumpswap {
                base_amount_out,
                maximum_quote_amount_in,
                any_mint_token_account_pubkey_bump_seed
            } => Self::buy_on_pumpswap(
                program_id,
                accounts,
                base_amount_out,
                maximum_quote_amount_in,
                any_mint_token_account_pubkey_bump_seed,
            ),
            Instruction::SellOnPumpswap {
                input_token_amount,
                minimum_output_token_amount,
                any_mint_token_account_pubkey_bump_seed,
                temporary_any_mint_token_account_pubkey_bump_seed,
            } => Self::sell_on_pumpswap(
                program_id,
                accounts,
                input_token_amount,
                minimum_output_token_amount,
                any_mint_token_account_pubkey_bump_seed,
                temporary_any_mint_token_account_pubkey_bump_seed,
            ),
            Instruction::ChangeManager => Self::change_manager(
                program_id,
                accounts,
            ),
            Instruction::ChangeTrader => Self::change_trader(
                program_id,
                accounts,
            ),
        }
    }
    fn initialize<'a>(
        _program_id: &'a Pubkey,
        accounts: &'a [AccountInfo],
        lamports_to_treasury: u64,
        w_sol_token_account_pubkey_bump_seed: u8,
        w_sol_token_account_authority_pubkey_bump_seed: u8,
        temporary_w_sol_token_account_pubkey_bump_seed: u8,
        temporary_w_sol_token_account_authority_pubkey_bump_seed: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_investor_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_manager_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_trader_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_authority_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_w_sol_token_account_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let rent_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_address_resolver = WSolTokenAccountAddressResolver {
            intermediary_pubkey: intermediary_account_info.key,
        };
        let w_sol_token_account_authority_address_resolver = WSolTokenAccountAuthorityAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let temporary_w_sol_token_account_address_resolver = TemporaryWSolTokenAccountAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
        };
        let temporary_w_sol_token_account_authority_address_resolver = TemporaryWSolTokenAccountAuthorityAddressResolver {
            temporary_w_sol_token_account_pubkey: temporary_w_sol_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let mut account_differentiator = HashSet::<&'_ Pubkey>::with_capacity(12);
        if !account_differentiator.insert(intermediary_investor_account_info.key)
            || !account_differentiator.insert(intermediary_account_info.key)
            || !account_differentiator.insert(intermediary_manager_account_info.key)
            || !account_differentiator.insert(intermediary_trader_account_info.key)
            || !account_differentiator.insert(w_sol_token_account_account_info.key)
            || !account_differentiator.insert(w_sol_token_account_authority_info.key)
            || !account_differentiator.insert(temporary_w_sol_token_account_account_info.key)
            || !account_differentiator.insert(temporary_w_sol_token_account_authority_account_info.key)
            || !account_differentiator.insert(w_sol_token_mint_account_info.key)
            || !account_differentiator.insert(system_program_account_info.key)
            || !account_differentiator.insert(rent_account_info.key)
            || !account_differentiator.insert(token_program_account_info.key)
            || *w_sol_token_account_account_info.key != w_sol_token_account_address_resolver.create(w_sol_token_account_pubkey_bump_seed)?
            || *w_sol_token_account_authority_info.key != w_sol_token_account_authority_address_resolver.create(w_sol_token_account_authority_pubkey_bump_seed)?
            || *temporary_w_sol_token_account_account_info.key != temporary_w_sol_token_account_address_resolver.create(temporary_w_sol_token_account_pubkey_bump_seed)?
            || *temporary_w_sol_token_account_authority_account_info.key
                != temporary_w_sol_token_account_authority_address_resolver.create(temporary_w_sol_token_account_authority_pubkey_bump_seed)?
            || *w_sol_token_mint_account_info.key != spl_token::native_mint::ID
            || *system_program_account_info.key != solana_program::system_program::ID
            || *rent_account_info.key != solana_program::sysvar::rent::ID
            || *token_program_account_info.key != spl_token::ID
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_investor_account_info.is_writable
            || !intermediary_investor_account_info.is_signer
            || !intermediary_account_info.is_writable
            || !intermediary_account_info.is_signer
            || !w_sol_token_account_account_info.is_writable
        {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        if intermediary_investor_account_info.owner != system_program_account_info.key
            || intermediary_manager_account_info.owner != system_program_account_info.key
            || intermediary_trader_account_info.owner != system_program_account_info.key
        {
            return Err(Error::InvalidAccountOwner.into());
        }
        if !intermediary_investor_account_info.data_is_empty()
            || !intermediary_account_info.data_is_empty()
            || !intermediary_manager_account_info.data_is_empty()
            || !intermediary_trader_account_info.data_is_empty()
            || !w_sol_token_account_account_info.data_is_empty()
        {
            return Err(Error::InvalidAccountData.into());
        }
        let intermediary = Intermediary {
            is_initialized: true,
            investor_pubkey: *intermediary_investor_account_info.key,
            manager_pubkey: *intermediary_manager_account_info.key,
            trader_pubkey: *intermediary_trader_account_info.key,
            w_sol_token_account_pubkey: *w_sol_token_account_account_info.key,
            w_sol_token_account_authority_pubkey: *w_sol_token_account_authority_info.key,
            temporary_w_sol_token_account_pubkey: *temporary_w_sol_token_account_account_info.key,
            temporary_w_sol_token_account_authority_pubkey: *temporary_w_sol_token_account_authority_account_info.key,
            w_sol_token_account_authority_pubkey_bump_seed,
            temporary_w_sol_token_account_pubkey_bump_seed,
            temporary_w_sol_token_account_authority_pubkey_bump_seed,
        };
        let intermediary_object_length = borsh::object_length(&intermediary)?;
        let rent = Rent::from_account_info(rent_account_info)?;
        let intermediary_rent_exemption_balance = rent.minimum_balance(intermediary_object_length);
        let token_account_rent_exemption_balance = rent.minimum_balance(<Account as Pack>::LEN);
        if intermediary_investor_account_info.lamports() < (intermediary_rent_exemption_balance + token_account_rent_exemption_balance + lamports_to_treasury + FEE_INITIALIZE)       // TODO все просчитать
        || intermediary_manager_account_info.lamports() == 0
        || intermediary_trader_account_info.lamports() == 0
        {
            return Err(Error::InvalidAccountLamports.into());
        }
        solana_program::program::invoke(
            &solana_program::system_instruction::create_account(
                intermediary_investor_account_info.key,
                intermediary_account_info.key,
                intermediary_rent_exemption_balance,
                intermediary_object_length as u64,
                &PROGRAM_PUBKEY,
            ),
            vec![
                intermediary_investor_account_info.clone(),
                intermediary_account_info.clone(),
            ]
            .as_slice(),
        )?;
        borsh::to_writer(
            &mut intermediary_account_info.data.borrow_mut()[..],
            &intermediary,
        )?;
        let mut w_sol_token_account_pubkey_seeds = w_sol_token_account_address_resolver.get_seeds();
        let w_sol_token_account_pubkey_bump_seed_ = [w_sol_token_account_pubkey_bump_seed];
        w_sol_token_account_pubkey_seeds.push(w_sol_token_account_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                intermediary_investor_account_info.key,
                w_sol_token_account_account_info.key,
                token_account_rent_exemption_balance + lamports_to_treasury,
                <Account as Pack>::LEN as u64,
                token_program_account_info.key,
            ),
            vec![
                intermediary_investor_account_info.clone(),
                w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
            [w_sol_token_account_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_account(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
                w_sol_token_mint_account_info.key,
                w_sol_token_account_authority_info.key,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
                w_sol_token_mint_account_info.clone(),
                w_sol_token_account_authority_info.clone(),
                rent_account_info.clone(),
            ]
            .as_slice(),
        )?;
        let w_sol_token_account = Account::unpack_unchecked(&w_sol_token_account_account_info.data.borrow())?;
        if !w_sol_token_account.is_native.is_some() {
            return Err(Error::TokenAccountInvalidMint.into());
        }
        if w_sol_token_account.amount != lamports_to_treasury {
            return Err(Error::TokenAccountInvalidAmount.into());
        }
        Ok(())
    }
    fn deposit_funds<'a>(_program_id: &'a Pubkey, accounts: &'a [AccountInfo], lamports_to_treasury: u64) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_investor_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let mut account_differentiator = HashSet::<&'_ Pubkey>::with_capacity(5);
        if !account_differentiator.insert(intermediary_investor_account_info.key)
            || !account_differentiator.insert(intermediary_account_info.key)
            || !account_differentiator.insert(w_sol_token_account_account_info.key)
            || !account_differentiator.insert(system_program_account_info.key)
            || !account_differentiator.insert(token_program_account_info.key)
            || *system_program_account_info.key != solana_program::system_program::ID
            || *token_program_account_info.key != spl_token::ID
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_investor_account_info.is_writable || !intermediary_investor_account_info.is_signer || !w_sol_token_account_account_info.is_writable {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        if intermediary_investor_account_info.lamports() < (lamports_to_treasury + FEE_DEPOSIT_FUNDS) {
            return Err(Error::InvalidAccountLamports.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(&intermediary_account_info.data.borrow())?;
        if !intermediary.is_initialized {
            return Err(Error::IntermediaryIsNotInitialized.into());
        }
        if *intermediary_investor_account_info.key != intermediary.investor_pubkey {
            return Err(Error::IntermediaryInvalidInvestor.into());
        }
        if *w_sol_token_account_account_info.key != intermediary.w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccount.into());
        }
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
                intermediary_investor_account_info.key,
                w_sol_token_account_account_info.key,
                lamports_to_treasury,
            ),
            vec![
                intermediary_investor_account_info.clone(),
                w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::sync_native(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
        )?;
        Ok(())
    }
    fn withdraw_funds<'a>(_program_id: &'a Pubkey, accounts: &'a [AccountInfo], lamports_from_treasury: u64) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_investor_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_authority_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_w_sol_token_account_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let rent_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let mut account_differentiator = HashSet::<&'_ Pubkey>::with_capacity(10);
        if !account_differentiator.insert(intermediary_investor_account_info.key)
            || !account_differentiator.insert(intermediary_account_info.key)
            || !account_differentiator.insert(w_sol_token_account_account_info.key)
            || !account_differentiator.insert(w_sol_token_account_authority_info.key)
            || !account_differentiator.insert(temporary_w_sol_token_account_account_info.key)
            || !account_differentiator.insert(temporary_w_sol_token_account_authority_account_info.key)
            || !account_differentiator.insert(w_sol_token_mint_account_info.key)
            || !account_differentiator.insert(system_program_account_info.key)
            || !account_differentiator.insert(rent_account_info.key)
            || !account_differentiator.insert(token_program_account_info.key)
            || *w_sol_token_mint_account_info.key != spl_token::native_mint::ID
            || *system_program_account_info.key != solana_program::system_program::ID
            || *rent_account_info.key != solana_program::sysvar::rent::ID
            || *token_program_account_info.key != spl_token::ID
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_investor_account_info.is_writable
            || !intermediary_investor_account_info.is_signer
            || !w_sol_token_account_account_info.is_writable
            || !temporary_w_sol_token_account_account_info.is_writable
        {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(&intermediary_account_info.data.borrow())?;
        if !intermediary.is_initialized {
            return Err(Error::IntermediaryIsNotInitialized.into());
        }
        if *intermediary_investor_account_info.key != intermediary.investor_pubkey {
            return Err(Error::IntermediaryInvalidInvestor.into());
        }
        if *w_sol_token_account_account_info.key != intermediary.w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccount.into());
        }
        if *w_sol_token_account_authority_info.key != intermediary.w_sol_token_account_authority_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccountAuthority.into());
        }
        if *temporary_w_sol_token_account_account_info.key != intermediary.temporary_w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidTemporaryWSolTokenAccount.into());
        }
        if *temporary_w_sol_token_account_authority_account_info.key != intermediary.temporary_w_sol_token_account_authority_pubkey {
            return Err(Error::IntermediaryInvalidTemporaryWSolTokenAccountAuthority.into());
        }
        let w_sol_token_account = Account::unpack_unchecked(&w_sol_token_account_account_info.data.borrow())?;
        if lamports_from_treasury > w_sol_token_account.amount {
            return Err(Error::TokenAccountInsufficientAmount.into());
        }
        let rent = Rent::from_account_info(rent_account_info)?;
        let token_account_rent_exemption_balance = rent.minimum_balance(<Account as Pack>::LEN);
        if intermediary_investor_account_info.lamports() < (token_account_rent_exemption_balance + FEE_WITHDRAW_FUNDS) {
            return Err(Error::InvalidAccountLamports.into());
        }
        let temporary_w_sol_token_account_address_resolver = TemporaryWSolTokenAccountAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
        };
        let mut temporary_w_sol_token_account_pubkey_seeds = temporary_w_sol_token_account_address_resolver.get_seeds();
        let temporary_w_sol_token_account_pubkey_bump_seed_ = [intermediary.temporary_w_sol_token_account_pubkey_bump_seed];
        temporary_w_sol_token_account_pubkey_seeds.push(temporary_w_sol_token_account_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                intermediary_investor_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                token_account_rent_exemption_balance,
                <Account as Pack>::LEN as u64,
                token_program_account_info.key,
            ),
            vec![
                intermediary_investor_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
            [temporary_w_sol_token_account_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_account(
                token_program_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_mint_account_info.key,
                temporary_w_sol_token_account_authority_account_info.key,
            )?,
            vec![
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_mint_account_info.clone(),
                temporary_w_sol_token_account_authority_account_info.clone(),
                rent_account_info.clone(),
            ]
            .as_slice(),
        )?;
        let w_sol_token_account_authority_address_resolver = WSolTokenAccountAuthorityAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let mut w_sol_token_account_authority_pubkey_seeds = w_sol_token_account_authority_address_resolver.get_seeds();
        let w_sol_token_account_authority_pubkey_bump_seed_ = [intermediary.w_sol_token_account_authority_pubkey_bump_seed];
        w_sol_token_account_authority_pubkey_seeds.push(w_sol_token_account_authority_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &spl_token::instruction::transfer(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_account_authority_info.key,
                [].as_slice(),
                lamports_from_treasury,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_account_authority_info.clone(),
            ]
            .as_slice(),
            [w_sol_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
        )?;
        let temporary_w_sol_token_account_authority_address_resolver = TemporaryWSolTokenAccountAuthorityAddressResolver {
            temporary_w_sol_token_account_pubkey: temporary_w_sol_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let mut temporary_w_sol_token_account_authority_pubkey_seeds = temporary_w_sol_token_account_authority_address_resolver.get_seeds();
        let temporary_w_sol_token_account_authority_pubkey_bump_seed_ = [intermediary.temporary_w_sol_token_account_authority_pubkey_bump_seed];
        temporary_w_sol_token_account_authority_pubkey_seeds.push(temporary_w_sol_token_account_authority_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &spl_token::instruction::close_account(
                token_program_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                intermediary_investor_account_info.key,
                temporary_w_sol_token_account_authority_account_info.key,
                [].as_slice(),
            )?,
            vec![
                temporary_w_sol_token_account_account_info.clone(),
                intermediary_investor_account_info.clone(),
                temporary_w_sol_token_account_authority_account_info.clone(),
            ]
            .as_slice(),
            [temporary_w_sol_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
        )?;
        Ok(())
    }
    fn buy_on_raydium<'a>(
        _program_id: &'a Pubkey,
        accounts: &'a [AccountInfo],
        input_token_amount: u64,
        minimum_output_token_amount: u64,
        any_mint_token_account_pubkey_bump_seed: u8,
        any_mint_token_account_authority_pubkey_bump_seed: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_trader_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_w_sol_token_account_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let rent_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_program_id = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_pool_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_open_orders_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_coin_vault_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_pc_vault_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_program_id_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_bids_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_asks_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_event_queue_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_coin_vault_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_pc_vault_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_vault_signer_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_address_resolver = AnyMintTokenAccountAddressResolver {
            any_mint_token_mint_pubkey: any_mint_token_mint_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let any_mint_token_account_authority_address_resolver = AnyMintTokenAccountAuthorityAddressResolver {
            any_mint_token_account_pubkey: any_mint_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        if *any_mint_token_account_account_info.key != any_mint_token_account_address_resolver.create(any_mint_token_account_pubkey_bump_seed)?
            || *any_mint_token_account_authority_account_info.key != any_mint_token_account_authority_address_resolver.create(any_mint_token_account_authority_pubkey_bump_seed)?
            || w_sol_token_mint_account_info.key == any_mint_token_mint_account_info.key
            || *w_sol_token_mint_account_info.key != spl_token::native_mint::ID
            || *system_program_account_info.key != solana_program::system_program::ID
            || *rent_account_info.key != solana_program::sysvar::rent::ID
            || *token_program_account_info.key != spl_token::ID
            || *amm_program_id.key != RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_PUBKEY
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_trader_account_info.is_signer
            || !intermediary_trader_account_info.is_writable
            || !w_sol_token_account_account_info.is_writable
            || !temporary_w_sol_token_account_account_info.is_writable
            || !any_mint_token_account_account_info.is_writable
            || !amm_pool_account_info.is_writable
            || !amm_open_orders_account_info.is_writable
            || !amm_coin_vault_account_info.is_writable
            || !amm_pc_vault_account_info.is_writable
            || !market_account_info.is_writable
            || !market_bids_account_info.is_writable
            || !market_asks_account_info.is_writable
            || !market_event_queue_account_info.is_writable
            || !market_coin_vault_account_info.is_writable
            || !market_pc_vault_account_info.is_writable
        {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        let rent = Rent::from_account_info(rent_account_info)?;
        let token_account_rent_exemption_balance = rent.minimum_balance(<Account as Pack>::LEN);
        if intermediary_trader_account_info.lamports() < (2 * token_account_rent_exemption_balance + FEE_BUY_ON_RAYDIUM) {
            return Err(Error::InvalidAccountLamports.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(&intermediary_account_info.data.borrow())?;
        if !intermediary.is_initialized {
            return Err(Error::IntermediaryIsNotInitialized.into());
        }
        if *intermediary_trader_account_info.key != intermediary.trader_pubkey {
            return Err(Error::IntermediaryInvalidInvestor.into());
        }
        if *w_sol_token_account_account_info.key != intermediary.w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccount.into());
        }
        if *w_sol_token_account_authority_account_info.key != intermediary.w_sol_token_account_authority_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccountAuthority.into());
        }
        if *temporary_w_sol_token_account_account_info.key != intermediary.temporary_w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidTemporaryWSolTokenAccount.into());
        }
        if *temporary_w_sol_token_account_authority_account_info.key != intermediary.temporary_w_sol_token_account_authority_pubkey {
            return Err(Error::IntermediaryInvalidTemporaryWSolTokenAccountAuthority.into());
        }
        let w_sol_token_account = Account::unpack_unchecked(&w_sol_token_account_account_info.data.borrow())?;
        if input_token_amount > w_sol_token_account.amount {
            return Err(Error::TokenAccountInsufficientAmount.into());
        }
        let temporary_w_sol_token_account_address_resolver = TemporaryWSolTokenAccountAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
        };
        let mut temporary_w_sol_token_account_pubkey_seeds = temporary_w_sol_token_account_address_resolver.get_seeds();
        let temporary_w_sol_token_account_pubkey_bump_seed_ = [intermediary.temporary_w_sol_token_account_pubkey_bump_seed];
        temporary_w_sol_token_account_pubkey_seeds.push(temporary_w_sol_token_account_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                intermediary_trader_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                token_account_rent_exemption_balance,
                <Account as Pack>::LEN as u64,
                token_program_account_info.key,
            ),
            vec![
                intermediary_trader_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
            [temporary_w_sol_token_account_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_account(
                token_program_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_mint_account_info.key,
                temporary_w_sol_token_account_authority_account_info.key,
            )?,
            vec![
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_mint_account_info.clone(),
                temporary_w_sol_token_account_authority_account_info.clone(),
                rent_account_info.clone(),
            ]
            .as_slice(),
        )?;
        let w_sol_token_account_authority_address_resolver = WSolTokenAccountAuthorityAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let mut w_sol_token_account_authority_pubkey_seeds = w_sol_token_account_authority_address_resolver.get_seeds();
        let w_sol_token_account_authority_pubkey_bump_seed_ = [intermediary.w_sol_token_account_authority_pubkey_bump_seed];
        w_sol_token_account_authority_pubkey_seeds.push(w_sol_token_account_authority_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &spl_token::instruction::transfer(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_account_authority_account_info.key,
                [].as_slice(),
                input_token_amount,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_account_authority_account_info.clone(),
            ]
            .as_slice(),
            [w_sol_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
        )?;
        let mut any_mint_token_account_pubkey_seeds = any_mint_token_account_address_resolver.get_seeds();
        let any_mint_token_account_pubkey_bump_seed_ = [any_mint_token_account_pubkey_bump_seed];
        any_mint_token_account_pubkey_seeds.push(any_mint_token_account_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                intermediary_trader_account_info.key,
                any_mint_token_account_account_info.key,
                token_account_rent_exemption_balance,
                <Account as Pack>::LEN as u64,
                token_program_account_info.key,
            ),
            vec![
                intermediary_trader_account_info.clone(),
                any_mint_token_account_account_info.clone(),
            ]
            .as_slice(),
            [any_mint_token_account_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_account(
                token_program_account_info.key,
                any_mint_token_account_account_info.key,
                any_mint_token_mint_account_info.key,
                any_mint_token_account_authority_account_info.key,
            )?,
            vec![
                any_mint_token_account_account_info.clone(),
                any_mint_token_mint_account_info.clone(),
                any_mint_token_account_authority_account_info.clone(),
                rent_account_info.clone(),
            ]
            .as_slice(),
        )?;
        let temporary_w_sol_token_account_authority_address_resolver = TemporaryWSolTokenAccountAuthorityAddressResolver {
            temporary_w_sol_token_account_pubkey: temporary_w_sol_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let mut temporary_w_sol_token_account_authority_pubkey_seeds = temporary_w_sol_token_account_authority_address_resolver.get_seeds();
        let temporary_w_sol_token_account_authority_pubkey_bump_seed_ = [intermediary.temporary_w_sol_token_account_authority_pubkey_bump_seed];
        temporary_w_sol_token_account_authority_pubkey_seeds.push(temporary_w_sol_token_account_authority_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &crate::extern_source::create_raydium_swap_base_in_instruction(
                amm_program_id.key,
                amm_pool_account_info.key,
                amm_authority_account_info.key,
                amm_open_orders_account_info.key,
                amm_coin_vault_account_info.key,
                amm_pc_vault_account_info.key,
                market_program_id_account_info.key,
                market_account_info.key,
                market_bids_account_info.key,
                market_asks_account_info.key,
                market_event_queue_account_info.key,
                market_coin_vault_account_info.key,
                market_pc_vault_account_info.key,
                market_vault_signer_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                any_mint_token_account_account_info.key,
                temporary_w_sol_token_account_authority_account_info.key,
                input_token_amount,
                minimum_output_token_amount,
            ),
            vec![
                token_program_account_info.clone(),
                amm_pool_account_info.clone(),
                amm_authority_account_info.clone(),
                amm_open_orders_account_info.clone(),
                amm_coin_vault_account_info.clone(),
                amm_pc_vault_account_info.clone(),
                market_program_id_account_info.clone(),
                market_account_info.clone(),
                market_bids_account_info.clone(),
                market_asks_account_info.clone(),
                market_event_queue_account_info.clone(),
                market_coin_vault_account_info.clone(),
                market_pc_vault_account_info.clone(),
                market_vault_signer_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
                any_mint_token_account_account_info.clone(),
                temporary_w_sol_token_account_authority_account_info.clone(),
            ]
            .as_slice(),
            [temporary_w_sol_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
        )?;
        if Account::unpack_unchecked(&any_mint_token_account_account_info.data.borrow())?.amount < minimum_output_token_amount {
            return Err(Error::TokenAccountInvalidAmount.into());
        }
        if Account::unpack_unchecked(&temporary_w_sol_token_account_account_info.data.borrow())?.amount != 0 {
            return Err(Error::TokenAccountInvalidAmount.into());
        };
        solana_program::program::invoke_signed(
            &spl_token::instruction::close_account(
                token_program_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_account_account_info.key,
                temporary_w_sol_token_account_authority_account_info.key,
                [].as_slice(),
            )?,
            vec![
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_account_account_info.clone(),
                temporary_w_sol_token_account_authority_account_info.clone(),
            ]
            .as_slice(),
            [temporary_w_sol_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::sync_native(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
        )?;
        Ok(())
    }
    fn sell_on_raydium<'a>(
        _program_id: &'a Pubkey,
        accounts: &'a [AccountInfo],
        input_token_amount: u64,
        minimum_output_token_amount: Option<u64>,
        any_mint_token_account_pubkey_bump_seed: u8,
        any_mint_token_account_authority_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_authority_pubkey_bump_seed: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_trader_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_any_mint_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_any_mint_token_account_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let rent_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_program_id = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_pool_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_open_orders_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_coin_vault_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let amm_pc_vault_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_program_id_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_bids_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_asks_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_event_queue_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_coin_vault_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_pc_vault_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let market_vault_signer_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_address_resolver = AnyMintTokenAccountAddressResolver {
            any_mint_token_mint_pubkey: any_mint_token_mint_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let any_mint_token_account_authority_address_resolver = AnyMintTokenAccountAuthorityAddressResolver {
            any_mint_token_account_pubkey: any_mint_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let temporary_any_mint_token_account_address_resolver = TemporaryAnyMintTokenAccountAddressResolver {
            any_mint_token_account_pubkey: any_mint_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let temporary_any_mint_token_account_authority_address_resolver = TemporaryAnyMintTokenAccountAuthorityAddressResolver {
            temporary_any_mint_token_account_pubkey: temporary_any_mint_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        if *any_mint_token_account_account_info.key != any_mint_token_account_address_resolver.create(any_mint_token_account_pubkey_bump_seed)?
            || *any_mint_token_account_authority_account_info.key != any_mint_token_account_authority_address_resolver.create(any_mint_token_account_authority_pubkey_bump_seed)?
            || *temporary_any_mint_token_account_account_info.key != temporary_any_mint_token_account_address_resolver.create(temporary_any_mint_token_account_pubkey_bump_seed)?
            || *temporary_any_mint_token_account_authority_account_info.key
                != temporary_any_mint_token_account_authority_address_resolver.create(temporary_any_mint_token_account_authority_pubkey_bump_seed)?
            || *any_mint_token_mint_account_info.key == spl_token::native_mint::ID
            || *system_program_account_info.key != solana_program::system_program::ID
            || *rent_account_info.key != solana_program::sysvar::rent::ID
            || *token_program_account_info.key != spl_token::ID
            || *amm_program_id.key != RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_PUBKEY
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_trader_account_info.is_signer
            || !intermediary_trader_account_info.is_writable
            || !w_sol_token_account_account_info.is_writable
            || !any_mint_token_account_account_info.is_writable
            || !temporary_any_mint_token_account_account_info.is_writable
            || !amm_pool_account_info.is_writable
            || !amm_open_orders_account_info.is_writable
            || !amm_coin_vault_account_info.is_writable
            || !amm_pc_vault_account_info.is_writable
            || !market_account_info.is_writable
            || !market_bids_account_info.is_writable
            || !market_asks_account_info.is_writable
            || !market_event_queue_account_info.is_writable
            || !market_coin_vault_account_info.is_writable
            || !market_pc_vault_account_info.is_writable
        {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        if any_mint_token_account_account_info.data_len() != <Account as Pack>::LEN {
            return Err(Error::InvalidAccountData.into());
        }
        let rent = Rent::from_account_info(rent_account_info)?;
        let token_account_rent_exemption_balance = rent.minimum_balance(<Account as Pack>::LEN);
        if intermediary_trader_account_info.lamports() < (token_account_rent_exemption_balance + FEE_SELL_ON_RAYDIUM) {
            return Err(Error::InvalidAccountLamports.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(&intermediary_account_info.data.borrow())?;
        if !intermediary.is_initialized {
            return Err(Error::IntermediaryIsNotInitialized.into());
        }
        if *intermediary_trader_account_info.key != intermediary.trader_pubkey {
            return Err(Error::IntermediaryInvalidInvestor.into());
        }
        if *w_sol_token_account_account_info.key != intermediary.w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccount.into());
        }
        let any_mint_token_account = Account::unpack_unchecked(&any_mint_token_account_account_info.data.borrow())?;
        if input_token_amount > any_mint_token_account.amount {
            return Err(Error::TokenAccountInsufficientAmount.into());
        }
        let temporary_any_mint_token_account_authority_pubkey_bump_seed_ = [temporary_any_mint_token_account_authority_pubkey_bump_seed];
        let mut any_mint_token_account_authority_pubkey_seeds = any_mint_token_account_authority_address_resolver.get_seeds();
        let any_mint_token_account_authority_pubkey_bump_seed_ = [any_mint_token_account_authority_pubkey_bump_seed];
        any_mint_token_account_authority_pubkey_seeds.push(any_mint_token_account_authority_pubkey_bump_seed_.as_slice());
        let (choosed_any_mint_token_account_account_info, choosed_any_mint_token_account_authority_account_info, choosed_any_mint_token_account_authority_pubkey_seeds) =
            if input_token_amount == any_mint_token_account.amount {
                (
                    any_mint_token_account_account_info,
                    any_mint_token_account_authority_account_info,
                    any_mint_token_account_authority_pubkey_seeds,
                )
            } else {
                let mut temporary_any_mint_token_account_pubkey_seeds = temporary_any_mint_token_account_address_resolver.get_seeds();
                let temporary_any_mint_token_account_pubkey_bump_seed_ = [temporary_any_mint_token_account_pubkey_bump_seed];
                temporary_any_mint_token_account_pubkey_seeds.push(temporary_any_mint_token_account_pubkey_bump_seed_.as_slice());
                solana_program::program::invoke_signed(
                    &solana_program::system_instruction::create_account(
                        intermediary_trader_account_info.key,
                        temporary_any_mint_token_account_account_info.key,
                        token_account_rent_exemption_balance,
                        <Account as Pack>::LEN as u64,
                        token_program_account_info.key,
                    ),
                    vec![
                        intermediary_trader_account_info.clone(),
                        temporary_any_mint_token_account_account_info.clone(),
                    ]
                    .as_slice(),
                    [temporary_any_mint_token_account_pubkey_seeds.as_slice()].as_slice(),
                )?;
                solana_program::program::invoke(
                    &spl_token::instruction::initialize_account(
                        token_program_account_info.key,
                        temporary_any_mint_token_account_account_info.key,
                        any_mint_token_mint_account_info.key,
                        temporary_any_mint_token_account_authority_account_info.key,
                    )?,
                    vec![
                    temporary_any_mint_token_account_account_info.clone(),
                    any_mint_token_mint_account_info.clone(),
                    temporary_any_mint_token_account_authority_account_info.clone(),
                    rent_account_info.clone(),
                ]
                    .as_slice(),
                )?;
                solana_program::program::invoke_signed(
                    &spl_token::instruction::transfer(
                        token_program_account_info.key,
                        any_mint_token_account_account_info.key,
                        temporary_any_mint_token_account_account_info.key,
                        any_mint_token_account_authority_account_info.key,
                        [].as_slice(),
                        input_token_amount,
                    )?,
                    vec![
                        any_mint_token_account_account_info.clone(),
                        temporary_any_mint_token_account_account_info.clone(),
                        any_mint_token_account_authority_account_info.clone(),
                    ]
                    .as_slice(),
                    [any_mint_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
                )?;
                let mut temporary_any_mint_token_account_authority_pubkey_seeds = temporary_any_mint_token_account_authority_address_resolver.get_seeds();
                temporary_any_mint_token_account_authority_pubkey_seeds.push(temporary_any_mint_token_account_authority_pubkey_bump_seed_.as_slice());
                (
                    temporary_any_mint_token_account_account_info,
                    temporary_any_mint_token_account_authority_account_info,
                    temporary_any_mint_token_account_authority_pubkey_seeds,
                )
            };
        let (minimum_output_token_amount_, current_w_sol_token_account_amount) = match minimum_output_token_amount {
            Some(minimum_output_token_amount__) => {
                (
                    minimum_output_token_amount__,
                    Some(Account::unpack_unchecked(&w_sol_token_account_account_info.data.borrow())?.amount),
                )
            }
            None => {
                (
                    0,
                    None,
                )
            }
        };
        solana_program::program::invoke_signed(
            &crate::extern_source::create_raydium_swap_base_in_instruction(
                amm_program_id.key,
                amm_pool_account_info.key,
                amm_authority_account_info.key,
                amm_open_orders_account_info.key,
                amm_coin_vault_account_info.key,
                amm_pc_vault_account_info.key,
                market_program_id_account_info.key,
                market_account_info.key,
                market_bids_account_info.key,
                market_asks_account_info.key,
                market_event_queue_account_info.key,
                market_coin_vault_account_info.key,
                market_pc_vault_account_info.key,
                market_vault_signer_account_info.key,
                choosed_any_mint_token_account_account_info.key,
                w_sol_token_account_account_info.key,
                choosed_any_mint_token_account_authority_account_info.key,
                input_token_amount,
                minimum_output_token_amount_,
            ),
            vec![
                token_program_account_info.clone(),
                amm_pool_account_info.clone(),
                amm_authority_account_info.clone(),
                amm_open_orders_account_info.clone(),
                amm_coin_vault_account_info.clone(),
                amm_pc_vault_account_info.clone(),
                market_program_id_account_info.clone(),
                market_account_info.clone(),
                market_bids_account_info.clone(),
                market_asks_account_info.clone(),
                market_event_queue_account_info.clone(),
                market_coin_vault_account_info.clone(),
                market_pc_vault_account_info.clone(),
                market_vault_signer_account_info.clone(),
                choosed_any_mint_token_account_account_info.clone(),
                w_sol_token_account_account_info.clone(),
                choosed_any_mint_token_account_authority_account_info.clone(),
            ]
            .as_slice(),
            [choosed_any_mint_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
        )?;
        if let Some(current_w_sol_token_account_amount_) = current_w_sol_token_account_amount {
            if (Account::unpack_unchecked(&w_sol_token_account_account_info.data.borrow())?.amount - current_w_sol_token_account_amount_) < minimum_output_token_amount_ {
                return Err(Error::TokenAccountInvalidAmount.into());
            }
        }
        if Account::unpack_unchecked(&choosed_any_mint_token_account_account_info.data.borrow())?.amount != 0 {
            return Err(Error::TokenAccountInvalidAmount.into());
        }
        solana_program::program::invoke_signed(
            &spl_token::instruction::close_account(
                token_program_account_info.key,
                choosed_any_mint_token_account_account_info.key,
                w_sol_token_account_account_info.key,
                choosed_any_mint_token_account_authority_account_info.key,
                [].as_slice(),
            )?,
            vec![
                choosed_any_mint_token_account_account_info.clone(),
                w_sol_token_account_account_info.clone(),
                choosed_any_mint_token_account_authority_account_info.clone(),
            ]
            .as_slice(),
            [choosed_any_mint_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::sync_native(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
        )?;
        Ok(())
    }
    fn buy_on_pumpswap<'a>(
        _program_id: &'a Pubkey,
        accounts: &'a [AccountInfo],
        base_amount_out: u64,
        maximum_quote_amount_in: u64,
        any_mint_token_account_pubkey_bump_seed: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_trader_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let rent_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let pumpfan_program_id_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let pool_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let global_config_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let pool_base_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let pool_quote_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let protocol_fee_recipient_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let protocol_fee_recipient_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let base_token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let quote_token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let associated_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let event_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_address_resolver = AnyMintTokenAccountAddressResolver {
            any_mint_token_mint_pubkey: any_mint_token_mint_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        if *any_mint_token_account_account_info.key != any_mint_token_account_address_resolver.create(any_mint_token_account_pubkey_bump_seed)?
            || w_sol_token_mint_account_info.key == any_mint_token_mint_account_info.key
            || *w_sol_token_mint_account_info.key != spl_token::native_mint::ID
            || *system_program_account_info.key != solana_program::system_program::ID
            || *rent_account_info.key != solana_program::sysvar::rent::ID
            || *token_program_account_info.key != spl_token::ID
            || *pumpfan_program_id_account_info.key != PUMPSWAP_PROGRAM_PUBKEY
            || *associated_token_account_account_info.key != SPL_ASSOCIATED_TOKEN_ACCOUNT_ID
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_trader_account_info.is_signer
            || !intermediary_trader_account_info.is_writable
            || !w_sol_token_account_account_info.is_writable
            || !temporary_w_sol_token_account_account_info.is_writable
            || !any_mint_token_account_account_info.is_writable
            || !pool_base_token_account_account_info.is_writable
            || !pool_quote_token_account_account_info.is_writable
            || !protocol_fee_recipient_token_account_account_info.is_writable
        {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        let rent = Rent::from_account_info(rent_account_info)?;
        let token_account_rent_exemption_balance = rent.minimum_balance(<Account as Pack>::LEN);
        if intermediary_trader_account_info.lamports() < (2 * token_account_rent_exemption_balance + FEE_BUY_ON_PUMPSWAP) {
            return Err(Error::InvalidAccountLamports.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(&intermediary_account_info.data.borrow())?;
        if !intermediary.is_initialized {
            return Err(Error::IntermediaryIsNotInitialized.into());
        }
        if *intermediary_trader_account_info.key != intermediary.trader_pubkey {
            return Err(Error::IntermediaryInvalidInvestor.into());
        }
        if *w_sol_token_account_account_info.key != intermediary.w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccount.into());
        }
        if *w_sol_token_account_authority_account_info.key != intermediary.w_sol_token_account_authority_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccountAuthority.into());
        }
        if *temporary_w_sol_token_account_account_info.key != intermediary.temporary_w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidTemporaryWSolTokenAccount.into());
        }
        let w_sol_token_account = Account::unpack_unchecked(&w_sol_token_account_account_info.data.borrow())?;
        if maximum_quote_amount_in > w_sol_token_account.amount {
            return Err(Error::TokenAccountInsufficientAmount.into());
        }
        let temporary_w_sol_token_account_address_resolver = TemporaryWSolTokenAccountAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
        };
        let mut temporary_w_sol_token_account_pubkey_seeds = temporary_w_sol_token_account_address_resolver.get_seeds();
        let temporary_w_sol_token_account_pubkey_bump_seed_ = [intermediary.temporary_w_sol_token_account_pubkey_bump_seed];
        temporary_w_sol_token_account_pubkey_seeds.push(temporary_w_sol_token_account_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                intermediary_trader_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                token_account_rent_exemption_balance,
                <Account as Pack>::LEN as u64,
                quote_token_program_account_info.key,
            ),
            vec![
                intermediary_trader_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
            [temporary_w_sol_token_account_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_account(
                quote_token_program_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_mint_account_info.key,
                intermediary_trader_account_info.key,
            )?,
            vec![
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_mint_account_info.clone(),
                intermediary_trader_account_info.clone(),
                rent_account_info.clone(),
            ]
            .as_slice(),
        )?;
        let w_sol_token_account_authority_address_resolver = WSolTokenAccountAuthorityAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let mut w_sol_token_account_authority_pubkey_seeds = w_sol_token_account_authority_address_resolver.get_seeds();
        let w_sol_token_account_authority_pubkey_bump_seed_ = [intermediary.w_sol_token_account_authority_pubkey_bump_seed];
        w_sol_token_account_authority_pubkey_seeds.push(w_sol_token_account_authority_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &spl_token::instruction::transfer(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_account_authority_account_info.key,
                [].as_slice(),
                maximum_quote_amount_in,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_account_authority_account_info.clone(),
            ]
            .as_slice(),
            [w_sol_token_account_authority_pubkey_seeds.as_slice()].as_slice(),
        )?;
        let mut any_mint_token_account_pubkey_seeds = any_mint_token_account_address_resolver.get_seeds();
        let any_mint_token_account_pubkey_bump_seed_ = [any_mint_token_account_pubkey_bump_seed];
        any_mint_token_account_pubkey_seeds.push(any_mint_token_account_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                intermediary_trader_account_info.key,
                any_mint_token_account_account_info.key,
                token_account_rent_exemption_balance,
                <Account as Pack>::LEN as u64,
                base_token_program_account_info.key,
            ),
            vec![
                intermediary_trader_account_info.clone(),
                any_mint_token_account_account_info.clone(),
            ]
            .as_slice(),
            [any_mint_token_account_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_account(
                quote_token_program_account_info.key,
                any_mint_token_account_account_info.key,
                any_mint_token_mint_account_info.key,
                intermediary_trader_account_info.key,
            )?,
            vec![
                any_mint_token_account_account_info.clone(),
                any_mint_token_mint_account_info.clone(),
                intermediary_trader_account_info.clone(),
                rent_account_info.clone(),
            ]
            .as_slice(),
        )?;
        solana_program::program::invoke(
            &crate::extern_source::create_pumpswap_buy_instruction(
                pumpfan_program_id_account_info.key,
                pool_account_info.key,
                intermediary_trader_account_info.key,
                global_config_account_info.key,
                any_mint_token_mint_account_info.key,
                w_sol_token_mint_account_info.key,
                any_mint_token_account_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                pool_base_token_account_account_info.key,
                pool_quote_token_account_account_info.key,
                protocol_fee_recipient_account_info.key,
                protocol_fee_recipient_token_account_account_info.key,
                base_token_program_account_info.key,
                quote_token_program_account_info.key,
                event_authority_account_info.key,
                base_amount_out,
                maximum_quote_amount_in,
            ),
            vec![
                pool_account_info.clone(),
                intermediary_trader_account_info.clone(),
                global_config_account_info.clone(),
                any_mint_token_mint_account_info.clone(),
                w_sol_token_mint_account_info.clone(),
                any_mint_token_account_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
                pool_base_token_account_account_info.clone(),
                pool_quote_token_account_account_info.clone(),
                protocol_fee_recipient_account_info.clone(),
                protocol_fee_recipient_token_account_account_info.clone(),
                base_token_program_account_info.clone(),
                quote_token_program_account_info.clone(),
                system_program_account_info.clone(),
                associated_token_account_account_info.clone(),
                event_authority_account_info.clone(),
                pumpfan_program_id_account_info.clone(),
            ]
            .as_slice(),
        )?;
        if Account::unpack_unchecked(&any_mint_token_account_account_info.data.borrow())?.amount < base_amount_out {
            return Err(Error::TokenAccountInvalidAmount.into());
        }
        solana_program::program::invoke(
            &spl_token::instruction::close_account(
                quote_token_program_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_account_account_info.key,
                intermediary_trader_account_info.key,
                [].as_slice(),
            )?,
            vec![
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_account_account_info.clone(),
                intermediary_trader_account_info.clone(),
            ]
            .as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::sync_native(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
        )?;
        Ok(())
    }
    fn sell_on_pumpswap<'a>(
        _program_id: &'a Pubkey,
        accounts: &'a [AccountInfo],
        input_token_amount: u64,
        minimum_output_token_amount: Option<u64>,
        any_mint_token_account_pubkey_bump_seed: u8,
        temporary_any_mint_token_account_pubkey_bump_seed: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_trader_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_w_sol_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let temporary_any_mint_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let w_sol_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_mint_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let rent_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let pumpfan_program_id_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let pool_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let global_config_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let pool_base_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let pool_quote_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let protocol_fee_recipient_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let protocol_fee_recipient_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let base_token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let quote_token_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let associated_token_account_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let event_authority_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let any_mint_token_account_address_resolver = AnyMintTokenAccountAddressResolver {
            any_mint_token_mint_pubkey: any_mint_token_mint_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        let temporary_any_mint_token_account_address_resolver = TemporaryAnyMintTokenAccountAddressResolver {
            any_mint_token_account_pubkey: any_mint_token_account_account_info.key,
            intermediary_pubkey: intermediary_account_info.key,
        };
        if *any_mint_token_account_account_info.key != any_mint_token_account_address_resolver.create(any_mint_token_account_pubkey_bump_seed)?
            || *temporary_any_mint_token_account_account_info.key != temporary_any_mint_token_account_address_resolver.create(temporary_any_mint_token_account_pubkey_bump_seed)?
            || *any_mint_token_mint_account_info.key == *w_sol_token_mint_account_info.key
            || *w_sol_token_mint_account_info.key != spl_token::native_mint::ID
            || *system_program_account_info.key != solana_program::system_program::ID
            || *rent_account_info.key != solana_program::sysvar::rent::ID
            || *token_program_account_info.key != spl_token::ID
            || *pumpfan_program_id_account_info.key != PUMPSWAP_PROGRAM_PUBKEY
            || *associated_token_account_account_info.key != SPL_ASSOCIATED_TOKEN_ACCOUNT_ID
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_trader_account_info.is_signer
            || !intermediary_trader_account_info.is_writable
            || !w_sol_token_account_account_info.is_writable
            || !temporary_w_sol_token_account_account_info.is_writable
            || !any_mint_token_account_account_info.is_writable
            || !temporary_any_mint_token_account_account_info.is_writable
            || !pool_base_token_account_account_info.is_writable
            || !pool_quote_token_account_account_info.is_writable
            || !protocol_fee_recipient_token_account_account_info.is_writable
        {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        if any_mint_token_account_account_info.data_len() != <Account as Pack>::LEN {
            return Err(Error::InvalidAccountData.into());
        }
        let rent = Rent::from_account_info(rent_account_info)?;
        let token_account_rent_exemption_balance = rent.minimum_balance(<Account as Pack>::LEN);
        if intermediary_trader_account_info.lamports() < (token_account_rent_exemption_balance + FEE_SELL_ON_PUMPSWAP) {
            return Err(Error::InvalidAccountLamports.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(&intermediary_account_info.data.borrow())?;
        if !intermediary.is_initialized {
            return Err(Error::IntermediaryIsNotInitialized.into());
        }
        if *intermediary_trader_account_info.key != intermediary.trader_pubkey {
            return Err(Error::IntermediaryInvalidInvestor.into());
        }
        if *w_sol_token_account_account_info.key != intermediary.w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidWSolTokenAccount.into());
        }
        if *temporary_w_sol_token_account_account_info.key != intermediary.temporary_w_sol_token_account_pubkey {
            return Err(Error::IntermediaryInvalidTemporaryWSolTokenAccount.into());
        }
        let any_mint_token_account = Account::unpack_unchecked(&any_mint_token_account_account_info.data.borrow())?;
        if input_token_amount > any_mint_token_account.amount {
            return Err(Error::TokenAccountInsufficientAmount.into());
        }
        let temporary_w_sol_token_account_address_resolver = TemporaryWSolTokenAccountAddressResolver {
            w_sol_token_account_pubkey: w_sol_token_account_account_info.key,
        };
        let mut temporary_w_sol_token_account_pubkey_seeds = temporary_w_sol_token_account_address_resolver.get_seeds();
        let temporary_w_sol_token_account_pubkey_bump_seed_ = [intermediary.temporary_w_sol_token_account_pubkey_bump_seed];
        temporary_w_sol_token_account_pubkey_seeds.push(temporary_w_sol_token_account_pubkey_bump_seed_.as_slice());
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                intermediary_trader_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                token_account_rent_exemption_balance,
                <Account as Pack>::LEN as u64,
                quote_token_program_account_info.key,
            ),
            vec![
                intermediary_trader_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
            [temporary_w_sol_token_account_pubkey_seeds.as_slice()].as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::initialize_account(
                quote_token_program_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_mint_account_info.key,
                intermediary_trader_account_info.key,
            )?,
            vec![
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_mint_account_info.clone(),
                intermediary_trader_account_info.clone(),
                rent_account_info.clone(),
            ]
            .as_slice(),
        )?;
        let choosed_any_mint_token_account_account_info =
            if input_token_amount == any_mint_token_account.amount {
                any_mint_token_account_account_info
            } else {
                let mut temporary_any_mint_token_account_pubkey_seeds = temporary_any_mint_token_account_address_resolver.get_seeds();
                let temporary_any_mint_token_account_pubkey_bump_seed_ = [temporary_any_mint_token_account_pubkey_bump_seed];
                temporary_any_mint_token_account_pubkey_seeds.push(temporary_any_mint_token_account_pubkey_bump_seed_.as_slice());
                solana_program::program::invoke_signed(
                    &solana_program::system_instruction::create_account(
                        intermediary_trader_account_info.key,
                        temporary_any_mint_token_account_account_info.key,
                        token_account_rent_exemption_balance,
                        <Account as Pack>::LEN as u64,
                        base_token_program_account_info.key,
                    ),
                    vec![
                    intermediary_trader_account_info.clone(),
                    temporary_any_mint_token_account_account_info.clone(),
                ]
                    .as_slice(),
                    [temporary_any_mint_token_account_pubkey_seeds.as_slice()].as_slice(),
                )?;
                solana_program::program::invoke(
                    &spl_token::instruction::initialize_account(
                        base_token_program_account_info.key,
                        temporary_any_mint_token_account_account_info.key,
                        any_mint_token_mint_account_info.key,
                        intermediary_trader_account_info.key,
                    )?,
                    vec![
                        temporary_any_mint_token_account_account_info.clone(),
                        any_mint_token_mint_account_info.clone(),
                        intermediary_trader_account_info.clone(),
                        rent_account_info.clone(),
                    ]
                    .as_slice(),
                )?;
                solana_program::program::invoke(
                    &spl_token::instruction::transfer(
                        base_token_program_account_info.key,
                        any_mint_token_account_account_info.key,
                        temporary_any_mint_token_account_account_info.key,
                        intermediary_trader_account_info.key,
                        [].as_slice(),
                        input_token_amount,
                    )?,
                    vec![
                        any_mint_token_account_account_info.clone(),
                        temporary_any_mint_token_account_account_info.clone(),
                        intermediary_trader_account_info.clone(),
                    ]
                    .as_slice(),
                )?;
                temporary_any_mint_token_account_account_info
            };
        let (minimum_output_token_amount_, is_need_to_check_minimum_otput_token_amount) = match minimum_output_token_amount {
            Some(minimum_output_token_amount__) => {
                (
                    minimum_output_token_amount__,
                    true,
                )
            }
            None => {
                (
                    0,
                    false,
                )
            }
        };
        solana_program::program::invoke(
            &crate::extern_source::create_pumpswap_sell_instruction(
                pumpfan_program_id_account_info.key,
                pool_account_info.key,
                intermediary_trader_account_info.key,
                global_config_account_info.key,
                any_mint_token_mint_account_info.key,
                w_sol_token_mint_account_info.key,
                choosed_any_mint_token_account_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                pool_base_token_account_account_info.key,
                pool_quote_token_account_account_info.key,
                protocol_fee_recipient_account_info.key,
                protocol_fee_recipient_token_account_account_info.key,
                base_token_program_account_info.key,
                quote_token_program_account_info.key,
                event_authority_account_info.key,
                input_token_amount,
                minimum_output_token_amount_,
            ),
            vec![
                pool_account_info.clone(),
                intermediary_trader_account_info.clone(),
                global_config_account_info.clone(),
                any_mint_token_mint_account_info.clone(),
                w_sol_token_mint_account_info.clone(),
                choosed_any_mint_token_account_account_info.clone(),
                temporary_w_sol_token_account_account_info.clone(),
                pool_base_token_account_account_info.clone(),
                pool_quote_token_account_account_info.clone(),
                protocol_fee_recipient_account_info.clone(),
                protocol_fee_recipient_token_account_account_info.clone(),
                base_token_program_account_info.clone(),
                quote_token_program_account_info.clone(),
                system_program_account_info.clone(),
                associated_token_account_account_info.clone(),
                event_authority_account_info.clone(),
                pumpfan_program_id_account_info.clone(),
            ]
            .as_slice(),
        )?;
        if is_need_to_check_minimum_otput_token_amount {
            if Account::unpack_unchecked(&temporary_w_sol_token_account_account_info.data.borrow())?.amount < minimum_output_token_amount_ {
                return Err(Error::TokenAccountInvalidAmount.into());
            }
        }
        if Account::unpack_unchecked(&choosed_any_mint_token_account_account_info.data.borrow())?.amount != 0 {
            return Err(Error::TokenAccountInvalidAmount.into());
        }
        solana_program::program::invoke(
            &spl_token::instruction::close_account(
                quote_token_program_account_info.key,
                temporary_w_sol_token_account_account_info.key,
                w_sol_token_account_account_info.key,
                intermediary_trader_account_info.key,
                [].as_slice(),
            )?,
            vec![
                temporary_w_sol_token_account_account_info.clone(),
                w_sol_token_account_account_info.clone(),
                intermediary_trader_account_info.clone(),
            ]
            .as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::close_account(
                base_token_program_account_info.key,
                choosed_any_mint_token_account_account_info.key,
                w_sol_token_account_account_info.key,
                intermediary_trader_account_info.key,
                [].as_slice(),
            )?,
            vec![
                choosed_any_mint_token_account_account_info.clone(),
                w_sol_token_account_account_info.clone(),
                intermediary_trader_account_info.clone(),
            ]
            .as_slice(),
        )?;
        solana_program::program::invoke(
            &spl_token::instruction::sync_native(
                token_program_account_info.key,
                w_sol_token_account_account_info.key,
            )?,
            vec![
                w_sol_token_account_account_info.clone(),
            ]
            .as_slice(),
        )?;
        Ok(())
    }
    fn change_manager<'a>(_program_id: &'a Pubkey, accounts: &'a [AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_investor_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_manager_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let mut account_differentiator = HashSet::<&'_ Pubkey>::with_capacity(4);
        if !account_differentiator.insert(intermediary_investor_account_info.key)
            || !account_differentiator.insert(intermediary_account_info.key)
            || !account_differentiator.insert(intermediary_manager_account_info.key)
            || !account_differentiator.insert(system_program_account_info.key)
            || *system_program_account_info.key != solana_program::system_program::ID
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_investor_account_info.is_signer || !intermediary_account_info.is_writable {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        if intermediary_manager_account_info.owner != system_program_account_info.key {
            return Err(Error::InvalidAccountOwner.into());
        }
        if !intermediary_manager_account_info.data_is_empty() {
            return Err(Error::InvalidAccountData.into());
        }
        if intermediary_investor_account_info.lamports() < FEE_CHANGE_MANAGER || intermediary_manager_account_info.lamports() == 0 {
            return Err(Error::InvalidAccountLamports.into());
        }
        let mut intermediary = borsh::from_slice::<Intermediary>(&intermediary_account_info.data.borrow())?;
        if !intermediary.is_initialized {
            return Err(Error::IntermediaryIsNotInitialized.into());
        }
        if *intermediary_investor_account_info.key != intermediary.investor_pubkey {
            return Err(Error::IntermediaryInvalidInvestor.into());
        }
        if *intermediary_manager_account_info.key == intermediary.investor_pubkey
            || *intermediary_manager_account_info.key == intermediary.manager_pubkey
            || *intermediary_manager_account_info.key == intermediary.trader_pubkey
        {
            return Err(Error::IntermediaryInvalidManager.into());
        }
        intermediary.manager_pubkey = *intermediary_manager_account_info.key;
        borsh::to_writer(
            &mut intermediary_account_info.data.borrow_mut()[..],
            &intermediary,
        )?;
        Ok(())
    }
    fn change_trader<'a>(_program_id: &'a Pubkey, accounts: &'a [AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let intermediary_manager_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let intermediary_trader_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let system_program_account_info = solana_program::account_info::next_account_info(account_info_iter)?;
        let mut account_differentiator = HashSet::<&'_ Pubkey>::with_capacity(4);
        if !account_differentiator.insert(intermediary_manager_account_info.key)
            || !account_differentiator.insert(intermediary_account_info.key)
            || !account_differentiator.insert(intermediary_trader_account_info.key)
            || !account_differentiator.insert(system_program_account_info.key)
            || *system_program_account_info.key != solana_program::system_program::ID
        {
            return Err(Error::InvalidAccountPubkey.into());
        }
        if !intermediary_manager_account_info.is_signer || !intermediary_account_info.is_writable {
            return Err(Error::InvalidAccountConfigurationFlags.into());
        }
        if intermediary_trader_account_info.owner != system_program_account_info.key {
            return Err(Error::InvalidAccountOwner.into());
        }
        if !intermediary_trader_account_info.data_is_empty() {
            return Err(Error::InvalidAccountData.into());
        }
        if intermediary_manager_account_info.lamports() < FEE_CHANGE_TRADER || intermediary_trader_account_info.lamports() == 0 {
            return Err(Error::InvalidAccountLamports.into());
        }
        let mut intermediary = borsh::from_slice::<Intermediary>(&intermediary_account_info.data.borrow())?;
        if !intermediary.is_initialized {
            return Err(Error::IntermediaryIsNotInitialized.into());
        }
        if *intermediary_manager_account_info.key != intermediary.manager_pubkey {
            return Err(Error::IntermediaryInvalidManager.into());
        }
        if *intermediary_trader_account_info.key == intermediary.investor_pubkey
            || *intermediary_trader_account_info.key == intermediary.manager_pubkey
            || *intermediary_trader_account_info.key == intermediary.trader_pubkey
        {
            return Err(Error::IntermediaryInvalidTrader.into());
        }
        intermediary.trader_pubkey = *intermediary_trader_account_info.key;
        borsh::to_writer(
            &mut intermediary_account_info.data.borrow_mut()[..],
            &intermediary,
        )?;
        Ok(())
    }
}
