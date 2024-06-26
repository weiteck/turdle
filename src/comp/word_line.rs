use std::time::{Duration, Instant};

use indexmap::IndexMap;
use tui_big_text::PixelSize;
use tuirealm::{
    command::{Cmd, CmdResult},
    tui::layout::{Constraint, Layout, Rect},
    AttrValue, Attribute, Frame, MockComponent, Props, State,
};

use crate::{
    data::{answers::ANSWERS, words::WORDS},
    model::LetterState,
};

use super::big_letter::BigLetter;

const ANIM_REVEAL_STEP_TIME: Duration = Duration::from_millis(350);

#[derive(Clone, Debug)]
pub struct WordLine {
    props: Props,
    state: WordLineState,
    letters: Vec<(char, LetterState)>,
    big_letter_size: PixelSize,
    cell_width: u16,
    cell_margin: u16,
    bg: Option<u8>,
    answer: String,
    animating_reveal: bool,
    revealed: usize,
    last_frame_time: Instant,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum WordLineState {
    #[default]
    None,
    Invalid, // Word not in either word list
    Incorrect(IndexMap<char, LetterState>),
    Correct(IndexMap<char, LetterState>),
}

impl WordLine {
    pub fn with_answer(mut self, answer: &str) -> Self {
        self.answer = answer.to_string();
        self
    }

    pub fn set_letter_size(&mut self, size: PixelSize) {
        self.big_letter_size = size;
    }

    pub fn set_width(&mut self, width: u16) {
        self.cell_width = width;
    }

    pub fn set_margin(&mut self, margin: u16) {
        self.cell_margin = margin;
    }

    pub fn set_window_bg(&mut self, bg: Option<u8>) {
        self.bg = bg;
    }

    pub fn push_char(&mut self, ch: char) {
        if self.letters.len() < 5 && ch.is_ascii_alphabetic() {
            let ch = ch.to_ascii_lowercase();
            self.letters.push((ch, LetterState::Entered));
        }
    }

    pub fn del_char(&mut self) {
        if !self.letters.is_empty() {
            self.letters.pop();
        }
    }

    pub fn get_letter_states(&self) -> Vec<LetterState> {
        self.letters.iter().map(|(_, ls)| *ls).collect()
    }

    // Assess entered word if 5 letters have been entered
    pub fn submit(&mut self) -> WordLineState {
        if self.letters.len() == 5 {
            let word = self.letters.iter().map(|(c, _)| c).collect::<String>();

            if word == self.answer {
                let res = self.validate_letters();
                self.begin_reveal();
                self.state = WordLineState::Correct(res)
            } else if ANSWERS.contains(word.as_str()) || WORDS.contains(word.as_str()) {
                let res = self.validate_letters();
                self.begin_reveal();
                self.state = WordLineState::Incorrect(res)
            } else {
                return WordLineState::Invalid;
            }
        }

        self.state.clone()
    }

    fn validate_letters(&mut self) -> IndexMap<char, LetterState> {
        // Helper function to remove `char`s from a `Vec<char>`.
        fn remove_char(chars: &mut Vec<char>, char_to_remove: &char) {
            if let Some(idx) = chars.iter().position(|c| c == char_to_remove) {
                chars.swap_remove(idx);
            }
        }

        let answer_chars = self.answer.chars().collect::<Vec<_>>();
        let mut answer_chars_not_matched = answer_chars.clone();
        let mut res = IndexMap::new();

        for i in 0..5 {
            let (entered_char, _) = self
                .letters
                .get(i)
                .expect("Entered letters did not have expected number of characters");
            let answer_char = answer_chars
                .get(i)
                .expect("Answer did not have expected number of characters");

            if entered_char == answer_char {
                remove_char(&mut answer_chars_not_matched, entered_char);
                res.insert(*entered_char, LetterState::Correct);
                self.letters[i] = (*entered_char, LetterState::Correct);
            } else if answer_chars_not_matched.contains(entered_char) {
                remove_char(&mut answer_chars_not_matched, entered_char);

                let current_state = res.insert(*entered_char, LetterState::Contains);
                // Don't downgrade `Correct` letters to `Contains`
                if current_state == Some(LetterState::Correct) {
                    res.insert(*entered_char, LetterState::Correct);
                }

                self.letters[i] = (*entered_char, LetterState::Contains);
            } else {
                let current_state = res.insert(*entered_char, LetterState::Incorrect);
                // Don't downgrade `Contains` letters to `Incorrect`
                if current_state == Some(LetterState::Contains) {
                    res.insert(*entered_char, LetterState::Contains);
                }
                self.letters[i] = (*entered_char, LetterState::Incorrect);
            }
        }
        res
    }

    fn begin_reveal(&mut self) {
        self.animating_reveal = true;
        self.last_frame_time = Instant::now();
    }
}

impl Default for WordLine {
    fn default() -> Self {
        Self {
            props: Default::default(),
            state: Default::default(),
            letters: Default::default(),
            big_letter_size: Default::default(),
            cell_width: Default::default(),
            cell_margin: Default::default(),
            bg: Default::default(),
            answer: Default::default(),
            animating_reveal: Default::default(),
            revealed: Default::default(),
            last_frame_time: Instant::now(),
        }
    }
}

impl MockComponent for WordLine {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            let margin = 1;

            // Outer cells with right-hand margin
            let col_rects = Layout::horizontal([
                Constraint::Length(self.cell_width + margin),
                Constraint::Length(self.cell_width + margin),
                Constraint::Length(self.cell_width + margin),
                Constraint::Length(self.cell_width + margin),
                Constraint::Length(self.cell_width + margin),
            ])
            .split(area);

            // Inner cells
            for i in 0..5 {
                let cell_rect = Layout::horizontal([Constraint::Length(self.cell_width)])
                    .split(col_rects[i])[0];

                if let Some((ch, state)) = self.letters.get(i) {
                    if self.animating_reveal && i <= self.revealed {
                        // Animate next letter
                        if self.last_frame_time.elapsed() >= ANIM_REVEAL_STEP_TIME {
                            self.revealed += 1;
                            self.last_frame_time = Instant::now();
                        }

                        let mut bl = BigLetter::default()
                            .with_char(Some(*ch))
                            .with_state(*state)
                            .with_size(self.big_letter_size)
                            .with_window_bg(self.bg)
                            .with_colour();
                        bl.view(frame, cell_rect);
                    } else {
                        let mut bl = BigLetter::default()
                            .with_char(Some(*ch))
                            .with_size(self.big_letter_size)
                            .with_window_bg(self.bg)
                            .with_colour();
                        bl.view(frame, cell_rect);
                    };
                } else {
                    // Empty cells
                    BigLetter::default()
                        .with_char(None)
                        .with_size(self.big_letter_size)
                        .with_window_bg(self.bg)
                        .view(frame, cell_rect);
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

    fn perform(&mut self, _: Cmd) -> CmdResult {
        CmdResult::None
    }
}
