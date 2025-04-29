use iced::mouse;
use iced::time;
use iced::time::Duration;
use iced::widget::canvas;
use iced::widget::canvas::stroke;
use iced::widget::canvas::Cache;
use iced::widget::canvas::Geometry;
use iced::widget::canvas::LineCap;
use iced::widget::canvas::Path;
use iced::widget::canvas::Stroke;
use iced::{alignment, Radians};
use iced::{Degrees, Font, Point, Rectangle, Renderer, Size, Subscription, Theme, Vector};

#[derive(Debug, Clone, Default)]
pub enum Screen {
    #[default]
    Screen1,
    Screen2,
    Screen3,
    Screen4,
}

#[derive(Debug, Clone)]
pub enum Message {
    Mint010,
    Mint011,
    ContentChanged(String),
    CreateFile,
    RsaEncryption,
    RsaDecryption,
    SwitchToScreen1,
    SwitchToScreen2,
    SwitchToScreen3,
    SwitchToScreen4,
    AutoUpdate,
    Tick(chrono::DateTime<chrono::Local>),
    TextInput(String),
    AskAI,
    Vrm,
}

pub struct State {
    pub content: String,
    pub current_screen: Screen,
    pub now: chrono::DateTime<chrono::Local>,
    pub clock: Cache,
    pub input: String,
    pub output: String,
}

impl State {
    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_millis(500)).map(|_| Message::Tick(chrono::offset::Local::now()))
    }

    pub fn theme(&self) -> Theme {
        Theme::ALL[(self.now.timestamp() as usize / 10) % Theme::ALL.len()].clone()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            content: "".to_string(),
            current_screen: Screen::Screen1,
            now: chrono::offset::Local::now(),
            clock: Cache::default(),
            input: "".to_string(),
            output: "".to_string(),
        }
    }
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        use chrono::Timelike;

        let clock = self.clock.draw(renderer, bounds.size(), |frame| {
            let palette = theme.extended_palette();

            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);
            frame.fill(&background, palette.secondary.strong.color);

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));

            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let width = radius / 100.0;

            let thin_stroke = || -> Stroke {
                Stroke {
                    width,
                    style: stroke::Style::Solid(palette.secondary.strong.text),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let wide_stroke = || -> Stroke {
                Stroke {
                    width: width * 3.0,
                    style: stroke::Style::Solid(palette.secondary.strong.text),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            frame.translate(Vector::new(center.x, center.y));
            let minutes_portion = Radians::from(hand_rotation(self.now.minute(), 60)) / 12.0;
            let hour_hand_angle =
                Radians::from(hand_rotation(self.now.hour(), 12)) + minutes_portion;

            frame.with_save(|frame| {
                frame.rotate(hour_hand_angle);
                frame.stroke(&short_hand, wide_stroke());
            });

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.minute(), 60));
                frame.stroke(&long_hand, wide_stroke());
            });

            frame.with_save(|frame| {
                let rotation = hand_rotation(self.now.second(), 60);

                frame.rotate(rotation);
                frame.stroke(&long_hand, thin_stroke());

                let rotate_factor = if rotation < 180.0 { 1.0 } else { -1.0 };

                frame.rotate(Degrees(-90.0 * rotate_factor));
                frame.fill_text(canvas::Text {
                    content: theme.to_string(),
                    size: (radius / 15.0).into(),
                    position: Point::new((0.78 * radius) * rotate_factor, -width * 2.0),
                    color: palette.secondary.strong.text,
                    horizontal_alignment: if rotate_factor > 0.0 {
                        alignment::Horizontal::Right
                    } else {
                        alignment::Horizontal::Left
                    },
                    vertical_alignment: alignment::Vertical::Bottom,
                    font: Font::MONOSPACE,
                    ..canvas::Text::default()
                });
            });

            // Draw clock numbers
            for hour in 1..=12 {
                let angle = Radians::from(hand_rotation(hour, 12)) - Radians::from(Degrees(90.0));
                let x = radius * angle.0.cos();
                let y = radius * angle.0.sin();

                frame.fill_text(canvas::Text {
                    content: format!("{}", hour),
                    size: (radius / 5.0).into(),
                    position: Point::new(x * 0.82, y * 0.82),
                    color: palette.secondary.strong.text,
                    horizontal_alignment: alignment::Horizontal::Center,
                    vertical_alignment: alignment::Vertical::Center,
                    font: Font::MONOSPACE,
                    ..canvas::Text::default()
                });
            }

            // Draw ticks
            for tick in 0..60 {
                let angle = hand_rotation(tick, 60);
                let width = if tick % 5 == 0 { 3.0 } else { 1.0 };

                frame.with_save(|frame| {
                    frame.rotate(angle);
                    frame.fill(
                        &Path::rectangle(Point::new(0.0, radius - 15.0), Size::new(width, 7.0)),
                        palette.secondary.strong.text,
                    );
                });
            }
        });

        vec![clock]
    }
}

fn hand_rotation(n: u32, total: u32) -> Degrees {
    let turns = n as f32 / total as f32;

    Degrees(360.0 * turns)
}
