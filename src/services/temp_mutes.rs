use crate::database::guild_config::GuildConfig;
use crate::database::moderation::TempMute;
use poise::serenity_prelude::CacheHttp;
use poise::serenity_prelude::Context;
use mongodb::Database;

pub(crate) async fn listen(ctx: Context, db: Database) {
    let temp_mutes = TempMute::expired(db.clone()).await;

    for temp_mute in temp_mutes {
        let guild = ctx.cache().unwrap().guild(temp_mute.guild_id).unwrap();
        let mut member = guild.member(&ctx.http(), temp_mute.user_id).await.unwrap();

        let config = GuildConfig::from_raw(
            db.clone(),
            &temp_mute.guild_id.to_string()
        ).await;

        if config.moderation.mute_role.is_none() {
            temp_mute.self_destruct(db.clone()).await;
        }

        member.remove_role(&ctx.http(), config.moderation.mute_role.unwrap()).await.unwrap();
        temp_mute.self_destruct(db.clone()).await;
    }
}