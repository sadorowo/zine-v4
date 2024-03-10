use poise::serenity_prelude::User;

use crate::database::moderation::{Punishment, PunishmentAction};
use crate::language::handler::LanguageHandler;
use crate::commands::moderation::check_ban;
use crate::{map_str, no_md};
use crate::theme::embeds::Embeds;
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("b", "deport-forever"),
required_permissions = "BAN_MEMBERS",
required_bot_permissions = "BAN_MEMBERS"
)]
pub async fn ban(
    ctx: Context<'_>,
    user: User,
    #[min = 0]
    #[max = 7]
    days: Option<u8>,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if let Some(ban_error) = check_ban(ctx, user.clone()).await {
        return Err(lang.translate(&ban_error));
    }

    let guild = ctx.guild().unwrap().clone();
    if guild
        .bans(&ctx.http(), None, None)
        .await
        .unwrap()
        .iter()
        .any(|ban| ban.user.id == user.id)
    {
        return Err(lang.translate("ban.already_banned"));
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());
    let days = days.unwrap_or(0);

    if days > 7 {
        return Err(lang.translate("ban.days_error").to_string());
    }

    match guild.ban_with_reason(
        &ctx.http(),
        user.clone(),
        days,
        &reason
    ).await {
        Ok(_) => {
            Punishment::new(
                &ctx.data().db,
                user.id,
                ctx.guild_id().unwrap(),
                reason.clone(),
                ctx.author().id,
                PunishmentAction::Ban
            ).await;
            
            let embed = embeds.success(
                &lang.translate("embed_title.ban"),
                &lang.translate_v("ban.success_description", map_str! {
                    "user" => no_md!(user.tag()),
                    "reason" => no_md!(reason)
                }),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => {
            return Err(lang.translate("ban.error_description"))
        }
    }
}