#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod extern_source;
pub mod instruction;
pub mod processor;
pub mod state;
use solana_program::pubkey::{
    Pubkey,
    PubkeyError,
};
pub const PROGRAM_PUBKEY: Pubkey = {
    solana_program::declare_id!("Dz463tPx4MBroW6LMFJFdsnqYQ2JM6N3GXmw3mMq3m68");
    id()
};
// https://github.com/raydium-io/raydium-amm/tree/d10a8e9fab9f7a3d87b4ae3891e3e4c24b75c041
pub const RAYDIUM_LIQUIDITY_POOL_V4_PROGRAM_PUBKEY: Pubkey = {
    solana_program::declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
    id()
};
pub const PUMPSWAP_PROGRAM_PUBKEY: Pubkey = {
    solana_program::declare_id!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");
    id()
};
pub const SPL_ASSOCIATED_TOKEN_ACCOUNT_ID: Pubkey = Pubkey::new_from_array(spl_associated_token_account::ID.to_bytes());
pub const FEE_INITIALIZE: u64 = 1010101; // TODO TODO unit works comission
pub const FEE_DEPOSIT_FUNDS: u64 = 1010101; // TODO TODO unit works comission
pub const FEE_WITHDRAW_FUNDS: u64 = 1010101; // TODO TODO unit works comission
pub const FEE_BUY_ON_RAYDIUM: u64 = 1010101; // TODO TODO unit works comission
pub const FEE_SELL_ON_RAYDIUM: u64 = 1010101; // TODO TODO unit works comission
pub const FEE_BUY_ON_PUMPSWAP: u64 = 1010101; // TODO TODO unit works comission
pub const FEE_SELL_ON_PUMPSWAP: u64 = 1010101; // TODO TODO unit works comission
pub const FEE_CHANGE_MANAGER: u64 = 1010101; // TODO TODO unit works comission
pub const FEE_CHANGE_TRADER: u64 = 1010101; // TODO TODO unit works comission
pub trait ProgramDerivedAddress<'a> {
    fn find<'b>(
        &'b self,
    ) -> (
        Pubkey,
        u8,
    ) {
        Pubkey::find_program_address(
            self.get_seeds().as_slice(),
            &PROGRAM_PUBKEY,
        )
    }
    fn create<'b>(&'b self, bump_seed: u8) -> Result<Pubkey, PubkeyError> {
        let bump_seed_ = [bump_seed];
        let mut seeds = self.get_seeds();
        seeds.push(bump_seed_.as_slice());
        Pubkey::create_program_address(
            seeds.as_slice(),
            &PROGRAM_PUBKEY,
        )
    }
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]>;
}
pub struct WSolTokenAccountAddressResolver<'a> {
    pub intermediary_pubkey: &'a Pubkey,
}
impl<'a> ProgramDerivedAddress<'a> for WSolTokenAccountAddressResolver<'a> {
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]> {
        vec![
            spl_token::native_mint::ID.as_ref(),
            self.intermediary_pubkey.as_ref(),
            PROGRAM_PUBKEY.as_ref(),
        ]
    }
}
pub struct WSolTokenAccountAuthorityAddressResolver<'a> {
    pub w_sol_token_account_pubkey: &'a Pubkey,
    pub intermediary_pubkey: &'a Pubkey,
}
impl<'a> ProgramDerivedAddress<'a> for WSolTokenAccountAuthorityAddressResolver<'a> {
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]> {
        vec![
            self.w_sol_token_account_pubkey.as_ref(),
            self.intermediary_pubkey.as_ref(),
            spl_token::native_mint::ID.as_ref(),
            PROGRAM_PUBKEY.as_ref(),
        ]
    }
}
pub struct TemporaryWSolTokenAccountAddressResolver<'a> {
    pub w_sol_token_account_pubkey: &'a Pubkey,
}
impl<'a> ProgramDerivedAddress<'a> for TemporaryWSolTokenAccountAddressResolver<'a> {
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]> {
        vec![
            self.w_sol_token_account_pubkey.as_ref(),
            spl_token::native_mint::ID.as_ref(),
            PROGRAM_PUBKEY.as_ref(),
        ]
    }
}
pub struct TemporaryWSolTokenAccountAuthorityAddressResolver<'a> {
    pub temporary_w_sol_token_account_pubkey: &'a Pubkey,
    pub intermediary_pubkey: &'a Pubkey,
}
impl<'a> ProgramDerivedAddress<'a> for TemporaryWSolTokenAccountAuthorityAddressResolver<'a> {
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]> {
        vec![
            self.temporary_w_sol_token_account_pubkey.as_ref(),
            spl_token::native_mint::ID.as_ref(),
            PROGRAM_PUBKEY.as_ref(),
            self.intermediary_pubkey.as_ref(),
        ]
    }
}
pub struct AnyMintTokenAccountAddressResolver<'a> {
    pub any_mint_token_mint_pubkey: &'a Pubkey,
    pub intermediary_pubkey: &'a Pubkey,
}
impl<'a> ProgramDerivedAddress<'a> for AnyMintTokenAccountAddressResolver<'a> {
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]> {
        vec![
            PROGRAM_PUBKEY.as_ref(),
            self.any_mint_token_mint_pubkey.as_ref(),
            self.intermediary_pubkey.as_ref(),
        ]
    }
}
pub struct AnyMintTokenAccountAuthorityAddressResolver<'a> {
    pub any_mint_token_account_pubkey: &'a Pubkey,
    pub intermediary_pubkey: &'a Pubkey,
}
impl<'a> ProgramDerivedAddress<'a> for AnyMintTokenAccountAuthorityAddressResolver<'a> {
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]> {
        vec![
            PROGRAM_PUBKEY.as_ref(),
            self.any_mint_token_account_pubkey.as_ref(),
            self.intermediary_pubkey.as_ref(),
        ]
    }
}
pub struct TemporaryAnyMintTokenAccountAddressResolver<'a> {
    pub any_mint_token_account_pubkey: &'a Pubkey,
    pub intermediary_pubkey: &'a Pubkey,
}
impl<'a> ProgramDerivedAddress<'a> for TemporaryAnyMintTokenAccountAddressResolver<'a> {
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]> {
        vec![
            self.any_mint_token_account_pubkey.as_ref(),
            PROGRAM_PUBKEY.as_ref(),
            self.intermediary_pubkey.as_ref(),
        ]
    }
}
pub struct TemporaryAnyMintTokenAccountAuthorityAddressResolver<'a> {
    pub temporary_any_mint_token_account_pubkey: &'a Pubkey,
    pub intermediary_pubkey: &'a Pubkey,
}
impl<'a> ProgramDerivedAddress<'a> for TemporaryAnyMintTokenAccountAuthorityAddressResolver<'a> {
    fn get_seeds<'b>(&'b self) -> Vec<&'a [u8]> {
        vec![
            PROGRAM_PUBKEY.as_ref(),
            self.intermediary_pubkey.as_ref(),
            self.temporary_any_mint_token_account_pubkey.as_ref(),
        ]
    }
}
