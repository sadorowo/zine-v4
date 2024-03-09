use crate::database::guild_config::GuildConfig;
use crate::database::moderation::TempMute;

use poise::serenity_prelude::{CacheHttp, GuildId, UserId};
use poise::serenity_prelude::Context;
use tokio::time::interval;
use std::time::Duration;
use mongodb::Database;
use std::ops::Deref;
use std::sync::Arc;
use tokio::select;

pub struct TempMuteService(Arc<Database>, Arc<Context>);

impl TempMuteService {
    pub fn new(db: Arc<Database>, ctx: Arc<Context>) -> Self {
        Self(db, ctx)
    }

    pub fn setup_task(self, every: u64) {
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(every));
            loop {
                select! {
                    _ = tick.tick() => {
                        self.listen().await;
                    }
                }
            }
        });
    }

    pub async fn listen(&self) {
        let database = self.0.deref();
        let context = self.1.deref();

        let temp_mutes = TempMute::expired(database).await;

        for temp_mute in temp_mutes {
            let guild_config = GuildConfig::from_raw(database.clone(), &temp_mute.guild_id.to_string()).await;

            if guild_config.moderation.mute_role.is_none() {
                temp_mute.self_destruct(database).await;
                continue;
            }

            context
                .http()
                .remove_member_role(
                    GuildId::new(temp_mute.guild_id),
                    UserId::new(temp_mute.user_id),
                    guild_config.moderation.mute_role.unwrap(),
                    Some("Automatic unmute")
                ).await.unwrap();

            temp_mute.self_destruct(database).await;
        }
    }
}