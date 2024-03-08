use crate::utils::types::no_generic_type;
use crate::utils::framework::Context;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use crate::services::Service;
use poise::Command;

pub mod moderation;
pub mod config;
pub mod utils;
pub mod util;
pub mod info;
pub mod bot;
pub mod dev;

pub struct Data {
    pub(crate) db: mongodb::Database,
    pub(crate) services: Vec<Service>,
    pub(crate) uptime: DateTime<Local>
}

#[poise::command(prefix_command, hide_in_help, owners_only, aliases("rcmd"))]
pub async fn register_commands(ctx: Context<'_>) -> Result<(), String> {
    let result = poise::builtins::register_application_commands_buttons(ctx).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

pub fn get_commands() -> Vec<Command<Data, String>> {
    let commands = vec![
        register_commands(),
        bot::help::help(),
        bot::ping::ping(),
        bot::stats::stats(),
        dev::shell::shell(),
        dev::services::services(),
        info::user::user(),
        info::channel::channel(),
        info::role::role(),
        info::server::server(),
        util::ascii_art::ascii_art(),
        config::prefix::prefix(),
        config::config::config(),
        config::theme::theme(),
        moderation::kick::kick(),
        moderation::ban::ban(),
        moderation::warn::warn(),
        moderation::tempban::tempban(),
        moderation::punishment::punishment(),
        moderation::delete_punishment::delete_punishment(),
        moderation::timeout::timeout(),
        moderation::untimeout::untimeout(),
        moderation::mute::mute(),
        moderation::unmute::unmute(),
        moderation::tempmute::tempmute(),
    ];

    commands
        .iter()
        .for_each(|cmd|
            assert_eq!(no_generic_type(cmd), "poise::structs::command::Command"));

    commands
}

pub fn get_localized<F, S, R>(ctx: Context<'_>, s: S, f: F) -> R
    where F: FnOnce(S) -> (HashMap<String, R>, R),
          R: Into<String> + Clone {
    let (localizations, default) = f(s);
    let locale = ctx
        .locale()
        .unwrap_or_else(|| "en-US");

    localizations
        .get(locale)
        .cloned()
        .unwrap_or_else(|| default)
}