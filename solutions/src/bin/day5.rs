fn main() {
    println!("Day 5");

    let content = std::fs::read_to_string("./inputs/day5.txt").unwrap();
    let (ranges, ingredients) = content.split_once("\n\n").unwrap();

    let mut ranges = parse_ranges(ranges);
    let ingredients = parse_ingredients(ingredients);

    let p1 = part1(&ranges, &ingredients);
    let p2 = part2(&mut ranges);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn part1(ranges: &Vec<(u64, u64)>, ingredients: &Vec<u64>) -> u64 {
    let mut total = 0;
    for i in ingredients {
        for (left, right) in ranges {
            if left <= i && i <= right {
                total += 1;
                break;
            }
        }
    }
    total
}

fn part2(ranges: &mut Vec<(u64, u64)>) -> u64 {
    ranges.sort();

    let mut total = 0;
    let (mut at_l, mut at_r) = ranges[0];
    for i in 1..ranges.len() {
        let (l, r) = ranges[i];
        if l <= at_r {
            at_r = at_r.max(r);
            continue;
        }

        total += at_r - at_l + 1;
        (at_l, at_r) = (l, r);
    }

    total + at_r - at_l + 1
}

fn parse_ranges(ranges: &str) -> Vec<(u64, u64)> {
    ranges
        .split("\n")
        .map(|l| {
            let (a, b) = l.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn parse_ingredients(ingredients: &str) -> Vec<u64> {
    ingredients
        .split("\n")
        .map(|l| l.parse().unwrap())
        .collect()
}
