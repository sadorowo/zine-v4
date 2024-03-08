use crate::database::moderation::{Punishment, PunishmentAction};
use crate::commands::moderation::check_hierarchy;
use crate::language::handler::LanguageHandler;
use poise::serenity_prelude::Member;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::{map_str, no_md};

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("w", "warning"),
required_permissions = "MODERATE_MEMBERS",
required_bot_permissions = "MODERATE_MEMBERS"
)]
pub async fn warn(
    ctx: Context<'_>,
    user: Member,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if let Some(warn_error) = check_hierarchy(ctx, user.clone()).await {
        return Err(lang.translate(&warn_error));
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());
    Punishment::new(
        ctx.data().db.clone(),
        user.user.id,
        ctx.guild_id().unwrap(),
        reason.clone(),
        ctx.author().id,
        PunishmentAction::Warn,
    ).await;

    let embed = embeds.success(
        &lang.translate("embed_title.warn"), 
        &lang.translate_v("warn.success_description", map_str! {
            "user" => no_md!(user.user.tag()),
            "reason" => no_md!(reason)
        }),
    ).await;

    embeds.send(embed).await;
    Ok(())
}