use tictactoe::notation::{format_pos, parse_moves, render_grid};
use tictactoe::{format_board, parse_board, parse_pos, Board, NotationError, Player, Pos, Status};

fn p(row: u8, col: u8) -> Pos {
    Pos::new(row, col).unwrap()
}

#[test]
fn parse_and_format_board_round_trip() {
    let text = "X.O/.X./..O";
    let board = parse_board(text).unwrap();
    assert_eq!(board.get(p(0, 0)), Some(Player::X));
    assert_eq!(board.get(p(0, 2)), Some(Player::O));
    assert_eq!(board.get(p(1, 1)), Some(Player::X));
    assert_eq!(board.get(p(2, 2)), Some(Player::O));
    assert_eq!(board.count(), 4);
    assert_eq!(format_board(&board), text);
}

#[test]
fn parse_board_is_case_insensitive_and_trims_whitespace() {
    let board = parse_board("  x.o/.X./..o\n").unwrap();
    assert_eq!(format_board(&board), "X.O/.X./..O");
}

#[test]
fn empty_board_formats_as_dots() {
    assert_eq!(format_board(&Board::new()), ".../.../...");
    assert_eq!(parse_board(".../.../...").unwrap(), Board::new());
}

#[test]
fn parse_board_rejects_bad_shapes() {
    assert!(matches!(parse_board("X.O/.X."), Err(NotationError::BadShape(_))));
    assert!(matches!(parse_board("X.O/.X./..O/..."), Err(NotationError::BadShape(_))));
    assert!(matches!(parse_board("X.O/.X../..O"), Err(NotationError::BadShape(_))));
    assert!(matches!(parse_board(""), Err(NotationError::BadShape(_))));
}

#[test]
fn parse_board_rejects_bad_chars() {
    assert_eq!(parse_board("X.O/.Z./..O"), Err(NotationError::BadChar('Z')));
    assert_eq!(parse_board("X O/.X./..O"), Err(NotationError::BadChar(' ')));
}

#[test]
fn parse_board_rejects_unreachable_counts() {
    // O has more marks than X.
    assert!(matches!(parse_board("O.O/.X./..."), Err(NotationError::Unreachable(_))));
    // X is two ahead of O.
    assert!(matches!(parse_board("X.X/.X./..O"), Err(NotationError::Unreachable(_))));
    // X one ahead is fine.
    assert!(parse_board("X.X/.O./...").is_ok());
    // Both with three in a row cannot happen.
    assert!(matches!(parse_board("XXX/OOO/..."), Err(NotationError::Unreachable(_))));
}

#[test]
fn parse_pos_accepts_letter_digit_and_single_digit() {
    assert_eq!(parse_pos("a1"), Ok(p(0, 0)));
    assert_eq!(parse_pos("B2"), Ok(p(1, 1)));
    assert_eq!(parse_pos("c3"), Ok(p(2, 2)));
    assert_eq!(parse_pos(" b3 "), Ok(p(1, 2)));
    assert_eq!(parse_pos("1"), Ok(p(0, 0)));
    assert_eq!(parse_pos("5"), Ok(p(1, 1)));
    assert_eq!(parse_pos("9"), Ok(p(2, 2)));
}

#[test]
fn parse_pos_rejects_garbage() {
    for bad in ["", "d1", "a4", "a0", "0", "10", "aa", "1a", "b", "b22"] {
        assert!(matches!(parse_pos(bad), Err(NotationError::BadPos(_))), "{bad:?}");
    }
}

#[test]
fn format_pos_uses_letter_digit() {
    assert_eq!(format_pos(p(0, 0)), "a1");
    assert_eq!(format_pos(p(1, 2)), "b3");
    assert_eq!(format_pos(p(2, 1)), "c2");
    for i in 0..9 {
        let pos = Pos::from_index(i).unwrap();
        assert_eq!(parse_pos(&format_pos(pos)), Ok(pos));
    }
}

#[test]
fn parse_moves_builds_a_game() {
    let game = parse_moves("a1, b2 ,c3").unwrap();
    assert_eq!(game.history(), &[p(0, 0), p(1, 1), p(2, 2)]);
    assert_eq!(game.current_player(), Player::O);
    assert_eq!(game.status(), Status::InProgress);
    assert_eq!(format_board(game.board()), "X../.O./..X");
}

#[test]
fn parse_moves_empty_is_new_game() {
    let game = parse_moves("").unwrap();
    assert!(game.history().is_empty());
    let game = parse_moves("   ").unwrap();
    assert!(game.history().is_empty());
}

#[test]
fn parse_moves_reports_bad_positions_and_illegal_play() {
    assert!(matches!(parse_moves("a1,zz"), Err(NotationError::BadPos(_))));
    assert!(matches!(parse_moves("a1,a1"), Err(NotationError::Unreachable(_))));
    assert!(matches!(parse_moves("a1,b1,a2,b2,a3,c3"), Err(NotationError::Unreachable(_))));
}

#[test]
fn render_grid_matches_documented_layout() {
    let board = parse_board("X.O/.X./..O").unwrap();
    let expected = "\
   1   2   3
a  X | . | O
  ---+---+---
b  . | X | .
  ---+---+---
c  . | . | O
";
    assert_eq!(render_grid(&board), expected);
}

#[test]
fn notation_error_display_is_readable() {
    assert_eq!(
        NotationError::BadShape("expected 3 rows".into()).to_string(),
        "invalid board: expected 3 rows"
    );
    assert_eq!(NotationError::BadChar('Z').to_string(), "invalid character 'Z'");
    assert_eq!(NotationError::BadPos("d1".into()).to_string(), "invalid position \"d1\"");
    assert_eq!(
        NotationError::Unreachable("O has more marks than X".into()).to_string(),
        "unreachable board: O has more marks than X"
    );
}
