type Grid = Vec<Vec<u8>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 4");

    let (mut grid, h, w) = load_grid("./inputs/day4.txt");

    let mut total: i32 = 0;
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == 0 {
                continue;
            }

            let count = count_adjacent(&grid, y, x);

            if count < 4 {
                total += 1;
            }
        }
    }
    println!("Part 1: {total}");

    let mut total = 0;
    loop {
        let total_start = total;
        for y in 0..h {
            for x in 0..w {
                if grid[y][x] == 0 {
                    continue;
                }

                let count = count_adjacent(&grid, y, x);

                if count < 4 {
                    grid[y][x] = 0;
                    total += 1;
                }
            }
        }
        if total == total_start {
            break;
        }
    }
    println!("Part 2: {total}");

    Ok(())
}

fn count_adjacent(g: &Grid, y: usize, x: usize) -> i32 {
    let mut sum = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny >= 0 && nx >= 0 {
                if let Some(row) = g.get(ny as usize) {
                    if let Some(v) = row.get(nx as usize) {
                        sum += *v as i32;
                    }
                }
            }
        }
    }
    sum
}

fn load_grid(path: &str) -> (Grid, usize, usize) {
    let content = std::fs::read_to_string(path).unwrap();
    let lines = content.split("\n");

    let transform = |s: &str| s.bytes().map(|b| (b == b'@') as u8).collect();
    let grid = lines.map(transform).collect::<Vec<Vec<u8>>>();

    let (h, w) = (grid.len(), grid[0].len());

    (grid, h, w)
}
