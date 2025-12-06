use std::fs;

fn main() {
    println!("Day 1");

    let content = fs::read_to_string("./inputs/day1.txt").unwrap();
    let lines = content.split("\n");

    let (_, p1, p2) = lines.into_iter().fold((50, 0, 0), |(dial, p1, p2), i| {
        let (a, b) = (to_n(i) / 100, to_n(i) % 100);
        let dial_ = dial + b;
        let on_zero = (dial_.rem_euclid(100) == 0) as i32;
        let over_zero = (dial != 0 && (dial_ < 0 || 100 < dial_)) as i32;
        (
            dial_.rem_euclid(100),
            p1 + on_zero,
            p2 + on_zero + over_zero + a.abs(),
        )
    });

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn to_n(s: &str) -> i32 {
    let (a, b) = s.split_at(1);
    let i: i32 = b.parse().unwrap();
    match a {
        "L" => -i,
        _ => i,
    }
}
