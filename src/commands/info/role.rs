use crate::language::handler::LanguageHandler;
use poise::serenity_prelude::Role;
use crate::theme::embeds::Embeds;
use crate::{no_md, to_timestamp};
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "info"
)]
pub async fn role(
    ctx: Context<'_>,
    #[rest]
    role: Role,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let embed = embeds.info(
            &lang.translate("embed_title.role"),
            "",
        ).await
        .fields([
            (
                lang.translate("role.name"),
                format!("`{}`", no_md![role.name]),
                true
            ),
            (
                lang.translate("role.id"),
                role.id.to_string(),
                true
            ),
            (
                lang.translate("role.color"),
                role.colour.0.to_string(),
                true
            ),
            (
                lang.translate("role.position"),
                role.position.to_string(),
                true
            ),
            (
                lang.translate("role.mentionable"),
                lang.translate_bool(role.mentionable),
                true
            ),
            (
                lang.translate("role.hoist"),
                lang.translate_bool(role.hoist),
                true
            ),
            (
                lang.translate("role.managed"),
                lang.translate_bool(role.managed),
                true
            ),
            (
                lang.translate("role.created_at"),
                to_timestamp![role.id.created_at()],
                true
            )
        ]);

    embeds.send(embed).await;
    Ok(())
}