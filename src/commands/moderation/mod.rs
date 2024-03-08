use poise::serenity_prelude::{CacheHttp, Member, RoleId, User};
use crate::utils::framework::Context;

pub mod ban;
pub mod kick;
pub mod warn;
pub mod mute;
pub mod unmute;
pub mod timeout;
pub mod tempban;
pub mod tempmute;
pub mod untimeout;
pub mod punishment;
pub mod delete_punishment;

async fn check_hierarchy(ctx: Context<'_>, target: Member) -> Option<String> {
    let author = ctx.author_member().await.unwrap();
    let action_type = ctx.invoked_command_name();

    let author_top_role = author.highest_role_info(ctx.cache().unwrap()).unwrap_or((RoleId(0), 0)).1;
    let target_top_role = target.highest_role_info(ctx.cache().unwrap()).unwrap_or((RoleId(0), 0)).1;

    if author_top_role <= target_top_role && author.user.id != ctx.guild().unwrap().owner_id {
        return Some(format!("{action_type}.highest_role"));
    }

    if author.user.id == target.user.id {
        return Some(format!("{action_type}.self"));
    }

    if target.user.id == ctx.framework().bot_id {
        return Some(format!("{action_type}.bot"));
    }

    None
}

async fn check_ban(ctx: Context<'_>, target: User) -> Option<String> {
    let author = ctx.author_member().await.unwrap();
    let target_member = ctx.guild().unwrap().member(&ctx.http(), target.id).await;

    if target_member.is_ok() {
        let author_top_role = author.highest_role_info(ctx.cache().unwrap()).unwrap_or((RoleId(0), 0)).1;
        let target_top_role = target_member
            .unwrap()
            .highest_role_info(ctx.cache().unwrap())
            .unwrap_or((RoleId(0), 0)).1;

        if author_top_role <= target_top_role && author.user.id != ctx.guild().unwrap().owner_id {
            return Some("ban.highest_role".to_string());
        }
    }

    if author.user.id == target.id {
        return Some("ban.self".to_string());
    }

    if target.id == ctx.framework().bot_id {
        return Some("ban.bot".to_string());
    }

    None
}