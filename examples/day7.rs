use std::collections::{BTreeMap, BTreeSet};

#[derive(Copy, Clone, PartialEq)]
enum Item {
    Empty,
    Splitter,
    Emitter,
    Beam,
}

struct Grid {
    cells: Vec<Item>,
    mx: usize,
    my: usize,
}

#[derive(Debug)]
struct Beam {
    pos: (usize, usize),
    done: bool,
}

impl Grid {
    fn at(&self, pos: (usize, usize)) -> Item {
        if pos.0 >= self.mx || pos.1 >= self.my {
            Item::Empty
        } else {
            self.cells[(pos.1 * self.mx) + pos.0]
        }
    }

    fn print(&self) {
        for y in 0..self.my {
            let mut str = "".to_string();
            for x in 0..self.mx {
                str += match self.at((x, y)) {
                    Item::Empty => ".",
                    Item::Beam => "|",
                    Item::Splitter => "^",
                    Item::Emitter => "S",
                }
            }
            println!("{str}");
        }
    }
}

fn f(input: &str) -> usize {
    let mut grid = Grid {
        cells: vec![],
        mx: input.lines().next().unwrap().len(),
        my: input.lines().count(),
    };

    let mut beams = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => grid.cells.push(Item::Empty),
                '^' => grid.cells.push(Item::Splitter),
                'S' => {
                    beams.push(Beam { pos: (x, y), done: false });
                    grid.cells.push(Item::Emitter);
                },
                _ => unreachable!(),
            }
        }
    }
    let max_y = input.lines().count();

    let mut split_count = 0;
    while beams.iter().any(|b| !b.done) {
        let mut new: Vec<Beam> = vec![];
        for beam in &beams {
            let new_pos = (beam.pos.0, beam.pos.1 + 1);

            if grid.at(new_pos) == Item::Splitter {
                //println!("split at: {new_pos:?}");
                let new_pos_a = (new_pos.0 - 1, new_pos.1);
                let new_pos_b = (new_pos.0 + 1, new_pos.1);

                let mut has_split = false;
                if new.iter().all(|b| b.pos != new_pos_a) {
                    new.push(Beam { pos: new_pos_a, done: false });
                    has_split = true;
                }
                if new.iter().all(|b| b.pos != new_pos_b) {
                    new.push(Beam { pos: new_pos_b, done: false });
                    has_split = true;
                }

                if has_split {
                    split_count += 1;
                }
            } else {
                new.push(Beam {
                    pos: new_pos,
                    done: beam.pos.1 >= max_y,
                })
            }
        }

        beams = new;
        //println!("{beams:?}");
    }

    split_count
}

fn f2(input: &str) -> usize {
    let mut grid = Grid {
        cells: vec![],
        mx: input.lines().next().unwrap().len(),
        my: input.lines().count(),
    };

    let mut overlaps: BTreeMap<(usize, usize), usize> = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => grid.cells.push(Item::Empty),
                '^' => grid.cells.push(Item::Splitter),
                'S' => {
                    overlaps.insert((x, y), 1);
                    grid.cells.push(Item::Emitter);
                },
                _ => unreachable!(),
            }
        }
    }

    for y in 1..grid.my {
        for x in 1..grid.mx {
            if grid.at((x, y)) == Item::Empty {
                let mut o = *overlaps.get(&(x, y - 1)).unwrap_or(&0);
                if grid.at((x - 1, y)) == Item::Splitter {
                    o += *overlaps.get(&(x - 1, y - 1)).unwrap_or(&0);
                }
                if grid.at((x + 1, y)) == Item::Splitter {
                    o += *overlaps.get(&(x + 1, y - 1)).unwrap_or(&0);
                }
                overlaps.insert((x, y), o);
            }
        }
    }

    let mut sum = 0;
    for x in 1..grid.mx {
        sum += overlaps.get(&(x, grid.my - 1)).unwrap_or(&0);
    }
    sum + 1
}

fn main() {
    println!("{}", f(include_str!("day7.txt")));
    println!("{}", f2(include_str!("day7.txt")));
}

#[test]
fn example() {
    let input = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;
    assert_eq!(f(&input), 21);
    assert_eq!(f2(&input), 40);
}