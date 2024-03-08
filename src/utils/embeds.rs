use crate::utils::framework::Context;

pub fn get_footer_text(ctx: &Context<'_>) -> String {
    let footer_text = format!(
        "requested by {}",
        ctx.author().tag()
    );

    footer_text
}