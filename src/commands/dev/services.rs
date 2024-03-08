use crate::language::handler::LanguageHandler;
use crate::{to_timestamp, map_str};
use crate::theme::embeds::Embeds;
use crate::commands::Context;

#[poise::command(
slash_command,
prefix_command,
owners_only,
category = "dev",
subcommands("services_enable", "services_disable", "services_show"),
aliases("srv"),
)]
pub async fn services(ctx: Context<'_>, _action: String) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    Err(lang.translate("subcommand_use"))
}

#[poise::command(
slash_command,
prefix_command,
owners_only,
rename = "show",
category = "dev",
aliases("display", "d", "s"),
)]
pub async fn services_show(ctx: Context<'_>) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let services = ctx.clone().data().services.clone();
    let mut embed = embeds.info(
        &lang.translate("embed_title.services.display"),
        ""
    ).await;

    for mut service in services.iter().cloned() {
        embed.field(
            service.name,
            &lang.translate_v("services.details", map_str! {
                "enabled" => lang.translate_bool(service.is_running()),
                "uptime" => to_timestamp!(service.started_date.unwrap()),
                "interval" => service.interval
            }),
            true
        );
    }

    embeds.send(embed).await;
    Ok(())
}

#[poise::command(
slash_command,
prefix_command,
owners_only,
rename = "enable",
category = "dev",
aliases("on"),
)]
pub async fn services_enable(ctx: Context<'_>, service_name: String) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let services = ctx.clone().data().services.clone();
    let service = services.iter().find(|s| s.name == service_name).cloned();

    match service {
        Some(mut service) => {
            service.resume();
            let embed = embeds.success(
                &lang.translate("embed_title.services.enabled"),
                &lang.translate("services.enabled")
            ).await;

            embeds.send(embed).await;
            Ok(())
        },
        None => {
            Err(lang.translate("services.not_found").to_string())
        }
    }
}

#[poise::command(
slash_command,
prefix_command,
owners_only,
rename = "disable",
category = "dev",
aliases("off"),
)]
pub async fn services_disable(ctx: Context<'_>, service_name: String) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let services = ctx.clone().data().services.clone();
    let service = services.iter().find(|s| s.name == service_name).cloned();

    match service {
        Some(mut service) => {
            service.pause();
            let embed = embeds.success(
                &lang.translate("embed_title.services.disabled"),
                &lang.translate("services.disabled")
            ).await;

            embeds.send(embed).await;
            Ok(())
        },
        None => {
            Err(lang.translate("services.not_found").to_string())
        }
    }
}