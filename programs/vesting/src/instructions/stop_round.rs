use crate::*;

pub fn instruction_stop_round(ctx: Context<StopRound>, _id: u64) -> Result<()> {
    let price_update = &mut ctx.accounts.price_update;
    let feed_id: [u8; 32] = get_feed_id_from_hex(FEED_ID)?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;

    if price.price < 0 {
        return Err(CustomError::NegativePrice.into());
    }

    let round = &mut ctx.accounts.round;
    round.end_ts = Clock::get()?.unix_timestamp as u64;
    round.price_close = price.price as u64;
    round.settled = true;
    Ok(())
}
