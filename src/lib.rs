// Copyright © 2019 Bader Alshaya
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use colored::*;
use rand::*;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

pub const MAX_FAILED_ATTEMPTS: u8 = 7;

pub struct GameProgress {
    pub rounds: Vec<RoundProgress>,
    pub words_list: Vec::<String>,
    pub total_points: i64,
}

pub struct RoundProgress {
    pub hidden_word: Vec<HiddenChar>,
    pub failed_attempts: Vec<char>,
    pub status: RoundStatus,
    pub points: i64,
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

// Game initializer
pub fn game_init() -> GameProgress {
    GameProgress {
        words_list: get_list(),
        rounds: Vec::new(),
        total_points: 0,
    }
}

// Round initializer
pub fn round_init(rand_word: String) -> RoundProgress {
    RoundProgress {
        hidden_word: hide_word(rand_word),
        failed_attempts: Vec::new(),
        status: RoundStatus::Ongoing,
        points: 0,
    }
}

// Return a random list of words assigned for this game
pub fn get_list() -> Vec<String> {
    let file = File::open("data/words.txt").unwrap();
    let buff = BufReader::new(file);
    let mut words = Vec::<String>::new();

    // Read the words from the file into a vector of strings
    for word in buff.lines() {
        words.push(word.unwrap());
    }

    // Shuffle the list of words
    thread_rng().shuffle(&mut words);

    words
}

// Return a random word from the words file as hidden letters
pub fn hide_word(rand_word: String) -> Vec<HiddenChar> {
    let mut result = Vec::<HiddenChar>::new();

    // rand_word.push_str(&game.words_list.remove(rand::thread_rng().gen_range(0, game.words_list.len())));
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

    println!("\nFailed Attempts [{}]\n", round.failed_attempts.len());
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
    let mut is_duplicate = false;
    let mut all_guessed = true;
    let mut points = 1;

    // Check if user guess matches any hidden characters
    for ch in &mut round.hidden_word {
        if ch.character == guess && ch.is_hidden {
            ch.is_hidden = false;
            failed_attempt = false;
            points *= points + 1;
        } else if ch.character == guess && !ch.is_hidden {
            is_duplicate = true;
            points = 0; // No points for correct, but repeated guess
        }
        if ch.is_hidden {
            all_guessed = false;
        }
    }

    // Update guess attempts and points
    if failed_attempt {
        // Check that the character was not used at the same round
        for ch in &round.failed_attempts {
            if guess == *ch {
                is_duplicate = true;
                break;
            }
        }

        if is_duplicate {
            print!("\n{} {} ", "You already entered the letter".purple(), guess);
        } else {
            println!("\n{}", "WRONG!".bold().red());
            round.failed_attempts.push(guess);
        }
    } else {
        println!("\n{}", "CORRECT!".bold().green());
        round.points += points;
    }

    // Update round status
    if all_guessed {
        round.status = RoundStatus::Won;
    } else if round.failed_attempts.len() == MAX_FAILED_ATTEMPTS as usize {
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
        match round.status {
            RoundStatus::Ongoing => {
                println!("{}{}", "Round ".cyan(), count.to_string().cyan());
                println!("{}{}", "- Hidden Word: ".italic(), get_hidden_word(&round).underline().cyan());
                println!("{}{}", "- Round Status: ".italic(), round.status.to_string().cyan());
                println!("{}{}", "- Round Points: ".italic(), round.points.to_string().cyan());
                println!("{}{}", "- Failed Attempts: ".italic(), round.failed_attempts.len().to_string().cyan());
            } RoundStatus::Lost => {
                println!("{}{}", "Round ".red(), count.to_string().red());
                println!("{}{}", "- Hidden Word: ".italic(), get_hidden_word(&round).underline().red());
                println!("{}{}", "- Round Status: ".italic(), round.status.to_string().red());
                println!("{}{}", "- Round Points: ".italic(), round.points.to_string().red());
                println!("{}{}", "- Failed Attempts: ".italic(), round.failed_attempts.len().to_string().red());
            } RoundStatus::Won => {
                println!("{}{}", "Round ".green(), count.to_string().green());
                println!("{}{}", "- Hidden Word: ".italic(), get_hidden_word(&round).underline().green());
                println!("{}{}", "- Round Status: ".italic(), round.status.to_string().green());
                println!("{}{}", "- Round Points: ".italic(), round.points.to_string().green());
                println!("{}{}", "- Failed Attempts: ".italic(), round.failed_attempts.len().to_string().green());
            }
        }
        println!("-------------------------");
        count += 1;
    }
}

// Return the evaluated input of the user
pub fn play_or_stop() -> bool {
    let mut input = String::new();
    let result;

    println!("{}", "- Enter 0 to exit the game and view your results.".yellow());
    println!("{}", "- Enter anything to go to the next round.".yellow());
    print!("{}", "Your input: ".yellow());
    io::stdout().flush().expect("FAILED!");

    match io::stdin().read_line(&mut input) {
        Ok(_) => result = input.chars().next().unwrap(),
        Err(_) => result = '!',
    }
    println!("-------------------------");

    result != '0' && result != '!'
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
