use anchor_lang::solana_program::keccak;
use crate::*;
use crate::init::State;
use crate::constants::DELAY;
use crate::request::RandomNumPda;
use RandomNumberFetched;
use hex::ToHex;

pub fn fetch(
    ctx: Context<Fetch>,
    id: Pubkey,
    user: Pubkey
) -> Result<u128> {
    require!(
        !ctx.accounts.user_random_num_pda.retrieved,
        FetchError::NumberAlreadyFetched
    );
    let user_request_slot = ctx.accounts.user_random_num_pda.request_slot;
    let seed_slot = ctx.accounts.state.seed_slot;
    msg!("req_slot: {}\nseed_slot: {}", user_request_slot, seed_slot);

    require!(
        seed_slot >= (user_request_slot + DELAY),
        FetchError::SeedNotRecentEnough
    );

    let mut seeds: Vec<u8> = Vec::new();
    msg!(
        "Seeds: {} ++ {:?} ++ {:?}",
        &ctx.accounts.state.latest_seed.encode_hex::<String>(),
        &id.encode_hex::<String>(),
        &user.encode_hex::<String>()
    );
    seeds.extend(ctx.accounts.state.latest_seed);
    seeds.extend(id.as_ref());
    seeds.extend(user.as_ref());
    let hash: [u8; 32] = keccak::hash(&seeds).0;
    let mut hash_sliced: [u8; 16] = [0; 16];
    hash_sliced.copy_from_slice(&hash[16..32]);
    msg!("hash: {}", hash.encode_hex::<String>());
    msg!("hash sliced: {}", hash_sliced.encode_hex::<String>());
    let random_num = u128::from_be_bytes(hash_sliced);
    msg!("{} -> {}", hash_sliced.encode_hex::<String>(), random_num);
    msg!("Random number: {}", random_num);
    ctx.accounts.user_random_num_pda.random_num = random_num;
    ctx.accounts.user_random_num_pda.locked = false;
    ctx.accounts.user_random_num_pda.retrieved = true;
    emit!(RandomNumberFetched {
        random_num,
        user,
        id,
    });
    Ok(random_num)
}

#[derive(Accounts)]
#[instruction(id: Pubkey, user: Pubkey)]
pub struct Fetch<'info> {
    #[account(mut)]
    pub proxy_signer: Signer<'info>,
    #[account(
        mut,
        seeds = [
            id.as_ref(),
            user.as_ref(),
            proxy_signer.key().as_ref()
        ],
        bump = user_random_num_pda.bump,
    )]
    pub user_random_num_pda: Account<'info, RandomNumPda>,

    #[account(
        seeds = [b"state"],
        bump = state.bump,
    )]
    pub state: Account<'info, State>,
}


#[error_code]
pub enum FetchError {
    #[msg("Number already fetched, please request another")]
    NumberAlreadyFetched,
    #[msg("Seed was not fetched recently enough, please wait and try again.")]
    SeedNotRecentEnough,
}