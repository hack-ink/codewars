fn bishop_and_pawn(bishop: &str, pawn: &str) -> bool {
    let mut b = bishop.bytes();
    let mut p = pawn.bytes();
    (b.next().unwrap() as i8 - p.next().unwrap() as i8).abs()
        == (b.next().unwrap() as i8 - p.next().unwrap() as i8).abs()
}
