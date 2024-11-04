pub mod commands;
pub mod db;

pub mod scheduler {
    use std::cmp;

    use crate::{
        algo::*,
        models::{Grade, Maturity, ReviewItemData},
    };
    use time::{Duration, PrimitiveDateTime as DateTime};

    /// Uses the data from the models and spaced repetition algorithm to determine
    pub struct Scheduler;

    impl Scheduler {
        // given an item, a grade and a moment in time, how should this item be scheduled?
        pub fn schedule(old: ReviewItemData, g: Grade, now: DateTime) -> ReviewItemData {
            use crate::models::Grade::*;
            use crate::models::Maturity::*;

            let ReviewItemData {
                difficulty: old_d,
                stability: old_s,
                maturity: old_m,
                last_review_date,
                ..
            } = old;
            let mut new = old.clone();
            let days = (now - last_review_date).whole_days();
            let r = retrievability(days as f32, old_s);

            let (new_s, new_d, dt, new_m) = match (old_m, g) {
                (New, Easy) => {
                    let s = update_stability_short_term(init_stability(g), g);
                    (
                        s,
                        init_difficulty(g),
                        Duration::days(s.round() as i64),
                        Review,
                    )
                }
                (New, _) => (
                    init_stability(g),
                    init_difficulty(g),
                    Duration::minutes(match g {
                        Fail => 1,
                        Hard => 5,
                        Good => 10,
                        Easy => unreachable!(),
                    }),
                    Learning,
                ),
                (Learning | ReLearning, Fail | Hard) => (
                    update_stability_short_term(old_s, g),
                    old_d,
                    Duration::minutes(match g {
                        Fail => 5,
                        Hard => 10,
                        _ => unreachable!(),
                    }),
                    old.maturity,
                ),
                (Learning | ReLearning, Good) => {
                    let s = update_stability_short_term(old_s, g);
                    (s, old_d, Duration::days(s.round() as i64), Review)
                }
                (Learning | ReLearning, Easy) => (
                    update_stability_short_term(old_s, Easy),
                    old_d,
                    Duration::days(cmp::max(
                        update_stability_short_term(old_s, Good).round() as i64 + 1,
                        update_stability_short_term(old_s, Easy).round() as i64,
                    )),
                    Review,
                ),
                (Review, Fail) => {
                    new.lapses += 1;
                    (
                        update_stability_fail(old_d, old_s, r),
                        update_difficulty(old_d, g),
                        Duration::minutes(5),
                        ReLearning,
                    )
                }
                (Review, g) => {
                    let harder_s = update_stability_success(old_d, old_s, r, g.prev());
                    let new_s = update_stability_success(old_d, old_s, r, g);
                    let new_d = update_difficulty(old_d, g);

                    let duration = Duration::days(match g {
                        Fail => unreachable!(),
                        Hard => cmp::min(harder_s.round() as i64, new_s.round() as i64),
                        Good => cmp::max(harder_s.round() as i64 + 1, new_s.round() as i64),
                        Easy => cmp::max(harder_s.round() as i64, new_s.round() as i64),
                    });

                    (new_s, new_d, duration, Review)
                }
            };

            new.last_review_date = now;
            new.reviews += 1;
            new.stability = new_s;
            new.difficulty = new_d;
            new.maturity = new_m;
            new.due = now + dt;

            new
        }
    }
}

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::init;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// struct defining the arguments and commands that the cli takes
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Init spbased in a directory. Will create a sqlite instance together with a local config file
    Init { directory: PathBuf },
}

pub fn handle_command(command: Command) -> Result<()> {
    match command {
        Command::Init { directory } => init(directory),
    }?;
    Ok(())
}
