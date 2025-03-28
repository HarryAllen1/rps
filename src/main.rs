// This code was made freely available by https://github.com/colored-rs/colored
use colored::Colorize;
// This code was made freely available by https://github.com/rust-random/rand
use rand::Rng;
// This code was made freely available by https://github.com/rust-lang/rust/tree/master/library/std
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

#[derive(PartialEq, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Clone, Copy)]
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
            "rock" | "paper" | "scissors" => {
                if past_games.len() == 0 {
                    let computer_move = random_move();
                    let player_move = str_to_move(&command);

                    let winner = determine_winner(&player_move, &computer_move);
                    let game_outcome = GameOutcome {
                        player_move,
                        computer_move,
                        result: winner,
                    };

                    past_games.push(game_outcome);
                    match winner {
                        GameResult::Win => {
                            println!(
                                "You win! {} beats {}.",
                                move_to_str(&player_move),
                                move_to_str(&computer_move)
                            );
                        }
                        GameResult::Loss => {
                            println!(
                                "You lose! {} beats {}.",
                                move_to_str(&computer_move),
                                move_to_str(&player_move)
                            );
                        }
                        GameResult::Tie => {
                            println!("You tied! You both chose {}.", move_to_str(&player_move));
                        }
                    }
                }
            }

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
    print!(
        "Welcome to {}, {}, {}!",
        "Rock".green().bold(),
        "Paper".yellow().bold(),
        "Scissors".cyan().bold()
    );
    println!(
        "In Rock, Paper, Scissors, you can choose one of three moves: rock, paper, or scissors."
    );
    println!("Rock beats scissors, scissors beats paper, and paper beats rock.");
    println!("You will play against the computer. The computer will get better over time.");
}

fn random_move() -> Move {
    let mut rng = rand::rng();
    let random_number = rng.random_range(0..3);
    match random_number {
        0 => Move::Rock,
        1 => Move::Paper,
        _ => Move::Scissors,
    }
}

fn str_to_move(str: &String) -> Move {
    match str.to_lowercase().trim() {
        "rock" => Move::Rock,
        "paper" => Move::Paper,
        _ => Move::Scissors,
    }
}

fn move_to_str(mv: &Move) -> String {
    match mv {
        Move::Rock => "rock".to_string(),
        Move::Paper => "paper".to_string(),
        Move::Scissors => "scissors".to_string(),
    }
}

fn determine_winner(player_move: &Move, computer_move: &Move) -> GameResult {
    if player_move == computer_move {
        return GameResult::Tie;
    }

    match (player_move, computer_move) {
        (Move::Rock, Move::Scissors) => GameResult::Win,
        (Move::Paper, Move::Rock) => GameResult::Win,
        (Move::Scissors, Move::Paper) => GameResult::Win,
        _ => GameResult::Loss,
    }
}
