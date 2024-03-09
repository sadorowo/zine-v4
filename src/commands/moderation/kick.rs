use crate::database::moderation::{Punishment, PunishmentAction};
use crate::commands::moderation::check_hierarchy;
use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::{map_str, no_md};

use poise::serenity_prelude::Member;

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("k", "deport"),
required_permissions = "KICK_MEMBERS",
required_bot_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    user: Member,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if let Some(kick_error) = check_hierarchy(ctx, user.clone()).await {
        return Err(lang.translate(&kick_error));
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());

    match user.kick_with_reason(&ctx.http(), &reason).await {
        Ok(_) => {
            Punishment::new(
                ctx.data().db.clone(),
                user.user.id,
                ctx.guild_id().unwrap(),
                reason.clone(),
                ctx.author().id,
                PunishmentAction::Kick
            ).await;
            
            let embed = embeds.success(
                &lang.translate("embed_title.kick"),
                &lang.translate_v("kick.success_description", map_str! {
                    "user" => no_md!(user.user.tag()),
                    "reason" => no_md!(reason)
                }),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => {
            return Err(lang.translate("kick.error_description"))
        }
    }
}