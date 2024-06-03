extern crate tuirealm;

use anyhow::{bail, Result};

use app::model::Model;
use indexmap::IndexMap;
use tuirealm::{PollStrategy, Update};

mod app;
mod comp;
mod data;
mod theme;

#[derive(Debug, PartialEq)]
pub enum Msg {
    Quit,
    None,
    LetterUpdate(IndexMap<char, LetterState>), // Bulk update letters with new state
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Id {
    Board,
    LetterPool,
    ToastNotification,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum LetterState {
    #[default]
    Unused,
    Entered,   // Letter input, unevaluated
    Incorrect, // Letter not in word
    Contains,  // Letter in word but different position
    Correct,   // Letter in word at entered position
}

fn main() -> Result<()> {
    let mut model = Model::default();

    // Init terminal
    model.terminal.enter_alternate_screen()?;
    model.terminal.enable_raw_mode()?;

    // Main loop
    while !model.quit {
        // Tick
        match model.app.tick(PollStrategy::Once) {
            Ok(messages) if !messages.is_empty() => {
                // Redraw if a message has been processed
                model.redraw = true;

                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }

            Err(e) => {
                bail!("Error: {}", e)
            }

            _ => {}
        }

        // Redraw
        if model.redraw {
            model.view()?;
            model.redraw = false;
        }
    }

    // Restore terminal
    model.terminal.leave_alternate_screen()?;
    model.terminal.disable_raw_mode()?;
    // model.terminal.clear_screen()?;

    Ok(())
}
