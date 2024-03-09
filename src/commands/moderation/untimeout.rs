use crate::database::moderation::{Punishment, PunishmentAction};
use crate::commands::moderation::check_hierarchy;
use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::{map_str, no_md};

use poise::serenity_prelude::{Member, Timestamp};

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("utm", "unshutupbydiscord", "unshutdiscord"),
required_permissions = "MODERATE_MEMBERS",
required_bot_permissions = "MODERATE_MEMBERS",
)]
pub async fn untimeout(
    ctx: Context<'_>,
    mut user: Member,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if let Some(untimeout_error) = check_hierarchy(ctx, user.clone()).await {
        return Err(lang.translate(&untimeout_error));
    }

    if user.communication_disabled_until.is_none()
        || user.communication_disabled_until.unwrap() < Timestamp::from(chrono::Utc::now())
    {
        return Err(lang.translate("untimeout.not_timeouted"));
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());
    match user.enable_communication(&ctx.http()).await {
        Ok(_) => {
            Punishment::new(
                ctx.data().db.clone(),
                user.user.id,
                ctx.guild_id().unwrap(),
                reason.clone(),
                ctx.author().id,
                PunishmentAction::UnTimeout
            ).await;

            let embed = embeds.success(
                &lang.translate("embed_title.untimeout"),
                &lang.translate_v("untimeout.success_description", map_str! {
                    "user" => no_md!(user.user.tag()),
                    "reason" => no_md!(reason)
                }),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => {
            return Err(lang.translate("untimeout.error_description"))
        }
    }
}