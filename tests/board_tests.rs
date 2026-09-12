use tictactoe::{Board, BoardError, Player, Pos};

fn p(row: u8, col: u8) -> Pos {
    Pos::new(row, col).unwrap()
}

#[test]
fn players_are_opposites() {
    assert_eq!(Player::X.other(), Player::O);
    assert_eq!(Player::O.other(), Player::X);
    assert_eq!(Player::X.to_string(), "X");
    assert_eq!(Player::O.to_string(), "O");
}

#[test]
fn positions_validate_bounds() {
    assert!(Pos::new(0, 0).is_ok());
    assert!(Pos::new(2, 2).is_ok());
    assert_eq!(Pos::new(3, 0), Err(BoardError::OutOfBounds));
    assert_eq!(Pos::new(0, 3), Err(BoardError::OutOfBounds));
    assert_eq!(Pos::from_index(9), Err(BoardError::OutOfBounds));
}

#[test]
fn positions_round_trip_through_index() {
    for i in 0..9 {
        let pos = Pos::from_index(i).unwrap();
        assert_eq!(pos.index(), i);
        assert_eq!(pos.row() as usize, i / 3);
        assert_eq!(pos.col() as usize, i % 3);
        assert_eq!(Pos::new(pos.row(), pos.col()).unwrap(), pos);
    }
}

#[test]
fn new_board_is_empty() {
    let board = Board::new();
    assert!(!board.is_full());
    assert_eq!(board.count(), 0);
    assert_eq!(board.winner(), None);
    assert_eq!(board.winning_line(), None);
    assert_eq!(board.empty_positions().len(), 9);
    for i in 0..9 {
        assert_eq!(board.get(Pos::from_index(i).unwrap()), None);
    }
}

#[test]
fn place_and_get() {
    let mut board = Board::new();
    board.place(p(1, 1), Player::X).unwrap();
    assert_eq!(board.get(p(1, 1)), Some(Player::X));
    assert_eq!(board.count(), 1);
    assert_eq!(board.empty_positions().len(), 8);
    assert!(!board.empty_positions().contains(&p(1, 1)));
}

#[test]
fn placing_on_occupied_cell_fails_and_leaves_board_unchanged() {
    let mut board = Board::new();
    board.place(p(0, 0), Player::X).unwrap();
    let before = board.clone();
    assert_eq!(board.place(p(0, 0), Player::O), Err(BoardError::Occupied(p(0, 0))));
    assert_eq!(board, before);
}

#[test]
fn clear_returns_previous_occupant() {
    let mut board = Board::new();
    board.place(p(2, 2), Player::O).unwrap();
    assert_eq!(board.clear(p(2, 2)), Some(Player::O));
    assert_eq!(board.get(p(2, 2)), None);
    assert_eq!(board.clear(p(2, 2)), None);
}

#[test]
fn empty_positions_are_row_major() {
    let mut board = Board::new();
    board.place(p(0, 1), Player::X).unwrap();
    board.place(p(2, 0), Player::O).unwrap();
    let expected: Vec<Pos> = [0, 2, 3, 4, 5, 7, 8]
        .iter()
        .map(|&i| Pos::from_index(i).unwrap())
        .collect();
    assert_eq!(board.empty_positions(), expected);
}

fn fill(cells: [Option<Player>; 9]) -> Board {
    let mut board = Board::new();
    for (i, cell) in cells.iter().enumerate() {
        if let Some(player) = cell {
            board.place(Pos::from_index(i).unwrap(), *player).unwrap();
        }
    }
    board
}

const X: Option<Player> = Some(Player::X);
const O: Option<Player> = Some(Player::O);
const E: Option<Player> = None;

#[test]
fn detects_every_row_column_and_diagonal() {
    let lines: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for line in lines {
        let mut cells = [E; 9];
        for i in line {
            cells[i] = X;
        }
        let board = fill(cells);
        assert_eq!(board.winner(), Some(Player::X), "line {line:?}");
        let expected: [Pos; 3] = [
            Pos::from_index(line[0]).unwrap(),
            Pos::from_index(line[1]).unwrap(),
            Pos::from_index(line[2]).unwrap(),
        ];
        assert_eq!(board.winning_line(), Some(expected));
    }
}

#[test]
fn lines_are_in_documented_scan_order() {
    let lines = Board::lines();
    let idx: Vec<[usize; 3]> = lines
        .iter()
        .map(|l| [l[0].index(), l[1].index(), l[2].index()])
        .collect();
    assert_eq!(
        idx,
        vec![
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6]
        ]
    );
}

#[test]
fn mixed_line_is_not_a_win() {
    let board = fill([X, O, X, E, E, E, E, E, E]);
    assert_eq!(board.winner(), None);
}

#[test]
fn full_board_without_winner() {
    let board = fill([X, O, X, X, O, O, O, X, X]);
    assert!(board.is_full());
    assert_eq!(board.count(), 9);
    assert_eq!(board.winner(), None);
    assert!(board.empty_positions().is_empty());
}

#[test]
fn full_board_with_winner_reports_winner() {
    let board = fill([X, O, X, O, X, O, O, X, X]);
    assert!(board.is_full());
    assert_eq!(board.winner(), Some(Player::X));
}

#[test]
fn winning_line_prefers_rows_then_columns_then_diagonals() {
    // X wins on both the top row and the left column; the row is reported.
    let board = fill([X, X, X, X, O, O, X, E, E]);
    let line = board.winning_line().unwrap();
    assert_eq!([line[0].index(), line[1].index(), line[2].index()], [0, 1, 2]);
}

#[test]
fn board_error_display_is_readable() {
    assert_eq!(BoardError::OutOfBounds.to_string(), "position is outside the 3x3 board");
    assert_eq!(BoardError::Occupied(p(1, 2)).to_string(), "cell b3 is already occupied");
}
