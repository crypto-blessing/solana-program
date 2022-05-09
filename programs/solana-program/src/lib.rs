use anchor_lang::prelude::*;

declare_id!("4a9qyjKPPpr1DdAK1kmBUG3h9KfANHPepcrwLE99XhV2");

#[program]
pub mod solana_program {
    use super::*;


    pub fn send_blessing(ctx: Context<SendBlessing>, token: Pubkey, amount: u64, count: u8) -> Result<()> {
        let blessing: &mut Account<Blessing> = &mut ctx.accounts.blessing;
        let sender: &Signer = &ctx.accounts.sender;
        let clock: Clock = Clock::get().unwrap();


        blessing.sender = *sender.key;
        blessing.ts = clock.unix_timestamp;
        blessing.token = token;
        blessing.amount = amount;
        blessing.count = count;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct SendBlessing<'info> {
    #[account(init, payer = sender, space = Blessing::LEN)]
    pub blessing: Account<'info, Blessing>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Blessing {
    pub ts: i64,
    pub sender: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
    pub count: u8,
    pub receivers: Vec<Pubkey>,
    pub state: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const TIMESTAMP_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const AMOUNT_LENGTH: usize = 8;
const COUNT_LENGTH: usize = 1;
const VEC_PREFIX_LENGTH: usize = 4;
const STATE_LENGTH: usize = 1;

impl Blessing {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + TIMESTAMP_LENGTH // Timestamp.
        + PUBLIC_KEY_LENGTH // sender.
        + PUBLIC_KEY_LENGTH // token.
        + AMOUNT_LENGTH
        + COUNT_LENGTH
        + VEC_PREFIX_LENGTH + (PUBLIC_KEY_LENGTH * 10)
        + STATE_LENGTH;
}