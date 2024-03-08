use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use tokio::time::Instant;
use crate::map_str;

#[poise::command(
slash_command,
prefix_command,
category = "bot",
aliases("pong", "latency", "lat")
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let t = Instant::now();
    let embed = embeds.info(
        &lang.translate("embed_title.ping"),
        &lang.translate("ping.pinging"),
    ).await;

    let pinging = embeds.send(embed).await;
    let embed = embeds.info(
        &lang.translate("embed_title.ping"),
        &lang.translate_v("ping.bot_latency", map_str! {
            "latency" => t.elapsed().as_millis()
        }),
    ).await;

    embeds.edit(pinging, embed).await;
    Ok(())
}