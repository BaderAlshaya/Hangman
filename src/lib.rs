// Copyright © 2019 Bader Alshaya
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use colored::*;
use rand::Rng;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub const MAX_FAILED_ATTEMPTS: u8 = 7;

pub struct GameProgress {
    pub rounds: Vec<RoundProgress>,
    pub total_points: i64,
}

pub struct RoundProgress {
    pub hidden_word: Vec<HiddenChar>,
    pub status: RoundStatus,
    pub points: i64,
    pub failed_attempts: u8,
}

pub struct HiddenChar {
    pub character: char,
    pub is_hidden: bool,
}

pub enum RoundStatus {
    Ongoing,
    Lost,
    Won,
}

// Print the homescreen data (logo + instructions)
pub fn print_homescreen() {
    let file = File::open("data/homescreen.txt").unwrap();
    let buff = BufReader::new(file);

    // Clear the Terminal Screen
    print!("\x1B[2J");

    // Print each line in the file
    for line in buff.lines() {
        println!("{}", line.unwrap().yellow().bold());
    }
}

// Game initializer
pub fn game_init() -> GameProgress {
    GameProgress {
        rounds: Vec::new(),
        total_points: 0,
    }
}

// Round initializer
pub fn round_init() -> RoundProgress {
    RoundProgress {
        hidden_word: random_word(),
        status: RoundStatus::Ongoing,
        points: 0,
        failed_attempts: 0,
    }
}

// Return a random word from the words file as hidden letters
pub fn random_word() -> Vec<HiddenChar> {
    let file = File::open("data/words.txt").unwrap();
    let buff = BufReader::new(file);
    let mut result = Vec::<HiddenChar>::new();

    // Read the words from the file into a vector of strings
    let mut words = Vec::<String>::new();
    for word in buff.lines() {
        words.push(word.unwrap());
    }

    // Index a random word from the vector and use it as the hidden word
    let mut rand_word = String::new();
    rand_word.push_str(&words[rand::thread_rng().gen_range(0, words.len())]);
    for ch in rand_word.chars() {
        result.push(HiddenChar {
            character: ch,
            is_hidden: true,
        });
    }

    result // return the random hidden word
}

// Return the hidden word for a single round
pub fn get_hidden_word(round: &RoundProgress) -> String {
    let mut word = String::new();
    for ch in &round.hidden_word {
        word.push(ch.character);
    }
    word
}

// Show the progress of the round
pub fn show_progress(round: &RoundProgress) {
    let mut current_progress = String::new();

    // Show the current progress of the hidden word
    for ch in &round.hidden_word {
        if ch.is_hidden {
            current_progress.push('_');
        } else {
            current_progress.push(ch.character);
        }
        current_progress.push(' ');
    }

    println!("\nFailed Attempts [{}]\n", round.failed_attempts);
    println!("PROGRESS: {}\n", current_progress);

    match round.status {
        RoundStatus::Ongoing => {}

        RoundStatus::Lost => {
            println!("-------------------------");
            println!("\n{}\n", "YOU LOST THIS ROUND!".red().bold());
            println!("It was ({})", get_hidden_word(&round).underline());
        }

        RoundStatus::Won => {
            println!("-------------------------");
            println!("\n{}\n", "YOU WON THIS ROUND!".green().bold());
            println!("It Was: {}", get_hidden_word(&round).underline());
        }
    }
}

// Return the evaluated input of the user
pub fn user_guess() -> char {
    let mut input = String::new();
    let result;

    print!("Please enter your guess: ");
    io::stdout().flush().expect("FAILED!");

    // Evaluate if input is a valid single English character
    match io::stdin().read_line(&mut input) {
        Ok(_) => result = evaluate_input(&input.to_lowercase()),
        Err(_) => result = '!',
    }

    // Return the valid character OR repeat
    if result == '!' {
        println!("Invalid input! Try again.");
        user_guess()
    } else {
        result
    }
}

// Return the input evaluation result
pub fn evaluate_input(input: &str) -> char {
    if input.trim().len() > 1 {
        '!' // ERROR
    } else {
        let character = input.chars().next().unwrap();
        if character.is_ascii_alphabetic() || character == '0' {
            character
        } else {
            '!' // ERROR
        }
    }
}

// Evaluate the guess and update round progress
pub fn submit_guess(round: &mut RoundProgress, guess: char) {
    let mut failed_attempt = true;
    let mut all_guessed = true;
    let mut points = 1;

    // Check if user guess matches any hidden characters
    for ch in &mut round.hidden_word {
        if ch.character == guess {
            ch.is_hidden = false;
            points *= points + 1;
            failed_attempt = false;
        }
        if ch.is_hidden {
            all_guessed = false;
        }
    }

    // Update guess attempts and points
    if failed_attempt {
        round.failed_attempts += 1;
    } else {
        round.points = points;
    }

    // Update round status
    if all_guessed {
        round.status = RoundStatus::Won;
    } else if round.failed_attempts == MAX_FAILED_ATTEMPTS {
        round.status = RoundStatus::Lost;
    }
}

// Check whether round is complete
pub fn is_round_complete(status: &RoundStatus) -> bool {
    match status {
        RoundStatus::Ongoing => false,
        RoundStatus::Lost => true,
        RoundStatus::Won => true,
    }
}

// Implement a Display for the RoundStatus enum
impl fmt::Display for RoundStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RoundStatus::Ongoing => write!(f, "Ongoing"),
            RoundStatus::Lost => write!(f, "Lost"),
            RoundStatus::Won => write!(f, "Won"),
        }
    }
}

// Print the game details
pub fn scoreboard(game: &GameProgress) {
    let mut count = 1;
    for round in &game.rounds {
        println!("Round {}", count);
        println!("- Hidden Word: {}", get_hidden_word(&round));
        println!("- Round Status: {}:", round.status);
        println!("- Round Points: {}:", round.points);
        println!("- Failed Attempts: {}:", round.failed_attempts);
        println!("-------------------------");
        count += 1;
    }
}
