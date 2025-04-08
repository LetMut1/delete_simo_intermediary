use {
    intermediary::{
        FEE_CHANGE_MANAGER,
        FEE_CHANGE_TRADER,
        FEE_DEPOSIT_FUNDS,
        FEE_INITIALIZE,
        FEE_WITHDRAW_FUNDS,
        PROGRAM_PUBKEY,
        ProgramDerivedAddress,
        TemporaryWSolTokenAccountAddressResolver,
        TemporaryWSolTokenAccountAuthorityAddressResolver,
        WSolTokenAccountAddressResolver,
        WSolTokenAccountAuthorityAddressResolver,
        instruction::Instruction,
        state::Intermediary,
    },
    solana_program::program_pack::Pack,
    solana_rpc_client::rpc_client::RpcClient,
    solana_sdk::{
        account::ReadableAccount,
        message::Message,
        pubkey::Pubkey,
        signer::{
            Signer,
            keypair::Keypair,
        },
        transaction::Transaction,
    },
    spl_token::state::Account,
    std::{
        collections::HashSet,
        error::Error,
        path::Path,
        str::FromStr,
    },
};
pub struct CommandProcessor;
impl CommandProcessor {
    const ERROR_INTERMEDIARY_IS_NOT_INITIALIZED: &'static str = "Intermediary is not initialized.";
    const ERROR_INTERMEDIARY_INVALID_INVESTOR: &'static str = "Intermediary invalid investor.";
    const ERROR_INTERMEDIARY_INVALID_MANAGER: &'static str = "Intermediary invalid manager.";
    const ERROR_INTERMEDIARY_INVALID_TRADER: &'static str = "Intermediary invalid trader.";
    const ERROR_INVALID_ACCOUNT_LAMPORTS: &'static str = "Invalid account lamports.";
    pub fn initialize<'a>(
        rpc_client: &'a RpcClient,
        intermediary_investor_keypair_file_path: &'a str,
        intermediary_manager_pubkey: &'a str,
        intermediary_trader_pubkey: &'a str,
        lamports_to_treasury: u64,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let intermediary_investor_keypair = Self::load_keypair_from_file(&intermediary_investor_keypair_file_path)?;
        let intermediary_investor_pubkey = intermediary_investor_keypair.pubkey();
        let intermediary_manager_pubkey_ = Pubkey::from_str(intermediary_manager_pubkey)?;
        let intermediary_trader_pubkey_ = Pubkey::from_str(intermediary_trader_pubkey)?;
        let mut account_differentiator = HashSet::<&Pubkey>::with_capacity(3);
        if !account_differentiator.insert(&intermediary_investor_pubkey)
            || !account_differentiator.insert(&intermediary_manager_pubkey_)
            || !account_differentiator.insert(&intermediary_trader_pubkey_)
        {
            return Err("Invalid account pubkey.".into());
        }
        let intermediary_investor_account = rpc_client.get_account(&intermediary_investor_pubkey)?;
        let intermediary_manager_account = rpc_client.get_account(&intermediary_manager_pubkey_)?;
        let intermediary_trader_account = rpc_client.get_account(&intermediary_trader_pubkey_)?;
        if intermediary_investor_account.owner != solana_sdk::system_program::id()
            || intermediary_manager_account.owner != solana_sdk::system_program::id()
            || intermediary_trader_account.owner != solana_sdk::system_program::id()
        {
            return Err("Invalid account owner.".into());
        }
        if intermediary_investor_account.data().len() != 0 || intermediary_manager_account.data().len() != 0 || intermediary_trader_account.data().len() != 0 {
            return Err("Invalid account data.".into());
        }
        let intermediary_balance_for_rent_exemption = rpc_client.get_minimum_balance_for_rent_exemption(borsh::max_serialized_size::<Intermediary>().unwrap())?;
        let w_sol_token_account_rent_exemption_balance = rpc_client.get_minimum_balance_for_rent_exemption(<Account as Pack>::LEN)?;
        if intermediary_investor_account.lamports < (intermediary_balance_for_rent_exemption + w_sol_token_account_rent_exemption_balance + lamports_to_treasury + FEE_INITIALIZE)
            || intermediary_manager_account.lamports == 0
            || intermediary_trader_account.lamports == 0
        {
            return Err(Self::ERROR_INVALID_ACCOUNT_LAMPORTS.into());
        }
        let intermediary_keypair = Keypair::new();
        let intermediary_pubkey = intermediary_keypair.pubkey();
        println!("Intermediary: {}", &intermediary_pubkey);
        let w_sol_token_account_address_resolver = WSolTokenAccountAddressResolver {
            intermediary_pubkey: &intermediary_pubkey,
        };
        let (w_sol_token_account_pubkey, w_sol_token_account_pubkey_bump_seed) = w_sol_token_account_address_resolver.find();
        println!("WSolTokenAccount: {}", &w_sol_token_account_pubkey);
        let w_sol_token_account_authority_address_resolver = WSolTokenAccountAuthorityAddressResolver {
            w_sol_token_account_pubkey: &w_sol_token_account_pubkey,
            intermediary_pubkey: &intermediary_pubkey,
        };
        let (w_sol_token_account_authority_pubkey, w_sol_token_account_authority_pubkey_bump_seed) = w_sol_token_account_authority_address_resolver.find();
        println!("WSolTokenAccountAuthority: {}", &w_sol_token_account_authority_pubkey);
        let temporary_w_sol_token_account_address_resolver = TemporaryWSolTokenAccountAddressResolver {
            w_sol_token_account_pubkey: &w_sol_token_account_pubkey,
        };
        let (temporary_w_sol_token_account_pubkey, temporary_w_sol_token_account_pubkey_bump_seed) = temporary_w_sol_token_account_address_resolver.find();
        println!("TemporaryWSolTokenAccount: {}", &temporary_w_sol_token_account_pubkey);
        let temporary_w_sol_token_account_authority_address_resolver = TemporaryWSolTokenAccountAuthorityAddressResolver {
            temporary_w_sol_token_account_pubkey: &temporary_w_sol_token_account_pubkey,
            intermediary_pubkey: &intermediary_pubkey,
        };
        let (temporary_w_sol_token_account_authority_pubkey, temporary_w_sol_token_account_authority_pubkey_bump_seed) =
            temporary_w_sol_token_account_authority_address_resolver.find();
        println!("TemporaryWSolTokenAccountAuthority: {}", &temporary_w_sol_token_account_authority_pubkey);
        let instructions = vec![
            Instruction::initialize(
                &PROGRAM_PUBKEY,
                &intermediary_investor_pubkey,
                &intermediary_pubkey,
                &intermediary_manager_pubkey_,
                &intermediary_trader_pubkey_,
                &w_sol_token_account_pubkey,
                &w_sol_token_account_authority_pubkey,
                &temporary_w_sol_token_account_pubkey,
                &temporary_w_sol_token_account_authority_pubkey,
                lamports_to_treasury,
                w_sol_token_account_pubkey_bump_seed,
                w_sol_token_account_authority_pubkey_bump_seed,
                temporary_w_sol_token_account_pubkey_bump_seed,
                temporary_w_sol_token_account_authority_pubkey_bump_seed
            )?,
        ];
        let signers = vec![&intermediary_investor_keypair, &intermediary_keypair];
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let message = Message::new_with_blockhash(
            instructions.as_slice(),
            Some(&intermediary_investor_pubkey),
            &recent_blockhash,
        );
        let transaction = Transaction::new(
            signers.as_slice(),
            message,
            recent_blockhash,
        );
        let signature = rpc_client.send_transaction(&transaction)?;
        println!("Signature: {}", &signature);
        Ok(())
    }
    pub fn deposit_funds<'a>(
        rpc_client: &'a RpcClient,
        intermediary_investor_keypair_file_path: &'a str,
        intermediary_pubkey: &'a str,
        lamports_to_treasury: u64,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let intermediary_investor_keypair = Self::load_keypair_from_file(&intermediary_investor_keypair_file_path)?;
        let intermediary_investor_pubkey = intermediary_investor_keypair.pubkey();
        let intermediary_pubkey = Pubkey::from_str(intermediary_pubkey)?;
        let intermediary_investor_account = rpc_client.get_account(&intermediary_investor_pubkey)?;
        if intermediary_investor_account.lamports < (lamports_to_treasury + FEE_DEPOSIT_FUNDS) {
            return Err(Self::ERROR_INVALID_ACCOUNT_LAMPORTS.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(rpc_client.get_account(&intermediary_pubkey)?.data.as_slice())?;
        if !intermediary.is_initialized {
            return Err(Self::ERROR_INTERMEDIARY_IS_NOT_INITIALIZED.into());
        }
        if intermediary_investor_pubkey != intermediary.investor_pubkey {
            return Err(Self::ERROR_INTERMEDIARY_INVALID_INVESTOR.into());
        }
        let instructions = vec![
            Instruction::deposit_funds(
                &PROGRAM_PUBKEY,
                &intermediary_investor_pubkey,
                &intermediary_pubkey,
                &intermediary.w_sol_token_account_pubkey,
                lamports_to_treasury,
            )?,
        ];
        let signers = vec![&intermediary_investor_keypair];
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let message = Message::new_with_blockhash(
            instructions.as_slice(),
            Some(&intermediary_investor_pubkey),
            &recent_blockhash,
        );
        let transaction = Transaction::new(
            signers.as_slice(),
            message,
            recent_blockhash,
        );
        let signature = rpc_client.send_transaction(&transaction)?;
        println!("Signature: {}", &signature);
        Ok(())
    }
    pub fn withdraw_funds<'a>(
        rpc_client: &'a RpcClient,
        intermediary_investor_keypair_file_path: &'a str,
        intermediary_pubkey: &'a str,
        lamports_from_treasury: u64,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let intermediary_investor_keypair = Self::load_keypair_from_file(&intermediary_investor_keypair_file_path)?;
        let intermediary_investor_pubkey = intermediary_investor_keypair.pubkey();
        let intermediary_pubkey = Pubkey::from_str(intermediary_pubkey)?;
        let intermediary_investor_account = rpc_client.get_account(&intermediary_investor_pubkey)?;
        let temporary_w_sol_token_account_rent_exemption_balance = rpc_client.get_minimum_balance_for_rent_exemption(<Account as Pack>::LEN)?;
        if intermediary_investor_account.lamports < (temporary_w_sol_token_account_rent_exemption_balance + FEE_WITHDRAW_FUNDS) {
            return Err(Self::ERROR_INVALID_ACCOUNT_LAMPORTS.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(rpc_client.get_account(&intermediary_pubkey)?.data.as_slice())?;
        if !intermediary.is_initialized {
            return Err(Self::ERROR_INTERMEDIARY_IS_NOT_INITIALIZED.into());
        }
        if intermediary_investor_pubkey != intermediary.investor_pubkey {
            return Err(Self::ERROR_INTERMEDIARY_INVALID_INVESTOR.into());
        }
        let w_sol_token_account = Account::unpack_unchecked(rpc_client.get_account(&intermediary.w_sol_token_account_pubkey)?.data.as_slice())?;
        if w_sol_token_account.amount < lamports_from_treasury {
            return Err(
                format!(
                    "The maximum number of lamports from treasury is {}",
                    w_sol_token_account.amount,
                )
                .into(),
            );
        }
        let instructions = vec![
            Instruction::withdraw_funds(
                &PROGRAM_PUBKEY,
                &intermediary_investor_pubkey,
                &intermediary_pubkey,
                &intermediary.w_sol_token_account_pubkey,
                &intermediary.w_sol_token_account_authority_pubkey,
                &intermediary.temporary_w_sol_token_account_pubkey,
                &intermediary.temporary_w_sol_token_account_authority_pubkey,
                lamports_from_treasury,
            )?,
        ];
        let signers = vec![&intermediary_investor_keypair];
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let message = Message::new_with_blockhash(
            instructions.as_slice(),
            Some(&intermediary_investor_pubkey),
            &recent_blockhash,
        );
        let transaction = Transaction::new(
            signers.as_slice(),
            message,
            recent_blockhash,
        );
        let signature = rpc_client.send_transaction(&transaction)?;
        println!("Signature: {}", &signature);
        Ok(())
    }
    pub fn change_manager<'a>(
        rpc_client: &'a RpcClient,
        intermediary_investor_keypair_file_path: &'a str,
        intermediary_pubkey: &'a str,
        intermediary_manager_pubkey: &'a str,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let intermediary_investor_keypair = Self::load_keypair_from_file(&intermediary_investor_keypair_file_path)?;
        let intermediary_investor_pubkey = intermediary_investor_keypair.pubkey();
        let intermediary_pubkey = Pubkey::from_str(intermediary_pubkey)?;
        let intermediary_manager_pubkey = Pubkey::from_str(intermediary_manager_pubkey)?;
        let intermediary_investor_account = rpc_client.get_account(&intermediary_investor_pubkey)?;
        let intermediary_manager_account = rpc_client.get_account(&intermediary_manager_pubkey)?;
        if intermediary_investor_account.lamports < FEE_CHANGE_MANAGER || intermediary_manager_account.lamports == 0 {
            return Err(Self::ERROR_INVALID_ACCOUNT_LAMPORTS.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(rpc_client.get_account(&intermediary_pubkey)?.data.as_slice())?;
        if !intermediary.is_initialized {
            return Err(Self::ERROR_INTERMEDIARY_IS_NOT_INITIALIZED.into());
        }
        if intermediary_investor_pubkey != intermediary.investor_pubkey {
            return Err(Self::ERROR_INTERMEDIARY_INVALID_INVESTOR.into());
        }
        if intermediary_manager_pubkey == intermediary.investor_pubkey
            || intermediary_manager_pubkey == intermediary.manager_pubkey
            || intermediary_manager_pubkey == intermediary.trader_pubkey
        {
            return Err(Self::ERROR_INTERMEDIARY_INVALID_MANAGER.into());
        }
        let instructions = vec![
            Instruction::change_manager(
                &PROGRAM_PUBKEY,
                &intermediary_investor_pubkey,
                &intermediary_pubkey,
                &intermediary_manager_pubkey,
            )?,
        ];
        let signers = vec![&intermediary_investor_keypair];
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let message = Message::new_with_blockhash(
            instructions.as_slice(),
            Some(&intermediary_investor_pubkey),
            &recent_blockhash,
        );
        let transaction = Transaction::new(
            signers.as_slice(),
            message,
            recent_blockhash,
        );
        let signature = rpc_client.send_transaction(&transaction)?;
        println!("Signature: {}", &signature);
        Ok(())
    }
    pub fn change_trader<'a>(
        rpc_client: &'a RpcClient,
        intermediary_manager_keypair_file_path: &'a str,
        intermediary_pubkey: &'a str,
        intermediary_trader_pubkey: &'a str,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let intermediary_manager_keypair = Self::load_keypair_from_file(&intermediary_manager_keypair_file_path)?;
        let intermediary_manager_pubkey = intermediary_manager_keypair.pubkey();
        let intermediary_pubkey = Pubkey::from_str(intermediary_pubkey)?;
        let intermediary_trader_pubkey = Pubkey::from_str(intermediary_trader_pubkey)?;
        let intermediary_manager_account = rpc_client.get_account(&intermediary_manager_pubkey)?;
        let intermediary_trader_account = rpc_client.get_account(&intermediary_trader_pubkey)?;
        if intermediary_manager_account.lamports < FEE_CHANGE_TRADER || intermediary_trader_account.lamports == 0 {
            return Err(Self::ERROR_INVALID_ACCOUNT_LAMPORTS.into());
        }
        let intermediary = borsh::from_slice::<Intermediary>(rpc_client.get_account(&intermediary_pubkey)?.data.as_slice())?;
        if !intermediary.is_initialized {
            return Err(Self::ERROR_INTERMEDIARY_IS_NOT_INITIALIZED.into());
        }
        if intermediary_manager_pubkey != intermediary.manager_pubkey {
            return Err(Self::ERROR_INTERMEDIARY_INVALID_MANAGER.into());
        }
        if intermediary_trader_pubkey == intermediary.investor_pubkey
            || intermediary_trader_pubkey == intermediary.manager_pubkey
            || intermediary_trader_pubkey == intermediary.trader_pubkey
        {
            return Err(Self::ERROR_INTERMEDIARY_INVALID_TRADER.into());
        }
        let instructions = vec![
            Instruction::change_trader(
                &PROGRAM_PUBKEY,
                &intermediary_manager_pubkey,
                &intermediary_pubkey,
                &intermediary_trader_pubkey,
            )?,
        ];
        let signers = vec![&intermediary_manager_keypair];
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let message = Message::new_with_blockhash(
            instructions.as_slice(),
            Some(&intermediary_manager_pubkey),
            &recent_blockhash,
        );
        let transaction = Transaction::new(
            signers.as_slice(),
            message,
            recent_blockhash,
        );
        let signature = rpc_client.send_transaction(&transaction)?;
        println!("Signature: {}", &signature);
        Ok(())
    }
    fn load_keypair_from_file<'a>(keypair_file_path: &'a str) -> Result<Keypair, Box<dyn Error + 'static>> {
        let keypair_file_path_ = Path::new(keypair_file_path);
        let keypair_file_data = if keypair_file_path_.try_exists()? {
            std::fs::read_to_string(keypair_file_path_)?
        } else {
            return Err("The keypair.json file does not exist.".into());
        };
        Ok(Keypair::from_bytes(serde_json::from_str::<Vec<u8>>(keypair_file_data.as_str())?.as_slice())?)
    }
}
