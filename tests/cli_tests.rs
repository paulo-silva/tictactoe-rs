use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(args: &[&str], stdin: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tictactoe"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("binary runs");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).into_owned()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

#[test]
fn help_prints_usage_and_exits_zero() {
    let out = run(&["--help"], "");
    assert!(out.status.success());
    assert!(stdout(&out).contains("--moves"));
    assert!(stdout(&out).contains("--best"));
}

#[test]
fn unknown_flag_exits_two() {
    let out = run(&["--bogus"], "");
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("--bogus"));
}

#[test]
fn moves_prints_grid_and_result() {
    let out = run(&["--moves", "a1,b1,a2,b2,a3"], "");
    assert!(out.status.success(), "{}", stderr(&out));
    let text = stdout(&out);
    assert!(text.contains("a  X | X | X"), "{text}");
    assert!(text.contains("b  O | O | ."), "{text}");
    assert!(text.trim_end().ends_with("X wins"), "{text}");
}

#[test]
fn moves_reports_draw_and_in_progress() {
    let out = run(&["--moves", "a1,a2,a3,b2,b1,b3,c2,c1,c3"], "");
    assert!(stdout(&out).trim_end().ends_with("Draw"));
    let out = run(&["--moves", "a1"], "");
    assert!(stdout(&out).trim_end().ends_with("O to move"));
}

#[test]
fn moves_with_bad_notation_exits_two() {
    let out = run(&["--moves", "a1,q9"], "");
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("invalid position"));
    let out = run(&["--moves", "a1,a1"], "");
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("unreachable"));
}

#[test]
fn best_prints_recommended_move() {
    let out = run(&["--best", "a1,b1,a2,b2"], "");
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(stdout(&out).trim_end(), "a3");
    let out = run(&["--easy", "--best", "a1,b2"], "");
    assert_eq!(stdout(&out).trim_end(), "a2");
}

#[test]
fn flag_where_value_expected_exits_two() {
    let out = run(&["--best", "--easy"], "");
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("--best"));
}

#[test]
fn best_on_finished_game_exits_two() {
    let out = run(&["--best", "a1,b1,a2,b2,a3"], "");
    assert_eq!(out.status.code(), Some(2));
    assert!(stderr(&out).contains("over"));
}

#[test]
fn interactive_human_game_plays_to_a_win() {
    let out = run(&["--human"], "a1\nb1\na2\nb2\na3\n");
    assert!(out.status.success(), "{}", stderr(&out));
    let text = stdout(&out);
    assert!(text.contains("X wins"), "{text}");
}

#[test]
fn interactive_rejects_bad_input_and_keeps_going() {
    let out = run(&["--human"], "zz\na1\na1\nb1\na2\nb2\na3\n");
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("invalid position"), "{text}");
    assert!(text.contains("already occupied"), "{text}");
    assert!(text.contains("X wins"), "{text}");
}

#[test]
fn interactive_undo_command_reverts_last_move() {
    // X plays a1, O plays b2, undo removes b2, then O plays b1 instead.
    let out = run(&["--human"], "a1\nb2\nundo\nb1\na2\nb2\na3\n");
    assert!(out.status.success());
    let text = stdout(&out);
    assert!(text.contains("X wins"), "{text}");
    assert!(text.contains("undo"), "{text}");
}

#[test]
fn interactive_against_ai_ends_on_eof_without_panicking() {
    let out = run(&[], "b2\n");
    assert!(out.status.success(), "{}", stderr(&out));
    assert!(stdout(&out).contains("O plays"));
}

#[test]
fn ai_as_x_moves_first() {
    let out = run(&["--ai", "X"], "");
    assert!(out.status.success(), "{}", stderr(&out));
    assert!(stdout(&out).contains("X plays a1"));
}
