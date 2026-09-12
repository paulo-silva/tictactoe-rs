//! The 3x3 board and its rules.

use std::fmt;

/// One of the two players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    X,
    O,
}

impl Player {
    /// The opponent of this player.
    pub fn other(self) -> Player {
        todo!()
    }
}

impl fmt::Display for Player {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/// A board cell: empty or occupied by a player.
pub type Cell = Option<Player>;

/// A position on the board, `row` and `col` in `0..3`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    row: u8,
    col: u8,
}

impl Pos {
    /// Builds a position, rejecting anything outside the 3x3 grid.
    pub fn new(_row: u8, _col: u8) -> Result<Pos, BoardError> {
        todo!()
    }

    /// Builds a position from a linear index `0..9` (row-major).
    pub fn from_index(_index: usize) -> Result<Pos, BoardError> {
        todo!()
    }

    pub fn row(self) -> u8 {
        todo!()
    }

    pub fn col(self) -> u8 {
        todo!()
    }

    /// Row-major linear index in `0..9`.
    pub fn index(self) -> usize {
        todo!()
    }
}

/// Errors produced by board operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardError {
    /// Row, column or index outside the 3x3 grid.
    OutOfBounds,
    /// The target cell is already occupied.
    Occupied(Pos),
}

impl fmt::Display for BoardError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for BoardError {}

/// The 3x3 grid.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Board {
    cells: [Cell; 9],
}

impl Board {
    /// An empty board.
    pub fn new() -> Board {
        todo!()
    }

    /// The cell at `pos`.
    pub fn get(&self, _pos: Pos) -> Cell {
        todo!()
    }

    /// Places `player` at `pos`. Fails if the cell is occupied.
    pub fn place(&mut self, _pos: Pos, _player: Player) -> Result<(), BoardError> {
        todo!()
    }

    /// Clears the cell at `pos`. Returns the previous occupant.
    pub fn clear(&mut self, _pos: Pos) -> Cell {
        todo!()
    }

    /// True when no cell is empty.
    pub fn is_full(&self) -> bool {
        todo!()
    }

    /// Number of occupied cells.
    pub fn count(&self) -> usize {
        todo!()
    }

    /// All empty positions in row-major order.
    pub fn empty_positions(&self) -> Vec<Pos> {
        todo!()
    }

    /// The player with three in a row, if any.
    pub fn winner(&self) -> Option<Player> {
        todo!()
    }

    /// The first completed line found, in the fixed scan order:
    /// rows top to bottom, columns left to right, main diagonal, anti-diagonal.
    pub fn winning_line(&self) -> Option<[Pos; 3]> {
        todo!()
    }

    /// The eight lines (rows, columns, diagonals) in the scan order above.
    pub fn lines() -> [[Pos; 3]; 8] {
        todo!()
    }
}
