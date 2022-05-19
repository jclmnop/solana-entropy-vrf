use std::rc::Rc;
use std::time::Duration;
use crate::*;
use anchor_client;
use anchor_client::anchor_lang::prelude::Pubkey;
use rayon::prelude::*;
use anchor_client::Client;
use anchor_client::solana_sdk::native_token::sol_to_lamports;
use rand;
use rand::Rng;
use entropy::request::RandomNumPda;

use entropy_vrf_crank::utils::*;

struct RandomNumRequest {
    pda: Pubkey,
    id: Pubkey,
    user: Pubkey,
}

pub fn test1(options: &Opt) -> Result<()> {
    let mut rng = rand::thread_rng();
    let proxy_signer_pair = read_keypair_file(&options.keypair).unwrap();
    let rpc_url = &options.rpc;
    let mut requests: Vec<RandomNumRequest> = Vec::new();
    for _ in 0..10 {
        let user = Keypair::new();
        let user_key = user.pubkey();
        let id = Pubkey::new_unique();
        let ws_url = rpc_url.replace("http", "ws");
        // let cluster = Cluster::Custom(rpc_url.clone(), ws_url);
        let cluster = Cluster::Devnet;
        let client: Client = Client::new_with_options(
            cluster, Rc::new(user), CommitmentConfig::confirmed()
        );
        let program = client.program(entropy::ID);
        let airdrop = program
            .rpc()
            .request_airdrop(&program.payer(), sol_to_lamports(0.1));
        if airdrop.is_err() {
            continue
        }
        info!("airdrop tx: {:?}", airdrop.unwrap());

        let tx_sig = program.request().instruction(
            request_ix(id, user_key, proxy_signer_pair.pubkey())
        ).send()?;
        info!(
            "[*] request made\n    tx: {:?}\n    user: {:?}\n    id: {:?}",
            tx_sig, user_key, id
        );

        requests.push(RandomNumRequest {
            pda: random_num_pda_pubkey(
                    id, user_key, proxy_signer_pair.pubkey()
                ),
            id,
            user: user_key,
        });

        let millis: u64 = rng.gen_range(10..5_000);
        std::thread::sleep(Duration::from_millis(millis));
    }

    let proxy_signer = proxy_signer_pair.pubkey();
    let cluster = Cluster::Devnet;
    let client: Client = Client::new_with_options(
        cluster, Rc::new(proxy_signer_pair), CommitmentConfig::confirmed()
    );

    let mut random_nums: Vec<u128> = Vec::new();

    let program = client.program(entropy::ID);
    for request in requests {
        let tx = program.request().instruction(fetch_ix(
            request.id, request.user, proxy_signer
        )).send();
        if tx.is_err() {
            info!("{:?}", tx.unwrap_err());
        } else {
            let pda: RandomNumPda = program.account(request.pda).unwrap();
            random_nums.push(pda.random_num)
        }
    }

    println!("random nums:\n {:?}", random_nums);

    Ok(())
}