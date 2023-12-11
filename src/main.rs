use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    let days = [
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
    ];
    if let Some(arg) = env::args().nth(1) {
        println!("Day {}:", arg);
        days[arg.parse::<usize>().unwrap() - 1]();
    } else {
        days.iter().enumerate().for_each(|(day, solution)| {
            println!("Day {}:", day + 1);
            solution();
            println!();
        });
    }
}
