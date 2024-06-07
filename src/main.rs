extern crate tuirealm;

use anyhow::{bail, Result};
use clap::{arg, Command};
use provider::AnswerProvider;
use model::Model;
use time::{Date, OffsetDateTime, Time};
use tuirealm::{PollStrategy, Update};

mod comp;
mod data;
mod model;
mod provider;
mod theme;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const APP_DESC: &str = env!("CARGO_PKG_DESCRIPTION");

pub enum AppMode {
    Random,
    Today(OffsetDateTime),
    Date(OffsetDateTime),
}

fn cli() -> Command {
    Command::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_DESC)
        .subcommand_required(false)
        .allow_external_subcommands(false)
        .subcommand(Command::new("random").about("Pick a random word (default)"))
        .subcommand(Command::new("today").about("Fetch today's solution from NYT"))
        .subcommand(
            Command::new("date")
                .about("Fetch the solution for the given date")
                .arg(arg!(date: <DATE> "The date in [YY]YY-MM-DD format"))
                .arg_required_else_help(true),
        )
}

fn parse_cli() -> Result<AppMode> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("today", _)) => {
            let today = OffsetDateTime::now_local()?;
            Ok(AppMode::Today(today))
        }

        Some(("date", date)) => {
            let date: Vec<&str> = date
                .get_one::<String>("date")
                .expect("Date should be provided")
                .split_terminator('-')
                .collect();
            if date.len() != 3 {
                bail!("Unable to parse date (use format YY-MM-DD)")
            };

            let year: i32 = date
                .first()
                .expect("Valid year should be provided (use format YY-MM-DD)")
                .parse()
                .expect("Valid year should be provided (use format YY-MM-DD)");
            // Allow for two-digit years
            // This will be a problem if Wordle is still a thing in 2100
            let year = if year < 100 { year + 2000 } else { year };
            let month: u8 = date
                .get(1)
                .expect("Valid month should be provided (use format YY-MM-DD)")
                .parse()
                .expect("Valid month should be provided (use format YY-MM-DD)");
            let month = time::Month::try_from(month)
                .expect("Valid month should be provided (use format YY-MM-DD)");
            let day: u8 = date
                .get(2)
                .expect("Valid day should be provided (use format YY-MM-DD)")
                .parse()
                .expect("Valid day should be provided (use format YY-MM-DD)");

            let date = OffsetDateTime::new_utc(
                Date::from_calendar_date(year, month, day)
                    .expect("Invalid date (use format YY-MM-DD)"),
                Time::MIDNIGHT,
            );

            Ok(AppMode::Date(date))
        }

        Some(("random", _)) => Ok(AppMode::Random),

        None => Ok(AppMode::Random), // Default

        _ => unreachable!("Not all valid CLI options were handled"),
    }
}

fn main() -> Result<()> {
    let mode = parse_cli()?;
    let answer = AnswerProvider.get_answer(mode)?;

    let mut model = Model::new(&answer);

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

    println!("The solution was: \"{}\"", answer);

    Ok(())
}
