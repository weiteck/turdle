use std::{borrow::Borrow, rc::Rc, sync::RwLock};

use indexmap::IndexMap;
use rand::Rng;
use tui_big_text::PixelSize;
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::Color,
    tui::{
        layout::{Constraint, Layout, Rect},
        style::Stylize,
        widgets::Block,
    },
    AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, Props, State,
};

use crate::{data::answers::ANSWERS, LetterState, Msg};

use super::{letter_pool::LetterPool, word_line::{WordLine, WordLineState}};

const LETTER_SIZE: PixelSize = PixelSize::Sextant;
const CELL_WIDTH: u16 = 10;
const CELL_HEIGHT: u16 = 4;
const CELL_VER_MARGIN: u16 = 1;
const CELL_HOR_MARGIN: u16 = 1;

#[derive(Clone, Debug)]
pub struct Board {
    props: Props,
    state: BoardState,
    lines: Vec<WordLine>,
    active_line: usize,
    bg: Option<u8>,
    letter_states: Rc<RwLock<IndexMap<char, LetterState>>>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum BoardState {
    #[default]
    Playing,
    Failed,
    Succeded,
}

impl Board {
    pub fn with_letter_state(mut self, ls: Rc<RwLock<IndexMap<char, LetterState>>>) -> Self {
        self.letter_states = ls;
        self
    }

    fn handle_input_char(&mut self, ch: char) -> CmdResult {
        let line = self
            .lines
            .get_mut(self.active_line)
            .expect("Could not get active word line");
        line.push_char(ch);

        CmdResult::None
    }

    fn handle_input_delete(&mut self) -> CmdResult {
        let line = self
            .lines
            .get_mut(self.active_line)
            .expect("Could not get active word line");
        line.del_char();

        CmdResult::None
    }

    fn handle_input_submit(&mut self) -> CmdResult {
        let line = self
            .lines
            .get_mut(self.active_line)
            .expect("Could not get active word line");

        let res = match line.submit() {
            WordLineState::Correct(res) => Some(res),
            WordLineState::Incorrect(res) => {
                if self.active_line < 5 {
                    self.active_line += 1;
                };
                Some(res)
            }
            WordLineState::None => None,
        };

        if let Some(res) = res {
            self.update_letter_pool(res);
        };

        CmdResult::None
    }

    fn update_letter_pool(&mut self, map: IndexMap<char, LetterState>) {
        let mut letter_pool_writer = self.letter_states.write().expect("Could not get write access to LetterStates.");
        for (ch, state) in map {
            letter_pool_writer.insert(ch, state);
        }
    }

    fn next_bg_colour(&mut self) {
        self.bg = match self.bg {
            None => Some(232), // Black
            Some(u8::MAX) => None,
            Some(idx) => Some(idx + 1),
        }
    }

    fn prev_bg_colour(&mut self) {
        self.bg = match self.bg {
            None => Some(u8::MAX), // White
            Some(232) => None,
            Some(idx) => Some(idx - 1),
        }
    }

    fn reset_bg_colour(&mut self) {
        self.bg = None;
    }
}

impl Default for Board {
    fn default() -> Self {
        let answers = ANSWERS.lines().collect::<Vec<_>>();
        let idx = rand::thread_rng().gen_range(0..answers.len());
        let answer = answers
            .get(idx)
            .expect("Could not get answer at index to start game");
        dbg!(answer);

        let lines = (0..6).map(|_| WordLine::default().with_answer(answer)).collect();
        Self {
            lines,
            props: Default::default(),
            state: Default::default(),
            active_line: Default::default(),
            bg: Default::default(),
            letter_states: Default::default(),
        }
    }
}

impl MockComponent for Board {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            if let Some(idx) = self.bg {
                let block = Block::default().bg(Color::Indexed(idx));
                frame.render_widget(block, frame.size())
            }

            let rects = Layout::vertical([
                Constraint::Length(CELL_HEIGHT + CELL_VER_MARGIN),
                Constraint::Length(CELL_HEIGHT + CELL_VER_MARGIN),
                Constraint::Length(CELL_HEIGHT + CELL_VER_MARGIN),
                Constraint::Length(CELL_HEIGHT + CELL_VER_MARGIN),
                Constraint::Length(CELL_HEIGHT + CELL_VER_MARGIN),
                Constraint::Length(CELL_HEIGHT + CELL_VER_MARGIN),
            ])
            .split(area);

            for i in 0..6 {
                let area = rects[i];

                // Update properties here to handle window resize later
                if let Some(wl) = self.lines.get_mut(i) {
                    wl.set_width(CELL_WIDTH);
                    wl.set_margin(CELL_HOR_MARGIN);
                    wl.set_letter_size(LETTER_SIZE);
                    wl.set_window_bg(self.bg);
                    wl.view(frame, area);
                }
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

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Type(ch) => self.handle_input_char(ch),
            Cmd::Delete => self.handle_input_delete(),
            Cmd::Submit => self.handle_input_submit(),
            _ => CmdResult::None,
        }
    }
}

impl Component<Msg, NoUserEvent> for Board {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            // Global hotkeys
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::Quit),

            // Background colour hotkeys
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.next_bg_colour();
                CmdResult::None
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageUp,
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.prev_bg_colour();
                CmdResult::None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home,
                modifiers: KeyModifiers::CONTROL,
            }) => {
                self.reset_bg_colour();
                CmdResult::None
            }

            // Input
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => self.perform(Cmd::Submit),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Type(ch)),

            _ => CmdResult::None,
        };

        Some(Msg::None)
    }
}
