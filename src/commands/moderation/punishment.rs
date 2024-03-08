use crate::{map_str, no_md, to_timestamp};
use crate::language::handler::LanguageHandler;
use crate::database::moderation::Punishment;
use poise::serenity_prelude::CacheHttp;
use crate::theme::embeds::Embeds;
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("case"),
)]
pub async fn punishment(
    ctx: Context<'_>,
    punishment_id: u64,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let db = ctx.data().db.clone();
    let punishment = Punishment::get(
        db,
        ctx.guild_id().unwrap(),
        punishment_id,
    ).await;

    if punishment.is_none() {
        return Err(lang.translate_v("punishment.not_found", map_str!("id" => punishment_id)));
    }

    let punishment = punishment.unwrap();
    let permissions = ctx.author_member().await.unwrap().permissions(ctx.cache().unwrap());

    if permissions.is_ok()
        && punishment.user_id != ctx.author().id.0
        && !permissions.unwrap().moderate_members()
    {
        return Err(lang.translate("punishment.not_allowed"));
    }

    let mut embed = embeds.success(
        &lang.translate("embed_title.punishment"),
        &lang.translate_v("punishment.info_description", map_str!("id" => punishment_id)),
    ).await;

    embed.fields([
        (
            &lang.translate("punishment.info_fields.punishment_id"),
            punishment.punishment_id.to_string(),
            false
        ),
        (
            &lang.translate("punishment.info_fields.punishment_type"),
            punishment.punishment_type.to_string(),
            false
        ),
        (
            &lang.translate("punishment.info_fields.user_id"),
            punishment.user_id.to_string(),
            false
        ),
        (
            &lang.translate("punishment.info_fields.moderator_id"),
            punishment.moderator_id.to_string(),
            false
        ),
        (
            &lang.translate("punishment.info_fields.reason"),
            no_md!(punishment.reason),
            false
        ),
        (
            &lang.translate("punishment.info_fields.created_at"),
            to_timestamp![punishment.created_at],
            false
        )
    ]);

    embeds.send(embed).await;
    Ok(())
}