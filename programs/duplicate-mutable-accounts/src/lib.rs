use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod duplicate_mutable_accounts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_player.player = ctx.accounts.payer.key();
        ctx.accounts.new_player.choice = None;
        Ok(())
    }

    pub fn rock_paper_scissors_shoot_insecure(
        ctx: Context<RockPaperScissorsInsecure>,
        player_one_choice: RockPaperScissors,
        player_two_choice: RockPaperScissors,
    ) -> Result<()> {
        if ctx.accounts.player_one.choice.is_some() || ctx.accounts.player_two.choice.is_some() {
            return Err(ErrorCode::ChoiceAlreadyMade.into());
        }

        ctx.accounts.player_one.choice = Some(player_one_choice);
        ctx.accounts.player_two.choice = Some(player_two_choice);
        Ok(())
    }

    pub fn rock_paper_scissors_shoot_secure(
        ctx: Context<RockPaperScissorsSecure>,
        player_one_choice: RockPaperScissors,
        player_two_choice: RockPaperScissors,
    ) -> Result<()> {
        if ctx.accounts.player_one.choice.is_some() || ctx.accounts.player_two.choice.is_some() {
            return Err(ErrorCode::ChoiceAlreadyMade.into());
        }

        require!(ctx.accounts.player_one.key() != ctx.accounts.player_two.key(), ErrorCode::PlayersCannotBeSame);

        ctx.accounts.player_one.choice = Some(player_one_choice);
        ctx.accounts.player_two.choice = Some(player_two_choice);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8
    )]
    pub new_player: Account<'info, PlayerState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RockPaperScissorsInsecure<'info> {
    #[account(mut)]
    pub player_one: Account<'info, PlayerState>,
    #[account(mut)]
    pub player_two: Account<'info, PlayerState>,
}

#[derive(Accounts)]
pub struct RockPaperScissorsSecure<'info> {
    #[account(
        mut,
        constraint = player_one.key() != player_two.key()
    )]
    pub player_one: Account<'info, PlayerState>,
    #[account(mut)]
    pub player_two: Account<'info, PlayerState>,
}

#[account]
pub struct PlayerState {
    player: Pubkey,
    choice: Option<RockPaperScissors>,
}

#[derive(Clone, Copy, BorshDeserialize, BorshSerialize)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Both players cannot have made a choice already.")]
    ChoiceAlreadyMade,

    #[msg("Players cannot be the same.")]
    PlayersCannotBeSame,
}
