//! Native command line front end.
//!
//! ```text
//! tictactoe                     interactive game, human (X) vs perfect AI (O)
//! tictactoe --human             interactive two-player game
//! tictactoe --ai X              the AI plays X, the human plays O
//! tictactoe --easy              the AI takes the first free cell instead of playing perfectly
//! tictactoe --moves a1,b2,c3    replay a move list, print the board and the result, exit
//! tictactoe --best a1,b2        replay a move list and print the AI's recommended next move
//! tictactoe --help
//! ```
//!
//! Exit codes: 0 on success, 2 on invalid arguments or notation.

use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use tictactoe::{
    best_move, format_pos, parse_moves, parse_pos, render_grid, Difficulty, Game, Player, Status,
};

const USAGE: &str = "\
Usage: tictactoe [OPTIONS]

Play tic-tac-toe in the terminal.

Options:
  --human             two-player game, no computer
  --ai <X|O>          which side the computer plays (default: O)
  --easy              computer takes the first free cell instead of playing perfectly
  --moves <LIST>      replay a comma separated move list (e.g. a1,b2,c3),
                      print the board and the result, then exit
  --best <LIST>       replay a move list and print the computer's recommended next move
  --help              show this help

Positions are a1..c3 (row letter, column digit) or 1..9 row-major.
During an interactive game, type `undo` to take back the last move or `quit` to exit.
";

enum Mode {
    Interactive { ai: Option<Player> },
    Replay(String),
    Best(String),
}

struct Options {
    mode: Mode,
    difficulty: Difficulty,
}

/// Takes the value following `flag`, refusing another flag in its place.
fn value<'a>(
    iter: &mut impl Iterator<Item = &'a String>,
    flag: &str,
    what: &str,
) -> Result<&'a String, String> {
    match iter.next() {
        Some(v) if !v.starts_with("--") => Ok(v),
        _ => Err(format!("{flag} requires {what}")),
    }
}

fn parse_args(args: &[String]) -> Result<Option<Options>, String> {
    let mut mode = Mode::Interactive {
        ai: Some(Player::O),
    };
    let mut difficulty = Difficulty::Perfect;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" | "-h" => return Ok(None),
            "--human" => mode = Mode::Interactive { ai: None },
            "--easy" => difficulty = Difficulty::Easy,
            "--ai" => {
                let side = value(&mut iter, "--ai", "X or O")?;
                let player = match side.to_ascii_uppercase().as_str() {
                    "X" => Player::X,
                    "O" => Player::O,
                    _ => return Err(format!("--ai expects X or O, got {side:?}")),
                };
                mode = Mode::Interactive { ai: Some(player) };
            }
            "--moves" => {
                let list = value(&mut iter, "--moves", "a move list")?;
                mode = Mode::Replay(list.clone());
            }
            "--best" => {
                let list = value(&mut iter, "--best", "a move list")?;
                mode = Mode::Best(list.clone());
            }
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    Ok(Some(Options { mode, difficulty }))
}

fn status_line(game: &Game) -> String {
    match game.status() {
        Status::Won(p) => format!("{p} wins"),
        Status::Draw => "Draw".to_string(),
        Status::InProgress => format!("{} to move", game.current_player()),
    }
}

fn replay(list: &str) -> Result<(), String> {
    let game = parse_moves(list).map_err(|e| e.to_string())?;
    print!("{}", render_grid(game.board()));
    println!("{}", status_line(&game));
    Ok(())
}

fn best(list: &str, difficulty: Difficulty) -> Result<(), String> {
    let game = parse_moves(list).map_err(|e| e.to_string())?;
    match best_move(&game, difficulty) {
        Some(pos) => {
            println!("{}", format_pos(pos));
            Ok(())
        }
        None => Err(format!("the game is over: {}", status_line(&game))),
    }
}

fn interactive(ai: Option<Player>, difficulty: Difficulty) -> Result<(), String> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut out = io::stdout();
    let mut game = Game::new();

    print!("{}", render_grid(game.board()));
    loop {
        if game.is_over() {
            println!("{}", status_line(&game));
            return Ok(());
        }
        let mover = game.current_player();
        if ai == Some(mover) {
            let pos = best_move(&game, difficulty).expect("game in progress");
            game.play(pos).expect("ai chooses empty cells");
            println!("{mover} plays {}", format_pos(pos));
            print!("{}", render_grid(game.board()));
            continue;
        }

        print!("{mover}> ");
        out.flush().map_err(|e| e.to_string())?;
        let line = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(e)) => return Err(e.to_string()),
            None => {
                println!();
                return Ok(());
            }
        };
        let input = line.trim();
        match input {
            "" => continue,
            "quit" | "exit" => return Ok(()),
            "undo" => {
                match game.undo() {
                    Ok(pos) => {
                        println!("undo {}", format_pos(pos));
                        // Against the computer, also take back the computer's reply
                        // so the human gets to replay their own move.
                        if ai.is_some() && ai != Some(game.current_player()) {
                            if let Ok(pos) = game.undo() {
                                println!("undo {}", format_pos(pos));
                            }
                        }
                    }
                    Err(e) => println!("{e}"),
                }
                print!("{}", render_grid(game.board()));
            }
            _ => match parse_pos(input) {
                Ok(pos) => match game.play(pos) {
                    Ok(_) => print!("{}", render_grid(game.board())),
                    Err(e) => println!("{e}"),
                },
                Err(e) => println!("{e}"),
            },
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = match parse_args(&args) {
        Ok(Some(options)) => options,
        Ok(None) => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Err(msg) => {
            eprintln!("error: {msg}\n\n{USAGE}");
            return ExitCode::from(2);
        }
    };

    let result = match options.mode {
        Mode::Replay(list) => replay(&list),
        Mode::Best(list) => best(&list, options.difficulty),
        Mode::Interactive { ai } => interactive(ai, options.difficulty),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("error: {msg}");
            ExitCode::from(2)
        }
    }
}
