use crate::database::moderation::{Punishment, PunishmentAction, TempMute};
use crate::commands::moderation::check_hierarchy;
use crate::database::guild_config::GuildConfig;
use crate::language::handler::LanguageHandler;
use crate::models::duration::Duration;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::{map_str, no_md};

use poise::serenity_prelude::Member;
use std::time;

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("tm", "mute-for-the-moment", "tmute"),
required_permissions = "MODERATE_MEMBERS",
required_bot_permissions = "MANAGE_ROLES"
)]
pub async fn tempmute(
    ctx: Context<'_>,
    user: Member,
    duration: Duration,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let config = GuildConfig::from_context(&ctx).await;
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let guild = ctx.guild().unwrap().clone();

    if let Some(tempmute_error) = check_hierarchy(ctx, user.clone()).await {
        return Err(lang.translate(&tempmute_error));
    }

    if duration.0 > time::Duration::from_secs(60 * 60 * 24 * 365) {
        return Err(lang.translate("tempmute.invalid_duration").to_string());
    }

    if config.moderation.mute_role.is_none() ||
        guild.roles.get(&config.moderation.mute_role.unwrap()).is_none()
    {
        return Err(lang.translate("tempmute.no_mute_role"));
    }

    let db = &ctx.data().db;
    if user.roles.contains(&config.moderation.mute_role.unwrap()) ||
        TempMute::is_muted(db, guild.id, user.user.id).await
    {
        return Err(lang.translate("tempmute.already_muted"));
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());
    match user.add_role(
        &ctx.http(),
        config.moderation.mute_role.unwrap(),
    ).await {
        Ok(_) => {
            Punishment::new(
                db,
                user.user.id,
                ctx.guild_id().unwrap(),
                reason.clone(),
                ctx.author().id,
                PunishmentAction::TempMute
            ).await;

            TempMute::new(
                db,
                user.user.id,
                ctx.guild_id().unwrap(),
                duration.0,
            ).await;

            let embed = embeds.success(
                &lang.translate("embed_title.tempmute"),
                &lang.translate_v("tempmute.success_description", map_str! {
                    "user" => no_md!(user.user.tag()),
                    "reason" => no_md!(reason),
                    "duration" => duration
                }),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => {
            return Err(lang.translate("tempmute.error_description"))
        }
    }
}