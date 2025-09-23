use poise::serenity_prelude::{Color, CreateEmbed};

// #[poise::command(slash_command)]
// pub async fn send_embed(ctx: Context<'_>) -> Result<(), Error> {
//     let embed = serenity::CreateEmbed::new()
//         .title("Tytuł Wspaniałego Embeda")
//         .color(0x00FF00)
//         .description("To jest **bardzo ważna** wiadomość embed, wysłana za pomocą **Poise**.")
//         .field("Pole 1", "Wartość pola numer jeden.", true) // true oznacza inline
//         .field("Pole 2", "Wartość drugiego pola obok.", true)
//         .field("Pole 3", "To pole zajmuje całą linię.", false) // false oznacza, że nie jest inline
//         .footer(serenity::CreateEmbedFooter::new("Stopka z datą")
//         )
//         .timestamp(serenity::Timestamp::now());
//     ctx.send(poise::CreateReply::default().embed(embed)).await?;
//
//     Ok(())
// }

pub fn generate_embed(success: bool, fury_dice: u8, response: String) -> CreateEmbed {
    match (success, fury_dice) {
        (true, 1) => CreateEmbed::new()
            .title("Sukces z komplikacją!")
            .color(Color::DARK_ORANGE)
            .description(response),
        (true, 6) => CreateEmbed::new()
            .title("Krytyczny sukces !")
            .color(Color::DARK_GREEN)
            .description(response),
        (false, 6) => CreateEmbed::new()
            .title("Porażka z pocieszeniem !")
            .color(Color::DARK_MAGENTA)
            .description(response),
        (false, 1) => CreateEmbed::new()
            .title("Sromotna porażka z komplikacją !")
            .color(Color::DARK_RED)
            .description(response),
        (true, _) => CreateEmbed::new()
            .title("Sukces !")
            .color(Color::GOLD)
            .description(response),
        (false, _) => CreateEmbed::new()
            .title("Porażka !")
            .color(Color::RED)
            .description(response),
    }
}
