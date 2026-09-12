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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::GameOver => f.write_str("the game is already over"),
            GameError::Board(e) => write!(f, "{e}"),
            GameError::NothingToUndo => f.write_str("there is no move to undo"),
        }
    }
}

impl std::error::Error for GameError {}

impl From<BoardError> for GameError {
    fn from(e: BoardError) -> Self {
        GameError::Board(e)
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
        Game::with_first_player(Player::X)
    }

    /// A new game where `first` moves first.
    pub fn with_first_player(first: Player) -> Game {
        Game {
            board: Board::new(),
            first,
            history: Vec::new(),
        }
    }

    /// Replays `moves` from an empty board, X first. Fails on the first illegal move.
    pub fn from_moves(moves: &[Pos]) -> Result<Game, GameError> {
        let mut game = Game::new();
        for &pos in moves {
            game.play(pos)?;
        }
        Ok(game)
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    /// The player who moves next. Meaningful only while the game is in progress.
    pub fn current_player(&self) -> Player {
        if self.history.len().is_multiple_of(2) {
            self.first
        } else {
            self.first.other()
        }
    }

    pub fn status(&self) -> Status {
        match self.board.winner() {
            Some(player) => Status::Won(player),
            None if self.board.is_full() => Status::Draw,
            None => Status::InProgress,
        }
    }

    pub fn is_over(&self) -> bool {
        self.status() != Status::InProgress
    }

    /// Moves played so far, oldest first.
    pub fn history(&self) -> &[Pos] {
        &self.history
    }

    /// Plays the current player's mark at `pos` and returns the new status.
    pub fn play(&mut self, pos: Pos) -> Result<Status, GameError> {
        if self.is_over() {
            return Err(GameError::GameOver);
        }
        self.board.place(pos, self.current_player())?;
        self.history.push(pos);
        Ok(self.status())
    }

    /// Reverts the most recent move and returns its position.
    pub fn undo(&mut self) -> Result<Pos, GameError> {
        let pos = self.history.pop().ok_or(GameError::NothingToUndo)?;
        self.board.clear(pos);
        Ok(pos)
    }
}
