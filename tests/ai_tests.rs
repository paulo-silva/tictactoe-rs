use tictactoe::ai::evaluate;
use tictactoe::{best_move, format_board, parse_moves, Difficulty, Game, Player, Pos, Status};

fn p(row: u8, col: u8) -> Pos {
    Pos::new(row, col).unwrap()
}

#[test]
fn easy_takes_first_free_cell_in_row_major_order() {
    let game = parse_moves("a1,b2").unwrap();
    assert_eq!(best_move(&game, Difficulty::Easy), Some(p(0, 1)));
}

#[test]
fn no_move_when_game_is_over() {
    let game = parse_moves("a1,b1,a2,b2,a3").unwrap();
    assert_eq!(game.status(), Status::Won(Player::X));
    assert_eq!(best_move(&game, Difficulty::Easy), None);
    assert_eq!(best_move(&game, Difficulty::Perfect), None);
}

#[test]
fn perfect_takes_an_immediate_win() {
    // X X .
    // O O .
    // . . .   X to move: a3 wins.
    let game = parse_moves("a1,b1,a2,b2").unwrap();
    assert_eq!(best_move(&game, Difficulty::Perfect), Some(p(0, 2)));
}

#[test]
fn perfect_prefers_winning_over_blocking() {
    // X X .
    // O O .
    // . . .   O to move: O can win at b3 even though X threatens a3.
    let game = parse_moves("a1,b1,a2,b2,c1").unwrap();
    assert_eq!(game.current_player(), Player::O);
    assert_eq!(best_move(&game, Difficulty::Perfect), Some(p(1, 2)));
}

#[test]
fn perfect_blocks_an_immediate_threat() {
    // X . .
    // . X .
    // O . .   O to move: must block c3.
    let game = parse_moves("a1,c1,b2").unwrap();
    assert_eq!(best_move(&game, Difficulty::Perfect), Some(p(2, 2)));
}

#[test]
fn perfect_creates_a_fork_when_available() {
    // X . .
    // . O .
    // . . X   O to move. Any edge is the only drawing reply; a corner loses to a fork.
    let game = parse_moves("a1,b2,c3").unwrap();
    let mv = best_move(&game, Difficulty::Perfect).unwrap();
    let edges = [p(0, 1), p(1, 0), p(1, 2), p(2, 1)];
    assert!(edges.contains(&mv), "expected an edge, got {mv:?}");
}

#[test]
fn perfect_chooses_the_fastest_win() {
    // X . .
    // X O .
    // . O .   X to move: c1 wins now; anything else takes longer or lets O block.
    let game = parse_moves("a1,b2,b1,c2").unwrap();
    assert_eq!(best_move(&game, Difficulty::Perfect), Some(p(2, 0)));
}

#[test]
fn evaluate_scores_forced_outcomes_with_depth() {
    // Immediate win available for X: +10 - 1.
    let game = parse_moves("a1,b1,a2,b2").unwrap();
    assert_eq!(evaluate(&game), 9);

    // O to move, X has two open threats: O loses in 2 plies: -10 + 2.
    // X . X
    // O X .
    // . . O   -> O must block one of a2 / c1, X wins on the other.
    let mut game = Game::new();
    for pos in [p(0, 0), p(1, 0), p(0, 2), p(2, 2), p(1, 1)] {
        game.play(pos).unwrap();
    }
    assert_eq!(game.current_player(), Player::O);
    assert_eq!(evaluate(&game), -8);

    // Finished draw scores 0.
    let game = parse_moves("a1,a2,a3,b2,b1,b3,c2,c1,c3").unwrap();
    assert_eq!(game.status(), Status::Draw);
    assert_eq!(evaluate(&game), 0);
}

#[test]
fn empty_board_is_a_draw_with_perfect_play() {
    assert_eq!(evaluate(&Game::new()), 0);
}

#[test]
fn perfect_ai_never_loses_against_every_first_move() {
    for first in 0..9 {
        let mut game = Game::new();
        game.play(Pos::from_index(first).unwrap()).unwrap();
        let score = evaluate(&game);
        assert!(
            score >= 0,
            "O should not be lost after X plays {first}, got {score}"
        );
    }
}

#[test]
fn perfect_ai_self_play_always_draws() {
    let mut game = Game::new();
    while !game.is_over() {
        let mv = best_move(&game, Difficulty::Perfect).unwrap();
        game.play(mv).unwrap();
    }
    assert_eq!(game.status(), Status::Draw);
}

#[test]
fn tie_breaking_is_lowest_index() {
    // X X .
    // . O O
    // X O .   X to move: a3 (index 2) and b1 (index 3) both win. Lowest index wins.
    let game = parse_moves("a1,b2,a2,b3,c1,c2").unwrap();
    assert_eq!(format_board(game.board()), "XX./.OO/XO.");
    assert_eq!(game.current_player(), Player::X);
    assert_eq!(best_move(&game, Difficulty::Perfect), Some(p(0, 2)));
}
