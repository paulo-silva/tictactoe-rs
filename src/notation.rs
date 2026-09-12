//! Text representations of boards, positions and games.
//!
//! Board notation: three rows separated by `/`, one char per cell:
//! `X`, `O` or `.` for empty. Example: `"X.O/.X./..O"`.
//!
//! Position notation: a letter `a`-`c` for the row and a digit `1`-`3` for the
//! column (`"b2"` is the centre), or a single digit `1`-`9` in row-major order.

use std::fmt;

use crate::board::{Board, Pos};
use crate::game::Game;

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
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for NotationError {}

/// Parses `"X.O/.X./..O"` style text into a board.
///
/// Whitespace around the text is ignored. Case is insensitive.
pub fn parse_board(_text: &str) -> Result<Board, NotationError> {
    todo!()
}

/// Formats a board as `"X.O/.X./..O"`.
pub fn format_board(_board: &Board) -> String {
    todo!()
}

/// Parses `"b2"` or `"5"` into a position.
pub fn parse_pos(_text: &str) -> Result<Pos, NotationError> {
    todo!()
}

/// Formats a position as `"b2"`.
pub fn format_pos(_pos: Pos) -> String {
    todo!()
}

/// Parses a comma separated move list such as `"a1,b2,c3"` into a game.
pub fn parse_moves(_text: &str) -> Result<Game, NotationError> {
    todo!()
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
pub fn render_grid(_board: &Board) -> String {
    todo!()
}
