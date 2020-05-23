extern crate rand;

use rand::distributions::{Distribution, Uniform};
use std::cmp;
mod statistics;

use crate::statistics::DataSet;

struct D20<T: Distribution<u8>> {
    rng: rand::rngs::ThreadRng,
    d20: T,
}

impl D20<Uniform<u8>> {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for D20<Uniform<u8>> {
    fn default() -> Self {
        D20 {
            rng: rand::thread_rng(),
            d20: Uniform::from(1..21),
        }
    }
}

impl<T: Distribution<u8>> D20<T> {
    fn roll(&mut self) -> u8 {
        self.d20.sample(&mut self.rng)
    }

    fn roll_advantage(&mut self) -> u8 {
        cmp::max(self.roll(), self.roll())
    }

    fn roll_disadvantage(&mut self) -> u8 {
        cmp::min(self.roll(), self.roll())
    }

    fn roll_advantage_of_disadvantage(&mut self) -> u8 {
        cmp::max(self.roll_disadvantage(), self.roll_disadvantage())
    }

    fn roll_disadvantage_of_advantage(&mut self) -> u8 {
        cmp::min(self.roll_advantage(), self.roll_advantage())
    }
}

fn rolls_over<F>(num: u8, mut fun: F) -> u16 where F: FnMut() -> u8 {
    let rolls: u16 = 10_000;
    let mut over: u16 = 0;
    for _ in 0..rolls {
        if num <= fun() {
            over += 1;
        }
    }
    over
}

fn main() {
    let mut d20 = D20::new();

    for beat_die in 2..21 {
        let mut results = std::collections::HashMap::new();

        results.insert("Single roll", rolls_over(beat_die, &mut || d20.roll()));
        results.insert("AdvOfDis", rolls_over(beat_die, &mut || d20.roll_advantage_of_disadvantage()));
        results.insert("DisOfAdv", rolls_over(beat_die, &mut || d20.roll_disadvantage_of_advantage()));

        print!("Rolls beating {}: ", beat_die);
        for (name, percent) in &results {
            print!("{}- {}, ", name, percent)
        }

        let winner = results.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
        print!("Winner is {:?}", winner);
        println!("");

        println!("------------------------------");

        let single_roll_set = DataSet::new((0..10_000).map(|_| {
            d20.roll()
        }).collect::<Vec<u8>>());
        let adv_of_dis_set = DataSet::new((0..10_000).map(|_| {
            d20.roll_advantage_of_disadvantage()
        }).collect::<Vec<u8>>());
        let dis_of_adv_set = DataSet::new((0..10_000).map(|_| {
            d20.roll_disadvantage_of_advantage()
        }).collect::<Vec<u8>>());
        println!("Mean: {:?}, Standard Deviation: {:?}", single_roll_set.mean, single_roll_set.std_deviation)
    }
}
