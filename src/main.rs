// Copyright © 2019 Bader Alshaya
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use hangman::game_init;
use hangman::is_round_complete;
use hangman::play_or_stop;
use hangman::print_homescreen;
use hangman::round_init;
use hangman::scoreboard;
use hangman::show_progress;
use hangman::submit_guess;
use hangman::user_guess;

fn main() {
    let mut game = game_init();
    let mut game_on: bool;
    let mut round_on: bool;

    print_homescreen();

    game_on = true;
    while game_on {
        let mut index = 0;

        // Check if there are still unique words to guess
        if game.words_list.is_empty() {
            println!("CONGRATULATIONS, YOU GUESSED ALL THE WORDS!");
            break;
        }

        game.rounds.push(round_init(&game.words_list.pop().unwrap()));

        round_on = true;
        while game_on && round_on {
            let guess;

            // Show current round progress
            index = game.rounds.len() - 1;
            show_progress(&game.rounds[index]);

            // Get user guess and evaluate it
            guess = user_guess();
            if guess == '0' {
                game_on = false; // Quit the game
            } else {
                // Submit user guess and update the round progress
                submit_guess(&mut game.rounds[index], guess);

                if is_round_complete(&game.rounds[index].status) {
                    show_progress(&game.rounds[index]);
                    round_on = false;
                }
            }
            println!("\n-------------------------");
        }

        // Count rounds total points
        game.total_points += game.rounds[index].points;

        // Check whether the user wants to play the next round
        if game_on {
            game_on = play_or_stop();
        }
    }

    // Show the overall game details and score
    scoreboard(&game);
}
