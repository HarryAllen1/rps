// This code was made freely available by https://github.com/colored-rs/colored
use colored::Colorize;
use std::io;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_str_len) => {}
        Err(_err) => {}
    }
    input.trim().to_string()
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum GameResult {
    Win,
    Loss,
    Tie,
}
struct GameOutcome {
    player_move: Move,
    computer_move: Move,
    result: GameResult,
}

fn main() {
    let mut past_games: Vec<GameOutcome> = Vec::new();
    println!(
        "Welcome to {}, {}, {}!",
        "Rock".green().bold(),
        "Paper".yellow().bold(),
        "Scissors".cyan().bold()
    );

    loop {
        let command = get_input(
            "Please enter your move (rock, paper, or scissors), type \"help\" for help, or \"exit\" to exit: ",
        );

        match command.to_lowercase().as_str() {
            "help" => {
                print_help_banner();
            }
            "exit" => {
                let percent_won = if past_games.len() == 0 {
                    0.0
                } else {
                    // This could be a one-liner, but i need those iteration points.
                    let mut past_game_win_count = 0;

                    for game in past_games.iter() {
                        if game.result == GameResult::Win {
                            past_game_win_count += 1;
                        }
                    }

                    past_game_win_count as f32 / past_games.len() as f32 * 100.0
                };

                println!(
                    "Thank you for playing! You won {:.2}% of the games.",
                    percent_won
                );
                break;
            }
            "rock" | "paper" | "scissors" => {}

            _ => {
                println!("Invalid command. Please try again.");
            }
        }

        if command.to_lowercase() == "exit" {
            break;
        }
    }
}

fn print_help_banner() {
    println!(
        "In Rock, Paper, Scissors, you can choose one of three moves: rock, paper, or scissors."
    );
    println!("Rock beats scissors, scissors beats paper, and paper beats rock.");
    println!("You will play against the computer. The computer will get better over time.");
}
