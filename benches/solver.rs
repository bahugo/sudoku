use sudoku::{Board, BoardItem};

fn main() {
    divan::main()
}

#[rustfmt::skip]
#[divan::bench]
fn simple_board() {
    let input = Board::new(
        [
            [BoardItem::known(5), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown()],
            [BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(3)],
            [BoardItem::known(4), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1)],
            [BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6)],
            [BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::known(8), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(5)],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::known(9)]
        ],
    );

    let _ = input.solve();
}

#[rustfmt::skip]
#[divan::bench]
fn less_simple_board() {
    let input = Board::new(
        [
            [BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1), BoardItem::known(9), BoardItem::known(5), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::known(9), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown()],
            [BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(3)],
            [BoardItem::known(4), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::known(3), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(1)],
            [BoardItem::known(7), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(6)],
            [BoardItem::unknown(), BoardItem::known(6), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(2), BoardItem::known(8), BoardItem::unknown()],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(4), BoardItem::known(1), BoardItem::known(9), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(5)],
            [BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(8), BoardItem::unknown(), BoardItem::unknown(), BoardItem::known(7), BoardItem::known(9)]
        ],
    );

    let _ = input.solve();
}
