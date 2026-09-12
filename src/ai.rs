//! A deterministic minimax opponent.

use crate::board::Pos;
use crate::game::{Game, Status};

/// How hard the computer plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// Takes the first empty cell in row-major order.
    Easy,
    /// Perfect play via full-depth minimax.
    Perfect,
}

const WIN: i32 = 10;

/// Score of `game` from the point of view of `game.current_player()`.
///
/// `+10 - depth` for a forced win, `-10 + depth` for a forced loss, `0` for a draw,
/// where `depth` is the number of plies until the result. Returns `0` on a finished draw.
pub fn evaluate(game: &Game) -> i32 {
    let mut scratch = game.clone();
    negamax(&mut scratch, 0)
}

/// The move the computer would play for the current player, or `None` if the game is over.
///
/// Ties are broken by the lowest row-major index so results are reproducible.
pub fn best_move(game: &Game, difficulty: Difficulty) -> Option<Pos> {
    if game.is_over() {
        return None;
    }
    match difficulty {
        Difficulty::Easy => game.board().empty_positions().into_iter().next(),
        Difficulty::Perfect => {
            let mut scratch = game.clone();
            let mut best: Option<(i32, Pos)> = None;
            for pos in game.board().empty_positions() {
                scratch.play(pos).expect("empty cell on a live game");
                let score = -negamax(&mut scratch, 1);
                scratch.undo().expect("move was just played");
                if best.is_none_or(|(s, _)| score > s) {
                    best = Some((score, pos));
                }
            }
            best.map(|(_, pos)| pos)
        }
    }
}

/// Negamax over the full tree. `depth` is the number of plies from the root.
///
/// The score is relative to the player to move at this node. A terminal
/// `Won(p)` node is always a loss for the player to move, since `p` just moved.
fn negamax(game: &mut Game, depth: i32) -> i32 {
    match game.status() {
        Status::Won(_) => return -(WIN - depth),
        Status::Draw => return 0,
        Status::InProgress => {}
    }
    let mut best = i32::MIN;
    for pos in game.board().empty_positions() {
        game.play(pos).expect("empty cell on a live game");
        let score = -negamax(game, depth + 1);
        game.undo().expect("move was just played");
        if score > best {
            best = score;
        }
    }
    best
}
