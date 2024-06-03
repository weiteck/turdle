use std::time::Duration;

use anyhow::Result;
use tuirealm::{
    terminal::TerminalBridge,
    tui::layout::{Constraint, Layout},
    Application, EventListenerCfg, NoUserEvent, Update,
};

use crate::comp::{board::Board, letter_pool::LetterPool, toast::ToastNotification};

use super::{Id, Msg};

pub struct Model {
    pub app: Application<Id, Msg, NoUserEvent>,
    pub quit: bool,
    pub redraw: bool,
    pub terminal: TerminalBridge,
}

impl Model {
    pub fn view(&mut self) -> Result<()> {
        self.terminal.raw_mut().draw(|frame| {
            let [_, rect_centre, _] = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(5 * 11),
                Constraint::Fill(1),
            ])
            .areas(frame.size());

            let [rect_toast, rect_board, _, rect_letter_pool, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(6 * 5), // Board
                Constraint::Length(1),
                Constraint::Length(2), // Letter pool
                Constraint::Fill(1),
            ])
            .areas(rect_centre);

            self.app.view(&Id::Board, frame, rect_board);
            self.app.view(&Id::LetterPool, frame, rect_letter_pool);
            self.app.view(&Id::ToastNotification, frame, rect_toast);
        })?;

        Ok(())
    }

    fn init_app() -> Result<Application<Id, Msg, NoUserEvent>> {
        let mut app = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_millis(50)),
        );

        // Mount components
        let (letter_pool, pool_rc) = LetterPool::new();
        let board = Board::default().with_letter_state(pool_rc);
        app.mount(Id::Board, Box::new(board), vec![])?;
        app.mount(Id::LetterPool, Box::new(letter_pool), vec![])?;
        app.mount(
            Id::ToastNotification,
            Box::<ToastNotification>::default(),
            vec![],
        )?;
        app.active(&Id::Board)?;

        Ok(app)
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            app: Self::init_app().expect("Could not initialise application"),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Could not initialise terminal"),
        }
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
