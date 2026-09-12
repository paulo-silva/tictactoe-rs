# tictactoe-rs

A test-driven tic-tac-toe engine in Rust with a deterministic minimax AI and a
native command line front end. No external dependencies.

## Layout

| Module          | Responsibility                                            |
| --------------- | --------------------------------------------------------- |
| `src/board.rs`  | 3x3 grid, `Pos`, placement, win detection, line scanning  |
| `src/game.rs`   | Turn order, `Status`, move history, undo, replay          |
| `src/ai.rs`     | `evaluate` (minimax) and `best_move` with two difficulties |
| `src/notation.rs` | Board text (`X.O/.X./..O`), positions (`b2`), move lists, grid rendering |
| `src/main.rs`   | CLI: interactive play, `--moves` replay, `--best` hint    |

Integration tests live in `tests/` and were written before the implementation.

## Build and run

```sh
cargo test
cargo run -- --help
cargo run -- --moves a1,b2,c3
cargo run -- --best a1,b1,a2,b2
cargo run                       # play X against the perfect AI
cargo run -- --human            # two humans at one keyboard
```

Positions are `a1`..`c3` (row letter, column digit) or `1`..`9` row-major.
Type `undo` during an interactive game to take back the last move.
