use std::time::Duration;

use anyhow::{bail, Result};
use tui_realm_stdlib::Phantom as GlobalListener;
use tuirealm::{
    props::Style,
    terminal::TerminalBridge,
    tui::{
        buffer::Buffer,
        layout::{Constraint, Layout},
        style::Stylize,
    },
    Application, EventListenerCfg, NoUserEvent, PollStrategy, Sub, SubClause, SubEventClause,
    Update,
};

use crate::{
    comp::{board::Board, letter_pool::LetterPool},
    provider::Solution,
    ResultGrid,
};

const TERM_REQ_WIDTH: u16 = 55;
const TERM_REQ_HEIGHT: u16 = 34;

#[derive(Debug, PartialEq)]
pub enum Msg {
    None,
    Succeded(ResultGrid),
    Quit,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Id {
    Board,
    LetterPool,
    GlobalListener,
}

pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
    pub result_grid: Option<ResultGrid>,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum LetterState {
    #[default]
    Unused,
    Entered,   // Letter input but not evaluated
    Incorrect, // Letter not in word
    Contains,  // Letter in word but different position
    Correct,   // Letter and position correct
}

impl Model {
    pub fn new(solution: &Solution) -> Self {
        Self {
            app: Self::init_app(solution).expect("Could not initialise application"),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Could not initialise terminal"),
            result_grid: None,
        }
    }

    pub fn view(&mut self) -> Result<()> {
        self.terminal.raw_mut().draw(|frame| {
            let [_, rect_centre, _] = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(5 * 11),
                Constraint::Fill(1),
            ])
            .areas(frame.size());

            let [_, rect_board, _, rect_letter_pool, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(6 * 5), // Board
                Constraint::Length(1),     // Margin
                Constraint::Length(3),     // Letter pool
                Constraint::Fill(1),
            ])
            .areas(rect_centre);

            // Render components
            // Check terminal size to avoid panics
            if terminal_size_ok(frame.buffer_mut()) {
                self.app.view(&Id::Board, frame, rect_board);
                self.app.view(&Id::LetterPool, frame, rect_letter_pool);
            }
        })?;

        Ok(())
    }

    fn init_app(solution: &Solution) -> Result<Application<Id, Msg, NoUserEvent>> {
        let mut app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_millis(50)),
        );

        // Mount components
        let (letter_pool, pool_rc) = LetterPool::new();
        let board = Board::new(solution).with_letter_state(pool_rc);
        app.mount(
            Id::Board,
            Box::new(board),
            vec![Sub::new(SubEventClause::Any, SubClause::Always)],
        )?;
        app.mount(
            Id::LetterPool,
            Box::new(letter_pool),
            vec![Sub::new(SubEventClause::Any, SubClause::Always)],
        )?;
        app.mount(Id::GlobalListener, Box::<GlobalListener>::default(), vec![])?;
        app.active(&Id::GlobalListener)?;

        Ok(app)
    }

    // Main loop
    pub fn run(&mut self) -> Result<()> {
        while !self.quit {
            // Tick
            match self.app.tick(PollStrategy::Once) {
                Ok(messages) if !messages.is_empty() => {
                    // Redraw if a message has been processed
                    self.redraw = true;

                    for msg in messages.into_iter() {
                        let mut msg = Some(msg);
                        while msg.is_some() {
                            msg = self.update(msg);
                        }
                    }
                }

                Err(e) => {
                    bail!("Error: {}", e)
                }

                _ => {}
            }

            // Redraw
            if self.redraw {
                self.view()?;
                self.redraw = false;
            }
        }

        Ok(())
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Redraw UI if there is a message
            self.redraw = true;

            // Handle messages
            match msg {
                Msg::Quit => {
                    self.quit = true;
                    None
                }

                Msg::Succeded(rg) => {
                    self.result_grid = Some(rg);
                    None
                }

                _ => None,
            }
        } else {
            None
        }
    }
}

// Returns true if terminal size is large enough to render
// Otherwise renders text message with size information
fn terminal_size_ok(buf: &mut Buffer) -> bool {
    if buf.area.width >= TERM_REQ_WIDTH && buf.area.height >= TERM_REQ_HEIGHT {
        true
    } else {
        let msg = format!(
            "Terminal too small (min. {}W x {}H)",
            TERM_REQ_WIDTH, TERM_REQ_HEIGHT
        );
        buf.set_string(0, 0, msg, Style::default().bold());

        if buf.area.height >= 2 {
            let diff_w = TERM_REQ_WIDTH.saturating_sub(buf.area.width);
            let diff_h = TERM_REQ_HEIGHT.saturating_sub(buf.area.height);

            if diff_w > 0 && diff_h > 0 {
                let msg = format!("{} more cols & {} more rows needed", diff_w, diff_h);
                buf.set_string(0, 1, msg, Style::default().dim());
            } else if diff_w > 0 {
                let msg = format!("{} more columns needed", diff_w);
                buf.set_string(0, 1, msg, Style::default().dim());
            } else {
                let msg = format!("{} more rows needed", diff_h);
                buf.set_string(0, 1, msg, Style::default().dim());
            }

            if buf.area.height >= 3 {
                buf.set_string(0, 2, "Resize or <Esc> to exit", Style::default().dim())
            }
        }

        false
    }
}
