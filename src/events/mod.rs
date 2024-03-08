mod ready;

use poise::serenity_prelude::{
    model::prelude::Ready,
    EventHandler,
    async_trait,
    Context
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready).await;
    }
}