use crate::database::moderation::{Punishment, PunishmentAction, TempMute};
use crate::commands::moderation::check_hierarchy;
use crate::database::guild_config::GuildConfig;
use crate::language::handler::LanguageHandler;
use poise::serenity_prelude::Member;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use crate::{map_str, no_md};

#[poise::command(
slash_command,
prefix_command,
category = "moderation",
aliases("um", "unshutup", "unshut"),
required_permissions = "MODERATE_MEMBERS",
required_bot_permissions = "MANAGE_ROLES",
)]
pub async fn unmute(
    ctx: Context<'_>,
    user: Member,
    #[rest]
    reason: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let config = GuildConfig::from_context(&ctx).await;
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if let Some(unmute_error) = check_hierarchy(ctx, user.clone()).await {
        return Err(lang.translate(&unmute_error));
    }

    if !config.moderation.mute_role.is_some() {
        return Err(lang.translate("unmute.no_mute_role"));
    }

    let db = &ctx.data().db;
    let temp_mute_data = TempMute::get_mute_data(
        db,
        ctx.guild_id().unwrap(), 
        user.user.id
    ).await;
    
    if !user.roles.contains(&config.moderation.mute_role.unwrap()) && temp_mute_data.is_none() {
        return Err(lang.translate("unmute.not_muted"));
    }

    let reason = reason.unwrap_or(lang.translate("no_reason").to_string());
    match user.remove_role(
        &ctx.http(),
        config.moderation.mute_role.unwrap()
    ).await {
        Ok(_) => {
            if temp_mute_data.is_some() {
                temp_mute_data.unwrap().self_destruct(&ctx.data().db.clone()).await;
            }
            
            Punishment::new(
                db,
                user.user.id,
                ctx.guild_id().unwrap(),
                reason.clone(),
                ctx.author().id,
                PunishmentAction::UnMute
            ).await;

            let embed = embeds.success(
                &lang.translate("embed_title.unmute"),
                &lang.translate_v("unmute.success_description", map_str! {
                    "user" => no_md!(user.user.tag()),
                    "reason" => no_md!(reason)
                }),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(_) => {
            return Err(lang.translate("unmute.error_description"))
        }
    }
}