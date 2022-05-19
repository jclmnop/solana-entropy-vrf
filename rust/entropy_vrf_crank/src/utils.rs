use std::str::FromStr;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::system_program::ID as system_program_id;
use crate::*;
use entropy::constants::*;
use anchor_client::anchor_lang::{InstructionData, ToAccountMetas};
use anchor_client::anchor_lang::prelude::Pubkey;
use entropy::request::RandomNumPda;


pub fn info_pubkey() -> Pubkey {
    Pubkey::find_program_address(
        &["info".as_ref()],
        &entropy::ID
    ).0
}

pub fn state_pubkey() -> Pubkey {
    Pubkey::find_program_address(
        &["state".as_ref()],
        &entropy::ID
    ).0
}

pub fn pyth() -> [Pubkey; 10] {
    [
        Pubkey::from_str(DOGE_PRICE_ACC).unwrap(),
        Pubkey::from_str(BTC_PRICE_ACC).unwrap(),
        Pubkey::from_str(ETH_PRICE_ACC).unwrap(),
        Pubkey::from_str(LUNA_PRICE_ACC).unwrap(),
        Pubkey::from_str(BNB_PRICE_ACC).unwrap(),
        Pubkey::from_str(SOL_PRICE_ACC).unwrap(),
        Pubkey::from_str(SRM_PRICE_ACC).unwrap(),
        Pubkey::from_str(PAI_PRICE_ACC).unwrap(),
        Pubkey::from_str(COPE_PRICE_ACC).unwrap(),
        Pubkey::from_str(AVAX_PRICE_ACC).unwrap()
    ]
}

pub fn crank_operator() -> Pubkey {
    Pubkey::from_str(CRANK_OPERATOR_WALLET).unwrap()
}

pub fn fee_wallet() -> Pubkey {
    Pubkey::from_str(FEE_WALLET).unwrap()
}

pub fn init_ix() -> Instruction {
    let accounts = entropy::accounts::Initialize {
        initializer: Pubkey::from_str(ADMIN).unwrap(),
        info: info_pubkey(),
        state: state_pubkey(),
        system_program: system_program_id,
        rent: anchor_client::solana_sdk::sysvar::rent::ID,
    };
    let args = entropy::instruction::Initialize;

    Instruction {
        program_id: entropy::ID,
        accounts: accounts.to_account_metas(None),
        data: args.data()
    }
}

pub fn crank_ix() -> Instruction {
    let pyth = pyth();
    let accounts = entropy::accounts::Crank {
        crank_operator: crank_operator(),
        fee_wallet: fee_wallet(),
        state: state_pubkey(),
        doge_price_info: pyth[0],
        btc_price_info: pyth[1],
        eth_price_info: pyth[2],
        luna_price_info: pyth[3],
        bnb_price_info: pyth[4],
        sol_price_info: pyth[5],
        srm_price_info: pyth[6],
        pai_price_info: pyth[7],
        cope_price_info: pyth[8],
        avax_price_info: pyth[9],
        system_program: system_program_id,
    };
    let args = entropy::instruction::TurnCrank;

    Instruction {
        program_id: entropy::ID,
        accounts: accounts.to_account_metas(None),
        data: args.data()
    }
}

pub fn request_ix(id: Pubkey, user: Pubkey, proxy_signer: Pubkey) -> Instruction {
    let accounts = entropy::accounts::Request {
        user,
        user_random_num_pda: random_num_pda_pubkey(id, user, proxy_signer),
        proxy_signer,
        state: state_pubkey(),
        crank_operator: crank_operator(),
        system_program: system_program_id,
        rent: anchor_client::solana_sdk::sysvar::rent::ID,
    };
    let args = entropy::instruction::Request {id};

    Instruction {
        program_id: entropy::ID,
        accounts: accounts.to_account_metas(None),
        data: args.data()
    }
}

pub fn fetch_ix(id: Pubkey, user: Pubkey, proxy_signer: Pubkey) -> Instruction {
    let accounts = entropy::accounts::Fetch {
        proxy_signer,
        user_random_num_pda: random_num_pda_pubkey(id, user, proxy_signer),
        state: state_pubkey()
    };
    let args = entropy::instruction::Fetch {
        id,
        user
    };

    Instruction {
        program_id: entropy::ID,
        accounts: accounts.to_account_metas(None),
        data: args.data()
    }
}

pub fn random_num_pda_pubkey(
    id: Pubkey,
    user: Pubkey,
    proxy_signer: Pubkey
) -> Pubkey {
    Pubkey::find_program_address(
        &[id.as_ref(), user.as_ref(), proxy_signer.as_ref()],
        &entropy::ID
    ).0
}