fn main() {
    println!("Day 1");

    let content = std::fs::read_to_string("./inputs/day3.txt").unwrap();
    let lines = content.split("\n");

    let p1 = lines
        .clone()
        .into_iter()
        .map(|s| find_highest_number(s, 2))
        .sum::<u64>();

    let p2 = lines
        .into_iter()
        .map(|s| find_highest_number(s, 12))
        .sum::<u64>();

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn find_highest_number(bank: &str, i: usize) -> u64 {
    let bank = bank.as_bytes();
    let mut total: u64 = 0;
    let mut at: usize = 0;

    for i in (100 - i + 1)..=100 {
        let imax = argmax(&bank[at..i]);
        let vmax = bank[at + imax];
        at += imax + 1;
        total = total * 10 + ((vmax - 48) as u64);
    }
    total
}

pub fn argmax(bytes: &[u8]) -> usize {
    let mut idx = 0;
    let mut max = bytes[0];
    for (i, &b) in bytes.iter().enumerate().skip(1) {
        if b > max {
            max = b;
            idx = i;
        }
    }

    idx
}
