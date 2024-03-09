use poise::serenity_prelude::{EditMessage, Message, ReactionType};
use rand::seq::SliceRandom;
use std::time::Duration;
use poise::CreateReply;
use rand::Rng;

use crate::language::handler::LanguageHandler;
use crate::utils::framework::Context;
use crate::theme::embeds::Embeds;
use crate::map_str;

pub struct Confirmation<'a> {
    ctx: Context<'a>,
    message: String,
    pub accepted: bool,
}

impl<'a> Confirmation<'a> {
    pub fn new(ctx: Context<'a>, message: String) -> Self {
        Self {
            ctx,
            message,
            accepted: false,
        }
    }

    fn generate_random_reactions() -> (Vec<String>, String) {
        let mut reactions: Vec<String> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..5 {
            let random_char: char = rng.gen_range('🇦'..='🇿');
            reactions.push(random_char.to_string());
        }

        let random_reaction = reactions.choose(&mut rng).unwrap().to_string();

        reactions.shuffle(&mut rng);
        (reactions, random_reaction)
    }

    pub async fn send(&mut self) -> Message {
        let mut embeds: Embeds = Embeds::from_context(self.ctx.clone());
        let lang: LanguageHandler = LanguageHandler::from_context(self.ctx.clone());

        let (reactions, correct_reaction) = Self::generate_random_reactions();
        let embed =
            embeds.info(
                &lang.translate("embed_title.confirmation"),
                &self.message
            ).await
            .field(
                &lang.translate("confirmation.instructions_title"),
                &lang.translate_v("confirmation.instructions_description", map_str!("correct" => correct_reaction)),
                false
            );

        let mut message = self.ctx
            .send(CreateReply::default().embed(embed))
            .await
            .unwrap()
            .into_message()
            .await
            .unwrap();

        for reaction in reactions {
            message.react(
                &self.ctx.http(),
                ReactionType::Unicode(reaction)
            ).await.unwrap();
        }

        let collector = message
            .await_reaction(&self.ctx)
            .timeout(Duration::from_secs(120))
            .author_id(self.ctx.author().id)
            .message_id(message.id)
            .await;

        if let Some(reaction) = collector {
            let reaction = reaction.emoji.as_data();

            if reaction == correct_reaction {
                self.accepted = true;
            } else {
                let error_embed = embeds.error(
                    &lang.translate("embed_title.error"),
                    &lang.translate("confirmation.invalid_reaction")
                ).await;

                message.edit(
                    &self.ctx.http(),
                    EditMessage::new().embed(error_embed)
                ).await.expect("cannot edit message");
            }

            message.delete_reactions(
                &self.ctx.http()
            ).await.expect("cannot delete reactions");
        }

        message
    }
}