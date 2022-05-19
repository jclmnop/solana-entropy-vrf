use crate::*;
use structopt::StructOpt;
use dotenv;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Entropy VRF Crank",
    about = "Basic program for running an Entropy VRF Crank Node"
)]
pub struct Opt {
    /// RPC endpoint URL
    #[structopt(short, long, global = true, default_value = "poop")]
    pub rpc: String,

    /// Log level - `warn, info, debug, trace` etc. Default: `warn`
    #[structopt(short, long, global = true, default_value = "warn")]
    pub log_level: String,

    /// Signing keypair + payer
    #[structopt(short, long, global = true, default_value = "poop")]
    pub keypair: String,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Initialize entropy account. Requires admin keypair.
    #[structopt(name = "init")]
    Init,

    /// Operate the `crank` for Entropy VRF. Runs indefinitely.
    #[structopt(name = "turn-crank")]
    OperateCrank,
}
