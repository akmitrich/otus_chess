fn main() {
    let mut cnt = vec![];
    for i in 0..=255_u64 {
        cnt.push(otus_chess::popcnt2(i));
    }
    println!("{cnt:?}");
}
