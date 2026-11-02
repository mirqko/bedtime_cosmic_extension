use std::time::{Duration, Instant};

use cosmic::iced::platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup};
use cosmic::iced::{window::Id, Limits, Subscription};
use cosmic::prelude::*;
use cosmic::widget;
use tokio::process::Command;

const APP_ID: &str = "io.github.mirqko.BedtimeCosmic";
const APPLET_ICON_WHITE: &[u8] = include_bytes!("../resources/icon_white.svg");
const APPLET_ICON_CYAN: &[u8] = include_bytes!("../resources/icon_cyan.svg");

#[derive(Default)]
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    deadline: Option<Instant>,
    status: String,
    timer_generation: u64,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    SetTimer(u64),
    CancelTimer,
    Tick,
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(core: cosmic::Core, _flags: Self::Flags) -> (Self, Task<cosmic::Action<Self::Message>>) {
        (
            Self {
                core,
                status: "No shutdown scheduled".to_string(),
                ..Self::default()
            },
            Task::none(),
        )
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }
    fn view(&self) -> Element<'_, Self::Message> {
        let icon_data = if self.deadline.is_some() {
            APPLET_ICON_CYAN
        } else {
            APPLET_ICON_WHITE
        };

        let icon = cosmic::iced::widget::svg::Svg::new(
            cosmic::iced::widget::svg::Handle::from_memory(icon_data),
        )
        .width(cosmic::iced::Length::Fixed(16.0))
        .height(cosmic::iced::Length::Fixed(16.0));

        widget::container(
            widget::mouse_area(icon)
                .on_press(Message::TogglePopup),
        )
        .height(cosmic::iced::Length::Fill)
        .align_y(cosmic::iced::Alignment::Center)
        .into()
    }


    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let content = widget::column::with_capacity(6)
            .spacing(8)
            .push(
                widget::text::body("Bedtime — shutdown timer")
            )
            .push(
                widget::text::body(self.status.as_str())
                    .class(cosmic::theme::Text::Color(
                        cosmic::iced::Color::from_rgb8(0, 210, 230),
                    )),
            )
            .push(
                widget::button::standard("Shutdown in 10 minutes")
                    .on_press(Message::SetTimer(10)),
            )
            .push(
                widget::button::standard("Shutdown in 30 minutes")
                    .on_press(Message::SetTimer(30)),
            )
            .push(
                widget::button::standard("Shutdown in 60 minutes")
                    .on_press(Message::SetTimer(60)),
            )
            .push(
                widget::button::standard("Cancel shutdown")
                    .on_press(Message::CancelTimer),
            );

        let content = widget::container(content)
            .class(cosmic::theme::Container::Card)
            .padding(12);

        self.core.applet.popup_container(content).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        cosmic::iced::time::every(Duration::from_secs(1))
            .map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<cosmic::Action<Message>> {
        match message {
            Message::TogglePopup => {
                return if let Some(id) = self.popup.take() {
                    destroy_popup(id)
                } else {
                    let id = Id::unique();
                    self.popup = Some(id);
                    let mut settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        id,
                        None,
                        None,
                        None,
                    );
                    settings.positioner.size_limits = Limits::NONE
                        .min_width(280.0)
                        .max_width(360.0)
                        .min_height(160.0)
                        .max_height(500.0);
                    get_popup(settings)
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::SetTimer(minutes) => {
                self.timer_generation = self.timer_generation.wrapping_add(1);
                self.deadline = Some(
                    Instant::now() + Duration::from_secs(minutes * 60)
                );
                self.status = format!("Shutdown in {minutes}:00");
            }

            Message::CancelTimer => {
                self.timer_generation = self.timer_generation.wrapping_add(1);
                self.deadline = None;
                self.status = "No shutdown scheduled".to_string();
            }

            Message::Tick => {
                if let Some(deadline) = self.deadline {
                    let remaining = deadline.saturating_duration_since(Instant::now());
                    if remaining.is_zero() {
                        self.deadline = None;
                        self.status = "Shutting down...".to_string();

                        return Task::perform(request_shutdown(), |_| {
                            cosmic::Action::App(Message::CancelTimer)
                        });
                    }

                    let seconds = remaining.as_secs();
                    self.status = format!(
                        "Shutdown in {:02}:{:02}",
                        seconds / 60,
                        seconds % 60
                    );
                }
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

async fn request_shutdown() {
    // Ask logind to power off the machine. On some setups this may be denied by
    // policy; see README.md for diagnostics and alternatives.
    let _ = Command::new("systemctl").arg("poweroff").status().await;
}
