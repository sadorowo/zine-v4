use crate::language::handler::LanguageHandler;
use crate::theme::embeds::Embeds;
use crate::commands::Context;
use std::process::Command;
use crate::no_md;

#[poise::command(
slash_command,
prefix_command,
owners_only,
category = "dev",
aliases("sh", "bash", "shell"),
)]
pub async fn shell(
    ctx: Context<'_>,
    #[rest]
    code: String,
) -> Result<(), String> {
    let lang: LanguageHandler = LanguageHandler::from_context(ctx);
    let mut embeds: Embeds = Embeds::from_context(ctx);

    let command = Command::new("sh")
        .arg("-c")
        .arg(code)
        .output();

    match command {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.is_empty() {
                return Err(format!("```sh\n{}```",
                                   no_md![&*String::from_utf8_lossy(&output.stderr)]));
            }

            let embed = embeds.success(
                &lang.translate("embed_title.shell"),
                &*format!("```sh\n{}```", no_md![stdout]),
            ).await;

            embeds.send(embed).await;
            Ok(())
        }

        Err(why) => {
            Err(format!("```sh\n{}```", no_md![why.to_string()]))
        }
    }
}