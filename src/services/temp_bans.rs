use crate::database::moderation::TempBan;
use poise::serenity_prelude::CacheHttp;
use poise::serenity_prelude::Context;
use mongodb::Database;

pub(crate) async fn listen(ctx: Context, db: Database) {
    let temp_bans = TempBan::expired(db.clone()).await;

    for temp_ban in temp_bans {
        let guild = ctx.cache().unwrap().guild(temp_ban.guild_id).unwrap();

        guild.unban(&ctx.http(), temp_ban.user_id).await.unwrap();
        temp_ban.self_destruct(db.clone()).await;
    }
}