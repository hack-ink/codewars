/*fn divide(cell: &str) -> (char, u32) {
    let (c, n) = cell.split_at(1);
    (c.chars().next().unwrap(), n.parse::<u32>().unwrap())
}

fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    let mut v = vec![];
    for (c, n) in vec![divide(cell1), divide(cell2)].into_iter() {
        let flag;
        match c {
            'A' | 'C' | 'E' | 'G' => flag = true,
            _ => flag = false
        }
        match n {
            1 | 3 | 5 | 7 => if flag { v.push(flag); } else { v.push(flag); }
            _ => if flag { v.push(!flag); } else { v.push(!flag); }
        }
    }
    if v[0] == v[1] { true } else { false }
}*/

/*fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    let mut c1 = cell1.bytes();
    let mut c2 = cell2.bytes();
    c1.next().unwrap() + c1.next().unwrap() & 1
        == c2.next().unwrap() + c2.next().unwrap() & 1
}*/

fn get_sum(cell: &str) -> u8 {
    cell.chars().map(|c| c as u8).sum()
}

fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    get_sum(cell1) % 2 == get_sum(cell2) % 2
}

#[test]
fn basic_tests() {
    assert_eq!(chessboard_cell_color("A1", "C3"), true);
    assert_eq!(chessboard_cell_color("A1", "H3"), false);
    assert_eq!(chessboard_cell_color("A1", "A2"), false);
    assert_eq!(chessboard_cell_color("A1", "C1"), true);
    assert_eq!(chessboard_cell_color("A1", "A1"), true);
    assert_eq!(chessboard_cell_color("F4", "D4"), true);
    assert_eq!(chessboard_cell_color("F6", "A3"), true);
}