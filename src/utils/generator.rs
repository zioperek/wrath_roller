use poise::serenity_prelude::{Color, CreateEmbed};

// TODO: Add info about test level

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
