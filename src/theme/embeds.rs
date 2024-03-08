use crate::database::guild_config::GuildConfig;
use crate::language::handler::LanguageHandler;
use poise::serenity_prelude::CreateEmbed;
use crate::utils::framework::Context;
use crate::commands::Data;
use crate::map_str;
use poise::ReplyHandle;

pub struct Embeds<'a> {
    ctx: Context<'a>,
    lang: LanguageHandler<'a>,
}

impl<'a> Embeds<'a> {
    pub fn from_context(ctx: poise::Context<'a, Data, String>) -> Self {
        Self {
            ctx,
            lang: LanguageHandler::from_context(ctx),
        }
    }

    pub fn get_footer_text(&self) -> String {
        self.lang.translate_v("embed_footer", map_str! {
            "user" => self.ctx.author().tag()
        })
    }

    pub fn get_footer_icon(&self) -> String {
        self.ctx.author().face()
    }

    pub fn gen(&mut self, title: &str, description: &str, color: u32) -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        embed.author(|a| a.name(title).icon_url(self.get_footer_icon()));
        embed.description(description);
        embed.color(color);
        embed.footer(|f| f.text(self.get_footer_text()));
        embed
    }

    pub async fn success(&mut self, title: &str, description: &str) -> CreateEmbed {
        let config: GuildConfig = GuildConfig::from_context(&self.ctx).await;
        let color: u32 = config.theme.color_success;

        self.gen(title, description, color)
    }

    pub async fn error(&mut self, title: &str, description: &str) -> CreateEmbed  {
        let config: GuildConfig = GuildConfig::from_context(&self.ctx).await;
        let color: u32 = config.theme.color_error;

        self.gen(title, description, color)
    }

    pub async fn info(&mut self, title: &str, description: &str) -> CreateEmbed {
        let config: GuildConfig = GuildConfig::from_context(&self.ctx).await;
        let color: u32 = config.theme.color_info;

        self.gen(title, description, color)
    }

    pub async fn warning(&mut self, title: &str, description: &str) -> CreateEmbed {
        let config: GuildConfig = GuildConfig::from_context(&self.ctx).await;
        let color: u32 = config.theme.color_warning;

        self.gen(title, description, color)
    }

    pub async fn send(&self, embed: CreateEmbed) -> ReplyHandle<'a> {
        let embed: &'static mut CreateEmbed = Box::leak(Box::new(embed));

        self.ctx.send(|m| m.embed(|e| {
            e.clone_from(embed);
            e
        }))
            .await
            .expect("error while sending embed")
    }

    pub async fn edit(&self, reply: ReplyHandle<'a>, embed: CreateEmbed) {
        let embed: &'static mut CreateEmbed = Box::leak(Box::new(embed));

        reply.edit(self.ctx, |m| m.embed(|e| {
            e.clone_from(embed);
            e
        }))
            .await
            .expect("error while editing embed");
    }
}