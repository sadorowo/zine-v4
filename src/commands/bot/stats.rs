use crate::language::handler::LanguageHandler;
use crate::{fmt_bytes, to_timestamp};
use crate::theme::embeds::Embeds;
use crate::commands::Context;

use sysinfo::System;

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

    let embed = embeds.info(
        &lang.translate("embed_title.stats"),
        "",
    ).await
    .fields([
        (
            lang.translate("stats.guilds"),
            ctx.cache().guilds().len().to_string(),
            true
        ),
        (
            lang.translate("stats.uptime"),
            to_timestamp!(ctx.data().uptime),
            true
        ),
        (
            lang.translate("stats.users"),
            ctx.cache().users().len().to_string(),
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
            format!("{} {}",
                    System::name().unwrap_or(lang.translate("not_accessible")),
                    System::os_version().unwrap()
            ).to_string(),
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