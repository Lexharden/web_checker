mod telegram;
mod web_checker;
mod config;

use std::time::Duration;
use crate::web_checker::check_websites_and_notify;
use crate::config::{load_env, get_env};

#[tokio::main]
async fn main() {
    // Cargar las variables de entorno
    load_env();

    // Obtener el token del bot y el ID del chat
    let bot_token = get_env("BOT_TOKEN").expect("El token del bot no está definido en el archivo .env");
    let chat_id = get_env("CHAT_ID").expect("El ID del chat no está definido en el archivo .env").parse().expect("El ID del chat no es un número válido");

    // Obtener el intervalo de tiempo del loop
    let loop_interval_seconds: u64 = get_env("LOOP_INTERVAL_SECONDS").expect("El intervalo de tiempo del loop no está definido en el archivo .env").parse().expect("El intervalo de tiempo del loop no es un número válido");
    let intervalo = Duration::from_secs(loop_interval_seconds);

    println!("Comenzando a escanear sitios, se escaneará cada {:?}", intervalo);

    loop {
        // Verificar el estado de las páginas web y notificar si es necesario
        if let Err(err) = check_websites_and_notify(&bot_token, chat_id).await {
            eprintln!("Error al verificar sitios web: {}", err);
        }

        // Esperar el intervalo de tiempo antes de la próxima revisión
        tokio::time::sleep(intervalo).await;
    }
}
