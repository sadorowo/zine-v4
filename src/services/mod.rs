use poise::serenity_prelude::Context;
use mongodb::Database;
use std::sync::Arc;

mod temp_bans;
mod temp_mutes;

use crate::services::temp_mutes::TempMuteService;
use crate::services::temp_bans::TempBanService;

pub fn start_services(ctx: Context, db: Database) {
    let shared_db = Arc::new(db);
    let shared_ctx = Arc::new(ctx);

    TempBanService::new(shared_db.clone(), shared_ctx.clone());
    TempMuteService::new(shared_db.clone(), shared_ctx.clone());
}