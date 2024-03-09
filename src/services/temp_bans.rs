use crate::database::moderation::TempBan;

use poise::serenity_prelude::{CacheHttp, Context, GuildId, UserId};
use tokio::time::interval;
use std::time::Duration;
use mongodb::Database;
use std::ops::Deref;
use std::sync::Arc;
use tokio::select;

pub struct TempBanService(Arc<Database>, Arc<Context>);

impl TempBanService {
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

        let temp_bans = TempBan::expired(database).await;

        for temp_ban in temp_bans {
            context
                .http()
                .remove_ban(
                    GuildId::new(temp_ban.guild_id),
                    UserId::new(temp_ban.user_id),
                    Some("Temp ban expired.")
                ).await.unwrap();

            temp_ban.self_destruct(database).await;
        }
    }
}