use crate::{commands::Data, database::guild_config::GuildConfig, map_str, no_md};
use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use std::num::ParseIntError;
use std::sync::Arc;

use poise::{FrameworkOptions, PrefixFrameworkOptions, EditTracker, Command, FrameworkError, Context::Application, serenity_prelude::{
    CreateAllowedMentions, CreateEmbed,
    RoleParseError, ChannelParseError,
    MemberParseError,
    GuildParseError,
    GuildChannelParseError,
    EmojiParseError,
    UserParseError,
}};

use poise::serenity_prelude::{CreateAttachment, CreateMessage, ReactionType};
use crate::models::duration::DurationParseError;
use crate::models::color::ColorError;
use tokio::io::AsyncWriteExt;

pub type Context<'a> = poise::Context<'a, Data, String>;

pub fn init_framework_options(vec: Vec<Command<Data, String>>) -> FrameworkOptions<Data, String> {
    FrameworkOptions {
        prefix_options: PrefixFrameworkOptions {
            stripped_dynamic_prefix: Some(|_, m, data| Box::pin(async move {
                if m.guild_id.is_none() {
                    return Ok(None);
                }

                let guild_id: String = m.guild_id.unwrap().to_string();
                let config: GuildConfig = GuildConfig::from_raw(data.db.clone(), &guild_id).await;

                for prefix in config.prefixes {
                    if m.content.starts_with(&prefix) {
                        return Ok(Some(m.content.split_at(prefix.len())));
                    }
                }

                Ok(None)
            })),
            edit_tracker: Some(Arc::new(
                EditTracker::for_timespan(std::time::Duration::from_secs(3600))
            )),
            case_insensitive_commands: true,
            ..Default::default()
        },
        pre_command: |ctx| Box::pin(async move {
            if let Application(ctx) = ctx {
                let _ = ctx.defer().await;
            } else {
                let _ = ctx.channel_id().broadcast_typing(ctx.http()).await;
            }
        }),
        on_error: |error| {
            Box::pin(async move {
                let ctx = error.ctx();
                if ctx.is_none() {
                    return;
                }

                let mut embed: Option<CreateEmbed> = None;
                let ctx = ctx.unwrap();
                let mut embeds: Embeds = Embeds::from_context(ctx);
                let lang: LanguageHandler = LanguageHandler::from_context(ctx);

                match error {
                    FrameworkError::Command { error, .. } => {
                        let text = format!("{:#?}", error);

                        if text.len() > 1024 {
                            let mut file = tokio::fs::File::create("error.txt").await.unwrap();
                            let _ = file.write_all(text.as_bytes()).await;

                            let m = CreateMessage::default()
                                .content(lang.translate("errors.command.title"))
                                .add_file(CreateAttachment::file(&file, "error.txt").await.unwrap());

                            let e = ctx
                                .channel_id()
                                .send_message(ctx, m)
                                .await;

                            if e.is_err() {
                                println!("Error while sending error message: {:#?}", e);
                            }
                        } else {
                            embed = Some(embeds.error(
                                &lang.translate("errors.commands.title"),
                                &error,
                            ).await);
                        }
                    }

                    FrameworkError::ArgumentParse { ctx, error, input, .. } => {
                        embed = Some(embeds.warning(
                            &lang.translate("errors.arguments.title"),
                            &handle_bad_argument(ctx, error, input),
                        ).await);
                    }

                    FrameworkError::NsfwOnly { .. } => {
                        embed = Some(embeds.error(
                            &lang.translate("errors.nsfw.title"),
                            &lang.translate("errors.nsfw.description"),
                        ).await);
                    }

                    FrameworkError::MissingUserPermissions { missing_permissions, .. } => {
                        embed = Some(embeds.error(
                            &lang.translate("errors.missing_user_permissions.title"),
                            &if missing_permissions.is_some() {
                                lang.translate_v("errors.missing_user_permissions.description_detailed", map_str! {
                                    "permissions" => missing_permissions
                                    .unwrap()
                                    .to_string()
                                    .replace(" and ", ", ")
                                })
                            } else {
                                lang.translate("errors.missing_user_permissions.description")
                            },
                        ).await);
                    }

                    FrameworkError::MissingBotPermissions { missing_permissions, .. } => {
                        embed = Some(embeds.error(
                            &lang.translate("errors.missing_bot_permissions.title"),
                            &lang.translate_v("errors.missing_bot_permissions.description_detailed", map_str! {
                                    "permissions" => missing_permissions
                                    .to_string()
                                    .replace(" and ", ", ")
                                }),
                        ).await);
                    }

                    FrameworkError::CooldownHit { remaining_cooldown, ctx, .. } => {
                        let result = ctx.http().create_reaction(
                            ctx.channel_id(),
                            ctx.id().into(),
                            &ReactionType::Unicode("🕓".to_string()),
                        ).await;

                        if result.is_err() {
                            embed = Some(embeds.warning(
                                &lang.translate("errors.cooldown.title"),
                                &lang.translate_v("errors.cooldown.description", map_str! {
                                    "seconds" => remaining_cooldown.as_secs().to_string()
                                }),
                            ).await);
                        }
                    }
                    _ => {}
                }

                if !embed.is_none() {
                    embeds.send(embed.unwrap()).await;
                }
            })
        },
        allowed_mentions: Some(
            CreateAllowedMentions::default()
        ),
        commands: vec,
        ..Default::default()
    }
}

fn handle_bad_argument(ctx: poise::Context<Data, String>, error: Box<dyn std::error::Error + Send + Sync>, input: Option<String>) -> String {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    #[allow(unused_assignments)]
        let mut message: String = String::new();

    if error.is::<poise::TooFewArguments>() {
        message = lang.translate("errors.arguments.too_few");
    } else if error.is::<poise::TooManyArguments>() {
        message = lang.translate("errors.arguments.too_many");
    } else if error.is::<MemberParseError>() {
        message = lang.translate("errors.arguments.member");
    } else if error.is::<GuildParseError>() {
        message = lang.translate("errors.arguments.guild");
    } else if error.is::<RoleParseError>() {
        message = lang.translate("errors.arguments.role");
    } else if error.is::<ChannelParseError>() {
        message = lang.translate("errors.arguments.channel");
    } else if error.is::<EmojiParseError>() {
        message = lang.translate("errors.arguments.emoji");
    } else if error.is::<UserParseError>() {
        message = lang.translate("errors.arguments.user");
    } else if error.is::<GuildChannelParseError>() {
        message = lang.translate("errors.arguments.guild_channel");
    } else if error.is::<ColorError>() {
        message = lang.translate("errors.arguments.color");
    } else if error.is::<poise::InvalidBool>() {
        message = lang.translate("errors.arguments.bool");
    } else if error.is::<poise::InvalidChoice>() {
        message = lang.translate("errors.arguments.choice");
    } else if error.is::<ParseIntError>() {
        message = lang.translate("errors.arguments.number");
    } else if error.is::<DurationParseError>() {
        message = lang.translate("errors.arguments.duration");
    } else {
        message = error.to_string()
    }

    if !input.is_none() {
        let input = input.unwrap_or(lang.translate("not_accessible").to_string());

        format!(
            "{}\n\n```r\n{}\n```",
            message,
            to_underline(
                &no_md![ctx.invocation_string()],
                &no_md![input.as_str()],
            )
        )
    } else { message }
}

fn to_underline(text: &str, target: &str) -> String {
    let index = text.find(target).unwrap_or(text.len());
    let underline = "'".repeat(target.len());
    let padding = " ".repeat(index);

    format!("{}\n{}{}", text, padding, underline)
}
