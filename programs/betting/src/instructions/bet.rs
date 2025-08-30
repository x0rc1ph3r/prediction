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
    // Deduct the bet amount from Player
    let transfer_instruction = system_instruction::transfer(
        &ctx.accounts.signer.key(), // Sender public key
        &ctx.accounts.bet.key(),    // Bet account public key
        amount,                     // Bet amount
    );
    anchor_lang::solana_program::program::invoke(
        &transfer_instruction,
        &[
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.bet.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let round = &mut ctx.accounts.round;

    if prediction_up {
        round.total_up = round.total_up.checked_add(amount).ok_or(CustomError::Overflow)?;
    } else {
        round.total_down = round.total_down.checked_add(amount).ok_or(CustomError::Overflow)?;
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
