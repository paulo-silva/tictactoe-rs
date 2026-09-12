use tictactoe::{BoardError, Game, GameError, Player, Pos, Status};

fn p(row: u8, col: u8) -> Pos {
    Pos::new(row, col).unwrap()
}

#[test]
fn new_game_starts_with_x_in_progress() {
    let game = Game::new();
    assert_eq!(game.current_player(), Player::X);
    assert_eq!(game.status(), Status::InProgress);
    assert!(!game.is_over());
    assert!(game.history().is_empty());
    assert_eq!(game.board().count(), 0);
}

#[test]
fn first_player_can_be_o() {
    let mut game = Game::with_first_player(Player::O);
    assert_eq!(game.current_player(), Player::O);
    game.play(p(0, 0)).unwrap();
    assert_eq!(game.board().get(p(0, 0)), Some(Player::O));
    assert_eq!(game.current_player(), Player::X);
}

#[test]
fn turns_alternate() {
    let mut game = Game::new();
    game.play(p(0, 0)).unwrap();
    assert_eq!(game.current_player(), Player::O);
    game.play(p(1, 1)).unwrap();
    assert_eq!(game.current_player(), Player::X);
    assert_eq!(game.board().get(p(0, 0)), Some(Player::X));
    assert_eq!(game.board().get(p(1, 1)), Some(Player::O));
    assert_eq!(game.history(), &[p(0, 0), p(1, 1)]);
}

#[test]
fn occupied_cell_is_rejected_and_turn_does_not_change() {
    let mut game = Game::new();
    game.play(p(0, 0)).unwrap();
    assert_eq!(
        game.play(p(0, 0)),
        Err(GameError::Board(BoardError::Occupied(p(0, 0))))
    );
    assert_eq!(game.current_player(), Player::O);
    assert_eq!(game.history().len(), 1);
}

#[test]
fn win_is_reported_by_play_and_status() {
    let mut game = Game::new();
    game.play(p(0, 0)).unwrap(); // X
    game.play(p(1, 0)).unwrap(); // O
    game.play(p(0, 1)).unwrap(); // X
    game.play(p(1, 1)).unwrap(); // O
    let status = game.play(p(0, 2)).unwrap(); // X wins top row
    assert_eq!(status, Status::Won(Player::X));
    assert_eq!(game.status(), Status::Won(Player::X));
    assert!(game.is_over());
}

#[test]
fn moves_after_game_over_are_rejected() {
    let mut game = Game::new();
    for pos in [p(0, 0), p(1, 0), p(0, 1), p(1, 1), p(0, 2)] {
        game.play(pos).unwrap();
    }
    assert_eq!(game.play(p(2, 2)), Err(GameError::GameOver));
    assert_eq!(game.history().len(), 5);
}

#[test]
fn draw_when_board_fills_without_winner() {
    // X O X
    // X O O
    // O X X
    let moves = [
        p(0, 0),
        p(0, 1),
        p(0, 2),
        p(1, 1),
        p(1, 0),
        p(1, 2),
        p(2, 1),
        p(2, 0),
        p(2, 2),
    ];
    let mut game = Game::new();
    let mut last = Status::InProgress;
    for pos in moves {
        last = game.play(pos).unwrap();
    }
    assert_eq!(last, Status::Draw);
    assert_eq!(game.status(), Status::Draw);
    assert!(game.is_over());
    assert!(game.board().is_full());
}

#[test]
fn winning_on_the_last_cell_is_a_win_not_a_draw() {
    // X O X
    // O X O
    // O X X   <- X completes the diagonal with the ninth move
    let moves = [
        p(0, 0),
        p(0, 1),
        p(0, 2),
        p(1, 0),
        p(1, 1),
        p(1, 2),
        p(2, 1),
        p(2, 0),
        p(2, 2),
    ];
    let game = Game::from_moves(&moves).unwrap();
    assert_eq!(game.status(), Status::Won(Player::X));
}

#[test]
fn undo_reverts_board_turn_and_history() {
    let mut game = Game::new();
    game.play(p(0, 0)).unwrap();
    game.play(p(1, 1)).unwrap();
    assert_eq!(game.undo(), Ok(p(1, 1)));
    assert_eq!(game.board().get(p(1, 1)), None);
    assert_eq!(game.current_player(), Player::O);
    assert_eq!(game.history(), &[p(0, 0)]);
    assert_eq!(game.undo(), Ok(p(0, 0)));
    assert_eq!(game.current_player(), Player::X);
    assert_eq!(game, Game::new());
}

#[test]
fn undo_on_empty_history_fails() {
    let mut game = Game::new();
    assert_eq!(game.undo(), Err(GameError::NothingToUndo));
}

#[test]
fn undo_reopens_a_finished_game() {
    let mut game = Game::new();
    for pos in [p(0, 0), p(1, 0), p(0, 1), p(1, 1), p(0, 2)] {
        game.play(pos).unwrap();
    }
    assert!(game.is_over());
    game.undo().unwrap();
    assert_eq!(game.status(), Status::InProgress);
    assert_eq!(game.current_player(), Player::X);
    assert_eq!(game.play(p(2, 2)), Ok(Status::InProgress));
}

#[test]
fn from_moves_replays_and_rejects_illegal_sequences() {
    let game = Game::from_moves(&[p(1, 1), p(0, 0)]).unwrap();
    assert_eq!(game.board().get(p(1, 1)), Some(Player::X));
    assert_eq!(game.board().get(p(0, 0)), Some(Player::O));
    assert_eq!(game.current_player(), Player::X);

    assert_eq!(
        Game::from_moves(&[p(1, 1), p(1, 1)]),
        Err(GameError::Board(BoardError::Occupied(p(1, 1))))
    );
    let after_win = [p(0, 0), p(1, 0), p(0, 1), p(1, 1), p(0, 2), p(2, 2)];
    assert_eq!(Game::from_moves(&after_win), Err(GameError::GameOver));
}

#[test]
fn game_error_display_is_readable() {
    assert_eq!(GameError::GameOver.to_string(), "the game is already over");
    assert_eq!(
        GameError::NothingToUndo.to_string(),
        "there is no move to undo"
    );
    assert_eq!(
        GameError::Board(BoardError::OutOfBounds).to_string(),
        "position is outside the 3x3 board"
    );
}
