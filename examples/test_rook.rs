fn main() {
    println!("{}", include_str!("0.BITS/3.Bitboard - Ладья/problem.txt"));
    otus_chess::tester::run_test("examples/0.BITS/3.Bitboard - Ладья", |data| {
        let pos = data.first().unwrap().parse::<u8>().unwrap();
        let moves = otus_chess::Chess::rook_moves(pos);
        format!("{}\r\n{}", moves.number_of_positions(), moves)
    });
}
