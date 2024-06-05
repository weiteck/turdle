extern crate tuirealm;

use anyhow::{bail, Result};
use model::Model;
use tuirealm::{PollStrategy, Update};

mod comp;
mod data;
mod model;
mod theme;

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

    Ok(())
}
