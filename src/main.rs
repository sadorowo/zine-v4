#![allow(dead_code)]

mod commands;
mod database;
mod language;
mod services;
mod events;
mod macros;
mod models;
mod theme;
mod utils;

use poise::{serenity_prelude::GatewayIntents, Framework};
use dotenv::{dotenv, var};
use std::time::SystemTime;
use mongodb::Database;

use crate::language::handler::LanguageHandler;
use crate::services::start_services;
use crate::commands::{
    Data,
    get_commands,
    utils::transform_commands
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token: String = var("TOKEN").expect("expected a token in the environment");
    let db: Database = database::connect::connect().await.database("zine");
    let language: LanguageHandler = LanguageHandler::no_context();

    let framework = Framework::builder()
        .token(token)
        .client_settings(|cs| cs.event_handler(events::Handler))
        .options(utils::framework::init_framework_options(
            transform_commands(language, get_commands()).await
        ))
        .intents(
            GatewayIntents::non_privileged()
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(|ctx, _, _| Box::pin(async move {
            let services = start_services(ctx.clone(), db.clone());
            Ok(Data { 
                db, 
                services,
                uptime: SystemTime::now().into()
            })
        }))
        .build()
        .await
        .unwrap();

    if let Err(why) = framework.start_autosharded().await {
        println!("an error occurred while running the client: {:?}", why);
    }
}
