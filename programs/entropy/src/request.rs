use anchor_lang::solana_program::clock::Slot;
use crate::*;
use crate::utils::*;
use crate::init::State;
use crate::constants::*;
use crate::{RandomNumRequested, FeePaid};

pub fn request(ctx: Context<Request>, id: Pubkey) -> Result<()> {
    require!(
        !ctx.accounts.user_random_num_pda.locked,
        RequestError::RandomNumPdaLocked
    );
    let request_slot = Clock::get().unwrap().slot;
    ctx.accounts.user_random_num_pda.set_inner(
        RandomNumPda {
            bump: *ctx.bumps.get("user_random_num_pda").unwrap(),
            request_slot,
            random_num: 0,
            retrieved: false,
            locked: true,
        }
    );

    // Transfer fee to crank operator
    let mut ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &ctx.accounts.crank_operator.key(),
        FEE_LAMPORTS,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.crank_operator.to_account_info(),
        ],
    )?;

    // Transfer signature fee to proxy_signer, with extra in case of errors
    ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &ctx.accounts.proxy_signer.key(),
        SIG_FEE_LAMPORTS*2,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.proxy_signer.to_account_info(),
        ],
    )?;

    emit!(FeePaid {
        amount: FEE_LAMPORTS,
        id
    });

    emit!(RandomNumRequested {
        request_slot
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: Pubkey)]
pub struct Request<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [
            id.as_ref(),
            user.key().as_ref(),
            proxy_signer.key().as_ref()
        ],
        bump,
        space = RandomNumPda::LEN
    )]
    pub user_random_num_pda: Account<'info, RandomNumPda>,

    /// CHECK: Not needed because account will sign a later tx
    #[account(mut)]
    pub proxy_signer: UncheckedAccount<'info>,
    #[account(
        seeds = [b"state"],
        bump = state.bump,
    )]
    pub state: Account<'info, State>,

    /// CHECK: Not needed because address is constrained
    #[account(mut, address = get_pubkey(CRANK_OPERATOR_WALLET))]
    pub crank_operator: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct RandomNumPda {
    pub bump: u8,               // 1
    pub request_slot: Slot,     // 8
    pub random_num: u128,       // 16
    pub retrieved: bool,        // 1
    pub locked: bool,           // 1
}
size!(RandomNumPda, 1 + 8 + 16 + 1 + 1);


#[error_code]
pub enum RequestError {
    RandomNumPdaLocked,
}
