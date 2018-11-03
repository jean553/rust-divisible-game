extern crate rand;

use std::io::stdin;
use std::time::Instant;

use rand::{thread_rng, Rng};

/**
 * Handles user input with expected error message if failure.
 *
 * Args:
 *
 * `destination` - where the input should be stored
 */
fn handle_input(destination: &mut String) {
    stdin().read_line(destination).expect("Invalid input");
}

/**
 *
 */
fn main() {

    println!("Rust Divisible Game");

    let mut question_number = 1;
    let mut mark = 0;

    let start_time = Instant::now();

    loop {

        let mut rng = thread_rng();

        let dividand: u16 = rng.gen();

        const MIN_DIVISOR_VALUE: u8 = 2;
        const MAX_DIVISOR_VALUE: u8 = 11;
        let divisor = rng.gen_range(
            MIN_DIVISOR_VALUE,
            MAX_DIVISOR_VALUE,
        );

        question_number += 1;

        println!(
            "{} - Is {} divisible by {} ? y/n",
            question_number,
            dividand,
            divisor,
        );

        let mut input = String::new();
        handle_input(&mut input);

        let answer = input.bytes().nth(0).unwrap();

        let rest = dividand % divisor as u16;
        let is_divisible = rest == 0;

        const YES_INPUT: u8 = 'y' as u8;

        if answer == YES_INPUT && is_divisible ||
           answer != YES_INPUT && !is_divisible {
            println!("Correct!");

            if is_divisible && (
                divisor == 9 ||
                divisor == 8 ||
                divisor == 4
            ) {

                println!("What is the rest of this division ?");
                handle_input(&mut input);

                let answer = input.trim().parse::<u16>().unwrap();

                if rest == answer {
                    mark += 1;
                    println!("Correct!");
                } else {
                    println!("Wrong!");
                }
            } else {
                mark += 1;
            }

        } else {
            println!("Wrong!");
        }

        const ALLOWED_TIME_SECONDS: u8 = 60;
        if start_time.elapsed().as_secs() as u8 >= ALLOWED_TIME_SECONDS {
            break;
        }
    }

    println!("Finished! You successfully answered {} questions!", mark);
}
