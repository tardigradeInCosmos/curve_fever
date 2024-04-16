use mockall::*;
use mockall::predicate::*;

use super::*;

#[automock]
// zamokowac TerminalSize

#[test]
fn board_new() {
    let b = Board::
    assert_eq!(b.w, 10u16, "board width value expected to match 10, got {}", b.w);
    assert_eq!(b.h, 10u16, "board height value expected to match 10, got {}", b.h);
}

