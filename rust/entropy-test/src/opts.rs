use crate::*;
use structopt::StructOpt;
use constants::*;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Entropy VRF Test",
    about = "8=====D~~~"
)]
pub struct Opt {
    /// RPC endpoint URL
    #[structopt(short, long, global = true, default_value = DEFAULT_RPC_URL)]
    pub rpc: String,

    /// Log level - `warn, info, debug, trace` etc. Default: `warn`
    #[structopt(short, long, global = true, default_value = "warn")]
    pub log_level: String,

    /// Signing keypair + payer
    #[structopt(short, long, global = true, default_value = DEFAULT_KEYPAIR)]
    pub keypair: String,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Run all tests.
    #[structopt(name = "all")]
    All,
}
