//! Text representations of boards, positions and games.
//!
//! Board notation: three rows separated by `/`, one char per cell:
//! `X`, `O` or `.` for empty. Example: `"X.O/.X./..O"`.
//!
//! Position notation: a letter `a`-`c` for the row and a digit `1`-`3` for the
//! column (`"b2"` is the centre), or a single digit `1`-`9` in row-major order.

use std::fmt;

use crate::board::{Board, Player, Pos};
use crate::game::{Game, GameError};

/// Errors produced while parsing notation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotationError {
    /// The text has the wrong shape (row count, row length, separators).
    BadShape(String),
    /// An unexpected character.
    BadChar(char),
    /// A position outside the grid or with an unknown format.
    BadPos(String),
    /// The board is not reachable by legal play (e.g. O has more marks than X).
    Unreachable(String),
}

impl fmt::Display for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotationError::BadShape(msg) => write!(f, "invalid board: {msg}"),
            NotationError::BadChar(c) => write!(f, "invalid character {c:?}"),
            NotationError::BadPos(text) => write!(f, "invalid position {text:?}"),
            NotationError::Unreachable(msg) => write!(f, "unreachable board: {msg}"),
        }
    }
}

impl std::error::Error for NotationError {}

/// Parses `"X.O/.X./..O"` style text into a board.
///
/// Whitespace around the text is ignored. Case is insensitive.
pub fn parse_board(text: &str) -> Result<Board, NotationError> {
    let text = text.trim();
    let rows: Vec<&str> = text.split('/').collect();
    if rows.len() != 3 {
        return Err(NotationError::BadShape(format!(
            "expected 3 rows separated by '/', found {}",
            rows.len()
        )));
    }
    let mut board = Board::new();
    for (r, row) in rows.iter().enumerate() {
        let chars: Vec<char> = row.chars().collect();
        if chars.len() != 3 {
            return Err(NotationError::BadShape(format!(
                "row {} has {} cells, expected 3",
                r + 1,
                chars.len()
            )));
        }
        for (c, ch) in chars.iter().enumerate() {
            let pos = Pos::new(r as u8, c as u8).expect("row and col below 3");
            match ch.to_ascii_uppercase() {
                'X' => board.place(pos, Player::X).expect("fresh cell"),
                'O' => board.place(pos, Player::O).expect("fresh cell"),
                '.' => {}
                other => return Err(NotationError::BadChar(other)),
            }
        }
    }
    check_reachable(&board)?;
    Ok(board)
}

fn check_reachable(board: &Board) -> Result<(), NotationError> {
    let xs = (0..9)
        .filter(|&i| board.get(Pos::from_index(i).unwrap()) == Some(Player::X))
        .count();
    let os = (0..9)
        .filter(|&i| board.get(Pos::from_index(i).unwrap()) == Some(Player::O))
        .count();
    if os > xs {
        return Err(NotationError::Unreachable("O has more marks than X".into()));
    }
    if xs > os + 1 {
        return Err(NotationError::Unreachable(
            "X has more than one mark more than O".into(),
        ));
    }
    let x_wins = Board::lines()
        .iter()
        .any(|l| l.iter().all(|&p| board.get(p) == Some(Player::X)));
    let o_wins = Board::lines()
        .iter()
        .any(|l| l.iter().all(|&p| board.get(p) == Some(Player::O)));
    if x_wins && o_wins {
        return Err(NotationError::Unreachable(
            "both players have three in a row".into(),
        ));
    }
    Ok(())
}

/// Formats a board as `"X.O/.X./..O"`.
pub fn format_board(board: &Board) -> String {
    let mut out = String::with_capacity(11);
    for r in 0..3u8 {
        if r > 0 {
            out.push('/');
        }
        for c in 0..3u8 {
            out.push(cell_char(board, Pos::new(r, c).unwrap()));
        }
    }
    out
}

fn cell_char(board: &Board, pos: Pos) -> char {
    match board.get(pos) {
        Some(Player::X) => 'X',
        Some(Player::O) => 'O',
        None => '.',
    }
}

/// Parses `"b2"` or `"5"` into a position.
pub fn parse_pos(text: &str) -> Result<Pos, NotationError> {
    let trimmed = text.trim();
    let bad = || NotationError::BadPos(text.to_string());
    let chars: Vec<char> = trimmed.chars().collect();
    match chars.as_slice() {
        [d] => {
            let n = d.to_digit(10).ok_or_else(bad)?;
            if (1..=9).contains(&n) {
                Pos::from_index(n as usize - 1).map_err(|_| bad())
            } else {
                Err(bad())
            }
        }
        [letter, digit] => {
            let row = match letter.to_ascii_lowercase() {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                _ => return Err(bad()),
            };
            let col = match digit {
                '1' => 0,
                '2' => 1,
                '3' => 2,
                _ => return Err(bad()),
            };
            Pos::new(row, col).map_err(|_| bad())
        }
        _ => Err(bad()),
    }
}

/// Formats a position as `"b2"`.
pub fn format_pos(pos: Pos) -> String {
    let row = (b'a' + pos.row()) as char;
    let col = (b'1' + pos.col()) as char;
    format!("{row}{col}")
}

/// Parses a comma separated move list such as `"a1,b2,c3"` into a game.
pub fn parse_moves(text: &str) -> Result<Game, NotationError> {
    let mut game = Game::new();
    if text.trim().is_empty() {
        return Ok(game);
    }
    for item in text.split(',') {
        let pos = parse_pos(item)?;
        game.play(pos).map_err(|e| match e {
            GameError::GameOver => NotationError::Unreachable(format!(
                "move {} comes after the game ended",
                format_pos(pos)
            )),
            other => NotationError::Unreachable(other.to_string()),
        })?;
    }
    Ok(game)
}

/// Renders a board as a human-friendly grid with coordinates:
///
/// ```text
///    1   2   3
/// a  X | . | O
///   ---+---+---
/// b  . | X | .
///   ---+---+---
/// c  . | . | O
/// ```
pub fn render_grid(board: &Board) -> String {
    let mut out = String::from("   1   2   3\n");
    for r in 0..3u8 {
        if r > 0 {
            out.push_str("  ---+---+---\n");
        }
        let label = (b'a' + r) as char;
        let cells: Vec<String> = (0..3u8)
            .map(|c| cell_char(board, Pos::new(r, c).unwrap()).to_string())
            .collect();
        out.push_str(&format!("{label}  {}\n", cells.join(" | ")));
    }
    out
}
