use crate::language::handler::LanguageHandler;
use crate::utils::confirmation::Confirmation;
use crate::database::moderation::Punishment;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::map_str;

use poise::serenity_prelude::EditMessage;

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("delcase", "delete-case", "delete-punishment", "delpun"),
required_bot_permissions = "ADD_REACTIONS | MANAGE_MESSAGES"
)]
pub async fn delete_punishment(
    ctx: Context<'_>,
    punishment_id: u64,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let db = &ctx.data().db;
    let punishment = Punishment::get(
        db,
        ctx.guild_id().unwrap(),
        punishment_id,
    ).await;

    if punishment.is_none() {
        return Err(lang.translate_v("punishment.not_found", map_str!("id" => punishment_id)));
    }

    let mut confirmation = Confirmation::new(ctx, lang.translate("punishment.delete_confirmation"));
    let mut message = confirmation.send().await;

    if !confirmation.accepted {
        return Ok(());
    }

    Punishment::delete(db, punishment_id).await;
    let embed = embeds.success(
        &lang.translate("embed_title.delete_punishment"),
        &lang.translate_v("delete_punishment.success_description", map_str! {
                "id" => punishment_id
        }),
    ).await;

    message.edit(
        &ctx.http(),
        EditMessage::new().embed(embed)
    ).await.expect("cannot edit message");

    Ok(())
}