//! A deterministic minimax opponent.

use crate::board::Pos;
use crate::game::Game;

/// How hard the computer plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// Takes the first empty cell in row-major order.
    Easy,
    /// Perfect play via full-depth minimax.
    Perfect,
}

/// Score of `game` from the point of view of `game.current_player()`.
///
/// `+10 - depth` for a forced win, `-10 + depth` for a forced loss, `0` for a draw,
/// where `depth` is the number of plies until the result. Returns `0` on a finished draw.
pub fn evaluate(_game: &Game) -> i32 {
    todo!()
}

/// The move the computer would play for the current player, or `None` if the game is over.
///
/// Ties are broken by the lowest row-major index so results are reproducible.
pub fn best_move(_game: &Game, _difficulty: Difficulty) -> Option<Pos> {
    todo!()
}
