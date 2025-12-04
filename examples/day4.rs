#[derive(Copy, Clone, PartialEq)]
enum Item {
    Empty,
    Roll,
}

struct Grid {
    cells: Vec<Item>,
    mx: i32,
    my: i32,
}

impl Grid {
    fn at(&self, x: i32, y: i32) -> Item {
        if x < 0 || y < 0 || x >= self.mx || y >= self.my {
            Item::Empty
        } else {
            self.cells[((y * self.mx) + x) as usize]
        }
    }

    fn num_surrounding_rolls(&self, x: i32, y: i32) -> usize {
        let mut cnt = 0;
        if self.at(x - 1, y - 1) == Item::Roll { cnt += 1; }
        if self.at(x - 0, y - 1) == Item::Roll { cnt += 1; }
        if self.at(x + 1, y - 1) == Item::Roll { cnt += 1; }
        if self.at(x - 1, y - 0) == Item::Roll { cnt += 1; }
        //if self.at(x - 0, y - 0) == Item::Roll { cnt += 1; }
        if self.at(x + 1, y - 0) == Item::Roll { cnt += 1; }
        if self.at(x - 1, y + 1) == Item::Roll { cnt += 1; }
        if self.at(x - 0, y + 1) == Item::Roll { cnt += 1; }
        if self.at(x + 1, y + 1) == Item::Roll { cnt += 1; }
        cnt
    }
}

fn f(input: &str) -> usize {
    let mut grid = Grid {
        cells: vec![],
        mx: input.lines().next().unwrap().len() as i32,
        my: input.lines().count() as i32,
    };

    for line in input.lines() {
        for char in line.chars() {
            grid.cells.push(match char {
                '.' => Item::Empty,
                '@' => Item::Roll,
                _ => unreachable!(),
            });
        }
    }

    let mut cnt = 0usize;
    for y in 0..grid.my {
        for x in 0..grid.mx {
            if grid.at(x, y) == Item::Roll && grid.num_surrounding_rolls(x, y) < 4 {
                cnt += 1;
            }
        }
    }

    cnt
}

fn f2(input: &str) -> usize {
    let mut grid = Grid {
        cells: vec![],
        mx: input.lines().next().unwrap().len() as i32,
        my: input.lines().count() as i32,
    };

    for line in input.lines() {
        for char in line.chars() {
            grid.cells.push(match char {
                '.' => Item::Empty,
                '@' => Item::Roll,
                _ => unreachable!(),
            });
        }
    }

    let mut cnt = 0;
    loop {
        let mut to_remove = vec![];
        for y in 0..grid.my {
            for x in 0..grid.mx {
                if grid.at(x, y) == Item::Roll && grid.num_surrounding_rolls(x, y) < 4 {
                    cnt += 1;
                    to_remove.push((x, y));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        } else {
            for (x, y) in to_remove {
                grid.cells[((y * grid.mx) + x) as usize] = Item::Empty;
            }
        }
    }

    cnt
}

fn main() {
    println!("{}", f(include_str!("day4.txt")));
    println!("{}", f2(include_str!("day4.txt")));
}

#[test]
fn example() {
    let input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
    assert_eq!(f(input), 13);
    assert_eq!(f2(input), 43);
}