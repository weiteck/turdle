use std::time::{Duration, Instant};

use tuirealm::{
    command::CmdResult,
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderSides, Color, TextModifiers},
    tui::{
        layout::{Constraint, Layout}, style::Stylize, widgets::{Block, Paragraph}
    },
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent, Props, State,
};

use crate::Msg;

const DEFAULT_DURATION: u64 = 5_000;
const DEFAULT_ALIGNMENT: Alignment = Alignment::Center;
const DEFAULT_FOREGROUND: Color = Color::White;
const DEFAULT_BACKGROUND: Color = Color::Reset;

pub struct ToastNotification {
    props: Props,
    start_time: Instant,
    duration: Duration,
    hor_padding: u16,
}

impl ToastNotification {
    pub fn show(&mut self, s: &str) {
        self.attr(Attribute::Text, AttrValue::String(s.into()));
        self.props.set(Attribute::Display, AttrValue::Flag(true));
        self.start_time = Instant::now();
    }

    pub fn duration(mut self, millis: u64) -> Self {
        self.duration = Duration::from_millis(millis);
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    fn fire_test_notification(&mut self) -> CmdResult {
        self.show("Test notification");
        CmdResult::None
    }
}

impl Default for ToastNotification {
    fn default() -> Self {
        // Hidden until we call `show` with some text
        let mut props = Props::default();
        props.set(Attribute::Display, AttrValue::Flag(false));

        Self {
            props,
            start_time: Instant::now(),
            duration: Duration::from_millis(DEFAULT_DURATION),
            hor_padding: 1,
        }
    }
}

impl MockComponent for ToastNotification {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::prelude::Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let text = self
                .props
                .get_or(Attribute::Text, AttrValue::String(String::default()))
                .unwrap_string();
            let alignment = self
                .props
                .get_or(
                    Attribute::TextAlign,
                    AttrValue::Alignment(DEFAULT_ALIGNMENT),
                )
                .unwrap_alignment();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(DEFAULT_FOREGROUND))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(DEFAULT_BACKGROUND))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();

            // Centre content in the rect
            let [_, area, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(3),
                Constraint::Fill(1)
            ]).areas(area);

            let hor_len = text.len() as u16 + 2 + (self.hor_padding * 2);
            let [_, area, _] = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(hor_len),
                Constraint::Fill(1)
            ]).areas(area);
            

            let block = Block::new()
                .fg(foreground)
                .bg(background)
                .add_modifier(modifiers)
                .borders(BorderSides::ALL)
                .border_style(DEFAULT_FOREGROUND);

            frame.render_widget(Paragraph::new(text).alignment(alignment).block(block), area);

            // Hide if duration elapsed
            if self.start_time.elapsed() >= self.duration {
                self.props.set(Attribute::Display, AttrValue::Flag(false));
            }
        }
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: tuirealm::command::Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Msg, NoUserEvent> for ToastNotification {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('t'),
                modifiers: KeyModifiers::CONTROL,
            }) => self.fire_test_notification(),
            _ => CmdResult::None,
        };

        Some(Msg::None)
    }
}
