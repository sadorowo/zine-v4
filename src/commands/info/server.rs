use crate::language::handler::LanguageHandler;
use poise::serenity_prelude::Mentionable;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::{no_md, to_timestamp};

#[poise::command(
slash_command,
prefix_command,
guild_only,
category = "info",
aliases("guild", "guildinfo", "serverinfo"),
)]
pub async fn server(ctx: Context<'_>) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let guild = ctx.guild().unwrap();
    let mut embed = embeds.info(
        &lang.translate("embed_title.server"),
        "",
    ).await;

    if let Some(icon) = guild.icon_url() {
        embed.thumbnail(icon);
    }

    if let Some(banner) = guild.banner_url() {
        embed.image(banner);
    }

    embed.fields([
        (
            lang.translate("server.name"),
            no_md![guild.name],
            true
        ),
        (
            lang.translate("server.id"),
            guild.id.to_string(),
            true
        ),
        (
            lang.translate("server.owner"),
            guild.owner_id.mention().to_string(),
            true
        ),
        (
            lang.translate("server.members"),
            guild.member_count.to_string(),
            true
        ),
        (
            lang.translate("server.roles"),
            guild.roles.len().to_string(),
            true
        ),
        (
            lang.translate("server.channels"),
            guild.channels.len().to_string(),
            true
        ),
        (
            lang.translate("server.emojis"),
            guild.emojis.len().to_string(),
            true
        ),
        (
            lang.translate("server.created_at"),
            to_timestamp![guild.id.created_at()],
            true
        ),
    ]);

    embeds.send(embed).await;
    Ok(())
}