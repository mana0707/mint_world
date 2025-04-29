use chrono::Local;
use iced::widget::text_input;
use iced::widget::{button, column, text};
pub mod state;
use iced::widget::{canvas, container};
use iced::{Element, Fill, Renderer, Theme};
use state::Message;
use state::Screen;
pub use state::State;
use std::io::Write;
use std::process::Command as Std_Command;

use std::process::Stdio;

fn get_vite_url() -> Result<String, Box<dyn std::error::Error>> {
    let output = Std_Command::new("cmd")
        .arg("/c")
        .arg("cd mainapp/three_vrm && npx vite --host")
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        return Err("failed to execute command".into());
    }

    let stdout = String::from_utf8(output.stdout)?;

    // 出力からURLを抽出するロジック（例：最後の行から抽出）
    let url = stdout.lines().last().unwrap_or("").to_string();

    Ok(url)
}

#[derive(Default)]
pub struct App {}

impl App {
    pub fn view(state: &State) -> Element<'_, Message> {
        // We use a column: a simple vertical layout
        match state.current_screen {
            Screen::Screen1 => column![
                text("home").size(30),
                button("Auto Update").on_press(Message::AutoUpdate),
                button("RSA program").on_press(Message::SwitchToScreen2),
                button("AI").on_press(Message::SwitchToScreen3),
                button("mint's program").on_press(Message::SwitchToScreen4),
                button("vrm").on_press(Message::Vrm),
                text("1").size(20),
                text(Local::now().format("%Y/%m/%d %H:%M").to_string()).size(10),
                container(canvas(&state as &State).width(Fill).height(Fill)).padding(20),
            ]
            .into(),
            Screen::Screen2 => column![
                text("RSA program").size(30),
                text_input::<Message, Theme, Renderer>("Type something here...", &state.content)
                    .on_input(Message::ContentChanged),
                button("create file").on_press(Message::CreateFile),
                button("rsa encryption").on_press(Message::RsaEncryption),
                button("rsa decryption").on_press(Message::RsaDecryption),
                button("home").on_press(Message::SwitchToScreen1),
                text("2").size(20),
                text(Local::now().format("%Y/%m/%d %H:%M").to_string()).size(10),
                container(canvas(&state as &State).width(Fill).height(Fill)).padding(20),
            ]
            .into(),
            Screen::Screen3 => column![
                text("AI").size(30),
                text_input::<Message, Theme, Renderer>("AI chat", &state.input)
                    .on_input(Message::TextInput),
                button("Ask AI").on_press(Message::AskAI),
                text(state.output.to_string()).size(20),
                button("home").on_press(Message::SwitchToScreen1),
                text("3").size(20),
                container(canvas(&state as &State).width(Fill).height(Fill)).padding(20),
            ]
            .into(),
            Screen::Screen4 => column![
                text("mint's program").size(30),
                button("mintv0.1.0").on_press(Message::Mint010),
                button("mintv0.1.1").on_press(Message::Mint011),
                button("home").on_press(Message::SwitchToScreen1),
                text("4").size(20),
            ]
            .into(),
        }
    }
    pub fn update(state: &mut State, message: Message) {
        match message {
            Message::Mint010 => {
                let output: Result<std::process::ExitStatus, std::io::Error> =
                    Std_Command::new("cmd")
                        .arg("/c")
                        .arg("cd mints && cd mint1563 && cargo run")
                        .status();
                println!("{:?}", output);
            }
            Message::Mint011 => {
                let output: Result<std::process::ExitStatus, std::io::Error> =
                    Std_Command::new("cmd")
                        .arg("/c")
                        .arg("cd mints && cd mint1563011 && cargo run")
                        .status();
                println!("{:?}", output);
            }
            Message::ContentChanged(content) => {
                state.content = content;
            }
            Message::TextInput(content) => {
                state.input = content;
            }
            Message::CreateFile => {
                let content_bytes = state.content.as_bytes();
                let mut file = std::fs::File::create("mainapp/.security/data.txt").unwrap();
                file.write_all(content_bytes).unwrap();
            }
            Message::RsaEncryption => {
                let output: Result<std::process::ExitStatus, std::io::Error> =
                    Std_Command::new("cmd")
                        .arg("/c")
                        .arg("cd mainapp\\.security && encryption\\target\\debug\\encryption.exe")
                        .status();
                println!("{:?}", output);
            }
            Message::RsaDecryption => {}
            Message::SwitchToScreen1 => {
                state.current_screen = Screen::Screen1;
            }
            Message::SwitchToScreen2 => {
                state.current_screen = Screen::Screen2;
            }
            Message::SwitchToScreen3 => {
                state.current_screen = Screen::Screen3;
            }
            Message::SwitchToScreen4 => {
                state.current_screen = Screen::Screen4;
            }
            Message::AutoUpdate => {
                let output: Result<std::process::ExitStatus, std::io::Error> =
                    Std_Command::new("cmd")
                        .arg("/c")
                        .arg("winget upgrade -r && rustup update && cargo update")
                        .status();
                println!("{:?}", output);
            }
            Message::Tick(local_time) => {
                let now = local_time;

                if now != state.now {
                    state.now = now;
                    state.clock.clear();
                }
            }
            Message::AskAI => {
                let output: Result<std::process::ExitStatus, std::io::Error> =
                    Std_Command::new("cmd")
                        .arg("/c")
                        .arg("python3 C:/mint1563/src/chat_api.py")
                        .status();
                state.output = format!("{:?}", output);
            }
            Message::Vrm => {
                let url = get_vite_url().unwrap();
                println!("{}", url);
            }
        }
    }
}
