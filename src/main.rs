use debug_print::debug_println;
use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;
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
        let mut device = VirtualDevice::default().unwrap();
        thread::sleep(Duration::from_secs(1));
        let mut speed_x = 0;
        let mut speed_y = 0;
        loop {
            // accept commands
            match reciever.recv_timeout(Duration::from_millis(10)) {
                Ok(control_message) => match control_message.control_type.as_str() {
                    "move" => {
                        speed_x = (control_message.movex * SPEED) as i32;
                        speed_y = (control_message.movey * -SPEED) as i32;
                    }
                    "click" => device.click(BTN_LEFT).unwrap(),
                    "fullscreen" => {
                        device.press(KEY_LEFTMETA).unwrap();
                        device.click(KEY_F).unwrap();
                        device.release(KEY_LEFTMETA).unwrap();
                    }
                    "keyboard" => handle_key(control_message.key, &mut device),
                    _ => (),
                },
                Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => (),
            }

            // move mouse
            device.smooth_move_mouse(speed_x, speed_y);
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

fn handle_key(key: String, device: &mut VirtualDevice) {
    let (btn, shift) = match key.as_str() {
        "a" => (KEY_A, false),
        "b" => (KEY_B, false),
        "c" => (KEY_C, false),
        "d" => (KEY_D, false),
        "e" => (KEY_E, false),
        "f" => (KEY_F, false),
        "g" => (KEY_G, false),
        "h" => (KEY_H, false),
        "i" => (KEY_I, false),
        "j" => (KEY_J, false),
        "k" => (KEY_K, false),
        "l" => (KEY_L, false),
        "m" => (KEY_M, false),
        "n" => (KEY_N, false),
        "o" => (KEY_O, false),
        "p" => (KEY_P, false),
        "q" => (KEY_Q, false),
        "r" => (KEY_R, false),
        "s" => (KEY_S, false),
        "t" => (KEY_T, false),
        "u" => (KEY_U, false),
        "v" => (KEY_V, false),
        "w" => (KEY_W, false),
        "x" => (KEY_X, false),
        "y" => (KEY_Y, false),
        "z" => (KEY_Z, false),
        "A" => (KEY_A, true),
        "B" => (KEY_B, true),
        "C" => (KEY_C, true),
        "D" => (KEY_D, true),
        "E" => (KEY_E, true),
        "F" => (KEY_F, true),
        "G" => (KEY_G, true),
        "H" => (KEY_H, true),
        "I" => (KEY_I, true),
        "J" => (KEY_J, true),
        "K" => (KEY_K, true),
        "L" => (KEY_L, true),
        "M" => (KEY_M, true),
        "N" => (KEY_N, true),
        "O" => (KEY_O, true),
        "P" => (KEY_P, true),
        "Q" => (KEY_Q, true),
        "R" => (KEY_R, true),
        "S" => (KEY_S, true),
        "T" => (KEY_T, true),
        "U" => (KEY_U, true),
        "V" => (KEY_V, true),
        "W" => (KEY_W, true),
        "X" => (KEY_X, true),
        "Y" => (KEY_Y, true),
        "Z" => (KEY_Z, true),
        "0" => (KEY_NUMERIC_0, false),
        "1" => (KEY_NUMERIC_1, false),
        "2" => (KEY_NUMERIC_2, false),
        "3" => (KEY_NUMERIC_3, false),
        "4" => (KEY_NUMERIC_4, false),
        "5" => (KEY_NUMERIC_5, false),
        "6" => (KEY_NUMERIC_6, false),
        "7" => (KEY_NUMERIC_7, false),
        "8" => (KEY_NUMERIC_8, false),
        "9" => (KEY_NUMERIC_9, false),
        "=" => (KEY_NUMERIC_0, true),
        "!" => (KEY_NUMERIC_1, true),
        "\"" => (KEY_NUMERIC_2, true),
        "#" => (KEY_NUMERIC_3, true),
        "¤" => (KEY_NUMERIC_4, true),
        "%" => (KEY_NUMERIC_5, true),
        "&" => (KEY_NUMERIC_6, true),
        "/" => (KEY_NUMERIC_7, true),
        "(" => (KEY_NUMERIC_8, true),
        ")" => (KEY_NUMERIC_9, true),
        "Enter" => (KEY_ENTER, false),
        "Backspace" => (KEY_BACKSPACE, false),
        " " => (KEY_SPACE, false),
        "." => (KEY_DOT, false),
        key => {
            debug_println!("unknown key ({key})");
            return;
        }
    };

    debug_println!("pressing_btn: {btn:?}");

    if shift {
        device.press(KEY_LEFTSHIFT);
    }

    device.click(btn);

    if shift {
        device.release(KEY_LEFTSHIFT);
    }
}
