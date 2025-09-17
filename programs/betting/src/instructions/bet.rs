use crate::*;

pub fn instruction_bet(
    ctx: Context<Bet>,
    _id: u64,
    amount: u64,
    prediction_up: bool,
) -> Result<()> {
    if amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }
    require!(
        **ctx.accounts.signer.to_account_info().lamports.borrow() >= amount,
        CustomError::InsufficientFunds
    );
    // Deduct the bet amount from Player's account and transfer it to the Round account
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.round.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;

    let round = &mut ctx.accounts.round;

    if prediction_up {
        round.total_up = round
            .total_up
            .checked_add(amount)
            .ok_or(CustomError::Overflow)?;
    } else {
        round.total_down = round
            .total_down
            .checked_add(amount)
            .ok_or(CustomError::Overflow)?;
    }

    let now = Clock::get()?.unix_timestamp;
    let bet = &mut ctx.accounts.bet;
    bet.user = ctx.accounts.signer.key();
    bet.amount = amount;
    bet.prediction_up = prediction_up;
    bet.deadline = now as u64 + 300; // 5 minutes from now
    bet.claimed = false;
    bet.bump = ctx.bumps.bet;

    Ok(())
}
