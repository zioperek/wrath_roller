use crate::utils::{dice::get_full_roll, generator::generate_embed};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

#[poise::command(slash_command, prefix_command)]
pub async fn skill_check(
    ctx: Context<'_>,
    #[description = "Ilość kości"] kosci: u8,
    #[description = "Poziom trudności"] trudnosc: u8,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let full_dice_roll = get_full_roll(kosci);
    let success = full_dice_roll.number_of_successes >= trudnosc;
    let response = format!(
        "{}[{}] z {} ikonami.",
        full_dice_roll.rolls.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", "),
        full_dice_roll.fury_dice, full_dice_roll.number_of_successes
    );
    let embed = generate_embed(success, full_dice_roll.fury_dice, response);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;

    Ok(())
}
