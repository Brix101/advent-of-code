use crate::{Runner, Selector};

mod day_01;
mod day_02;

use day_01::*;
use day_02::*;

pub fn run_2023(which: Selector) {
    let mut day01 = Day01::new();
    let mut day02 = Day02::new();

    let mut days: Vec<&mut dyn Runner> = vec![&mut day01, &mut day02];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            let d = &mut days[last];
            crate::run_solution(*d);
        }
        Selector::All => {
            for d in days {
                crate::run_solution(d);
            }
        }
        Selector::One(num) => {
            let d = &mut days[num - 1];
            crate::run_solution(*d);
        }
    }
}
