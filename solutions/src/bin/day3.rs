fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 1");

    let content = std::fs::read_to_string("./inputs/day3_example.txt")?;
    let lines = content.split("\n");

    // for line in lines {}
    // let total = find_highest_number("234234234234278", 2);

    // let p1 = lines
    //     .clone()
    //     .into_iter()
    //     .map(|s| find_highest_number(s, 2))
    //     .sum::<u64>();

    let p2 = lines
        .into_iter()
        .map(|s| find_highest_number(s, 12))
        .sum::<u64>();

    // println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    // 170486901138223 too high

    Ok(())
}

fn find_highest_number(bank: &str, i: usize) -> u64 {
    let bank = bank.as_bytes();
    let mut total: u64 = 0;
    let mut at: usize = 0;

    for i in (15 - i + 1)..=15 {
        // println!("\n");
        let imax = argmax(&bank[at..i]);
        let vmax = bank[at + imax];
        // println!("{:?}[{}] = {} <- {}", &bank[at..i], imax, vmax, vmax - 48);
        at = imax + 1;
        // print!("{}", vmax - 48);
        total = total * 10 + ((vmax - 48) as u64);
    }
    println!("total: {total}");
    total
}

/*
def get(bank, length):
    bank = [int(i) for i in bank]
    at = 0
    maxes = []

    for i in range(101-length, 101):
        l = bank[:i]

        im = np.argmax(l[at:])
        m = l[at+im]
        maxes.append(m)
        at = at+im+1

    return int("".join([str(_) for _ in maxes]))

print("Part 1:", sum([get(inp, 2) for inp in inputs]))
print("Part 2:", sum([get(inp, 12) for inp in inputs]))
*/

pub fn argmax(bytes: &[u8]) -> usize {
    let mut idx = 0;
    let mut max = bytes[0];
    for (i, &b) in bytes.iter().enumerate().skip(1) {
        if b > max {
            max = b;
            idx = i;
            // println!("  new max = {b} @ {i}");
        }
    }

    // println!("[argmax] {:?} {}", bytes, idx);

    idx
}
