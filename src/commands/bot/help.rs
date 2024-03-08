use crate::commands::utils::{get_command, starts_with_localized_name};
use crate::commands::{Context, Data, get_localized};
use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use std::collections::HashMap;
use crate::map_str;
use poise::Command;

pub async fn command_autocomplete<'a>(
    ctx: poise::Context<'a, Data, String>,
    partial: &'a str,
) -> impl Iterator<Item=String> + 'a {
    let filter = move |cmd: &&Command<Data, String>| {
        return (cmd.name.starts_with(partial) ||
        starts_with_localized_name(cmd, partial)) &&
        !cmd.hide_in_help
    };

    ctx.framework()
        .options()
        .commands
        .iter()
        .filter(move |cmd| filter(cmd))
        .map(move |cmd| get_localized(
            ctx.clone(),
            cmd,
            |c| (c.name_localizations.clone(), c.name.clone()
        )))
}

#[poise::command(
slash_command,
prefix_command,
category = "bot",
aliases("h", "halp", "cmds", "commands"),
)]
pub async fn help(
    ctx: Context<'_>,
    #[autocomplete = "command_autocomplete"]
    command_name: Option<String>,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    if command_name.is_none() {
        let mut categories = HashMap::<Option<&str>, Vec<&Command<Data, String>>>::new();
        let mut fields: Vec<(String, String, bool)> = vec![];

        for cmd in &ctx.framework().options().commands {
            if cmd.hide_in_help {
                continue;
            }

            let category = categories.get(&cmd.clone().category);
            if category.is_none() {
                categories.insert(cmd.category, vec![cmd]);
            } else {
                let mut category = category.unwrap().clone();
                category.push(cmd);

                categories.insert(cmd.category, category);
            }
        }

        for (category_name, commands) in categories {
            fields.push((
                category_name.unwrap_or("n/a").to_string(),
                format_command_list(ctx, &commands),
                false,
            ));
        }

        let mut embed = embeds.success(
            &lang.translate("embed_title.help"),
            &lang.translate("help.general"),
        ).await;

        embed.fields(fields);
        embeds.send(embed).await;
    } else {
        let command = get_command(ctx.clone(), command_name.as_ref().unwrap());

        if command.is_none() {
            return Err(lang.translate_v("help.command_not_found", map_str! {
                "command" => command_name.unwrap()
            }));
        }

        let command = command.unwrap();
        let mut embed = embeds.success(
            &lang.translate("embed_title.help"),
            &lang.translate_v("help.with_command.description", map_str! {
                "command" => get_localized(
                    ctx,
                    &command,
                    |c| (c.name_localizations.clone(), c.name.clone())
                )
            }),
        ).await;

        let mut usage = format!("{}{}", ctx.prefix(), command_name.unwrap());
        let mut subcommands = String::new();
        if command.subcommands.len() > 0 {
            usage.push_str(" <...>");
            command.subcommands.iter().for_each(|subcommand| {
                let localized_name = get_localized(
                    ctx,
                    &subcommand,
                    |c| (c.name_localizations.clone(), c.name.clone()),
                );

                subcommands.push_str(format!(" `{}`", localized_name).as_str());
            });

            embed.field(
                lang.translate("help.with_command.subcommands"),
                subcommands,
                false,
            );
        } else {
            command.parameters.iter().for_each(|param| {
                let localized_name = get_localized(
                    ctx,
                    &param,
                    |p| (p.name_localizations.clone(), p.name.clone()),
                );

                usage.push_str(if param.required {
                    format!(" `<{}>`", localized_name)
                } else {
                    format!(" `[{}]`", localized_name)
                }.as_str());
            })
        };

        embed.fields([
            (
                lang.translate("help.with_command.usage"),
                usage,
                false,
            ),
            (
                lang.translate("help.with_command.aliases"),
                if command.aliases.len() > 0
                { command.aliases.join(", ") } else { "n/a".to_string() },
                false,
            ),
            (
                lang.translate("help.with_command.category"),
                command.category.unwrap_or("n/a").to_string(),
                false,
            ),
        ]);

        embeds.send(embed).await;
    }

    Ok(())
}

fn format_command_list(ctx: Context<'_>, commands: &[&Command<Data, String>]) -> String {
    commands
        .iter()
        .map(|cmd| format!("`{}`", get_localized(
            ctx,
            &cmd,
            |c| (c.name_localizations.clone(), c.name.clone()),
        )))
        .collect::<Vec<String>>()
        .join(", ")
}