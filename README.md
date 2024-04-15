# Web Checker

Este es un programa escrito en Rust para verificar el estado de varias páginas web y enviar notificaciones a través de
Telegram si alguna de ellas está caída.

## Instalación

1. Clona este repositorio en tu máquina local:

```bash
git clone https://github.com/Lexharden/web_checker.git
```

2. Asegúrate de tener Rust instalado. Puedes instalarlo siguiendo las instrucciones
   en [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

3. Crea un archivo `.env` en la raíz del proyecto y define las siguientes variables de entorno:

```plaintext
BOT_TOKEN=your_bot_token
CHAT_ID=your_chat_id
LOOP_INTERVAL_SECONDS=60
WEBSITE_URLS=https://example.com,https://google.com.mx
```

- `BOT_TOKEN`: El token de tu bot de Telegram.
- `CHAT_ID`: El ID del chat al que deseas enviar las notificaciones.
- `LOOP_INTERVAL_SECONDS`: El intervalo de tiempo en segundos entre cada revisión de las páginas web.
- `WEBSITE_URLS`: Una lista de URLs separadas por comas que deseas verificar.

4. Compila y ejecuta el programa con Cargo:

```bash
cargo run
```

## Contribuir

Si encuentras algún error o deseas realizar mejoras, ¡no dudes en abrir un issue o enviar un pull request!

## Licencia

Este proyecto está licenciado bajo la [Licencia MIT](LICENSE).
