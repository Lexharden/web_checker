use reqwest::StatusCode;
use crate::telegram::send_message_to_chat;
use teloxide::prelude::*;
use crate::config;

// Estructura para representar una URL y su estado
#[derive(Debug)]
pub struct UrlStatus {
    pub url: String,
    pub status: StatusCode,
}

// Función para verificar el estado de las páginas web y enviar mensajes de Telegram si el estado no es "OK"
pub async fn check_websites_and_notify(bot_token: &str, chat_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    // Cargar las URLs desde las variables de entorno
    let urls_env = config::get_env("WEBSITE_URLS").expect("Las URLs de los sitios web no están definidas en el archivo .env");
    let urls: Vec<&str> = urls_env.split(',').collect();

    // Inicializar el bot de Telegram
    let bot = Bot::new(bot_token);

    // Verificar el estado de cada URL
    for url in urls {
        match check_url(url).await {
            Ok(status) => {
                if status != StatusCode::OK {
                    let message = format!("¡Alerta! La URL {} tiene un estado: {}", url, status);
                    send_message_to_chat(&bot, chat_id, &message).await?;
                }
            }
            Err(err) => {
                println!("Error al revisar {}: {}", url, err);
            }
        }
    }

    Ok(())
}

// Función para verificar el estado de una URL específica
async fn check_url(url: &str) -> Result<StatusCode, reqwest::Error> {
    // Realizar la solicitud HTTP
    let response = reqwest::get(url).await?;
    // Obtener el código de estado de la respuesta
    let status = response.status();
    Ok(status)
}
