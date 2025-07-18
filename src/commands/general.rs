use crate::utils::dice::get_full_roll;
use poise::serenity_prelude as serenity;
pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn skill_check(
    ctx: Context<'_>,
    #[description = "Ilość kości"] dice: u32,
) -> Result<(), Error> {
    let (rolls, number_of_successes, fury_dice) = get_full_roll(dice);
    let response = format!(
        "{:?} rolled with {} successes. With fury dice {:?}",
        rolls, number_of_successes, fury_dice
    );
    ctx.say(response).await?;
    Ok(())
}
