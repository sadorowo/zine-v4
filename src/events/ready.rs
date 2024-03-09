use poise::serenity_prelude::{model::prelude::Ready, Context, OnlineStatus, ActivityData};

pub async fn ready(ctx: Context, ready: Ready) {
    println!("{} successfully connected to discord gateway", ready.user.name);

    ctx.set_presence(
        Some(ActivityData::listening("your commands")),
        OnlineStatus::Idle,
    );
}