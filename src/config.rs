use dotenv::dotenv;
use std::env;

// Carga las variables de entorno del archivo .env
pub fn load_env() {
    dotenv().ok();
}

// Función para obtener el valor de una variable de entorno
pub fn get_env(key: &str) -> Option<String> {
    env::var(key).ok()
}
