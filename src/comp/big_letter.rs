use tui_big_text::{BigText, PixelSize};
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{Alignment, BorderType, Color, Style},
    tui::{
        layout::Rect,
        style::Stylize,
        widgets::{Block, Borders},
    },
    AttrValue, Attribute, Frame, MockComponent, Props, State,
};

use crate::{model::LetterState, theme};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BigLetter {
    props: Props,
    state: LetterState,
    value: Option<char>,
    size: PixelSize,
    bg: Option<u8>,
}

impl BigLetter {
    pub fn char(&self) -> Option<char> {
        self.value
    }

    pub fn with_char(mut self, ch: Option<char>) -> Self {
        if let Some(ch) = ch {
            self.value = Some(ch);
        } else {
            self.value = None;
        }
        self
    }

    pub fn with_size(mut self, size: PixelSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_window_bg(mut self, bg: Option<u8>) -> Self {
        self.bg = bg;
        self
    }

    pub fn letter_state(&self) -> LetterState {
        self.state
    }

    pub fn with_state(mut self, state: LetterState) -> Self {
        self.state = state;
        self
    }

    pub fn set_state(&mut self, state: LetterState) {
        self.state = state;
    }
}

impl MockComponent for BigLetter {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // TODO Move to style prop?
            let bg = match self.state {
                LetterState::Incorrect => theme::CELL_BG_INCORRECT,
                LetterState::Contains => theme::CELL_BG_CONTAINS,
                LetterState::Correct => theme::CELL_BG_CORRECT,
                _ => theme::CELL_BG_EMPTY,
            };

            let fg = theme::LETTER_FG;

            // Bottom border background to match window
            let window_bg = if self.bg.is_some() {
                Color::Indexed(self.bg.unwrap())
            } else {
                Color::Reset
            };

            let block = Block::default()
                .fg(fg)
                .bg(bg)
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(bg).bg(window_bg))
                .border_type(BorderType::QuadrantInside);
            frame.render_widget(block, area);

            let ch = self.value.unwrap_or_default().to_ascii_uppercase();
            let big_text = BigText::builder()
                .pixel_size(self.size)
                .style(Style::default().white())
                .alignment(Alignment::Center)
                .lines(vec![ch.to_string().into()])
                .build()
                .expect("Could not build BigText");
            let area = Rect {
                y: area.y + 1,
                // x: area.x + 1,
                ..area
            };
            frame.render_widget(big_text, area);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value)
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}
