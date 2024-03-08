use poise::serenity_prelude::{model::prelude::Ready, Context, Activity, OnlineStatus};

pub async fn ready(ctx: Context, ready: Ready) {
    println!("{} successfully connected to discord gateway", ready.user.name);

    ctx.set_presence(
        Some(Activity::listening("your commands")),
        OnlineStatus::Idle,
    ).await;
}