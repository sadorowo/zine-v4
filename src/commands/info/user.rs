use poise::serenity_prelude::{Mentionable, User};
use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use crate::{no_md, to_timestamp};
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
category = "info"
)]
pub async fn user(
    ctx: Context<'_>,
    #[rest]
    user: Option<User>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let user = user.unwrap_or_else(|| ctx.author().clone());
    let guild = ctx.guild().unwrap().clone();
    let member = guild.member(&ctx.http(), user.id).await;

    let mut embed = embeds.info(
        &lang.translate("embed_title.user"),
        "",
    ).await
    .image(user.face())
    .fields([
        (
            lang.translate("user.username"),
            no_md!(user.name),
            true
        ),
        (
            lang.translate("user.id"),
            user.id.to_string(),
            true
        ),
        (
            lang.translate("user.bot"),
            lang.translate_bool(user.bot),
            true
        ),
        (
            lang.translate("user.created_at"),
            to_timestamp![user.id.created_at()],
            true
        ),
    ]);

    if let Ok(member) = member {
        embed = embed.fields([
            (
                lang.translate("user.joined_at"),
                if !member.joined_at.is_none()
                { to_timestamp![member.joined_at.unwrap()] } else { lang.translate("not_accessible") },
                true
            ),
            (
                lang.translate("user.nick"),
                member.nick.clone().unwrap_or_else(|| lang.translate("user.no_nick")),
                true
            ),
            (
                lang.translate("user.roles"),
                if member.roles.len() > 0 {
                    member.roles.iter()
                        .filter(|role_id| role_id.to_string() != ctx.guild_id().unwrap().to_string())
                        .map(|role_id| role_id.mention().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                } else { lang.translate("user.no_roles") },
                true
            ),
        ]);
    }

    embeds.send(embed).await;
    Ok(())
}