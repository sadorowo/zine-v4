use crate::database::moderation::{Punishment, PunishmentAction, TempBan};
use poise::serenity_prelude::{CacheHttp, User};
use crate::language::handler::LanguageHandler;
use crate::commands::moderation::check_ban;
use crate::models::duration::Duration;
use crate::{map_str, no_md};
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use std::time;

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("tb", "deport-for-the-moment"),
required_permissions = "BAN_MEMBERS",
required_bot_permissions = "BAN_MEMBERS"
)]
pub async fn tempban(
    ctx: Context<'_>,
    user: User,
    #[min = 0]
    #[max = 7]
    days: Option<u8>,
    duration: Duration,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if let Some(ban_error) = check_ban(ctx, user.clone()).await {
        return Err(lang.translate(&ban_error));
    }

    if ctx.guild().unwrap()
        .bans(&ctx.http()).await.unwrap()
        .iter()
        .any(|ban| ban.user.id == user.id)
    {
        return Err(lang.translate("ban.already_banned"));
    }

    if duration.0 > time::Duration::from_secs(60 * 60 * 24 * 365) {
        return Err(lang.translate("tempban.invalid_duration").to_string());
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());
    let days = days.unwrap_or(0);

    if days > 7 {
        return Err(lang.translate("ban.days_error").to_string());
    }

    match ctx.guild().unwrap().ban_with_reason(
        &ctx.http(),
        user.clone(),
        days,
        &reason
    ).await {
        Ok(_) => {
            Punishment::new(
                ctx.data().db.clone(),
                user.id,
                ctx.guild_id().unwrap(),
                reason.clone(),
                ctx.author().id,
                PunishmentAction::TempBan
            ).await;

            TempBan::new(
                ctx.data().db.clone(),
                user.id,
                ctx.guild_id().unwrap(),
                duration.0,
            ).await;

            let embed = embeds.success(
                &lang.translate("embed_title.tempban"),
                &lang.translate_v("tempban.success_description", map_str! {
                    "user" => no_md!(user.tag()),
                    "reason" => no_md!(reason),
                    "duration" => duration
                }),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => {
            return Err(lang.translate("tempban.error_description"))
        }
    }
}