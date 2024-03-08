#![allow(unused_must_use)]
use crate::language::handler::LanguageHandler;
use crate::commands::Context;
use figlet_rs::FIGfont;
use crate::code_block;

#[poise::command(
slash_command,
prefix_command,
category = "util",
aliases("ascii-art", "asciiart", "ascii")
)]
pub async fn ascii_art(
    ctx: Context<'_>,
    #[rest]
    text: String
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let font = FIGfont::standard().unwrap();
    let figure = font.convert(text.as_str());

    if figure.is_none() {
        return Err(lang.translate("ascii_art.no_result"));
    }

    let figure = figure.unwrap().to_string();
    if figure.len() > 2000 {
        ctx.say(lang.translate("ascii_art.too_long")).await;
        return Ok(());
    }

    ctx.say(code_block! {
        "yaml",
        figure
    }).await;
    Ok(())
}