//! Game state: turn order, status, history and undo.

use std::fmt;

use crate::board::{Board, BoardError, Player, Pos};

/// Whether the game is still running, won, or drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    InProgress,
    Won(Player),
    Draw,
}

/// Errors produced by game operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameError {
    /// A move was attempted after the game ended.
    GameOver,
    /// The board rejected the move.
    Board(BoardError),
    /// Undo was called with an empty history.
    NothingToUndo,
}

impl fmt::Display for GameError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for GameError {}

impl From<BoardError> for GameError {
    fn from(_e: BoardError) -> Self {
        todo!()
    }
}

/// A full game: board plus whose turn it is and what has been played.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    board: Board,
    first: Player,
    history: Vec<Pos>,
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

impl Game {
    /// A new game where X moves first.
    pub fn new() -> Game {
        todo!()
    }

    /// A new game where `first` moves first.
    pub fn with_first_player(_first: Player) -> Game {
        todo!()
    }

    /// Replays `moves` from an empty board, X first. Fails on the first illegal move.
    pub fn from_moves(_moves: &[Pos]) -> Result<Game, GameError> {
        todo!()
    }

    pub fn board(&self) -> &Board {
        todo!()
    }

    /// The player who moves next. Meaningful only while the game is in progress.
    pub fn current_player(&self) -> Player {
        todo!()
    }

    pub fn status(&self) -> Status {
        todo!()
    }

    pub fn is_over(&self) -> bool {
        todo!()
    }

    /// Moves played so far, oldest first.
    pub fn history(&self) -> &[Pos] {
        todo!()
    }

    /// Plays the current player's mark at `pos` and returns the new status.
    pub fn play(&mut self, _pos: Pos) -> Result<Status, GameError> {
        todo!()
    }

    /// Reverts the most recent move and returns its position.
    pub fn undo(&mut self) -> Result<Pos, GameError> {
        todo!()
    }
}
