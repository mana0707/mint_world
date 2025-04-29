mod apps;
use apps::state::State;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application("mint's program", apps::App::update, apps::App::view)
        .subscription(State::subscription)
        .theme(State::theme)
        .antialiasing(true)
        .run()
}
