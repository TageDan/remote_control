use debug_print::debug_println;
use enigo::Keyboard;
use enigo::Mouse;
use serde::Deserialize;
use std::sync::mpsc::RecvTimeoutError;
use std::thread;
use std::time::Duration;
use tokio;

use axum::{
    Form, Router,
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::{Html, Response},
    routing::{any, get},
};

const PASSWORD: &'static str = "TEST123";
const SPEED: f32 = 10.;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(login_page).post(index_page))
        .route("/ws", any(control_socket_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login_page() -> Html<&'static str> {
    Html(include_str!("./pages/login.html"))
}

#[derive(Deserialize, Debug)]
struct PassForm {
    password: String,
}

async fn index_page(Form(input): Form<PassForm>) -> Html<String> {
    if input.password == *PASSWORD {
        Html(include_str!("./pages/control.html").replace("{{password}}", PASSWORD))
    } else {
        Html(include_str!("./pages/failed_login.html").to_string())
    }
}

async fn control_socket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_control_socket)
}

#[derive(Deserialize, Debug)]
struct ControlMessage {
    control_type: String,
    password: String,
    movex: f32,
    movey: f32,
    key: String,
}

async fn handle_control_socket(mut socket: WebSocket) {
    let (sender, reciever) = std::sync::mpsc::channel::<ControlMessage>();
    let control_thread = thread::spawn(move || {
        let mut device = enigo::Enigo::new(&enigo::Settings::default()).unwrap();

        thread::sleep(Duration::from_secs(1));
        let mut speed_x = 0;
        let mut speed_y = 0;
        loop {
            // accept commands
            if let Err(e) = match reciever.recv_timeout(Duration::from_millis(10)) {
                Ok(control_message) => match control_message.control_type.as_str() {
                    "move" => {
                        speed_x = (control_message.movex * SPEED) as i32;
                        speed_y = (control_message.movey * SPEED) as i32;
                        Ok(())
                    }
                    "click" => device.button(enigo::Button::Left, enigo::Direction::Click),
                    "fullscreen" => Ok(()),
                    "keyboard" => handle_key(control_message.key, &mut device),
                    _ => (Ok(())),
                },
                Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => (Ok(())),
            } {
                eprintln!("{e}");
            }

            // move mouse
            device.move_mouse(speed_x, speed_y, enigo::Coordinate::Rel);
        }
    });
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg { msg } else { return };

        let control_message: ControlMessage =
            if let Ok(message) = serde_json::from_str(msg.to_text().unwrap()) {
                message
            } else {
                debug_println!("failed to deserialize!");
                return;
            };

        if control_message.password != PASSWORD {
            return;
        }

        debug_println!("{control_message:?}");
        sender.send(control_message).unwrap();
    }
}

enum Mod {
    None,
    Shift,
    AltGr,
}

fn handle_key(key: String, device: &mut enigo::Enigo) -> Result<(), enigo::InputError> {
    match key.as_str() {
        "Enter" => device.key(enigo::Key::Return, enigo::Direction::Click),
        "Backspace" => device.key(enigo::Key::Backspace, enigo::Direction::Click),
        key => device.text(key),
    }
}
