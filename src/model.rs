use std::time::Duration;

use anyhow::Result;
use tui_realm_stdlib::Phantom;
use tuirealm::{
    terminal::TerminalBridge,
    tui::layout::{Constraint, Layout},
    Application, EventListenerCfg, NoUserEvent, Sub, SubClause, SubEventClause, Update,
};

use crate::comp::{board::Board, letter_pool::LetterPool};

#[derive(Debug, PartialEq)]
pub enum Msg {
    None,
    Notification(String),
    Quit,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Id {
    Board,
    LetterPool,
    ToastNotification,
    GlobalListener,
}

pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
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
    pub fn new(answer: &str) -> Self {
        Self {
            app: Self::init_app(answer).expect("Could not initialise application"),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Could not initialise terminal"),
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

            self.app.view(&Id::Board, frame, rect_board);
            self.app.view(&Id::LetterPool, frame, rect_letter_pool);
            self.app.view(&Id::ToastNotification, frame, frame.size());
        })?;

        Ok(())
    }

    fn init_app(answer: &str) -> Result<Application<Id, Msg, NoUserEvent>> {
        let mut app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_millis(50)),
        );

        // Mount components
        let (letter_pool, pool_rc) = LetterPool::new();
        let board = Board::new(answer).with_letter_state(pool_rc); // TODO
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
        app.mount(Id::GlobalListener, Box::<Phantom>::default(), vec![])?;
        app.active(&Id::GlobalListener)?;

        Ok(app)
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
                _ => None,
            }
        } else {
            None
        }
    }
}
