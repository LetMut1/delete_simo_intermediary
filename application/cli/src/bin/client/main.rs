mod command_processor;
use {
    clap::{
        Arg,
        Command,
    },
    command_processor::CommandProcessor,
    solana_rpc_client::rpc_client::RpcClient,
    std::{
        error::Error,
        time::Duration,
    },
};
const LOGIC_ERROR: &'static str = "Logic error.";
fn main() -> Result<(), Box<dyn Error + 'static>> {
    match std::panic::catch_unwind(Processor::process) {
        Ok(result) => result,
        Err(_) => Err(LOGIC_ERROR.into()),
    }
}
struct Processor;
impl Processor {
    fn process() -> Result<(), Box<dyn Error + 'static>> {
        const COMMAND_INITIALIZE: &'static str = "initialize";
        const COMMAND_DEPOSIT_FUNDS: &'static str = "deposit_funds";
        const COMMAND_WITHDRAW_FUNDS: &'static str = "withdraw_funds";
        const COMMAND_CHANGE_MANAGER: &'static str = "change_manager";
        const COMMAND_CHANGE_TRADER: &'static str = "change_trader";
        const INTERMEDIARY_INVESTOR: &'static str = "intermediary_investor";
        const INTERMEDIARY_MANAGER: &'static str = "intermediary_manager";
        const INTERMEDIARY_TRADER: &'static str = "intermediary_trader";
        const LAMPORTS_TO_TREASURY: &'static str = "lamports_to_treasury";
        const LAMPORTS_FROM_TREASURY: &'static str = "lamports_from_treasury";
        const INTERMEDIARY: &'static str = "intermediary";
        const SOLANA_RPC_URL: &'static str = "solana_rpc_url";
        let arg_matches = clap::command!()
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand(
                Command::new(COMMAND_INITIALIZE)
                    .arg(Arg::new(INTERMEDIARY_INVESTOR).required(true).long(INTERMEDIARY_INVESTOR).help("Fee payer keypair.json file path."))
                    .arg(Arg::new(INTERMEDIARY_MANAGER).required(true).long(INTERMEDIARY_MANAGER).help("Intermediary manager pubkey."))
                    .arg(Arg::new(INTERMEDIARY_TRADER).required(true).long(INTERMEDIARY_TRADER).help("Intermediary trader pubkey."))
                    .arg(Arg::new(LAMPORTS_TO_TREASURY).required(true).long(LAMPORTS_TO_TREASURY).help("Lamports to treasury.")),
            )
            .subcommand(
                Command::new(COMMAND_DEPOSIT_FUNDS)
                    .arg(Arg::new(INTERMEDIARY_INVESTOR).required(true).long(INTERMEDIARY_INVESTOR).help("Fee payer keypair.json file path."))
                    .arg(Arg::new(INTERMEDIARY).required(true).long(INTERMEDIARY).help("Intermediary pubkey."))
                    .arg(Arg::new(LAMPORTS_TO_TREASURY).required(true).long(LAMPORTS_TO_TREASURY).help("Lamports to treasury.")),
            )
            .subcommand(
                Command::new(COMMAND_WITHDRAW_FUNDS)
                    .arg(Arg::new(INTERMEDIARY_INVESTOR).required(true).long(INTERMEDIARY_INVESTOR).help("Fee payer keypair.json file path."))
                    .arg(Arg::new(INTERMEDIARY).required(true).long(INTERMEDIARY).help("Intermediary pubkey."))
                    .arg(Arg::new(LAMPORTS_FROM_TREASURY).required(true).long(LAMPORTS_FROM_TREASURY).help("Lamports from treasury.")),
            )
            .subcommand(
                Command::new(COMMAND_CHANGE_MANAGER)
                    .arg(Arg::new(INTERMEDIARY_INVESTOR).required(true).long(INTERMEDIARY_INVESTOR).help("Fee payer keypair.json file path."))
                    .arg(Arg::new(INTERMEDIARY).required(true).long(INTERMEDIARY).help("Intermediary pubkey."))
                    .arg(Arg::new(INTERMEDIARY_MANAGER).required(true).long(INTERMEDIARY_MANAGER).help("Intermediary manager pubkey.")),
            )
            .subcommand(
                Command::new(COMMAND_CHANGE_TRADER)
                    .arg(Arg::new(INTERMEDIARY_MANAGER).required(true).long(INTERMEDIARY_MANAGER).help("Fee payer keypair.json file path."))
                    .arg(Arg::new(INTERMEDIARY).required(true).long(INTERMEDIARY).help("Intermediary pubkey."))
                    .arg(Arg::new(INTERMEDIARY_TRADER).required(true).long(INTERMEDIARY_TRADER).help("Intermediary trader pubkey.")),
            )
            .arg(Arg::new(SOLANA_RPC_URL).required(true).long(SOLANA_RPC_URL))
            .get_matches();
        let solana_rpc_url = arg_matches.get_one::<String>(SOLANA_RPC_URL).unwrap();
        let rpc_client = RpcClient::new_with_timeout(
            solana_rpc_url.clone(),
            Duration::from_secs(90),
        );
        match arg_matches.subcommand().unwrap() {
            (COMMAND_INITIALIZE, arg_matches_) => {
                CommandProcessor::initialize(
                    &rpc_client,
                    arg_matches_.get_one::<String>(INTERMEDIARY_INVESTOR).unwrap().as_str(),
                    arg_matches_.get_one::<String>(INTERMEDIARY_MANAGER).unwrap().as_str(),
                    arg_matches_.get_one::<String>(INTERMEDIARY_TRADER).unwrap().as_str(),
                    arg_matches_.get_one::<String>(LAMPORTS_TO_TREASURY).unwrap().parse::<u64>()?,
                )
            }
            (COMMAND_DEPOSIT_FUNDS, arg_matches_) => {
                CommandProcessor::deposit_funds(
                    &rpc_client,
                    arg_matches_.get_one::<String>(INTERMEDIARY_INVESTOR).unwrap().as_str(),
                    arg_matches_.get_one::<String>(INTERMEDIARY).unwrap().as_str(),
                    arg_matches_.get_one::<String>(LAMPORTS_TO_TREASURY).unwrap().parse::<u64>()?,
                )
            }
            (COMMAND_WITHDRAW_FUNDS, arg_matches_) => {
                CommandProcessor::withdraw_funds(
                    &rpc_client,
                    arg_matches_.get_one::<String>(INTERMEDIARY_INVESTOR).unwrap().as_str(),
                    arg_matches_.get_one::<String>(INTERMEDIARY).unwrap().as_str(),
                    arg_matches_.get_one::<String>(LAMPORTS_FROM_TREASURY).unwrap().parse::<u64>()?,
                )
            }
            (COMMAND_CHANGE_MANAGER, arg_matches_) => {
                CommandProcessor::change_manager(
                    &rpc_client,
                    arg_matches_.get_one::<String>(INTERMEDIARY_INVESTOR).unwrap().as_str(),
                    arg_matches_.get_one::<String>(INTERMEDIARY).unwrap().as_str(),
                    arg_matches_.get_one::<String>(INTERMEDIARY_MANAGER).unwrap().as_str(),
                )
            }
            (COMMAND_CHANGE_TRADER, arg_matches_) => {
                CommandProcessor::change_trader(
                    &rpc_client,
                    arg_matches_.get_one::<String>(INTERMEDIARY_MANAGER).unwrap().as_str(),
                    arg_matches_.get_one::<String>(INTERMEDIARY).unwrap().as_str(),
                    arg_matches_.get_one::<String>(INTERMEDIARY_TRADER).unwrap().as_str(),
                )
            }
            _ => Err(LOGIC_ERROR.into()),
        }
    }
}
