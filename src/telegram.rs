use teloxide::prelude::*;
use teloxide::types::ChatId;

// Función para enviar un mensaje a un chat específico
pub async fn send_message_to_chat(bot: &Bot, chat_id: i64, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    bot.send_message(ChatId(chat_id), message).await?;
    Ok(())
}
