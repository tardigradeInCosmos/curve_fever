use super::*;

#[test]
fn board_build_with_terminal_size_inaccesible() {
    let mut tsa_mock = MockTerminalSizeAcquisitor::new();
    tsa_mock.expect_get().return_const(None);
    let b = Board::build_with(tsa_mock);
    assert_eq!(
        b.w, 10u16,
        "board width value expected to match 10, got {}",
        b.w
    );
    assert_eq!(
        b.h, 10u16,
        "board height value expected to match 10, got {}",
        b.h
    );
}

#[test]
fn board_build_with_terminal_size_accessible() {
    let mut tsa_mock = MockTerminalSizeAcquisitor::new();
    tsa_mock
        .expect_get()
        .return_const(Some((Width(11), Height(31))));
    let b = Board::build_with(tsa_mock);
    assert_eq!(
        b.w, 11u16,
        "board width value expected to match 11, got {}",
        b.w
    );
    assert_eq!(
        b.h, 31u16,
        "board height value expected to match 31, got {}",
        b.h
    );
}

#[test]
fn board_create_printable_0() {
    let b = Board::new(Some((Width(4), Height(5))));
    let result = b.create_printable();
    let expected = "\
____
|  |
|  |
|__|
";
    assert_eq!(expected, result);
}

#[test]
fn board_create_printable_1() {
    let b = Board::new(Some((Width(10), Height(7))));
    let result = b.create_printable();
    let expected = "\
__________
|        |
|        |
|        |
|        |
|________|
";
    assert_eq!(expected, result);
}
