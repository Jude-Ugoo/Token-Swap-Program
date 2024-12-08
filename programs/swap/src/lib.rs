use anchor_lang::prelude::*;

pub mod constants;
pub use constants::*;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod error;


declare_id!("DobXydDk24bf8LemLwiPQxP18B56a2EC3Ypy23efFt8Q");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&ctx, token_a_offered_amount)?;
        instructions::make_offer::save_offer(ctx, id, token_b_wanted_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_wanted_tokens_to_maker(&ctx)?;
        instructions::take_offer::withdraw_and_close_vault(ctx)
    }
}
