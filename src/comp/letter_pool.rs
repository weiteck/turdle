use std::{rc::Rc, sync::RwLock};

use indexmap::IndexMap;
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{Alignment, Color, Style},
    tui::{
        layout::Rect,
        style::Stylize,
        text::{Line, Span},
        widgets::Paragraph,
    },
    AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, Props, State,
};

use crate::{theme, LetterState, Msg};

#[derive(Debug, Clone, Default)]
pub struct LetterPool {
    props: Props,
    pool: Rc<RwLock<IndexMap<char, LetterState>>>,

}

impl LetterPool {
    // pub fn with_letter_state(mut self, ls: Rc<RwLock<IndexMap<char, LetterState>>>) -> Self {
    //     self.pool = ls;
    //     self
    // }

    pub fn new() -> (Self, Rc<RwLock<IndexMap<char, LetterState>>>) {
        let mut pool = IndexMap::with_capacity(26);
        ('a'..='z').for_each(|c| _ = pool.insert(c, LetterState::Unused));

        let pool = Rc::new(RwLock::new(pool));
        let pool_rc = Rc::clone(&pool);

        let letter_pool = Self {
            props: Default::default(),
            pool,
        };

        (letter_pool, pool_rc)
    }

    // pub fn set_state(&mut self, ch: char, state: LetterState) {
    //     self.pool.insert(ch, state);
    // }

    // pub fn set_state_batch(&mut self, states: IndexMap<char, LetterState>) {
    //     for (ch, state) in states {
    //         self.pool.insert(ch, state);
    //     }
    // }
}

// impl Default for LetterPool {
//     fn default() -> Self {
//         let mut pool = IndexMap::new();
//         ('a'..='z').for_each(|c| _ = pool.insert(c, LetterState::Unused));

//         Self {
//             props: Default::default(),
//             pool,
//         }
//     }
// }

impl MockComponent for LetterPool {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let mut letter_spans_1 = vec![];
            let mut letter_spans_2 = vec![];

            let guard = self.pool.read().unwrap();
            for (ch, state) in &*guard {
                let fg = if *state == LetterState::Incorrect {
                    theme::LETTER_FG_INCORRECT
                } else {
                    theme::LETTER_FG
                };

                let bg = match state {
                    LetterState::Incorrect => theme::LETTER_BG_INCORRECT,
                    LetterState::Contains => theme::LETTER_BG_CONTAINS,
                    LetterState::Correct => theme::LETTER_BG_CORRECT,
                    _ => theme::LETTER_BG_UNUSED,
                };

                if *ch < 'n' {
                    let ch = ch.to_ascii_uppercase().to_string();
                    letter_spans_1.push(Span::styled("▐", Style::default().fg(bg)));
                    letter_spans_1.push(Span::styled(ch, Style::default().bg(bg).fg(fg).bold()));
                    letter_spans_1.push(Span::styled("▌", Style::default().fg(bg)));
                } else {
                    let ch = ch.to_ascii_uppercase().to_string();
                    letter_spans_2.push(Span::styled("▐", Style::default().fg(bg)));
                    letter_spans_2.push(Span::styled(ch, Style::default().bg(bg).fg(fg).bold()));
                    letter_spans_2.push(Span::styled("▌", Style::default().fg(bg)));
                }
            }
            let line_1 = Line::from(letter_spans_1);
            let line_2 = Line::from(letter_spans_2);
            let par_1 = Paragraph::new(line_1).alignment(Alignment::Center);
            let par_2 = Paragraph::new(line_2).alignment(Alignment::Center);

            frame.render_widget(par_1, area);

            // Offset second line
            let area = Rect {
                x: area.x + 1,
                y: area.y + 1,
                ..area
            };
            frame.render_widget(par_2, area);
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
