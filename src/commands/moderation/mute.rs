use crate::database::moderation::{Punishment, PunishmentAction};
use crate::commands::moderation::check_hierarchy;
use crate::database::guild_config::GuildConfig;
use crate::language::handler::LanguageHandler;
use poise::serenity_prelude::Member;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::{map_str, no_md};

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("m", "shutup", "shut"),
required_permissions = "MODERATE_MEMBERS",
required_bot_permissions = "MANAGE_ROLES",
)]
pub async fn mute(
    ctx: Context<'_>,
    user: Member,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let config = GuildConfig::from_context(&ctx).await;
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let guild = ctx.guild().unwrap().clone();

    if let Some(mute_error) = check_hierarchy(ctx, user.clone()).await {
        return Err(lang.translate(&mute_error));
    }

    if config.moderation.mute_role.is_none() ||
        guild.roles.get(&config.moderation.mute_role.unwrap()).is_none()
    {
        return Err(lang.translate("mute.no_mute_role"));
    }

    if user.roles.contains(&config.moderation.mute_role.unwrap()) {
        return Err(lang.translate("mute.already_muted"));
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());
    match user.add_role(
        &ctx.http(),
        config.moderation.mute_role.unwrap(),
    ).await {
        Ok(_) => {
            Punishment::new(
                ctx.data().db.clone(),
                user.user.id,
                ctx.guild_id().unwrap(),
                reason.clone(),
                ctx.author().id,
                PunishmentAction::Mute
            ).await;

            let embed = embeds.success(
                &lang.translate("embed_title.mute"),
                &lang.translate_v("mute.success_description", map_str! {
                    "user" => no_md!(user.user.tag()),
                    "reason" => no_md!(reason)
                }),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => {
            return Err(lang.translate("mute.error_description"))
        }
    }
}