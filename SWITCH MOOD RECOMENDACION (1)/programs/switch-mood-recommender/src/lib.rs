use anchor_lang::prelude::*;

declare_id!("9zwwutCcUjhyJkezNA66ASfQ2YNviCfvwRV24L5rmnU7");

#[program]
pub mod switch_mood_recommender {
    use super::*;

    pub fn create_mood(
        ctx: Context<CreateMood>,
        username: String,
        mood: String
    ) -> Result<()> {

        let mood_account = &mut ctx.accounts.mood_account;

        mood_account.username = username;
        mood_account.mood = mood.clone();
        mood_account.game = recommend_game(&mood);

        Ok(())
    }

    pub fn update_mood(
        ctx: Context<UpdateMood>,
        new_mood: String
    ) -> Result<()> {

        let mood_account = &mut ctx.accounts.mood_account;

        mood_account.mood = new_mood.clone();
        mood_account.game = recommend_game(&new_mood);

        Ok(())
    }

    pub fn delete_mood(
        _ctx: Context<DeleteMood>
    ) -> Result<()> {

        Ok(())
    }
}

fn recommend_game(mood: &String) -> String {

    if mood == "relajado" {
        return "Animal Crossing".to_string();
    }

    if mood == "competitivo" {
        return "Mario Kart".to_string();
    }

    if mood == "aventura" {
        return "Zelda Breath of the Wild".to_string();
    }

    if mood == "pelea" {
        return "Super Smash Bros".to_string();
    }

    "Mario Odyssey".to_string()
}

#[derive(Accounts)]
pub struct CreateMood<'info> {

    #[account(
        init,
        seeds = [b"mood", user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + 32 + 64 + 64 + 64
    )]
    pub mood_account: Account<'info, MoodAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMood<'info> {

    #[account(
        mut,
        seeds = [b"mood", user.key().as_ref()],
        bump
    )]
    pub mood_account: Account<'info, MoodAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteMood<'info> {

    #[account(
        mut,
        close = user,
        seeds = [b"mood", user.key().as_ref()],
        bump
    )]
    pub mood_account: Account<'info, MoodAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct MoodAccount {

    pub username: String,
    pub mood: String,
    pub game: String,
}