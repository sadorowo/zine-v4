use crate::database::guild_config::GuildConfig;
use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "config",
aliases("pr", "prf"),
subcommands("prefix_add", "prefix_remove", "prefix_list"),
required_permissions = "MANAGE_GUILD",
)]
pub async fn prefix(ctx: Context<'_>, _prefix: String) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    Err(lang.translate("subcommand_use"))
}

#[poise::command(
slash_command,
prefix_command,
rename = "add",
category = "config",
aliases("+"),
required_permissions = "MANAGE_GUILD",
)]
async fn prefix_add(
    ctx: Context<'_>,
    prefix: String,
) -> Result<(), String> {
    let mut config = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if config.prefixes.contains(&prefix) {
        return Err(lang.translate("prefix.already_added"));
    }

    config.prefixes.push(prefix);
    match config.save(&ctx).await {
        Ok(_) => {
            let embed = embeds.success(
                &lang.translate("embed_title.prefix.add_success"),
                &lang.translate("prefix.add_success"),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }
        Err(_) => return Err(lang.translate("prefix.add_error")),
    }
}

#[poise::command(
slash_command,
prefix_command,
rename = "remove",
category = "config",
aliases("-"),
required_permissions = "MANAGE_GUILD",
)]
async fn prefix_remove(
    ctx: Context<'_>,
    prefix: String,
) -> Result<(), String> {
    let mut config = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if !config.prefixes.contains(&prefix) {
        return Err(lang.translate("prefix.not_found"));
    }

    config.prefixes.retain(|p| p != &prefix);
    match config.save(&ctx).await {
        Ok(_) => {
            let embed = embeds.success(
                &lang.translate("embed_title.prefix.remove_success"),
                &lang.translate("prefix.remove_success"),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }
        Err(_) => return Err(lang.translate("prefix.remove_error")),
    }
}

#[poise::command(
slash_command,
prefix_command,
rename = "list",
category = "config",
aliases("l"),
)]
async fn prefix_list(
    ctx: Context<'_>,
) -> Result<(), String> {
    let config = GuildConfig::from_context(&ctx).await;
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let mut prefixes = String::new();
    for prefix in &config.prefixes {
        prefixes.push_str(&format!("`{}` ", prefix));
    }

    let embed = embeds.success(
        &lang.translate("embed_title.prefix.list"),
        &format!("{}\n\n{}", lang.translate("prefix.list"), prefixes),
    ).await;

    embeds.send(embed).await;
    Ok(())
}