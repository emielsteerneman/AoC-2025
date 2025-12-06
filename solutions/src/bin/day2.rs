static PRIMES: [usize; 4] = [3, 5, 7, 11];

fn main() {
    println!("Day 2");

    let content = std::fs::read_to_string("./inputs/day2.txt").unwrap();
    let ranges = content.split(",").collect::<Vec<&str>>();

    let mut durations = Vec::with_capacity(1000);
    let mut answer = (0, 0);

    for _ in 0..500 {
        let start = std::time::Instant::now();
        let total = run(&ranges);
        let stop = std::time::Instant::now();
        let ms = (stop - start).as_millis() as u32;
        durations.push(ms);
        answer = total.unwrap();
    }

    let avg_ms = (durations.iter().sum::<u32>() as f32) / 500.;

    println!("Part 1: {}    Part 2: {}", answer.0, answer.1);
    println!("Average = {:.2}ms", avg_ms);
}

fn run(ranges: &Vec<&str>) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let mut t1 = 0;
    let mut t2 = 0;

    let mut buffer = Vec::<u8>::with_capacity(32);

    for r in ranges {
        let (a, b) = r.split_once("-").unwrap();

        if PRIMES.contains(&a.len()) && PRIMES.contains(&b.len()) {
            continue;
        }

        let (a, b) = (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap());
        for i in a..(b + 1) {
            let (p1, p2) = check(&mut buffer, &i.to_string());
            if p1 {
                t1 += i;
            };
            if p2 {
                t2 += i;
            };
        }
    }

    Ok((t1, t2))
}

fn check(buffer: &mut Vec<u8>, s: &str) -> (bool, bool) {
    // Shaves off like 5ms. Sick
    if PRIMES.contains(&s.len()) {
        return (false, false);
    }

    let b = s.as_bytes();
    // Reusing the buffer saves around 5-10ms
    buffer.truncate(0);
    buffer.extend_from_slice(&b[1..]);
    buffer.extend_from_slice(&b[..b.len() - 1]);
    let p2 = buffer.windows(b.len()).any(|window| window == b);

    // If p2 is true, then p1 might be true as well (two halves being equal)
    let p1 = p2 && {
        let mid = b.len() / 2;
        match s.len() % 2 {
            0 => b[..mid] == b[mid..],
            _ => false,
        }
    };

    (p1, p2)
}
