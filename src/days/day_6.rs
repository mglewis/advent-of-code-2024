struct Direction {
    x: i64,
    y: i64,
    symbol: char,
}

static DIRECTION_CHARS: [char; 4] = ['^', '>', 'v', '<'];

static DIRECTIONS: [Direction; 4] = [
    Direction {
        x: 0,
        y: -1,
        symbol: '^',
    },
    Direction {
        x: 1,
        y: 0,
        symbol: '>',
    },
    Direction {
        x: 0,
        y: 1,
        symbol: 'v',
    },
    Direction {
        x: -1,
        y: 0,
        symbol: '<',
    },
];

fn build_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start_position(grid: &Vec<Vec<char>>) -> (i64, i64) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return (x as i64, y as i64);
            }
        }
    }
    panic!("No start position found");
}

fn count_visited_cells(grid: &Vec<Vec<char>>) -> u32 {
    grid.iter().fold(0, |acc, row| {
        // check cell is not in DIRECTION_CHARS
        acc + row
            .iter()
            .filter(|&&cell| DIRECTION_CHARS.contains(&cell))
            .count() as u32
    })
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!("\n\n");
}

fn process(grid: &mut Vec<Vec<char>>) -> &Vec<Vec<char>> {
    let mut direction_iter = DIRECTIONS.iter().cycle();

    let mut position = find_start_position(&grid);
    let mut direction = direction_iter.next().unwrap();

    loop {
        let next_position: (i64, i64) = (position.0 + direction.x, position.1 + direction.y);
        let next_position_is_out_of_bounds = next_position.0 < 0
            || next_position.0 >= grid[0].len() as i64
            || next_position.1 < 0
            || next_position.1 >= grid.len() as i64;

        if next_position_is_out_of_bounds {
            return grid;
        }

        let next_position_is_obstacle =
            grid[next_position.1 as usize][next_position.0 as usize] == '#';

        if next_position_is_obstacle {
            direction = direction_iter.next().unwrap();
        } else {
            position = next_position;
            grid[position.1 as usize][next_position.0 as usize] = direction.symbol;
        }
    }
}

pub fn part_a(input: &str) -> u32 {
    let mut grid = build_grid(input);
    process(&mut grid);
    count_visited_cells(&grid)
}

pub fn part_b(input: &str) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2024::read_test_file;

    #[test]
    fn test_part_a() {
        let input = read_test_file(6);
        assert_eq!(part_a(&input), 41);
    }

    #[test]
    fn test_part_b() {
        let input = read_test_file(6);
        assert_eq!(part_b(&input), 1);
    }
}
