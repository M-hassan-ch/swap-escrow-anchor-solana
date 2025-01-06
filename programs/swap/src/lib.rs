pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("C2ZHJdpPYFUBjB4ZZK4mEFPrJL6HZdMPfxnVtwEEdhPr");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        amount_a_offered: u64,
        token_b_amount_wanted: u64,
    ) -> Result<()> {
        instructions::make_offer::send_offered_token_to_vault(&ctx, amount_a_offered)?;

        let offer = &mut ctx.accounts.offer;

        offer.set_inner(Offer {
            id,
            maker: ctx.accounts.maker.key(),
            token_mint_a: ctx.accounts.token_mint_a.key(),
            token_mint_b: ctx.accounts.token_mint_b.key(),
            token_b_amount_wanted,
            bump: ctx.bumps.offer,
        });
        Ok(())
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_wanted_tokens_to_maker(&context)?;
        instructions::take_offer::withdraw_and_close_vault(context)
    }
}
