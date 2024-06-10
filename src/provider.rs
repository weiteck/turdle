use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

use anyhow::{bail, Result};
use rand::Rng;
use time::{Month, OffsetDateTime};

use crate::{data::answers::ANSWERS, AppMode};

const REQ_TIMEOUT: u64 = 10;

#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    pub wordle_number: Option<u64>,
    pub answer: String,
}

#[derive(Default)]
pub struct SolutionProvider;

impl SolutionProvider {
    pub fn get_answer(&self, mode: AppMode) -> Result<Solution> {
        match mode {
            AppMode::Random => Ok(random_answer()),
            AppMode::Today(date) | AppMode::Date(date) => self.get_answer_for_date(date),
        }
    }

    fn get_answer_for_date(&self, date: OffsetDateTime) -> Result<Solution> {
        // Check date is not before the first Wordle was published
        let first_wordle_date = OffsetDateTime::new_utc(
            time::Date::from_calendar_date(2021, Month::June, 19)?,
            time::Time::MIDNIGHT,
        );
        if date < first_wordle_date {
            bail!("Date cannot be before first Wordle was published (2021-06-19)");
        }

        let (year, month, day) = date.to_calendar_date();
        print!("Retrieving solution for {} {} {} ... ", day, month, year);
        io::stdout().flush()?;

        let month = u8::from(month);
        let url = format!(
            "https://www.nytimes.com/svc/wordle/v2/{}-{:02}-{:02}.json",
            year, month, day
        );

        let client = reqwest::blocking::Client::new();
        let req = client
            .get(url)
            .timeout(Duration::from_secs(REQ_TIMEOUT))
            .build()?;
        let res = client.execute(req)?.text()?;

        let json: serde_json::Value = serde_json::from_str(&res)?;
        let wordle_number = json
            .get("days_since_launch")
            .expect("Could not retrieve Worlde number from NYT API")
            .as_u64()
            .expect("Could not retrieve Wordle number from NYT API");
        let answer = json
            .get("solution")
            .expect("Could not retrieve solution from NYT API")
            .as_str()
            .expect("Could not retrieve solution from NYT API");

        println!("OK"); // Was able to retrieve solution
        sleep(Duration::from_secs(1)); // Delay so output is readable

        Ok(Solution {
            wordle_number: Some(wordle_number),
            answer: answer.into(),
        })
    }
}

fn random_answer() -> Solution {
    let answers = ANSWERS.lines().collect::<Vec<_>>();
    let idx = rand::thread_rng().gen_range(0..answers.len());
    let answer = *answers
        .get(idx)
        .expect("Could not get random answer at index to start game");

    Solution {
        wordle_number: None,
        answer: answer.into(),
    }
}
