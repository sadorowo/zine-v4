use crate::database::moderation::{Punishment, PunishmentAction};
use poise::serenity_prelude::{Member, Timestamp};
use crate::commands::moderation::check_hierarchy;
use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::{map_str, no_md};

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("tm", "shutupbydiscord", "shutdsc"),
required_permissions = "MODERATE_MEMBERS",
required_bot_permissions = "MODERATE_MEMBERS",
)]
pub async fn timeout(
    ctx: Context<'_>,
    mut user: Member,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if let Some(timeout_error) = check_hierarchy(ctx, user.clone()).await {
        return Err(lang.translate(&timeout_error));
    }

    if user.permissions(&ctx.cache()).unwrap().administrator() {
        return Err(lang.translate("timeout.administrator"));
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());
    match user.disable_communication_until_datetime(
        &ctx.http(),
        Timestamp::from(chrono::Utc::now() + chrono::Duration::try_days(28).unwrap()),
    ).await {
        Ok(_) => {
            Punishment::new(
                &ctx.data().db,
                user.user.id,
                ctx.guild_id().unwrap(),
                reason.clone(),
                ctx.author().id,
                PunishmentAction::Timeout,
            ).await;

            let embed = embeds.success(
                &lang.translate("embed_title.timeout"),
                &lang.translate_v("timeout.success_description", map_str! {
                    "user" => no_md!(user.user.tag()),
                    "reason" => no_md!(reason)
                }),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => {
            return Err(lang.translate("timeout.error_description"));
        }
    }
}