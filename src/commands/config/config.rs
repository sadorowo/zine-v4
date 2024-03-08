use crate::database::guild_config::GuildConfig;
use crate::language::handler::LanguageHandler;
use crate::utils::discord::format_role;
use poise::serenity_prelude::Role;
use crate::theme::embeds::Embeds;
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "config",
aliases("c", "conf"),
subcommands("config_show", "mute_role"),
required_permissions = "MANAGE_GUILD",
)]
pub async fn config(ctx: Context<'_>, _prefix: String) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    Err(lang.translate("subcommand_use"))
}

#[poise::command(
slash_command,
prefix_command,
rename = "show",
category = "config",
aliases("display"),
)]
async fn config_show(
    ctx: Context<'_>,
) -> Result<(), String> {
    let config = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let mut embed = embeds.success(
        &lang.translate("embed_title.config.show"),
        ""
    ).await;

    embed.fields([
        (
            lang.translate("config.embed_field.mute_role"),
            format_role(ctx, config.moderation.mute_role),
            true
        )
    ]);

    embeds.send(embed).await;
    Ok(())
}

#[poise::command(
slash_command,
prefix_command,
category = "config",
aliases("mr", "mute-role"),
required_permissions = "MANAGE_GUILD",
)]
async fn mute_role(
    ctx: Context<'_>,
    #[rest]
    role: Role
) -> Result<(), String> {
    let mut config = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if role.managed {
        return Err(lang.translate("config.mute_role.managed"));
    }
    
    if role.id.0 == ctx.guild_id().unwrap().0 {
        return Err(lang.translate("config.mute_role.everyone"));
    }

    config.moderation.mute_role = Some(role.id);
    match config.save(ctx).await {
        Ok(_) => {
            let embed = embeds.success(
                &lang.translate("embed_title.config.mute_role"),
                &lang.translate("config.mute_role.success"),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => return Err(lang.translate("config.mute_role.error")),
    }
}