fn main() {
    println!("Day 6");

    let content = std::fs::read_to_string("./inputs/day6.txt").unwrap();
    let lines = content.split("\n").collect::<Vec<&str>>();

    let matrix = parse_matrix_p1(&lines[..lines.len() - 1]);
    let ops = parse_operators(lines[lines.len() - 1]);

    let p1 = part1(&matrix, &ops);
    let p2 = part2(&lines[..lines.len() - 1], lines[lines.len() - 1]);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn part1(matrix: &Vec<Vec<u64>>, ops: &Vec<Op>) -> u64 {
    let mut total = 0;
    for x in 0..matrix[0].len() {
        let values = matrix.iter().map(|v| v[x]).collect::<Vec<u64>>();
        let op = &ops[x];
        total += values.into_iter().reduce(|a, b| op.apply(a, b)).unwrap();
    }
    total
}

fn part2(lines: &[&str], ops: &str) -> u64 {
    let chars: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let ops = ops.chars().collect::<Vec<char>>();

    let mut total = 0;

    let mut nums = Vec::new();
    for idx in (0..chars[0].len()).rev() {
        let c = chars
            .iter()
            .map(|row| row[idx])
            .filter(|c| c != &' ')
            .collect::<Vec<char>>();

        if c.is_empty() {
            continue;
        }

        let n = c.iter().collect::<String>().parse::<u64>().unwrap();
        nums.push(n);

        match ops[idx] {
            '+' => {
                total += nums.iter().sum::<u64>();
                nums.clear();
            }
            '*' => {
                total += nums.iter().fold(1u64, |acc, &x| acc * x);
                nums.clear();
            }
            _ => (),
        }
    }

    total
}

enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

fn parse_operators(line: &str) -> Vec<Op> {
    line.split_whitespace()
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Unknown value"),
        })
        .collect()
}

fn parse_matrix_p1(lines: &[&str]) -> Vec<Vec<u64>> {
    lines
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}
