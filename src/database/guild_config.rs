use crate::database::moderation::PunishmentAction;
use crate::utils::framework::Context;
use serde::{Deserialize, Serialize};
use poise::serenity_prelude::RoleId;
use mongodb::results::UpdateResult;
use mongodb::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildTheme {
    pub color_success: u32,
    pub color_error: u32,
    pub color_info: u32,
    pub color_warning: u32,
    pub enable_embeds: bool,
}

impl GuildTheme {
    pub fn default() -> Self {
        Self {
            color_success: 0x2f3136,
            color_error: 0x2f3136,
            color_info: 0x2f3136,
            color_warning: 0x2f3136,
            enable_embeds: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationSettings {
    pub mute_role: Option<RoleId>,
    pub warn_threshold: Option<u32>,
    pub warn_action: Option<PunishmentAction>,
    pub warn_action_duration: Option<u32>,
}

impl Default for ModerationSettings {
    fn default() -> Self {
        Self {
            mute_role: None,
            warn_threshold: None,
            warn_action: None,
            warn_action_duration: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    pub guild_id: String,
    pub prefixes: Vec<String>,
    pub language: String,
    pub theme: GuildTheme,
    pub moderation: ModerationSettings,
}

impl GuildConfig {
    pub async fn from_context(ctx: &Context<'_>) -> Self {
        let guild_id: String = ctx.guild_id().unwrap().to_string();
        let db: Database = ctx.data().db.clone();

        let field = db.collection("guilds").find_one(mongodb::bson::doc! {
            "guild_id": guild_id.clone()
        }, None)
            .await;

        match field {
            Ok(config) => config.unwrap(),
            Err(_) => Self::default(guild_id),
        }
    }

    pub async fn from_raw(db: Database, guild_id: &str) -> Self {
        let guild_id: String = guild_id.to_string();
        let field = db.collection("guilds").find_one(mongodb::bson::doc! {
            "guild_id": guild_id.clone()
        }, None)
            .await;

        match field {
            Ok(config) => config.unwrap(),
            Err(_) => Self::default(guild_id),
        }
    }

    pub fn default(guild_id: String) -> Self {
        Self {
            guild_id,
            prefixes: vec![".".to_string()],
            language: "en-US".to_string(),
            theme: GuildTheme::default(),
            moderation: ModerationSettings::default()
        }
    }

    pub async fn save(&mut self, ctx: Context<'_>) -> mongodb::error::Result<UpdateResult> {
        let db: Database = ctx.data().db.clone();

        db.collection::<Self>("guilds").update_one(mongodb::bson::doc! {
            "guild_id": self.guild_id.clone()
        }, mongodb::bson::doc! {
            "$set": mongodb::bson::to_document(&self).unwrap()
        }, None).await
    }
}

impl std::fmt::Display for GuildConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prefixes = String::new();
        for prefix in &self.prefixes {
            prefixes.push_str(&format!("`{}` ", prefix));
        }

        write!(f, "prefixes: {}\nlanguage: {}", prefixes, self.language)
    }
}