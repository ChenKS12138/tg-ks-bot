use std::env;

use futures::StreamExt;
use telegram_bot::*;

const COMMAND_KS: &str = "/ks";
const COMMAND_GITHUB: &str = "/github";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ks_gif =
        types::FileRef::from("https://github.com/ChenKS12138/tg-ks-bot/raw/master/images/ks.gif");
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);
    let reg = regex::Regex::new(r"@.*+").expect("Invalid regex");

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        if update.is_ok() {
            let update = update.ok();
            if let Some(update) = update {
                if let UpdateKind::Message(message) = update.kind {
                    match message.kind {
                        MessageKind::Text { ref data, .. } => {
                            let command = data.as_str();
                            let command = reg.replace_all(&command, "").to_owned().to_string();
                            match command.as_str() {
                                COMMAND_KS => {
                                    api.send(types::requests::SendVideo::new(
                                        &message.chat,
                                        &ks_gif,
                                    ))
                                    .await?;
                                }
                                COMMAND_GITHUB => {
                                    api.send(types::requests::SendMessage::new(
                                        &message.chat,
                                        "See https://github.com/ChenKS12138/tg-ks-bot",
                                    ))
                                    .await?;
                                }
                                _ => {
                                    api.send(types::requests::SendMessage::new(
                                        &message.chat,
                                        "Say /ks",
                                    ))
                                    .await?;
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
    Ok(())
}
