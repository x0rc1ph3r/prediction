use crate::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Bet<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + BetInfo::INIT_SPACE,
        seeds = [b"bet".as_ref(), round.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, BetInfo>,

    #[account(
        mut,
        seeds = [b"round".as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Claim<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bet".as_ref(), round.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, BetInfo>,
    #[account(
        mut,
        seeds = [b"round".as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,
    #[account(
        seeds = [b"treasury".as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTreasury<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [b"treasury".as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTreasury<'info> {
    #[account(mut)]
    pub treasurer: Signer<'info>,

    #[account(
        mut,
        has_one = treasurer,
    )]
    pub treasury: Account<'info, Treasury>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct StartRound<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury.treasurer
    )]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"treasury".as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init,
        payer = signer,
        space = 8 + Round::INIT_SPACE,
        seeds = [b"round".as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,
    pub price_update: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct StopRound<'info> {
    #[account(
        mut,
        constraint = signer.key() == treasury.treasurer
    )]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"treasury".as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
        seeds = [b"round".as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,
    pub price_update: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}
