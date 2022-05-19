pub mod utils;
mod opt;
mod operate_crank;


use std::rc::Rc;
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
use dotenv;

use structopt::StructOpt;
use crate::opt::{Command, Opt};
use utils::*;

use entropy::RandomNumRequested;
use entropy::FeePaid;
use crate::operate_crank::run_crank;

fn setup_logging(log_level: String) -> Result<()> {
    let level = LevelFilter::from_str(&*log_level)?;
    Builder::new().filter_level(level).target(Target::Stdout).init();
    Ok(())
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let options: Opt = Opt::from_args();
    setup_logging(options.log_level)?;

    let keypair_path: String = {
        if options.keypair == "poop" {
            dotenv::var("CRANK_OPERATOR_KEYPAIR").unwrap()
        } else {
            options.keypair
        }
    };

    let rpc_url: String = {
        if options.rpc == "poop" {
            dotenv::var("RPC").unwrap()
        } else {
            options.rpc
        }
    };

    let payer: Keypair =
        read_keypair_file(
            keypair_path
        ).unwrap();

    let ws_url = rpc_url.replace("http", "ws");

    let url = Cluster::Custom(rpc_url, ws_url);


    match options.cmd {
        Command::Init => {
            let client = Client::new_with_options(
                url, Rc::new(payer), CommitmentConfig::confirmed()
            );
            let program = client.program(entropy::ID);
            let tx_sig = program.request()
                .instruction(init_ix())
                .send()?;
            info!("Initialized: {:?}", tx_sig);
            Ok(())
        }
        Command::OperateCrank => {
            run_crank(url, payer.to_bytes())
        }
    }
}
