// This code was made freely available by https://github.com/colored-rs/colored
use colored::Colorize;
// This code was made freely available by https://github.com/rust-random/rand
use rand::Rng;
// This code was made freely available by https://github.com/rust-lang/rust/tree/master/library/std
use std::io;

/// Prompts the user for input (on the same line) and returns the string entered after the user presses enter.
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

/// Represents the status of the player (i.e., if the player wins, it's Win)
#[derive(PartialEq, Clone, Copy)]
enum GameResult {
    Win,
    Loss,
    Tie,
}
#[derive(PartialEq, Clone, Copy)]
struct GameOutcome {
    player_move: Move,
    computer_move: Move,
    result: GameResult,
}

fn print_welcome() {
    println!(
        "Welcome to {}, {}, {}!",
        "Rock".green().bold(),
        "Paper".yellow().bold(),
        "Scissors".cyan().bold()
    );
}

fn main() {
    print_welcome();

    // Storing the past moves allows the user to predict the next move based on the last item
    // and also allows it to generate game statistics.
    let mut past_games: Vec<GameOutcome> = Vec::new();
    loop {
        // get the command from the user via the cli
        let command = get_input(
            "Please enter your move (rock, paper, or scissors), type \"help\" for help, or \"exit\" to exit: ",
        );

        match command.to_lowercase().as_str() {
            "help" => {
                print_help_banner();
            }
            "exit" => {
                // calculate the percentage of games won.
                // Ensure that if the player won no games, they won 0% of games.
                let percent_won = calculate_percentage_won(&past_games);
                println!(
                    "Thank you for playing! You won {:.2}% of the games (ignoring ties).",
                    percent_won
                );
                if percent_won > 60.0 {
                    println!("You are a really lucky person.")
                } else if percent_won < 40.0 {
                    println!("You suck at rock paper scissors.")
                }
                break;
            }
            "rock" | "paper" | "scissors" => {
                // if the computer winrate is lower than 50%, the player has probably figured out the computer's strategy, so go back to the random move strategy.
                // Although, if only a few games have been played, its probably just sheer luck that the player is doing so well.
                // (also handles the first move where there is no prior move).
                let computer_move = if (calculate_percentage_won(&past_games) > 55.0
                    && past_games.len() > 5)
                    || past_games.len() == 0
                {
                    random_move()
                } else {
                    find_best_move(&past_games[past_games.len() - 1])
                };
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
                        println!("The computer chose {}.", move_to_str(&computer_move));
                        println!(
                            "{} {} beats {}.",
                            "You win!".green(),
                            capitalize_first_letter(move_to_str(&player_move)),
                            move_to_str(&computer_move)
                        );
                    }
                    GameResult::Loss => {
                        println!("The computer chose {}.", move_to_str(&computer_move));
                        println!(
                            "{} {} beats {}.",
                            "You lose!".red(),
                            capitalize_first_letter(move_to_str(&computer_move)),
                            move_to_str(&player_move)
                        );
                    }
                    GameResult::Tie => {
                        println!("The computer chose {}.", move_to_str(&computer_move));
                        println!(
                            "{} You both chose {}.",
                            "You tied!".yellow(),
                            move_to_str(&player_move)
                        );
                    }
                }
            }

            _ => {
                println!("Invalid command/move. Please try again.");
            }
        }

        if command.to_lowercase() == "exit" {
            break;
        }
    }
}

fn print_help_banner() {
    print_welcome();
    println!(
        "In Rock, Paper, Scissors, you can choose one of three moves: rock, paper, or scissors."
    );
    println!("Rock beats scissors, scissors beats paper, and paper beats rock.");
    // encourage "randomness"
    println!(
        "You will play against the computer. The computer will try to predict your move, so try to play unpredictably."
    );
    println!(
        "Although the computer can easily cheat (it knows your move before it plays its own, after all), I promise it doesn't cheat. Just trust me (and look at the program code)."
    );
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

/// Convert a string move to a Move enum to improve performance (maybe since it's number comparison???)
fn str_to_move(str: &String) -> Move {
    match str.to_lowercase().trim() {
        "rock" => Move::Rock,
        "paper" => Move::Paper,
        _ => Move::Scissors,
    }
}

/// Finds the best possible computer move based on the user's last move.
/// If the player wins the past round, they are more likely to play the same move, so play the move which beats that.
/// If the player loses the last round, they are more likely to play the move which beat their last move, so play the move which beats that.
/// If it's a tie, just play a random move.
fn find_best_move(previous_round: &GameOutcome) -> Move {
    if previous_round.result == GameResult::Tie {
        return random_move();
    }

    if previous_round.result == GameResult::Win {
        return match previous_round.player_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        };
    }

    // game result is lose
    match previous_round.player_move {
        Move::Rock => Move::Scissors,
        Move::Paper => Move::Rock,
        Move::Scissors => Move::Paper,
    }
}

/// Converts a Move enum member to a string so it can be printed nicely
fn move_to_str(mv: &Move) -> String {
    match mv {
        Move::Rock => "rock".to_string(),
        Move::Paper => "paper".to_string(),
        Move::Scissors => "scissors".to_string(),
    }
}

/// Returns the string with the first letter capitalized
fn capitalize_first_letter(str: String) -> String {
    let mut chars = str.chars();

    match chars.next() {
        None => String::new(),
        Some(next) => next.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// returns the percent of games which the player has won (0-100)
fn calculate_percentage_won(past_games: &Vec<GameOutcome>) -> f32 {
    let mut past_game_win_count = 0;
    let mut not_tie_games = 0;

    for game in past_games.iter() {
        if game.result == GameResult::Win {
            past_game_win_count += 1;
        }

        if game.result != GameResult::Tie {
            not_tie_games += 1;
        }
    }

    // prevent dividing by zero
    if not_tie_games == 0 {
        return 0.0;
    }

    past_game_win_count as f32 / not_tie_games as f32 * 100.0
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
