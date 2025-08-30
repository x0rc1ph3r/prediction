use crate::*;

pub fn instruction_claim(ctx: Context<Claim>, _id: u64) -> Result<()> {
    let bet = &mut ctx.accounts.bet;

    if bet.claimed {
        return Err(CustomError::AlreadyClaimed.into());
    }

    if bet.deadline > Clock::get()?.unix_timestamp as u64 {
        return Err(CustomError::BetNotExpired.into());
    }

    let round = &mut ctx.accounts.round;

    if round.settled == false {
        return Err(CustomError::RoundNotSettled.into());
    }

    let round_end_price = round.price_close;
    let round_start_price = round.price_open;

    **round.to_account_info().try_borrow_mut_lamports()? -= bet.amount;

    // return back the same bet amount for now

    if bet.prediction_up && round_end_price > round_start_price {
        **ctx
            .accounts
            .signer
            .to_account_info()
            .try_borrow_mut_lamports()? += bet.amount;
    } else {
        **ctx
            .accounts
            .treasury
            .to_account_info()
            .try_borrow_mut_lamports()? += bet.amount;
    };

    bet.claimed = true;
    Ok(())
}
