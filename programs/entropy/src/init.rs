use crate::*;
use crate::constants::{ADMIN, CRANK_OPERATOR_WALLET, DELAY, FEE_LAMPORTS};
use crate::utils::*;

pub fn init(ctx: Context<Initialize>) -> Result<()> {
    if ctx.accounts.info.initialised {
        ctx.accounts.info.init(
            *ctx.bumps.get("info")
                .expect("init account error")
        );
    } else {
        ctx.accounts.info.init(
            *ctx.bumps.get("info")
                .expect("init account error")
        );
        ctx.accounts.state.bump =
            *ctx.bumps.get("state")
                .expect("state account error");
    }
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(address = get_pubkey(ADMIN), mut)]
    pub initializer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = initializer,
        seeds = [b"info"],
        bump,
        space = Info::LEN
    )]
    pub info: Account<'info, Info>,

    #[account(
        init_if_needed,
        payer = initializer,
        seeds = [b"state"],
        bump,
        space = State::LEN
    )]
    pub state: Account<'info, State>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Info {
    pub bump: u8,               // 1
    pub admin: Pubkey,          // 32
    pub crank_operator: Pubkey, // 32
    pub fee_lamports: u64,      // 8
    pub delay: u64,             // 8
    pub initialised: bool,      // 1
}
size!(Info, 82);

impl Info {
    pub fn init(&mut self, bump: u8) {
        self.bump = bump;
        self.admin = get_pubkey(ADMIN);
        self.crank_operator = get_pubkey(CRANK_OPERATOR_WALLET);
        self.fee_lamports = FEE_LAMPORTS;
        self.delay = DELAY;
        self.initialised = true;
    }
}

#[account]
pub struct State {
    pub bump: u8,               // 1
    pub request_slot: u64,      // 8
    pub latest_seed: [u8; 10],  // 10
    pub seed_slot: u64,         // 8
}
size!(State, 27);

