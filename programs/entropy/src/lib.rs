pub mod constants;
pub mod request;
pub mod fetch;
pub mod crank;
mod utils;
mod init;

use anchor_lang::prelude::*;
use pyth_client;
use std::str::FromStr;
use anchor_lang::solana_program::clock::Slot;

use crate::init::*;
use crate::request::*;
use crate::crank::*;
use crate::fetch::*;

declare_id!("RNGzsvoHhtAfdW7oiAn83pjqVeMDcHtj6XJ24Pq7Z9j");

#[program]
pub mod entropy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        init(ctx)
    }

    /// Request a random number to be fetched after at least `n` slots, where
    /// `n` is constants::DELAY. It's recommended to wait for a few extra slots
    /// before fetching, to ensure the crank has been turned.
    ///
    /// id - A pubkey to be used as an identifier, can be the user's pubkey,
    /// a program's pubkey, or the pubkey of an NFT mint. Used to ensure that
    /// a unique number is returned even when the user is requesting multiple
    /// random numbers within the same slot
    pub fn request(
        ctx: Context<Request>,
        id: Pubkey,
    ) -> Result<()> {
        request::request(ctx, id)
    }

    /// Crank function to be turned by designated crank operator, updates the
    /// seeds with latest prices from pyth accounts. This instruction is
    /// completely isolated from the user and proxy signer.
    pub fn turn_crank(ctx: Context<Crank>) -> Result<()> {
        crank::turn_crank(ctx)
    }

    /// Fetch the random number after it's been calculated, using the proxy
    /// signer passed in to the earlier request instruction to initiate the
    /// transaction. Using a different signer than the proxy account delegate
    /// earlier is not possible.
    ///
    /// id: Same id used earlier
    /// user: User's pubkey
    pub fn fetch(ctx: Context<Fetch>, id: Pubkey, user: Pubkey) -> Result<u128> {
        fetch::fetch(ctx, id, user)
    }
}

#[event]
#[derive(Copy, Clone, Debug)]
pub struct RandomNumRequested {
    pub request_slot: Slot,
}

#[event]
#[derive(Copy, Clone, Debug)]
pub struct FeePaid {
    pub amount: u64,
    pub id: Pubkey,
}

#[event]
#[derive(Copy, Clone, Debug)]
pub struct CrankTurned {
    slot: u64,
    seed: [u8; 10],
}

#[event]
#[derive(Copy, Clone, Debug)]
pub struct RandomNumberFetched {
    pub random_num: u128,
    pub user: Pubkey,
    pub id: Pubkey,
}
