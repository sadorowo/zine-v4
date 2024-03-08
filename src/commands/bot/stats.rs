use crate::language::handler::LanguageHandler;
use poise::serenity_prelude::CacheHttp;
use crate::{fmt_bytes, to_timestamp};
use crate::theme::embeds::Embeds;
use sysinfo::{SystemExt, System};
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "bot",
aliases("statistics", "info")
)]
pub async fn stats(ctx: Context<'_>) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);
    let mem = System::new_all();

    let mut embed = embeds.info(
        &lang.translate("embed_title.stats"),
        "",
    ).await;

    embed.fields([
        (
            lang.translate("stats.guilds"),
            ctx.cache().unwrap().guilds().len().to_string(),
            true
        ),
        (
            lang.translate("stats.uptime"),
            to_timestamp!(ctx.data().uptime),
            true
        ),
        (
            lang.translate("stats.users"),
            ctx.cache().unwrap().users().len().to_string(),
            true
        ),
        (
            lang.translate("stats.memory"),
            format!("{}/{}",
                    fmt_bytes![mem.used_memory()],
                    fmt_bytes![mem.total_memory()]
            ).to_string(),
            true
        ),
        (
            lang.translate("stats.system"),
            format!("{} {}", mem.name().unwrap_or(lang.translate("not_accessible")), mem.os_version().unwrap()).to_string(),
            true
        ),
        (
            lang.translate("stats.commands"),
            ctx.framework().options.commands.len().to_string(),
            true
        ),
        (
            lang.translate("stats.version"),
            env!("CARGO_PKG_VERSION").to_string(),
            true
        ),
    ]);

    embeds.send(embed).await;
    Ok(())
}