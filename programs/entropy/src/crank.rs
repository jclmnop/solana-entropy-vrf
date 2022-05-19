use anchor_lang::solana_program::native_token::sol_to_lamports;
use crate::*;
use crate::constants::*;
use crate::utils::*;
use CrankTurned;

use pyth_client::{load_price};

pub fn turn_crank(ctx: Context<Crank>) -> Result<()> {
    let current_slot = Clock::get().unwrap().slot;
    msg!("slot: {}", current_slot);
    require!(
        current_slot >= ctx.accounts.state.request_slot + DELAY,
        CrankError::DelayTooSmall
    );
    msg!("getting data...");
    let doge_price_data = &ctx.accounts.doge_price_info.try_borrow_data()?;
    let btc_price_data = &ctx.accounts.btc_price_info.try_borrow_data()?;
    let eth_price_data = &ctx.accounts.eth_price_info.try_borrow_data()?;
    let luna_price_data = &ctx.accounts.luna_price_info.try_borrow_data()?;
    let bnb_price_data = &ctx.accounts.bnb_price_info.try_borrow_data()?;
    let sol_price_data = &ctx.accounts.sol_price_info.try_borrow_data()?;
    let srm_price_data = &ctx.accounts.srm_price_info.try_borrow_data()?;
    let pai_price_data = &ctx.accounts.pai_price_info.try_borrow_data()?;
    let cope_price_data = &ctx.accounts.cope_price_info.try_borrow_data()?;
    let avax_price_data = &ctx.accounts.avax_price_info.try_borrow_data()?;
    msg!("getting prices...");
    let prices = [
        load_price(doge_price_data).unwrap().agg.price,
        load_price(btc_price_data).unwrap().agg.price,
        load_price(eth_price_data).unwrap().agg.price,
        load_price(luna_price_data).unwrap().agg.price,
        load_price(bnb_price_data).unwrap().agg.price,
        load_price(sol_price_data).unwrap().agg.price,
        load_price(srm_price_data).unwrap().agg.price,
        load_price(pai_price_data).unwrap().agg.price,
        load_price(cope_price_data).unwrap().agg.price,
        load_price(avax_price_data).unwrap().agg.price,
    ];

    msg!("DOGE: {}", prices[0]);
    msg!("BTC: {}", prices[1]);
    msg!("ETH: {}", prices[2]);
    msg!("LUNA: {}", prices[3]);
    msg!("BNB: {}", prices[4]);
    msg!("SOL: {}", prices[5]);
    msg!("SRM: {}", prices[6]);
    msg!("ADA: {}", prices[7]);
    msg!("COPE: {}", prices[8]);
    msg!("AVAX: {}", prices[9]);

    msg!("calculating seed...");
    ctx.accounts.state.latest_seed = calc_seed(prices);
    ctx.accounts.state.seed_slot = current_slot;
    msg!("new seed slot: {}", current_slot);

    let balance: u64 = ctx.accounts.crank_operator.lamports();
    let buffer_lamports = sol_to_lamports(0.1);
    msg!("Paying fee...");
    if balance > buffer_lamports {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.crank_operator.key(),
            &ctx.accounts.fee_wallet.key(),
            balance - buffer_lamports,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.crank_operator.to_account_info(),
                ctx.accounts.fee_wallet.to_account_info(),
            ],
        )?;
    }

    emit!(CrankTurned{
        slot: current_slot,
        seed: ctx.accounts.state.latest_seed,
    });

    Ok(())
}

fn calc_seed(prices: [i64; 10]) -> [u8; 10] {
    let mut res: [u8; 10] = [0; 10];
    for i in 0..prices.len() {
        res[i] = *prices[i].to_be_bytes().last().unwrap();
    }
    res
}

#[derive(Accounts)]
pub struct Crank<'info> {
    #[account(mut, address = get_pubkey(CRANK_OPERATOR_WALLET))]
    pub crank_operator: Signer<'info>,

    /// CHECK: Not necessary because address is constrained
    #[account(mut, address = get_pubkey(FEE_WALLET))]
    pub fee_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
    )]
    pub state: Account<'info, State>,

    // Pyth price accounts
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(DOGE_PRICE_ACC))]
    pub doge_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(BTC_PRICE_ACC))]
    pub btc_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(ETH_PRICE_ACC))]
    pub eth_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(LUNA_PRICE_ACC))]
    pub luna_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(BNB_PRICE_ACC))]
    pub bnb_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(SOL_PRICE_ACC))]
    pub sol_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(SRM_PRICE_ACC))]
    pub srm_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(PAI_PRICE_ACC))]
    pub pai_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(COPE_PRICE_ACC))]
    pub cope_price_info: AccountInfo<'info>,
    /// CHECK: Not necessary because address is constrained
    #[account(address = get_pubkey(AVAX_PRICE_ACC))]
    pub avax_price_info: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CrankError {
    #[msg("No requests have been made since last crank turn")]
    RequestNotMade,
    #[msg("Last request was too recent")]
    DelayTooSmall,
}