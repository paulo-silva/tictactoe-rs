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
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => f.write_str("X"),
            Player::O => f.write_str("O"),
        }
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
    pub fn new(row: u8, col: u8) -> Result<Pos, BoardError> {
        if row < 3 && col < 3 {
            Ok(Pos { row, col })
        } else {
            Err(BoardError::OutOfBounds)
        }
    }

    /// Builds a position from a linear index `0..9` (row-major).
    pub fn from_index(index: usize) -> Result<Pos, BoardError> {
        if index < 9 {
            Ok(Pos {
                row: (index / 3) as u8,
                col: (index % 3) as u8,
            })
        } else {
            Err(BoardError::OutOfBounds)
        }
    }

    pub fn row(self) -> u8 {
        self.row
    }

    pub fn col(self) -> u8 {
        self.col
    }

    /// Row-major linear index in `0..9`.
    pub fn index(self) -> usize {
        self.row as usize * 3 + self.col as usize
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::OutOfBounds => f.write_str("position is outside the 3x3 board"),
            BoardError::Occupied(pos) => write!(
                f,
                "cell {} is already occupied",
                crate::notation::format_pos(*pos)
            ),
        }
    }
}

impl std::error::Error for BoardError {}

/// The 3x3 grid.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Board {
    cells: [Cell; 9],
}

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

impl Board {
    /// An empty board.
    pub fn new() -> Board {
        Board { cells: [None; 9] }
    }

    /// The cell at `pos`.
    pub fn get(&self, pos: Pos) -> Cell {
        self.cells[pos.index()]
    }

    /// Places `player` at `pos`. Fails if the cell is occupied.
    pub fn place(&mut self, pos: Pos, player: Player) -> Result<(), BoardError> {
        let cell = &mut self.cells[pos.index()];
        if cell.is_some() {
            return Err(BoardError::Occupied(pos));
        }
        *cell = Some(player);
        Ok(())
    }

    /// Clears the cell at `pos`. Returns the previous occupant.
    pub fn clear(&mut self, pos: Pos) -> Cell {
        self.cells[pos.index()].take()
    }

    /// True when no cell is empty.
    pub fn is_full(&self) -> bool {
        self.cells.iter().all(Option::is_some)
    }

    /// Number of occupied cells.
    pub fn count(&self) -> usize {
        self.cells.iter().filter(|c| c.is_some()).count()
    }

    /// All empty positions in row-major order.
    pub fn empty_positions(&self) -> Vec<Pos> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_none())
            .map(|(i, _)| Pos::from_index(i).expect("index within board"))
            .collect()
    }

    /// The player with three in a row, if any.
    pub fn winner(&self) -> Option<Player> {
        self.winning_line().and_then(|line| self.get(line[0]))
    }

    /// The first completed line found, in the fixed scan order:
    /// rows top to bottom, columns left to right, main diagonal, anti-diagonal.
    pub fn winning_line(&self) -> Option<[Pos; 3]> {
        Board::lines().into_iter().find(|line| {
            let first = self.get(line[0]);
            first.is_some() && self.get(line[1]) == first && self.get(line[2]) == first
        })
    }

    /// The eight lines (rows, columns, diagonals) in the scan order above.
    pub fn lines() -> [[Pos; 3]; 8] {
        LINES.map(|line| line.map(|i| Pos::from_index(i).expect("index within board")))
    }
}
