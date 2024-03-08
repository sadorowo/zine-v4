use crate::database::guild_config::GuildConfig;
use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use crate::models::color::Color;
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "config",
aliases("th", "themed", "emb", "embeds"),
subcommands("success", "warning", "info", "error"),
required_permissions = "MANAGE_GUILD",
)]
pub async fn theme(ctx: Context<'_>, _prefix: String) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    Err(lang.translate("subcommand_use"))
}

#[poise::command(
slash_command,
prefix_command,
category = "config",
aliases("succ", "s"),
required_permissions = "MANAGE_GUILD",
)]
async fn success(
    ctx: Context<'_>,
    color: Color
) -> Result<(), String> {
    let mut config: GuildConfig = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    config.theme.color_success = color.0;
    match config.save(ctx).await {
        Ok(_) => {
            let embed = embeds.success(
                &lang.translate("embed_title.theme_updated"),
                &lang.translate("theme.color_change_success"),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }
        Err(_) => return Err(lang.translate("theme.color_change_error")),
    }
}

#[poise::command(
slash_command,
prefix_command,
category = "config",
aliases("warn", "w"),
required_permissions = "MANAGE_GUILD",
)]
async fn warning(
    ctx: Context<'_>,
    color: Color
) -> Result<(), String> {
    let mut config: GuildConfig = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    config.theme.color_warning = color.0;
    match config.save(ctx).await {
        Ok(_) => {
            let embed = embeds.success(
                &lang.translate("embed_title.theme_updated"),
                &lang.translate("theme.color_change_success"),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }
        Err(_) => return Err(lang.translate("theme.color_change_error")),
    }
}

#[poise::command(
slash_command,
prefix_command,
category = "config",
aliases("i", "information"),
required_permissions = "MANAGE_GUILD",
)]
async fn info(
    ctx: Context<'_>,
    color: Color
) -> Result<(), String> {
    let mut config: GuildConfig = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    config.theme.color_info = color.0;
    match config.save(ctx).await {
        Ok(_) => {
            let embed = embeds.success(
                &lang.translate("embed_title.theme_updated"),
                &lang.translate("theme.color_change_success"),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }
        Err(_) => return Err(lang.translate("theme.color_change_error")),
    }
}

#[poise::command(
slash_command,
prefix_command,
category = "config",
aliases("err", "e"),
required_permissions = "MANAGE_GUILD",
)]
async fn error(
    ctx: Context<'_>,
    color: Color
) -> Result<(), String> {
    let mut config: GuildConfig = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    config.theme.color_error = color.0;
    match config.save(ctx).await {
        Ok(_) => {
            let embed = embeds.success(
                &lang.translate("embed_title.theme_updated"),
                &lang.translate("theme.color_change_success"),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }
        Err(_) => return Err(lang.translate("theme.color_change_error")),
    }
}