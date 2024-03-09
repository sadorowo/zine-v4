use poise::serenity_prelude::{ChannelType, GuildChannel};
use crate::language::handler::LanguageHandler;
use crate::{no_md, to_timestamp, truncate};
use crate::theme::embeds::Embeds;
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "info"
)]
pub async fn channel(
    ctx: Context<'_>,
    #[channel_types("Text", "Voice", "News")]
    #[rest]
    channel: GuildChannel,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if channel.kind == ChannelType::Unknown(0) {
        return Err(lang.translate("channel.invalid_type"));
    }

    let embed = embeds.info(
        &lang.translate("embed_title.channel"),
        "",
    ).await
    .fields([
        (
            lang.translate("channel.name"),
            no_md![channel.name],
            true
        ),
        (
            lang.translate("channel.id"),
            channel.id.to_string(),
            true
        ),
        (
            lang.translate("channel.category"),
            if channel.parent_id.is_some() {
                format!("<#{}>", channel.parent_id.unwrap().to_string())
            } else { lang.translate("not_accessible") },
            true
        ),
        (
            lang.translate("channel.created_at"),
            to_timestamp![channel.id.created_at()],
            true
        ),
        (
            lang.translate("channel.topic"),
            truncate! {
                no_md! [channel.topic.unwrap_or_else(|| lang.translate("channel.no_topic"))],
                20
            },
            false
        ),
        (
            lang.translate("channel.type"),
            lang.translate(&format!("channel.types.{}", channel.kind.name().to_lowercase())),
            true
        )
    ]);

    embeds.send(embed).await;
    Ok(())
}