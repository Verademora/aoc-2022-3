use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub const LOWER_START: u8 = 'a' as u8;
pub const LOWER_OFFSET: u8 = LOWER_START - 1;
pub const CAP_START: u8 = 'A' as u8;
pub const CAP_OFFSET: u8 = CAP_START - 27;

fn main() -> Result<(), Box<dyn Error>> {
    // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut total_score: u16 = 0;

    // Split the input and collect
    let rucksacks: Vec<&str> = buffer.trim().split('\n').collect();
    for rucksack in rucksacks {
        let mid_index = rucksack.len() / 2;

        let first_half = &rucksack[..mid_index];
        let second_half = &rucksack[mid_index..];


        for character in first_half.chars() {
            let shared_char = second_half.chars().any(|c| c == character);
            if shared_char {
                let mut ascii_value = character as u8;
                if character.is_uppercase() {
                    ascii_value -= CAP_OFFSET;
                } else {
                    ascii_value -= LOWER_OFFSET;
                }
                println!("Shared character: {}", &character);
                println!("Priority score: {}", &ascii_value);
                total_score += ascii_value as u16;
                break;
            }
        }
    }
    println!("Total score: {}", total_score);

    Ok(())
}
