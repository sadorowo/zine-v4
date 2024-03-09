use crate::language::handler::LanguageHandler;
use super::{Data, Context};

fn format_parameter(lang: LanguageHandler<'_>, command_name: &str, param: poise::CommandParameter<Data, String>) -> poise::CommandParameter<Data, String> {
    poise::CommandParameter {
        name_localizations: lang.get_localizations(&format!("param_name.{}.{}", command_name, param.name)),
        description_localizations: lang.get_localizations(&format!("param_description.{}.{}", command_name, param.name)),
        ..param
    }
}

pub async fn transform_command(lang: LanguageHandler<'_>, command: poise::Command<Data, String>) -> poise::Command<Data, String> {
    poise::Command {
        name_localizations: lang.get_localizations(&format!("command_name.{}", &command.name)),
        description_localizations: lang.get_localizations(&format!("command_description.{}", &command.name)),
        subcommands: command.subcommands
            .into_iter()
            .map(|subcommand| poise::Command {
                name_localizations: lang.get_localizations(&format!("subcommand_name.{}.{}", command.name, subcommand.name)),
                description_localizations: lang.get_localizations(&format!("subcommand_description.{}.{}", command.name, subcommand.name)),
                parameters: subcommand.parameters
                    .into_iter()
                    .map(|param| format_parameter(lang.clone(), &command.name, param))
                    .collect(),
                ..subcommand
            })
            .collect(),
        parameters: command.parameters
            .into_iter()
            .map(|param| format_parameter(lang.clone(), &command.name, param))
            .collect(),
        ..command
    }
}

pub async fn transform_commands(lang: LanguageHandler<'_>, commands: Vec<poise::Command<Data, String>>) -> Vec<poise::Command<Data, String>> {
    let mut transformed_commands: Vec<poise::Command<Data, String>> = Vec::new();
    for command in commands {
        transformed_commands.push(transform_command(lang.clone(), command).await);
    }

    transformed_commands
}

pub fn get_command<'a>(ctx: Context<'a>, query: &str) -> Option<&'a poise::Command<Data, String>> {
    let commands = &ctx.framework().options().commands;
    let mut command = None;

    for cmd in commands {
        if cmd.name == query || cmd.aliases.contains(&query.to_string()) {
            command = Some(cmd);
            break;
        }

        let values = cmd.name_localizations.values();
        for value in values {
            if value == query {
                command = Some(cmd);
                break;
            }
        }
    }

    command
}

pub fn starts_with_localized_name(command: &poise::Command<Data, String>, query: &str) -> bool {
    let values = command.name_localizations.values();
    for value in values {
        if value.starts_with(query) {
            return true;
        }
    }

    false
}