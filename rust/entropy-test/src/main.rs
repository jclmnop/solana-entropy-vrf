mod opts;
mod constants;
mod test1;

use std::str::FromStr;
use env_logger::{Builder, Target};
use log::{LevelFilter, debug, info, warn, trace};
use anyhow::Result;
use anchor_client;
use anchor_client::anchor_lang::prelude;
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::signature::{Keypair, read_keypair_file};
use anchor_client::solana_sdk::signer::Signer;
use crate::opts::{Command, Opt};
use entropy_vrf_crank::utils;
use structopt::StructOpt;

fn setup_logging(log_level: String) -> Result<()> {
    let level = LevelFilter::from_str(&*log_level)?;
    Builder::new().filter_level(level).target(Target::Stdout).init();
    Ok(())
}

fn main() -> Result<()> {
    let options: Opt = Opt::from_args();
    setup_logging(options.log_level.clone())?;

    match options.cmd {
        Command::All => {
            test1::test1(&options)
        }
    }
}
