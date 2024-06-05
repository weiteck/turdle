use std::{rc::Rc, sync::RwLock};

use indexmap::IndexMap;
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{Alignment, Style},
    tui::{
        layout::Rect,
        style::Stylize,
        text::{Line, Span},
        widgets::Paragraph,
    },
    AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, Props, State,
};

use crate::{
    model::{LetterState, Msg},
    theme,
};

#[derive(Debug, Clone, Default)]
pub struct LetterPool {
    props: Props,
    pool: Rc<RwLock<IndexMap<char, LetterState>>>,
    qwerty_mode: bool,
}

impl LetterPool {
    pub fn new() -> (Self, Rc<RwLock<IndexMap<char, LetterState>>>) {
        let mut pool = IndexMap::with_capacity(26);
        ('a'..='z').for_each(|c| _ = pool.insert(c, LetterState::Unused));

        let pool = Rc::new(RwLock::new(pool));
        let pool_rc = Rc::clone(&pool);

        let letter_pool = Self {
            props: Default::default(),
            pool,
            qwerty_mode: false,
        };

        (letter_pool, pool_rc)
    }

    pub fn toggle_qwerty_mode(&mut self) {
        self.qwerty_mode = !self.qwerty_mode;
    }
}

impl MockComponent for LetterPool {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let row_ordering_template = if self.qwerty_mode {
                vec![
                    vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
                    vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l'],
                    vec!['z', 'x', 'c', 'v', 'b', 'n', 'm'],
                ]
            } else {
                vec![
                    ('a'..'n').collect(),
                    ('n'..='z').collect(),
                    vec![], // Empty third line if alphabetical
                ]
            };

            let mut rows: Vec<Vec<Span>> = vec![vec![], vec![], vec![]];

            let guard = self.pool.read().unwrap();
            for (i, row) in row_ordering_template.iter().enumerate() {
                for ch in row {
                    let fg;
                    let bg;
                    if let Some(state) = guard.get(ch) {
                        fg = if *state == LetterState::Incorrect {
                            theme::LETTER_FG_INCORRECT
                        } else {
                            theme::LETTER_FG
                        };

                        bg = match state {
                            LetterState::Incorrect => theme::LETTER_BG_INCORRECT,
                            LetterState::Contains => theme::LETTER_BG_CONTAINS,
                            LetterState::Correct => theme::LETTER_BG_CORRECT,
                            _ => theme::LETTER_BG_UNUSED,
                        };

                        let ch = ch.to_ascii_uppercase().to_string();
                        if let Some(r) = rows.get_mut(i) {
                            r.push(Span::styled("▐", Style::default().fg(bg)));
                            r.push(Span::styled(ch, Style::default().bg(bg).fg(fg).bold()));
                            r.push(Span::styled("▌", Style::default().fg(bg)));
                        }
                    }
                }
            }

            for (i, row) in rows.into_iter().enumerate() {
                let line = Line::from(row);
                let par = Paragraph::new(line).alignment(Alignment::Center);

                // Offset third row left in QWERTY mode
                // Offset second row right in alpha mode
                let offset: i16 = if self.qwerty_mode && i == 2 {
                    -1
                } else if !self.qwerty_mode && i == 1 {
                    1
                } else {
                    0
                };
                let area = Rect {
                    x: (area.x as i16 + offset) as u16, // Horizontal offset
                    y: area.y + i as u16,               // Move to next row
                    ..area
                };

                // Render row
                frame.render_widget(par, area);
            }
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

impl Component<Msg, NoUserEvent> for LetterPool {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            _ => CmdResult::None,
        };

        Some(Msg::None)
    }
}
