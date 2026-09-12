//! A tic-tac-toe engine built test-first.
//!
//! The crate is split into small, composable modules:
//!
//! - [`board`]: the 3x3 grid, cell placement and win detection.
//! - [`game`]: turn order, game status, move history and undo.
//! - [`ai`]: a deterministic minimax opponent.
//! - [`notation`]: parsing and formatting of boards and positions.

pub mod ai;
pub mod board;
pub mod game;
pub mod notation;

pub use ai::{best_move, Difficulty};
pub use board::{Board, BoardError, Cell, Player, Pos};
pub use game::{Game, GameError, Status};
pub use notation::{format_board, format_pos, parse_board, parse_moves, parse_pos, render_grid, NotationError};
