use crate::language::handler::LanguageHandler;
use crate::utils::framework::Context;
use poise::serenity_prelude::RoleId;

pub fn format_role(ctx: Context<'_>, role_id: Option<RoleId>) -> String {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);

    if role_id.is_none() {
        return lang.translate("not_accessible")
    }

    let guild = ctx
        .guild()
        .unwrap();

    let role = guild
        .roles
        .get(&role_id.unwrap());

    return if role.is_some() {
        format!("<@&{}>", role_id.unwrap())
    } else {
        lang.translate("not_accessible")
    }
}