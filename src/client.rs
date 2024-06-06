use anyhow::{bail, Result};
use rand::Rng;
use reqwest::Url;
use time::OffsetDateTime;

use crate::{data::answers::ANSWERS, AppMode};

#[derive(Default)]
pub struct AnswerClient;

impl AnswerClient {
    pub fn get_answer(&self, mode: AppMode) -> Result<String> {
        match mode {
            AppMode::Random => Ok(random_answer()),
            AppMode::Today(date) | AppMode::Date(date) => self.get_answer_for_date(date),
        }
    }

    fn get_answer_for_date(&self, date: OffsetDateTime) -> Result<String> {
        let first_wordle_date = OffsetDateTime::new_utc(
            time::Date::from_calendar_date(2021, time::Month::June, 19)?,
            time::Time::MIDNIGHT,
        );

        if date < first_wordle_date {
            bail!("Date cannot be before first Wordle was published (2021-06-19)");
        }

        let (year, month, day) = date.to_calendar_date();

        print!("Retrieving solution for {} {} {} ... ", day, month, year);

        let month = u8::from(month);
        let url = format!(
            "https://www.nytimes.com/svc/wordle/v2/{}-{:02}-{:02}.json",
            year, month, day
        );
        let url = Url::parse(url.as_str())?;
        let res = reqwest::blocking::get(url.clone())?.text()?;
        let res: serde_json::Value = serde_json::from_str(&res)?;
        let answer = res
            .get("solution")
            .expect("Could not retrieve solution from NYT API")
            .as_str()
            .expect("Could not retrieve solution from NYT API");

        println!("OK");
        Ok(answer.into())
    }
}

fn random_answer() -> String {
    let mut rng = rand::thread_rng();

    let answers = ANSWERS.lines().collect::<Vec<_>>();
    let idx = rng.gen_range(0..answers.len());
    let answer = *answers
        .get(idx)
        .expect("Could not get answer at index to start game");

    // let answer = "aback"; // TESTING
    

    answer.into()
}
