use poise::serenity_prelude::{GuildId, UserId, Timestamp};
use serde::{Deserialize, Serialize};
use poise::futures_util::StreamExt;
use std::time::Duration;
use mongodb::bson::doc;
use mongodb::Database;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PunishmentAction {
    Ban,
    Kick,
    Mute,
    Warn,
    Timeout,
    UnTimeout,
    TempBan,
    TempMute,
    UnBan,
    UnMute,
}

impl std::fmt::Display for PunishmentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PunishmentAction::Ban => write!(f, "ban"),
            PunishmentAction::Kick => write!(f, "kick"),
            PunishmentAction::Mute => write!(f, "mute"),
            PunishmentAction::Warn => write!(f, "warn"),
            PunishmentAction::TempBan => write!(f, "tempban"),
            PunishmentAction::TempMute => write!(f, "tempmute"),
            PunishmentAction::UnBan => write!(f, "unban"),
            PunishmentAction::UnMute => write!(f, "unmute"),
            PunishmentAction::Timeout => write!(f, "timeout"),
            PunishmentAction::UnTimeout => write!(f, "untimeout"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Punishment {
    pub user_id: u64,
    pub guild_id: u64,
    pub reason: String,
    pub moderator_id: u64,
    pub punishment_id: u64,
    pub punishment_type: PunishmentAction,
    pub created_at: Timestamp,
}

impl Punishment {
    pub async fn get_all(db: Database, guild_id: GuildId, user_id: UserId) -> Vec<Punishment> {
        let collection = db.collection::<Punishment>("punishments");
        let filter = doc! {
            "user_id": user_id.get() as i64,
            "guild_id": guild_id.get() as i64
        };

        let mut cursor = collection.find(filter, None).await.unwrap();
        let mut punishments: Vec<Punishment> = Vec::new();

        while let Some(Ok(punishment)) = cursor.next().await {
            punishments.push(punishment);
        }

        punishments
    }

    pub async fn get(db: Database, guild_id: GuildId, punishment_id: u64) -> Option<Punishment> {
        let collection = db.collection::<Punishment>("punishments");
        let filter = doc! {
            "guild_id": guild_id.get() as i64,
            "punishment_id": punishment_id as i64
        };

        collection.find_one(filter, None).await.unwrap()
    }

    pub async fn get_newest(db: Database, guild_id: GuildId) -> Option<Punishment> {
        let collection = db.collection::<Punishment>("punishments");
        let filter = doc! {
            "guild_id": guild_id.get() as i64,
        };

        let mut cursor = collection.find(filter, None).await.unwrap();
        let mut punishments: Vec<Punishment> = Vec::new();

        while let Some(Ok(punishment)) = cursor.next().await {
            punishments.push(punishment);
        }

        punishments.sort_by(|a, b| b.punishment_id.cmp(&a.punishment_id));
        punishments.first().cloned()
    }

    pub async fn new(
        db: Database,
        user_id: UserId,
        guild_id: GuildId,
        reason: String,
        moderator_id: UserId,
        punishment_type: PunishmentAction,
    ) {
        let punishment_id = Punishment::get_newest(db.clone(), guild_id).await.map_or(1, |p| p.punishment_id + 1);
        let punishment = Punishment {
            user_id: user_id.get(),
            guild_id: guild_id.get(),
            reason,
            moderator_id: moderator_id.get(),
            punishment_id,
            punishment_type,
            created_at: Timestamp::from(chrono::Utc::now()),
        };

        let collection = db.collection("punishments");
        collection.insert_one(punishment, None).await.unwrap();
    }
    
    pub async fn delete(db: Database, punishment_id: u64) {
        let collection = db.collection::<Punishment>("punishments");
        let filter = doc! {
            "punishment_id": punishment_id as i64
        };

        collection.delete_one(filter, None).await.unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TempBan {
    pub user_id: u64,
    pub guild_id: u64,
    pub expires_at: i64,
}

impl TempBan {
    pub async fn new(db: Database, user_id: UserId, guild_id: GuildId, time: Duration) {
        let expires_at = time.as_secs() as i64 + chrono::Utc::now().timestamp();

        let temp_ban = TempBan {
            user_id: user_id.get(),
            guild_id: guild_id.get(),
            expires_at,
        };

        let collection = db.collection("temp_bans");
        collection.insert_one(temp_ban, None).await.unwrap();
    }

    pub async fn expired(db: &Database) -> Vec<TempBan> {
        let collection = db.collection::<TempBan>("temp_bans");
        let filter = doc! {
            "expires_at": {
                "$lte": chrono::Utc::now().timestamp()
            }
        };

        let mut cursor = collection.find(filter, None).await.unwrap();
        let mut temp_bans: Vec<TempBan> = Vec::new();

        while let Some(Ok(temp_ban)) = cursor.next().await {
            temp_bans.push(temp_ban);
        }

        temp_bans
    }

    pub async fn self_destruct(&self, db: &Database) {
        let collection = db.collection::<TempBan>("temp_bans");
        collection.delete_one(doc! {
            "user_id": self.user_id as i64,
            "guild_id": self.guild_id as i64,
            "expires_at": self.expires_at,
        }, None).await.unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TempMute {
    pub user_id: u64,
    pub guild_id: u64,
    pub expires_at: i64,
}

impl TempMute {
    pub async fn new(db: Database, user_id: UserId, guild_id: GuildId, time: Duration) {
        let expires_at = time.as_secs() as i64 + chrono::Utc::now().timestamp();

        let temp_mute = TempMute {
            user_id: user_id.get(),
            guild_id: guild_id.get(),
            expires_at,
        };

        let collection = db.collection("temp_mutes");
        collection.insert_one(temp_mute, None).await.unwrap();
    }

    pub async fn expired(db: &Database) -> Vec<TempMute> {
        let collection = db.collection::<TempMute>("temp_mutes");
        let filter = doc! {
            "expires_at": {
                "$lte": chrono::Utc::now().timestamp()
            }
        };

        let mut cursor = collection.find(filter, None).await.unwrap();
        let mut temp_mutes: Vec<TempMute> = Vec::new();

        while let Some(Ok(temp_mute)) = cursor.next().await {
            temp_mutes.push(temp_mute);
        }

        temp_mutes
    }

    pub async fn self_destruct(&self, db: &Database) {
        let collection = db.collection::<TempMute>("temp_mutes");
        collection.delete_one(doc! {
            "user_id": self.user_id as i64,
            "guild_id": self.guild_id as i64,
            "expires_at": self.expires_at,
        }, None).await.unwrap();
    }
    
    pub async fn is_muted(db: Database, guild_id: GuildId, user_id: UserId) -> bool {
        let collection = db.collection::<TempMute>("temp_mutes");
        let filter = doc! {
            "guild_id": guild_id.get() as i64,
            "user_id": user_id.get() as i64
        };

        let result = collection.find_one(filter, None)
            .await;
        
        result.is_ok() && result.unwrap().is_some()
    }
    
    pub async fn get_mute_data(db: Database, guild_id: GuildId, user_id: UserId) -> Option<TempMute> {
        let collection = db.collection::<TempMute>("temp_mutes");
        let filter = doc! {
            "guild_id": guild_id.get() as i64,
            "user_id": user_id.get() as i64
        };

        let result = collection.find_one(filter, None)
            .await;

        if result.is_err() {
            return None;
        }
        
        result.unwrap()
    }
}