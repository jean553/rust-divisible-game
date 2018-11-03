extern crate rand;

use std::io::stdin;

use rand::{thread_rng, Rng};

/**
 *
 */
fn main() {

    println!("Rust Divisible Game");

    let mut question_number = 1;
    let mut mark = 0;

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
        stdin().read_line(&mut input).expect("Invalid input.");

        let answer = input.bytes().nth(0).unwrap();

        let is_divisible = dividand % divisor as u16 == 0;

        const YES_INPUT: u8 = 'y' as u8;
        
        if answer == YES_INPUT && is_divisible ||
           answer != YES_INPUT && !is_divisible {
            mark += 1;
            println!("Correct!");
        } else {
            println!("Wrong!");
        }

        if question_number == 20 {
            break;
        }
    }

    println!("Finished! Your mark is {} / 20.", mark);
}
